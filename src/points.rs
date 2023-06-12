//! Defines the [PdfPoints] struct, the basic unit of measurement within the internal
//! coordinate system inside a `PdfDocument`.

use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

/// The internal coordinate system inside a `PdfDocument` is measured in Points, a
/// device-independent unit equal to 1/72 inches, roughly 0.358 mm. Points are converted to pixels
/// when a `PdfPage` is rendered into a `PdfBitmap`.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct PdfPoints {
    pub value: f32,
}

impl PdfPoints {
    /// A [PdfPoints] object with the identity value 0.0.
    pub const ZERO: PdfPoints = PdfPoints::zero();

    /// Creates a new [PdfPoints] object with the given value.
    #[inline]
    pub const fn new(value: f32) -> Self {
        Self { value }
    }

    /// Creates a new [PdfPoints] object with the value 0.0.
    ///
    /// Consider using the compile-time constant value [PdfPoints::ZERO]
    /// rather than calling this function directly.
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0.0)
    }

    /// Creates a new [PdfPoints] object from the given measurement in inches.
    #[inline]
    pub fn from_inches(inches: f32) -> Self {
        Self::new(inches * 72.0)
    }

    /// Creates a new [PdfPoints] object from the given measurement in centimeters.
    #[inline]
    pub fn from_cm(cm: f32) -> Self {
        Self::from_inches(cm / 2.54)
    }

    /// Creates a new [PdfPoints] object from the given measurement in millimeters.
    #[inline]
    pub fn from_mm(mm: f32) -> Self {
        Self::from_cm(mm / 10.0)
    }

    /// Converts the value of this [PdfPoints] object to inches.
    #[inline]
    pub fn to_inches(&self) -> f32 {
        self.value / 72.0
    }

    /// Converts the value of this [PdfPoints] object to centimeters.
    #[inline]
    pub fn to_cm(&self) -> f32 {
        self.to_inches() * 2.54
    }

    /// Converts the value of this [PdfPoints] object to millimeters.
    #[inline]
    pub fn to_mm(self) -> f32 {
        self.to_cm() * 10.0
    }
}

impl Add<PdfPoints> for PdfPoints {
    type Output = PdfPoints;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        PdfPoints::new(self.value + rhs.value)
    }
}

impl AddAssign<PdfPoints> for PdfPoints {
    #[inline]
    fn add_assign(&mut self, rhs: PdfPoints) {
        self.value += rhs.value;
    }
}

impl Sub<PdfPoints> for PdfPoints {
    type Output = PdfPoints;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        PdfPoints::new(self.value - rhs.value)
    }
}

impl SubAssign<PdfPoints> for PdfPoints {
    #[inline]
    fn sub_assign(&mut self, rhs: PdfPoints) {
        self.value -= rhs.value;
    }
}

impl Mul<f32> for PdfPoints {
    type Output = PdfPoints;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        PdfPoints::new(self.value * rhs)
    }
}

impl Div<f32> for PdfPoints {
    type Output = PdfPoints;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        PdfPoints::new(self.value / rhs)
    }
}

impl Neg for PdfPoints {
    type Output = PdfPoints;

    #[inline]
    fn neg(self) -> Self::Output {
        PdfPoints::new(-self.value)
    }
}
