//! Defines the [PdfPageUnsupportedObject] struct, exposing functionality related to a single
//! page object of type [PdfPageObjectType::Unsupported].

use crate::bindgen::FPDF_PAGEOBJECT;
use crate::error::PdfiumError;
use crate::pdf::document::page::object::private::internal::PdfPageObjectPrivate;
use crate::pdf::document::page::object::PdfPageObjectOwnership;
use crate::pdf::matrix::{PdfMatrix, PdfMatrixValue};
use crate::pdf::points::PdfPoints;
use crate::pdfium::PdfiumLibraryBindingsAccessor;
use crate::{create_transform_getters, create_transform_setters};
use std::marker::PhantomData;

#[cfg(doc)]
use {crate::pdf::document::page::object::PdfPageObjectType, crate::pdf::document::page::PdfPage};

/// A single [PdfPageObject] of any object type not supported by Pdfium.
pub struct PdfPageUnsupportedObject<'a> {
    object_handle: FPDF_PAGEOBJECT,
    ownership: PdfPageObjectOwnership,
    lifetime: PhantomData<&'a FPDF_PAGEOBJECT>,
}

impl<'a> PdfPageUnsupportedObject<'a> {
    pub(crate) fn from_pdfium(
        object_handle: FPDF_PAGEOBJECT,
        ownership: PdfPageObjectOwnership,
    ) -> Self {
        PdfPageUnsupportedObject {
            object_handle,
            ownership,
            lifetime: PhantomData,
        }
    }

    create_transform_setters!(
        &mut Self,
        Result<(), PdfiumError>,
        "this [PdfPageUnsupportedObject]",
        "this [PdfPageUnsupportedObject].",
        "this [PdfPageUnsupportedObject],"
    );

    // The transform_impl() function required by the create_transform_setters!() macro
    // is provided by the PdfPageObjectPrivate trait.

    create_transform_getters!(
        "this [PdfPageUnsupportedObject]",
        "this [PdfPageUnsupportedObject].",
        "this [PdfPageUnsupportedObject],"
    );

    // The get_matrix_impl() function required by the create_transform_getters!() macro
    // is provided by the PdfPageObjectPrivate trait.
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageUnsupportedObject<'a> {
    #[inline]
    fn object_handle(&self) -> FPDF_PAGEOBJECT {
        self.object_handle
    }

    #[inline]
    fn ownership(&self) -> &PdfPageObjectOwnership {
        &self.ownership
    }

    #[inline]
    fn set_ownership(&mut self, ownership: PdfPageObjectOwnership) {
        self.ownership = ownership;
    }
}

impl<'a> Drop for PdfPageUnsupportedObject<'a> {
    /// Closes this [PdfPageUnsupportedObject], releasing held memory.
    fn drop(&mut self) {
        self.drop_impl();
    }
}

impl<'a> PdfiumLibraryBindingsAccessor<'a> for PdfPageUnsupportedObject<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Send for PdfPageUnsupportedObject<'a> {}

#[cfg(feature = "thread_safe")]
unsafe impl<'a> Sync for PdfPageUnsupportedObject<'a> {}
