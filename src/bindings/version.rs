//! Defines the [PdfiumApiVersion] enum, the set of Pdfium API versions supported by `pdfium-render`.

/// A specific Pdfium FPDF_* API release version.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PdfiumApiVersion {
    Future, // For changes published to Pdfium's repository but yet to be released in a binary
    V6721,
    V6666,
    V6611,
    V6569,
    V6555,
    V6490,
    V6406,
    V6337,
    V6295,
    V6259,
    V6164,
    V6124,
    V6110,
    V6084,
    V6043,
    V6015,
    V5961,
}

impl PdfiumApiVersion {
    /// Returns the currently selected `PdfiumApiVersion` based on compile-time feature flags.
    pub(crate) fn current() -> Self {
        #[cfg(feature = "pdfium_future")]
        return PdfiumApiVersion::Future;

        #[cfg(feature = "pdfium_6721")]
        return PdfiumApiVersion::V6721;

        #[cfg(feature = "pdfium_6666")]
        return PdfiumApiVersion::V6666;

        #[cfg(feature = "pdfium_6611")]
        return PdfiumApiVersion::V6611;

        #[cfg(feature = "pdfium_6569")]
        return PdfiumApiVersion::V6569;

        #[cfg(feature = "pdfium_6555")]
        return PdfiumApiVersion::V6555;

        #[cfg(feature = "pdfium_6490")]
        return PdfiumApiVersion::V6490;

        #[cfg(feature = "pdfium_6406")]
        return PdfiumApiVersion::V6406;

        #[cfg(feature = "pdfium_6337")]
        return PdfiumApiVersion::V6337;

        #[cfg(feature = "pdfium_6295")]
        return PdfiumApiVersion::V6295;

        #[cfg(feature = "pdfium_6259")]
        return PdfiumApiVersion::V6259;

        #[cfg(feature = "pdfium_6164")]
        return PdfiumApiVersion::V6164;

        #[cfg(feature = "pdfium_6124")]
        return PdfiumApiVersion::V6124;

        #[cfg(feature = "pdfium_6110")]
        return PdfiumApiVersion::V6110;

        #[cfg(feature = "pdfium_6084")]
        return PdfiumApiVersion::V6084;

        #[cfg(feature = "pdfium_6043")]
        return PdfiumApiVersion::V6043;

        #[cfg(feature = "pdfium_6015")]
        return PdfiumApiVersion::V6015;

        #[cfg(feature = "pdfium_5961")]
        return PdfiumApiVersion::V5961;
    }
}
