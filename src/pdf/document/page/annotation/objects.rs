//! Defines the [PdfPageAnnotationObjects] struct, exposing functionality related to the
//! page objects contained within a single `PdfPageAnnotation`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::page::object::private::internal::PdfPageObjectPrivate;
use crate::pdf::document::page::object::PdfPageObject;
use crate::pdf::document::page::objects::common::{
    PdfPageObjectIndex, PdfPageObjectsCommon, PdfPageObjectsIterator,
};
use crate::pdf::document::page::objects::private::internal::PdfPageObjectsPrivate;
use std::os::raw::c_int;

/// The page objects contained within a single `PdfPageAnnotation`.
///
/// Content in an annotation is structured as a stream of [PdfPageObject] objects of different types:
/// text objects, image objects, path objects, and so on.
///
/// Note that Pdfium does not support or recognize all PDF page object types. For instance,
/// Pdfium does not currently support or recognize the External Object ("XObject") page object type
/// supported by Adobe Acrobat and Foxit's commercial PDF SDK. In these cases, Pdfium will return
/// `PdfPageObjectType::Unsupported`.
///
/// Page objects can be retrieved from any type of `PdfPageAnnotation`, but Pdfium currently
/// only permits adding new page objects to, or removing existing page objects from, annotations
/// of types `PdfPageAnnotationType::Ink` and `PdfPageAnnotationType::Stamp`. All other annotation
/// types are read-only.
pub struct PdfPageAnnotationObjects<'a> {
    document_handle: FPDF_DOCUMENT,
    page_handle: FPDF_PAGE,
    annotation_handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
    do_regenerate_page_content_after_each_change: bool,
}

impl<'a> PdfPageAnnotationObjects<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        Self {
            document_handle,
            page_handle,
            annotation_handle,
            bindings,
            do_regenerate_page_content_after_each_change: false,
        }
    }

    /// Returns the internal `FPDF_ANNOTATION` handle for the [PdfPageAnnotation] containing
    /// this [PdfPageAnnotationObjects] collection.
    #[inline]
    pub(crate) fn get_annotation_handle(&self) -> &FPDF_ANNOTATION {
        &self.annotation_handle
    }

    /// Sets whether or not this [PdfPageAnnotationObjects] collection should trigger
    /// content regeneration on its containing [PdfPage] when the collection is mutated.
    #[inline]
    pub(crate) fn do_regenerate_page_content_after_each_change(
        &mut self,
        do_regenerate_page_content_after_each_change: bool,
    ) {
        self.do_regenerate_page_content_after_each_change =
            do_regenerate_page_content_after_each_change;
    }
}

impl<'a> PdfPageObjectsPrivate<'a> for PdfPageAnnotationObjects<'a> {
    #[inline]
    fn document_handle(&self) -> FPDF_DOCUMENT {
        self.document_handle
    }

    #[inline]
    fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn len_impl(&self) -> PdfPageObjectIndex {
        self.bindings
            .FPDFAnnot_GetObjectCount(self.annotation_handle) as PdfPageObjectIndex
    }

    fn get_impl(&self, index: PdfPageObjectIndex) -> Result<PdfPageObject<'a>, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::PageObjectIndexOutOfBounds);
        }

        let object_handle = self
            .bindings
            .FPDFAnnot_GetObject(self.annotation_handle, index as c_int);

        if object_handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfPageObject::from_pdfium(
                object_handle,
                None,
                Some(self.annotation_handle),
                self.bindings,
            ))
        }
    }

    #[inline]
    fn iter_impl(&'a self) -> PdfPageObjectsIterator<'a> {
        PdfPageObjectsIterator::new(self)
    }

    fn add_object_impl(
        &mut self,
        mut object: PdfPageObject<'a>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        object.add_object_to_annotation(self).and_then(|_| {
            if self.do_regenerate_page_content_after_each_change {
                if self
                    .bindings
                    .is_true(self.bindings.FPDFPage_GenerateContent(self.page_handle))
                {
                    Ok(object)
                } else {
                    Err(PdfiumError::PdfiumLibraryInternalError(
                        PdfiumInternalError::Unknown,
                    ))
                }
            } else {
                Ok(object)
            }
        })
    }

    fn remove_object_impl(
        &mut self,
        mut object: PdfPageObject<'a>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        object.remove_object_from_annotation().and_then(|_| {
            if self.do_regenerate_page_content_after_each_change {
                if self
                    .bindings
                    .is_true(self.bindings.FPDFPage_GenerateContent(self.page_handle))
                {
                    Ok(object)
                } else {
                    Err(PdfiumError::PdfiumLibraryInternalError(
                        PdfiumInternalError::Unknown,
                    ))
                }
            } else {
                Ok(object)
            }
        })
    }
}
