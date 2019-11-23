use crate::style::values::computed::{ToComputedValue, ComputeContext};
use crate::style::values::{CSSFloat, specified};
use app_units::Au;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// The computed `<length>` value.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CSSPixelLength(CSSFloat);

impl From<CSSPixelLength> for Au {
    #[inline]
    fn from(len: CSSPixelLength) -> Self {
        Au::from_f32_px(len.0)
    }
}

impl From<Au> for CSSPixelLength {
    #[inline]
    fn from(len: Au) -> Self {
        CSSPixelLength::new(len.to_f32_px())
    }
}


impl Add for CSSPixelLength {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.px() + other.px())
    }
}

impl AddAssign for CSSPixelLength {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Div<CSSFloat> for CSSPixelLength {
    type Output = Self;

    #[inline]
    fn div(self, other: CSSFloat) -> Self {
        Self::new(self.px() / other)
    }
}

impl Mul<CSSFloat> for CSSPixelLength {
    type Output = Self;

    #[inline]
    fn mul(self, other: CSSFloat) -> Self {
        Self::new(self.px() * other)
    }
}

impl Neg for CSSPixelLength {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        CSSPixelLength::new(-self.0)
    }
}

impl Sub for CSSPixelLength {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(self.px() - other.px())
    }
}

impl CSSPixelLength {
    /// Return a new CSSPixelLength.
    #[inline]
    pub fn new(px: CSSFloat) -> Self {
        CSSPixelLength(px)
    }

    /// Return the containing pixel value.
    #[inline]
    pub fn px(&self) -> CSSFloat {
        self.0
    }

    /// Return the length with app_unit i32 type.
    #[inline]
    pub fn to_i32_au(&self) -> i32 {
        Au::from(*self).0
    }

    /// Return the absolute value of this length.
    #[inline]
    pub fn abs(self) -> Self {
        CSSPixelLength::new(self.0.abs())
    }

    /// Return the clamped value of this length.
    #[inline]
    pub fn clamp_to_non_negative(self) -> Self {
        CSSPixelLength::new(self.0.max(0.))
    }

    /// Returns the minimum between `self` and `other`.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        CSSPixelLength::new(self.0.min(other.0))
    }

    /// Returns the maximum between `self` and `other`.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        CSSPixelLength::new(self.0.max(other.0))
    }

    /// Sets `self` to the maximum between `self` and `other`.
    pub fn max_assign(&mut self, other: Self) {
        *self = self.max(other);
    }
}

impl ToComputedValue for specified::AbsoluteLength {
    type ComputedValue = CSSPixelLength;

    fn to_computed_value(&self, context: &ComputeContext) -> Self::ComputedValue {
        CSSPixelLength(self.to_px())
    }
}
