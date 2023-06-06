//! Defines the [PdfPageAnnotations] struct, exposing functionality related to the
//! annotations that have been added to a single `PdfPage`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_FORMHANDLE, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::error::{PdfiumError, PdfiumInternalError};
use crate::page_annotation::{PdfPageAnnotation, PdfPageAnnotationType};
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;
use crate::page_annotation_text::PdfPageTextAnnotation;
use crate::prelude::{PdfPageInkAnnotation, PdfPageLinkAnnotation};
use std::ops::Range;
use std::os::raw::c_int;

pub type PdfPageAnnotationIndex = usize;

/// The annotations that have been added to a single `PdfPage`.
pub struct PdfPageAnnotations<'a> {
    document_handle: FPDF_DOCUMENT,
    page_handle: FPDF_PAGE,
    form_handle: Option<FPDF_FORMHANDLE>,
    do_regenerate_page_content_after_each_change: bool,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageAnnotations<'a> {
    #[inline]
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        form_handle: Option<FPDF_FORMHANDLE>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageAnnotations {
            document_handle,
            page_handle,
            form_handle,
            do_regenerate_page_content_after_each_change: false,
            bindings,
        }
    }

    /// Sets whether or not this [PdfPageAnnotations] collection should trigger content regeneration
    /// on its containing [PdfPage] when the collection is mutated.
    #[inline]
    pub(crate) fn do_regenerate_page_content_after_each_change(
        &mut self,
        do_regenerate_page_content_after_each_change: bool,
    ) {
        for mut annotation in self.iter() {
            annotation
                .objects_mut_impl()
                .do_regenerate_page_content_after_each_change(
                    do_regenerate_page_content_after_each_change,
                );
        }

        self.do_regenerate_page_content_after_each_change =
            do_regenerate_page_content_after_each_change;
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfPageAnnotations] collection.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    /// Returns the total number of annotations that have been added to the containing `PdfPage`.
    #[inline]
    pub fn len(&self) -> PdfPageAnnotationIndex {
        self.bindings().FPDFPage_GetAnnotCount(self.page_handle) as PdfPageAnnotationIndex
    }

    /// Returns true if this [PdfPageAnnotations] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a Range from 0..(number of annotations) for this [PdfPageAnnotations] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfPageAnnotationIndex> {
        0..self.len()
    }

    /// Returns a single [PdfPageAnnotation] from this [PdfPageAnnotations] collection.
    pub fn get(&self, index: PdfPageAnnotationIndex) -> Result<PdfPageAnnotation<'a>, PdfiumError> {
        if index >= self.len() {
            return Err(PdfiumError::PageAnnotationIndexOutOfBounds);
        }

        let annotation_handle = self
            .bindings()
            .FPDFPage_GetAnnot(self.page_handle, index as c_int);

        if annotation_handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            Ok(PdfPageAnnotation::from_pdfium(
                self.document_handle,
                self.page_handle,
                annotation_handle,
                self.form_handle,
                self.bindings,
            ))
        }
    }

    /// Returns the first [PdfPageAnnotation] in this [PdfPageAnnotations] collection.
    #[inline]
    pub fn first(&self) -> Result<PdfPageAnnotation<'a>, PdfiumError> {
        if !self.is_empty() {
            self.get(0)
        } else {
            Err(PdfiumError::NoAnnotationsInCollection)
        }
    }

    /// Returns the last [PdfPageAnnotation] in this [PdfPageAnnotations] collection.
    #[inline]
    pub fn last(&self) -> Result<PdfPageAnnotation<'a>, PdfiumError> {
        if !self.is_empty() {
            self.get(self.len() - 1)
        } else {
            Err(PdfiumError::NoAnnotationsInCollection)
        }
    }

    /// Returns an iterator over all the annotations in this [PdfPageAnnotations] collection.
    #[inline]
    pub fn iter(&self) -> PdfPageAnnotationsIterator {
        PdfPageAnnotationsIterator::new(self)
    }

    // Regenerates the content of the containing [PdfPage] if necessary after this
    // [PdfPageAnnotations] collection has been mutated.
    fn regenerate_content(&self) -> Result<(), PdfiumError> {
        if self.do_regenerate_page_content_after_each_change {
            if !self
                .bindings
                .is_true(self.bindings.FPDFPage_GenerateContent(self.page_handle))
            {
                Err(PdfiumError::PdfiumLibraryInternalError(
                    PdfiumInternalError::Unknown,
                ))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    /// Creates a new annotation of the given [PdfPageAnnotationType] by passing the result of calling
    /// `FPDFPage_CreateAnnot()` to an annotation constructor function.
    ///
    /// If the containing `PdfPage` has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub(crate) fn create_annotation<T>(
        &mut self,
        annotation_type: PdfPageAnnotationType,
        constructor: fn(
            FPDF_DOCUMENT,
            FPDF_PAGE,
            FPDF_ANNOTATION,
            &'a dyn PdfiumLibraryBindings,
        ) -> T,
    ) -> Result<T, PdfiumError> {
        let handle = self
            .bindings()
            .FPDFPage_CreateAnnot(self.page_handle, annotation_type.as_pdfium());

        if handle.is_null() {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        } else {
            let annotation = constructor(
                self.document_handle,
                self.page_handle,
                handle,
                self.bindings(),
            );

            self.regenerate_content().map(|()| annotation)
        }
    }

    /// Creates a new [PdfPageInkAnnotation] in this [PdfPageAnnotations] collection,
    /// returning the newly created annotation.
    ///
    /// If the containing `PdfPage` has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    #[inline]
    pub fn create_ink_annotation(&mut self) -> Result<PdfPageInkAnnotation<'a>, PdfiumError> {
        self.create_annotation(
            PdfPageAnnotationType::Ink,
            PdfPageInkAnnotation::from_pdfium,
        )
    }

    /// Creates a new [PdfPageTextAnnotation] containing the given text in this [PdfPageAnnotations]
    /// collection, returning the newly created annotation.
    ///
    /// If the containing `PdfPage` has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    #[inline]
    pub fn create_text_annotation(
        &mut self,
        text: &str,
    ) -> Result<PdfPageTextAnnotation<'a>, PdfiumError> {
        let mut annotation = self.create_annotation(
            PdfPageAnnotationType::Text,
            PdfPageTextAnnotation::from_pdfium,
        )?;

        annotation.set_text(text)?;

        Ok(annotation)
    }

    /// Creates a new [PdfPageLinkAnnotation] with the given URI in this [PdfPageAnnotations]
    /// collection, returning the newly created annotation.
    ///
    /// If the containing `PdfPage` has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub fn create_link_annotation(
        &mut self,
        uri: &str,
    ) -> Result<PdfPageLinkAnnotation<'a>, PdfiumError> {
        let mut annotation = self.create_annotation(
            PdfPageAnnotationType::Link,
            PdfPageLinkAnnotation::from_pdfium,
        )?;

        annotation.set_link(uri)?;

        Ok(annotation)
    }

    /// Removes the given [PdfPageAnnotation] from this [PdfPageAnnotations] collection,
    /// consuming the [PdfPageAnnotation].
    ///
    /// If the containing `PdfPage` has a content regeneration strategy of
    /// `PdfPageContentRegenerationStrategy::AutomaticOnEveryChange` then content regeneration
    /// will be triggered on the page.
    pub fn delete_annotation(
        &mut self,
        annotation: PdfPageAnnotation<'a>,
    ) -> Result<(), PdfiumError> {
        let index = self
            .bindings
            .FPDFPage_GetAnnotIndex(self.page_handle, annotation.handle());

        if index == -1 {
            return Err(PdfiumError::PageAnnotationIndexOutOfBounds);
        }

        if self
            .bindings
            .is_true(self.bindings.FPDFPage_RemoveAnnot(self.page_handle, index))
        {
            self.regenerate_content()
        } else {
            Err(PdfiumError::PdfiumLibraryInternalError(
                PdfiumInternalError::Unknown,
            ))
        }
    }
}

/// An iterator over all the [PdfPageAnnotation] objects in a [PdfPageAnnotations] collection.
pub struct PdfPageAnnotationsIterator<'a> {
    annotations: &'a PdfPageAnnotations<'a>,
    next_index: PdfPageAnnotationIndex,
}

impl<'a> PdfPageAnnotationsIterator<'a> {
    #[inline]
    pub(crate) fn new(annotations: &'a PdfPageAnnotations<'a>) -> Self {
        PdfPageAnnotationsIterator {
            annotations,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for PdfPageAnnotationsIterator<'a> {
    type Item = PdfPageAnnotation<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.annotations.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}
