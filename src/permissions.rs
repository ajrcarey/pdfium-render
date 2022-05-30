//! Defines the [PdfPermissions] collection, containing information on the permissions
//! and security handlers set for a single [PdfDocument].

use crate::bindings::PdfiumLibraryBindings;
use crate::document::PdfDocument;
use crate::error::PdfiumError;
use bitflags::bitflags;
use std::os::raw::c_int;

bitflags! {
    struct FpdfPermissions: u32 {
        const RESERVED_BIT_1 =                          0b100000000000;
        const RESERVED_BIT_2 =                          0b010000000000;
        const CAN_PRINT_BIT_3 =                         0b001000000000;
        const CAN_MODIFY_BIT_4 =                        0b000100000000;
        const CAN_EXTRACT_TEXT_AND_GRAPHICS_BIT_5 =     0b000010000000;
        const CAN_ANNOTATE_AND_FORM_FILL_BIT_6 =        0b000001000000;
        const RESERVED_BIT_7 =                          0b000000100000;
        const RESERVED_BIT_8 =                          0b000000010000;
        const V3_CAN_FORM_FILL_BIT_9 =                  0b000000001000;
        const V3_CAN_EXTRACT_TEXT_AND_GRAPHICS_BIT_10 = 0b000000000100;
        const V3_CAN_ASSEMBLE_DOCUMENT_BIT_11 =         0b000000000010;
        const V3_CAN_PRINT_HIGH_QUALITY_BIT_12 =        0b000000000001;
    }
}

/// The revision of the standard security handler for a single [PdfDocument].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfSecurityHandlerRevision {
    Unprotected,
    Revision2,
    Revision3,
    Revision4,
}

impl PdfSecurityHandlerRevision {
    pub(crate) fn from_pdfium(value: c_int) -> Option<Self> {
        match value {
            -1 => Some(PdfSecurityHandlerRevision::Unprotected),
            2 => Some(PdfSecurityHandlerRevision::Revision2),
            3 => Some(PdfSecurityHandlerRevision::Revision3),
            4 => Some(PdfSecurityHandlerRevision::Revision4),
            _ => None,
        }
    }
}

/// The collection of document permissions and security handler settings for a single [PdfDocument].
///
/// Please note that Pdfium currently only offers support for reading the existing of a
/// document. It does not support changing existing permissions or setting new permissions on
/// a newly created document.
pub struct PdfPermissions<'a> {
    document: &'a PdfDocument<'a>,
}

impl<'a> PdfPermissions<'a> {
    /// Creates a new [PdfPermissions] collection from the given [PdfDocument].
    #[inline]
    pub(crate) fn new(document: &'a PdfDocument<'a>) -> Self {
        Self { document }
    }

    /// Returns a reference to the [PdfDocument] that contains this [PdfPermissions] collection.
    #[inline]
    pub(crate) fn get_document(&self) -> &PdfDocument {
        self.document
    }

