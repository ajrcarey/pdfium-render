//! Defines the [PdfDate] struct, a date and time value accepted by the date setters of
//! [PdfPageAnnotationCommon](crate::pdf::document::page::annotation::PdfPageAnnotationCommon).

use std::time::SystemTime;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use std::time::{Duration, UNIX_EPOCH};

/// A point in time, independent of any particular date and time crate.
///
/// Functions that accept a date take `impl Into<PdfDate>`. A [PdfDate] is created from a
/// [std::time::SystemTime], and the date and time types of other crates can all be converted
/// to one:
///
/// * `chrono::DateTime`: `SystemTime::from(date_time)`
/// * `jiff::Timestamp`: `SystemTime::from(timestamp)`
/// * `time::OffsetDateTime`: `SystemTime::from(date_time)`
///
/// A [PdfDate] is always an instant in UTC; it carries no time zone. No type from any date and
/// time crate appears in this type or in any public function signature, so the crate used
/// internally to format dates can change without changing the public API.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PdfDate(pub(crate) SystemTime);

impl PdfDate {
    /// Returns the current date and time.
    #[inline]
    pub fn now() -> Self {
        // The standard system clock is not implemented on wasm32-unknown-unknown, where
        // SystemTime::now() panics; read the clock of the JavaScript host instead.
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        let now = UNIX_EPOCH + Duration::from_millis(js_sys::Date::now().max(0.0) as u64);

        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        let now = SystemTime::now();

        now.into()
    }
}

impl From<SystemTime> for PdfDate {
    #[inline]
    fn from(time: SystemTime) -> Self {
        PdfDate(time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compile-time check that the date setters accept the supported date types.
    #[allow(dead_code)]
    fn setters_accept_supported_date_types<
        A: crate::pdf::document::page::annotation::PdfPageAnnotationCommon,
    >(
        annotation: &mut A,
    ) {
        let _ = annotation.set_creation_date(SystemTime::now());
        let _ = annotation.set_modification_date(PdfDate::now());
    }
}
