//! Defines the [PdfPageObject] enum, exposing functionality related to a single page object.

use crate::bindgen::{
    FPDF_PAGEOBJECT, FPDF_PAGEOBJ_FORM, FPDF_PAGEOBJ_IMAGE, FPDF_PAGEOBJ_PATH,
    FPDF_PAGEOBJ_SHADING, FPDF_PAGEOBJ_TEXT, FPDF_PAGEOBJ_UNKNOWN,
};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::page::{PdfPage, PdfRect};
use crate::page_object::internal::PdfPageObjectPrivate;
use crate::page_object_form_fragment::PdfPageFormFragmentObject;
use crate::page_object_image::PdfPageImageObject;
use crate::page_object_path::PdfPagePathObject;
use crate::page_object_shading::PdfPageShadingObject;
use crate::page_object_text::PdfPageTextObject;
use crate::page_object_unsupported::PdfPageUnsupportedObject;
use crate::page_objects::PdfPageObjectIndex;

/// The type of a single [PdfPageObject].
///
/// Note that Pdfium does not support or recognize all PDF page object types. For instance,
/// Pdfium does not currently support or recognize the External Object ("XObject") page object
/// type supported by Adobe Acrobat and Foxit's commercial PDF SDK. In these cases, Pdfium
/// will return `PdfPageObjectType::Unsupported`.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum PdfPageObjectType {
    Unsupported = FPDF_PAGEOBJ_UNKNOWN as isize,
    Text = FPDF_PAGEOBJ_TEXT as isize,
    Path = FPDF_PAGEOBJ_PATH as isize,
    Image = FPDF_PAGEOBJ_IMAGE as isize,
    Shading = FPDF_PAGEOBJ_SHADING as isize,
    FormFragment = FPDF_PAGEOBJ_FORM as isize,
}

impl PdfPageObjectType {
    pub(crate) fn from_pdfium(value: u32) -> Result<PdfPageObjectType, PdfiumError> {
        match value {
            FPDF_PAGEOBJ_UNKNOWN => Ok(PdfPageObjectType::Unsupported),
            FPDF_PAGEOBJ_TEXT => Ok(PdfPageObjectType::Text),
            FPDF_PAGEOBJ_PATH => Ok(PdfPageObjectType::Path),
            FPDF_PAGEOBJ_IMAGE => Ok(PdfPageObjectType::Image),
            FPDF_PAGEOBJ_SHADING => Ok(PdfPageObjectType::Shading),
            FPDF_PAGEOBJ_FORM => Ok(PdfPageObjectType::FormFragment),
            _ => Err(PdfiumError::UnknownPdfPageObjectType),
        }
    }
}

