//! Defines the [PdfPageTextChars] struct, a collection of all the distinct characters
//! in a bounded rectangular region of a single `PdfPage`.

use crate::bindings::PdfiumLibraryBindings;
use crate::error::PdfiumError;
use crate::page::PdfPoints;
use crate::page_text::PdfPageText;
use crate::page_text_char::PdfPageTextChar;
use std::ops::Range;
use std::os::raw::c_double;

pub type PdfPageTextCharIndex = usize;

pub struct PdfPageTextChars<'a> {
    text: &'a PdfPageText<'a>,
    start: i32,
    len: i32,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageTextChars<'a> {
    #[inline]
    pub(crate) fn new(
        text: &'a PdfPageText<'a>,
        start: i32,
        len: i32,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageTextChars {
            text,
            start,
            len,
            bindings,
        }
    }

    /// Returns the index in the containing `PdfPage` of the first character in this
    /// [PdfPageTextChars] collection.
    #[inline]
    pub fn first_char_index(&self) -> PdfPageTextCharIndex {
        self.start as PdfPageTextCharIndex
    }

    /// Returns the number of individual characters in this [PdfPageTextChars] collection.
    #[inline]
    pub fn len(&self) -> PdfPageTextCharIndex {
        self.len as PdfPageTextCharIndex
    }

    /// Returns the index in the containing `PdfPage` of the last character in this
    /// [PdfPageTextChars] collection.
    #[inline]
    pub fn last_char_index(&self) -> PdfPageTextCharIndex {
        (self.start + self.len - 1) as PdfPageTextCharIndex
    }

    /// Returns the valid index range of this [PdfPageTextChars] collection.
    #[inline]
    pub fn as_range(&self) -> Range<PdfPageTextCharIndex> {
        self.first_char_index()..self.last_char_index()
    }

    /// Returns `true` if this [PdfPageTextChars] collection is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a single [PdfPageTextChar] from this [PdfPageTextChars] collection.
    #[inline]
    pub fn get(&self, index: PdfPageTextCharIndex) -> Result<PdfPageTextChar, PdfiumError> {
        let index = index as i32;

        if index < self.start || index >= self.start + self.len {
            Err(PdfiumError::CharIndexOutOfBounds)
        } else {
            Ok(PdfPageTextChar::from_pdfium(
                self.text,
                index,
                self.bindings,
            ))
        }
    }

    /// Returns the character at the given x and y positions on the containing `PdfPage`, if any.
    #[inline]
    pub fn get_char_at_point(&self, x: PdfPoints, y: PdfPoints) -> Option<PdfPageTextChar> {
        self.get_char_near_point(x, PdfPoints::ZERO, y, PdfPoints::ZERO)
    }

    /// Returns the character near to the given x and y positions on the containing `PdfPage`, if any.
    /// The returned character will be no further from the given positions than the given
    /// tolerance values.
    pub fn get_char_near_point(
        &self,
        x: PdfPoints,
        tolerance_x: PdfPoints,
        y: PdfPoints,
        tolerance_y: PdfPoints,
    ) -> Option<PdfPageTextChar> {
        match self.bindings.FPDFText_GetCharIndexAtPos(
            *self.text.get_handle(),
            x.value as c_double,
            y.value as c_double,
            tolerance_x.value as c_double,
            tolerance_y.value as c_double,
        ) {
            -1 => None, // No character at position within tolerances
            -3 => None, // An error occurred, but we'll eat it
            index => self.get(index as PdfPageTextCharIndex).ok(),
        }
    }

    /// Returns an iterator over all the characters in this [PdfPageTextChars] collection.
    #[inline]
    pub fn iter(&self) -> PdfPageTextCharsIterator {
        PdfPageTextCharsIterator::new(self)
    }
}

/// An iterator over all the [PdfPageTextChar] objects in a [PdfPageTextChars] collection.
pub struct PdfPageTextCharsIterator<'a> {
    chars: &'a PdfPageTextChars<'a>,
    next_index: PdfPageTextCharIndex,
}

impl<'a> PdfPageTextCharsIterator<'a> {
    #[inline]
    pub(crate) fn new(chars: &'a PdfPageTextChars<'a>) -> Self {
        PdfPageTextCharsIterator {
            chars,
            next_index: chars.first_char_index(),
        }
    }
}

impl<'a> Iterator for PdfPageTextCharsIterator<'a> {
    type Item = PdfPageTextChar<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.chars.get(self.next_index);

        self.next_index += 1;

        next.ok()
    }
}
