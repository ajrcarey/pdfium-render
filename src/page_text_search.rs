use crate::{
    bindgen::FPDF_SCHHANDLE,
    prelude::{PdfPageText, PdfPageTextSegments, PdfiumLibraryBindings},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct SearchOption {
    pub match_case: bool,
    pub match_whole_world: bool,
    pub consecutive: bool,
}

impl SearchOption {
    pub(crate) fn as_pdfium(&self) -> u64 {
        // convert SearchOption to pdfium search flag
        let mut flag = 0;
        if self.match_case {
            flag |= SearchFlag::MatchCase.as_pdfium();
        }
        if self.match_whole_world {
            flag |= SearchFlag::MatchWholeWord.as_pdfium();
        }
        if self.consecutive {
            flag |= SearchFlag::Consecutive.as_pdfium();
        }
        flag
    }
}
pub(crate) enum SearchFlag {
    MatchCase,
    MatchWholeWord,
    Consecutive,
}
impl SearchFlag {
    #[inline]
    pub(crate) fn as_pdfium(&self) -> u64 {
        match self {
            SearchFlag::MatchCase => 0x1,
            SearchFlag::MatchWholeWord => 0x2,
            SearchFlag::Consecutive => 0x4,
        }
    }
}

pub struct PageTextSearch<'a> {
    handle: FPDF_SCHHANDLE,
    text_page: &'a PdfPageText<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PageTextSearch<'a> {
    pub(crate) fn from_pdfium(
        handle: FPDF_SCHHANDLE,
        text_page: &'a PdfPageText<'a>,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PageTextSearch {
            handle,
            text_page,
            bindings,
        }
    }

    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    pub fn handle(&self) -> &FPDF_SCHHANDLE {
        &self.handle
    }

    #[inline]
    fn find_prev(&self) -> bool {
        self.bindings.FPDFText_FindPrev(self.handle) != 0
    }

    #[inline]
    fn find_next(&self) -> bool {
        self.bindings.FPDFText_FindNext(self.handle) != 0
    }

    pub fn get_next_result(&self, search_forward: bool) -> Option<PdfPageTextSegments> {
        let has_next = if search_forward {
            self.find_next()
        } else {
            self.find_prev()
        };
        if has_next {
            let start_index = self.bindings.FPDFText_GetSchResultIndex(self.handle);
            let sch_count = self.bindings.FPDFText_GetSchCount(self.handle);

            return Some(self.text_page.select_segments(start_index, sch_count));
        } else {
            None
        }
    }

    pub fn iter(&self, search_forward: bool) -> PageTextSearchIterator {
        PageTextSearchIterator::new(self, search_forward)
    }
}

pub struct PageTextSearchIterator<'a> {
    search: &'a PageTextSearch<'a>,
    search_forward: bool,
}

impl<'a> PageTextSearchIterator<'a> {
    pub(crate) fn new(search: &'a PageTextSearch<'a>, search_forward: bool) -> Self {
        PageTextSearchIterator {
            search,
            search_forward,
        }
    }
}

impl<'a> Iterator for PageTextSearchIterator<'a> {
    type Item = PdfPageTextSegments<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.search.get_next_result(self.search_forward)
    }
}