pub enum PdfPageObject<'a> {
    Text(PdfPageTextObject<'a>),
    Path(PdfPagePathObject<'a>),
    Image(PdfPageImageObject<'a>),
    Shading(PdfPageShadingObject<'a>),
    FormFragment(PdfPageFormFragmentObject<'a>),

    /// Common properties shared by all [PdfPageObject] types can still be accessed for
    /// page objects not recognized by Pdfium, but object-specific functionality
    /// will be unavailable.
    Unsupported(PdfPageUnsupportedObject<'a>),
}

impl<'a> PdfPageObject<'a> {
    pub(crate) fn from_pdfium(
        index: PdfPageObjectIndex,
        handle: FPDF_PAGEOBJECT,
        page: &'a PdfPage<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        match PdfPageObjectType::from_pdfium(bindings.FPDFPageObj_GetType(handle) as u32)
            .unwrap_or(PdfPageObjectType::Unsupported)
        {
            PdfPageObjectType::Unsupported => PdfPageObject::Unsupported(
                PdfPageUnsupportedObject::from_pdfium(index, handle, bindings),
            ),
            PdfPageObjectType::Text => PdfPageObject::Text(PdfPageTextObject::from_pdfium(
                index, handle, page, bindings,
            )),
            PdfPageObjectType::Path => {
                PdfPageObject::Path(PdfPagePathObject::from_pdfium(index, handle, bindings))
            }
            PdfPageObjectType::Image => {
                PdfPageObject::Image(PdfPageImageObject::from_pdfium(index, handle, bindings))
            }
            PdfPageObjectType::Shading => {
                PdfPageObject::Shading(PdfPageShadingObject::from_pdfium(index, handle, bindings))
            }
            PdfPageObjectType::FormFragment => PdfPageObject::FormFragment(
                PdfPageFormFragmentObject::from_pdfium(index, handle, bindings),
            ),
        }
    }

    #[inline]
    pub(crate) fn unwrap_as_trait(&self) -> &dyn PdfPageObjectPrivate {
        match self {
            PdfPageObject::Text(object) => object,
            PdfPageObject::Path(object) => object,
            PdfPageObject::Image(object) => object,
            PdfPageObject::Shading(object) => object,
            PdfPageObject::FormFragment(object) => object,
            PdfPageObject::Unsupported(object) => object,
        }
    }

    /// The object type of this [PdfPageObject].
    ///
    /// Note that Pdfium does not support or recognize all PDF page object types. For instance,
    /// Pdfium does not currently support or recognize the External Object ("XObject") page object
    /// type supported by Adobe Acrobat and Foxit's commercial PDF SDK. In these cases, Pdfium
    /// will return `PdfPageObjectType::Unsupported`.
    #[inline]
    pub fn object_type(&self) -> PdfPageObjectType {
        match self {
            PdfPageObject::Text(_) => PdfPageObjectType::Text,
            PdfPageObject::Path(_) => PdfPageObjectType::Path,
            PdfPageObject::Image(_) => PdfPageObjectType::Image,
            PdfPageObject::Shading(_) => PdfPageObjectType::Shading,
            PdfPageObject::FormFragment(_) => PdfPageObjectType::FormFragment,
            PdfPageObject::Unsupported(_) => PdfPageObjectType::Unsupported,
        }
    }

    /// Returns `true` if this [PdfPageObject] has an object type of [PdfPageObjectType::Unsupported].
    ///
    /// Common properties shared by all [PdfPageObject] types can still be accessed for
    /// page objects not recognized by Pdfium, but object-specific functionality
    /// will be unavailable.
    #[inline]
    pub fn is_unsupported(&self) -> bool {
        self.object_type() == PdfPageObjectType::Unsupported
    }

    /// Returns the underlying [PdfPageTextObject] for this [PdfPageObject], if this page object
    /// has an object type of [PdfPageObjectType::Text].
    #[inline]
    pub fn as_text_object(&self) -> Option<&PdfPageTextObject> {
        match self {
            PdfPageObject::Text(object) => Some(object),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPagePathObject] for this [PdfPageObject], if this page object
    /// has an object type of [PdfPageObjectType::Path].
    #[inline]
    pub fn as_path_object(&self) -> Option<&PdfPagePathObject> {
        match self {
            PdfPageObject::Path(object) => Some(object),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageImageObject] for this [PdfPageObject], if this page object
    /// has an object type of [PdfPageObjectType::Image].
    #[inline]
    pub fn as_image_object(&self) -> Option<&PdfPageImageObject> {
        match self {
            PdfPageObject::Image(object) => Some(object),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageShadingObject] for this [PdfPageObject], if this page object
    /// has an object type of [PdfPageObjectType::Shading].
    #[inline]
    pub fn as_shading_object(&self) -> Option<&PdfPageShadingObject> {
        match self {
            PdfPageObject::Shading(object) => Some(object),
            _ => None,
        }
    }

    /// Returns the underlying [PdfPageFormFragmentObject] for this [PdfPageObject], if this page object
    /// has an object type of [PdfPageObjectType::FormFragment].
    #[inline]
    pub fn as_form_fragment_object(&self) -> Option<&PdfPageFormFragmentObject> {
        match self {
            PdfPageObject::FormFragment(object) => Some(object),
            _ => None,
        }
    }
}

/// Functionality common to all [PdfPageObject] objects, regardless of their [PdfPageObjectType].
pub trait PdfPageObjectCommon<'a> {
    /// Returns the zero-based object index of this [PdfPageObject] in its containing [PdfPage].
    fn index(&self) -> PdfPageObjectIndex;

    /// Returns `true` if this [PdfPageObject] contains transparency.
    fn has_transparency(&self) -> bool;

    /// Returns the bounding box of this [PdfPageObject].
    fn bounding(&self) -> Result<PdfRect, PdfiumError>;
}

pub(crate) mod internal {
    // We want to make the PdfPageObjectPrivate trait private while providing a blanket
    // implementation of PdfPageObjectCommon for any type T where T: PdfPageObjectPrivate.
    // Rust complains, however, that by doing so we are leaking the private trait outside
    // the crate.

    // Instead of making the PdfPageObjectPrivate trait private, we leave it public but place it
    // inside this pub(crate) module in order to prevent it from being visible outside the crate.

    use crate::bindgen::{FPDF_PAGEOBJECT, FS_RECTF};
    use crate::bindings::PdfiumLibraryBindings;
    use crate::error::PdfiumError;
    use crate::page::PdfRect;
    use crate::page_objects::PdfPageObjectIndex;
    use std::os::raw::c_float;

    /// Internal crate-specific functionality common to all [PdfPageObject] objects.
    pub trait PdfPageObjectPrivate<'a>: super::PdfPageObjectCommon<'a> {
        /// Returns the internal FPDF_PAGEOBJECT handle for this [PdfPageObject].
        fn get_handle(&self) -> &FPDF_PAGEOBJECT;

        /// Internal implementation of [PdfPageObjectCommon::index()].
        fn index_impl(&self) -> PdfPageObjectIndex;

        /// Returns `true` if the memory allocated to this [PdfPageObject] is owned by a containing
        /// [PdfPage]. Page objects that are contained within a [PdfPage] do not require their
        /// data buffers to be de-allocated when references to them are dropped. Returns `false`
        /// for a [PdfPageObject] that has been created programmatically but not yet added to an
        /// existing [PdfPage].
        fn is_object_memory_owned_by_page(&self) -> bool;

        fn get_bindings(&self) -> &dyn PdfiumLibraryBindings;

        /// Internal implementation of [PdfPageObjectCommon::has_transparency()].
        #[inline]
        fn has_transparency_impl(&self) -> bool {
            let bindings = self.get_bindings();

            bindings.is_true(bindings.FPDFPageObj_HasTransparency(*self.get_handle()))
        }

        /// Internal implementation of [PdfPageObjectCommon::bounding()].
        #[inline]
        fn bounding_impl(&self) -> Result<PdfRect, PdfiumError> {
            // Clippy doesn't want us to cast to c_float because c_float == f32 in the
            // development environment, but we don't want to assume that will be the case
            // on every target architecture.
            #[allow(clippy::unnecessary_cast)]
            let mut left = 0.0 as c_float;

            #[allow(clippy::unnecessary_cast)]
            let mut bottom = 0.0 as c_float;

            #[allow(clippy::unnecessary_cast)]
            let mut right = 0.0 as c_float;

            #[allow(clippy::unnecessary_cast)]
            let mut top = 0.0 as c_float;

            let result = self.get_bindings().FPDFPageObj_GetBounds(
                *self.get_handle(),
                &mut left,
                &mut bottom,
                &mut right,
                &mut top,
            );

            PdfRect::from_pdfium_as_result(
                result,
                FS_RECTF {
                    left,
                    top,
                    right,
                    bottom,
                },
                self.get_bindings(),
            )
        }
    }
}

// Blanket implementation for all PdfPageObject types.

impl<'a, T> PdfPageObjectCommon<'a> for T
where
    T: internal::PdfPageObjectPrivate<'a>,
{
    #[inline]
    fn index(&self) -> PdfPageObjectIndex {
        self.index_impl()
    }

    #[inline]
    fn has_transparency(&self) -> bool {
        self.has_transparency_impl()
    }

    #[inline]
    fn bounding(&self) -> Result<PdfRect, PdfiumError> {
        self.bounding_impl()
    }
}

impl<'a> internal::PdfPageObjectPrivate<'a> for PdfPageObject<'a> {
    #[inline]
    fn get_handle(&self) -> &FPDF_PAGEOBJECT {
        self.unwrap_as_trait().get_handle()
    }

    #[inline]
    fn index_impl(&self) -> PdfPageObjectIndex {
        self.unwrap_as_trait().index_impl()
    }

    #[inline]
    fn is_object_memory_owned_by_page(&self) -> bool {
        self.unwrap_as_trait().is_object_memory_owned_by_page()
    }

    #[inline]
    fn get_bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.unwrap_as_trait().get_bindings()
    }
}

impl<'a> Drop for PdfPageObject<'a> {
    /// Closes the [PdfPageObject], releasing held memory.
    #[inline]
    fn drop(&mut self) {
        // The documentation for FPDFPageObj_Destroy() states that we only need
        // call the function for page objects created by FPDFPageObj_CreateNew*() or
        // FPDFPageObj_New*Obj() _and_ where the newly-created object was _not_ subsequently
        // added to a PdfPage via a call to FPDFPage_InsertObject() or FPDFAnnot_AppendObject().
        // In other words, retrieving a page object that already exists in a document evidently
        // does not allocate any additional resources, so we don't need to free anything.
        // (Indeed, if we try to, Pdfium segfaults.)

        if !self.is_object_memory_owned_by_page() {
            let object = self.unwrap_as_trait();

            object
                .get_bindings()
                .FPDFPageObj_Destroy(*object.get_handle());
        }
    }
}
