use crate::bindgen::FPDF_PAGE;
use crate::error::PdfiumError;

#[allow(dead_code)] // During development of feature
fn flatten(_page_handle: FPDF_PAGE) -> Result<(), PdfiumError> {
    unimplemented!()
}
