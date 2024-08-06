//! Defines the [PdfPageXObjectFormObject] struct, exposing functionality related to a single
//! page object of type `PdfPageObjectType::XObjectForm`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_PAGE, FPDF_PAGEOBJECT};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::pdf::document::page::object::private::internal::PdfPageObjectPrivate;
use crate::pdf::document::page::object::PdfPageObject;
use crate::pdf::document::page::objects::common::{PdfPageObjectIndex, PdfPageObjectsIterator};
use crate::pdf::document::page::objects::private::internal::PdfPageObjectsPrivate;
use std::ops::{Range, RangeInclusive};
use std::os::raw::c_ulong;

/// A single `PdfPageObject` of type `PdfPageObjectType::XObjectForm`. The page object contains a
/// content stream that itself may consist of multiple other page objects. When this page object
/// is rendered, it renders all its constituent page objects, effectively serving as a template or
/// stamping object.
///
/// Despite the page object name including "form", this page object type bears no relation
/// to an interactive form containing form fields.
pub struct PdfPageXObjectFormObject<'a> {
    object_handle: FPDF_PAGEOBJECT,
    page_handle: Option<FPDF_PAGE>,
    annotation_handle: Option<FPDF_ANNOTATION>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageXObjectFormObject<'a> {
    pub(crate) fn from_pdfium(
        object_handle: FPDF_PAGEOBJECT,
        page_handle: Option<FPDF_PAGE>,
        annotation_handle: Option<FPDF_ANNOTATION>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageXObjectFormObject {
            object_handle,
            page_handle,
            annotation_handle,
            bindings,
        }
    }

    /// Returns the total number of child page objects in this [PdfPageXObjectFormObject].
    #[inline]
    pub fn len(&self) -> PdfPageObjectIndex {
        self.len_impl()
    }

    /// Returns true if this page objects collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from `0..(number of objects)` for the child page objects in
    /// this [PdfPageXObjectFormObject].
    #[inline]
    pub fn as_range(&self) -> Range<PdfPageObjectIndex> {
        0..self.len()
    }

    /// Returns an inclusive Range from `0..=(number of objects - 1)` for the child page objects
    /// in this [PdfPageXObjectFormObject].
    #[inline]
    pub fn as_range_inclusive(&self) -> RangeInclusive<PdfPageObjectIndex> {
        if self.is_empty() {
            0..=0
        } else {
            0..=(self.len() - 1)
        }
    }

    /// Returns a single child [PdfPageObject] from this [PdfPageXObjectFormObject].
    #[inline]
    pub fn get(&self, index: PdfPageObjectIndex) -> Result<PdfPageObject<'a>, PdfiumError> {
        self.get_impl(index)
    }

    /// Returns the first child [PdfPageObject] in this [PdfPageXObjectFormObject].
    #[inline]
    pub fn first(&self) -> Result<PdfPageObject<'a>, PdfiumError> {
        if !self.is_empty() {
            self.get(0)
        } else {
            Err(PdfiumError::NoPageObjectsInCollection)
        }
    }

    /// Returns the last child [PdfPageObject] in this [PdfPageXObjectFormObject].
    #[inline]
    pub fn last(&self) -> Result<PdfPageObject<'a>, PdfiumError> {
        if !self.is_empty() {
            self.get(self.len() - 1)
        } else {
            Err(PdfiumError::NoPageObjectsInCollection)
        }
    }

    /// Returns an iterator over all the child [PdfPageObject] objects in this [PdfPageXObjectFormObject].
    #[inline]
    pub fn iter(&'a self) -> PdfPageObjectsIterator<'a> {
        self.iter_impl()
    }
}

impl<'a> PdfPageObjectPrivate<'a> for PdfPageXObjectFormObject<'a> {
    #[inline]
    fn get_object_handle(&self) -> FPDF_PAGEOBJECT {
        self.object_handle
    }

    #[inline]
    fn get_page_handle(&self) -> Option<FPDF_PAGE> {
        self.page_handle
    }

    #[inline]
    fn set_page_handle(&mut self, page: FPDF_PAGE) {
        self.page_handle = Some(page);
    }

    #[inline]
    fn clear_page_handle(&mut self) {
        self.page_handle = None;
    }

    #[inline]
    fn get_annotation_handle(&self) -> Option<FPDF_ANNOTATION> {
        self.annotation_handle
    }

    #[inline]
    fn set_annotation_handle(&mut self, annotation: FPDF_ANNOTATION) {
        self.annotation_handle = Some(annotation);
    }

    #[inline]
    fn clear_annotation_handle(&mut self) {
        self.annotation_handle = None;
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn is_copyable_impl(&self) -> bool {
        false
    }

    #[inline]
    fn try_copy_impl<'b>(
        &self,
        _: FPDF_DOCUMENT,
        _: &'b dyn PdfiumLibraryBindings,
    ) -> Result<PdfPageObject<'b>, PdfiumError> {
        Err(PdfiumError::PageObjectNotCopyable)
    }
}

impl<'a> PdfPageObjectsPrivate<'a> for PdfPageXObjectFormObject<'a> {
    #[inline]
    fn document_handle(&self) -> FPDF_DOCUMENT {
        unimplemented!()
    }

    #[inline]
    fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn len_impl(&self) -> PdfPageObjectIndex {
        self.bindings.FPDFFormObj_CountObjects(self.object_handle) as PdfPageObjectIndex
    }

    fn get_impl(&self, index: PdfPageObjectIndex) -> Result<PdfPageObject<'a>, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::PageObjectIndexOutOfBounds);
        }

        let object_handle = self
            .bindings
            .FPDFFormObj_GetObject(self.object_handle, index as c_ulong);

        if object_handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfPageObject::from_pdfium(
                object_handle,
                self.page_handle,
                self.annotation_handle,
                self.bindings,
            ))
        }
    }

    #[inline]
    fn iter_impl(&'a self) -> PdfPageObjectsIterator<'a> {
        PdfPageObjectsIterator::new(self)
    }

    // The child objects collection is read-only, so add_object_impl() and remove_object_impl()
    // are necessarily incomplete.

    fn add_object_impl(
        &mut self,
        _object: PdfPageObject<'a>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        unimplemented!()
    }

    fn remove_object_impl(
        &mut self,
        _object: PdfPageObject<'a>,
    ) -> Result<PdfPageObject<'a>, PdfiumError> {
        unimplemented!()
    }
}