    /// Returns the [PdfiumLibraryBindings] used by the containing [PdfDocument].
    #[inline]
    pub(crate) fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.get_document().get_bindings()
    }

    /// Returns the raw permissions bitflags for the containing [PdfDocument].
    #[inline]
    fn get_permissions_bits(&self) -> FpdfPermissions {
        FpdfPermissions::from_bits_truncate(
            self.get_bindings()
                .FPDF_GetDocPermissions(*self.get_document().get_handle()) as u32,
        )
    }

    /// Returns the revision of the standard security handler used by the containing [PdfDocument].
    /// As of PDF version 1.7, possible revision numbers are 2, 3, or 4.
    pub fn security_handler_revision(&self) -> Result<PdfSecurityHandlerRevision, PdfiumError> {
        PdfSecurityHandlerRevision::from_pdfium(
            self.get_bindings()
                .FPDF_GetSecurityHandlerRevision(*self.document.get_handle()),
        )
        .ok_or(PdfiumError::UnknownPdfSecurityHandlerRevision)
    }

    /// Returns `true` if the containing [PdfDocument] can be printed to a representation
    /// from which a faithful digital copy of the original content could be recovered.
    pub fn can_print_high_quality(&self) -> Result<bool, PdfiumError> {
        let permissions = self.get_permissions_bits();

        let result = match self.security_handler_revision()? {
            PdfSecurityHandlerRevision::Unprotected => true,
            PdfSecurityHandlerRevision::Revision2 => {
                permissions.contains(FpdfPermissions::CAN_PRINT_BIT_3)
            }
            PdfSecurityHandlerRevision::Revision3 | PdfSecurityHandlerRevision::Revision4 => {
                permissions.contains(FpdfPermissions::CAN_PRINT_BIT_3)
                    && permissions.contains(FpdfPermissions::V3_CAN_PRINT_HIGH_QUALITY_BIT_12)
            }
        };

        Ok(result)
    }

    /// Returns `true` if the containing [PdfDocument] can be only be printed to a low-level
    /// representation of the appearance of the document, possibly of degraded quality,
    /// from which a faithful digital copy of the original content could _not_ be recovered.
    pub fn can_print_only_low_quality(&self) -> Result<bool, PdfiumError> {
        let permissions = self.get_permissions_bits();

        let result = match self.security_handler_revision()? {
            PdfSecurityHandlerRevision::Unprotected | PdfSecurityHandlerRevision::Revision2 => {
                false
            }
            PdfSecurityHandlerRevision::Revision3 | PdfSecurityHandlerRevision::Revision4 => {
                permissions.contains(FpdfPermissions::CAN_PRINT_BIT_3)
                    && !permissions.contains(FpdfPermissions::V3_CAN_PRINT_HIGH_QUALITY_BIT_12)
            }
        };

        Ok(result)
    }

    /// Returns `true` if the containing [PdfDocument] can be "assembled"; that is, the
    /// document can have pages inserted, rotated, or deleted, can have bookmarks created,
    /// or can have thumbnail page images created.
    pub fn can_assemble_document(&self) -> Result<bool, PdfiumError> {
        let permissions = self.get_permissions_bits();

        let result = match self.security_handler_revision()? {
            PdfSecurityHandlerRevision::Unprotected => true,
            PdfSecurityHandlerRevision::Revision2 => {
                permissions.contains(FpdfPermissions::CAN_MODIFY_BIT_4)
            }
            PdfSecurityHandlerRevision::Revision3 | PdfSecurityHandlerRevision::Revision4 => {
                permissions.contains(FpdfPermissions::V3_CAN_ASSEMBLE_DOCUMENT_BIT_11)
            }
        };

        Ok(result)
    }

    /// Returns `true` if the containing [PdfDocument] allows general modification of
    /// the document contents.
    ///
    /// For security handler revisions 3 and later, general document modification can be disabled
    /// while still permitting selected modification of annotations and interactive form fields.
    pub fn can_modify_document_content(&self) -> Result<bool, PdfiumError> {
        let permissions = self.get_permissions_bits();

        let result = match self.security_handler_revision()? {
            PdfSecurityHandlerRevision::Unprotected => true,
            _ => permissions.contains(FpdfPermissions::CAN_MODIFY_BIT_4),
        };

        Ok(result)
    }

    /// Returns `true` if the containing [PdfDocument] permits text and graphics to be extracted.
    pub fn can_extract_text_and_graphics(&self) -> Result<bool, PdfiumError> {
        let permissions = self.get_permissions_bits();

        let result = match self.security_handler_revision()? {
            PdfSecurityHandlerRevision::Unprotected => true,
            PdfSecurityHandlerRevision::Revision2 => {
                permissions.contains(FpdfPermissions::CAN_EXTRACT_TEXT_AND_GRAPHICS_BIT_5)
            }
            // TODO: AJRC - 27/5/22 - what operations are permitted by bit 10 but prevented by bit 5?
            PdfSecurityHandlerRevision::Revision3 | PdfSecurityHandlerRevision::Revision4 => {
                permissions.contains(FpdfPermissions::V3_CAN_EXTRACT_TEXT_AND_GRAPHICS_BIT_10)
            }
        };

        Ok(result)
    }

    /// Returns `true` if the containing [PdfDocument] permits any existing form fields,
    /// including signature fields, to be filled in by a user.
    pub fn can_fill_existing_interactive_form_fields(&self) -> Result<bool, PdfiumError> {
        let permissions = self.get_permissions_bits();

        let result = match self.security_handler_revision()? {
            PdfSecurityHandlerRevision::Unprotected => true,
            PdfSecurityHandlerRevision::Revision2 => {
                permissions.contains(FpdfPermissions::CAN_ANNOTATE_AND_FORM_FILL_BIT_6)
            }
            PdfSecurityHandlerRevision::Revision3 | PdfSecurityHandlerRevision::Revision4 => {
                permissions.contains(FpdfPermissions::V3_CAN_FORM_FILL_BIT_9)
            }
        };

        Ok(result)
    }

    /// Returns `true` if the containing [PdfDocument] allows the creation of new form fields,
    /// including new signature fields.
    pub fn can_create_new_interactive_form_fields(&self) -> Result<bool, PdfiumError> {
        let permissions = self.get_permissions_bits();

        let result = match self.security_handler_revision()? {
            PdfSecurityHandlerRevision::Unprotected => true,
            _ => {
                permissions.contains(FpdfPermissions::CAN_MODIFY_BIT_4)
                    && permissions.contains(FpdfPermissions::CAN_ANNOTATE_AND_FORM_FILL_BIT_6)
            }
        };

        Ok(result)
    }

    /// Returns `true` if the containing [PdfDocument] allows the addition or modification
    /// of text annotations.
    pub fn can_add_or_modify_text_annotations(&self) -> Result<bool, PdfiumError> {
        let permissions = self.get_permissions_bits();

        let result = match self.security_handler_revision()? {
            PdfSecurityHandlerRevision::Unprotected => true,
            _ => permissions.contains(FpdfPermissions::CAN_ANNOTATE_AND_FORM_FILL_BIT_6),
        };

        Ok(result)
    }
}
