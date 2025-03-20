use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LongDouble {
    pub opaque: [u8; 16],
}

extern "C" {
    fn c_long_double_infinity() -> LongDouble;
    fn c_long_double_pi() -> LongDouble;

    fn c_long_double_from_float(a: f32) -> LongDouble;
    fn c_long_double_from_double(a: f64) -> LongDouble;
    fn c_long_double_to_float(a: LongDouble) -> f32;
    fn c_long_double_to_double(a: LongDouble) -> f64;

    fn c_long_double_add(a: LongDouble, b: LongDouble) -> LongDouble;
    fn c_long_double_sub(a: LongDouble, b: LongDouble) -> LongDouble;
    fn c_long_double_mul(a: LongDouble, b: LongDouble) -> LongDouble;
    fn c_long_double_div(a: LongDouble, b: LongDouble) -> LongDouble;
    fn c_long_double_neg(a: LongDouble) -> LongDouble;

    fn c_long_double_sqrt(a: LongDouble) -> LongDouble;
    fn c_long_double_cbrt(a: LongDouble) -> LongDouble;
    fn c_long_double_cos(a: LongDouble) -> LongDouble;
    fn c_long_double_acos(a: LongDouble) -> LongDouble;
    fn c_long_double_tan(a: LongDouble) -> LongDouble;
    fn c_long_double_exp(a: LongDouble) -> LongDouble;
    fn c_long_double_abs(a: LongDouble) -> LongDouble;
    fn c_long_double_pow(a: LongDouble, b: LongDouble) -> LongDouble;

    fn c_long_double_isinf(a: LongDouble) -> bool;
    fn c_long_double_isnan(a: LongDouble) -> bool;
    fn c_long_double_gt(a: LongDouble, b: LongDouble) -> bool;
    fn c_long_double_gte(a: LongDouble, b: LongDouble) -> bool;
    fn c_long_double_lt(a: LongDouble, b: LongDouble) -> bool;
    fn c_long_double_lte(a: LongDouble, b: LongDouble) -> bool;
    fn c_long_double_eq(a: LongDouble, b: LongDouble) -> bool;
    fn c_long_double_ne(a: LongDouble, b: LongDouble) -> bool;
}

impl LongDouble {
    pub fn infinity() -> Self {
        unsafe { c_long_double_infinity() }
    }

    pub fn pi() -> Self {
        unsafe { c_long_double_pi() }
    }

    pub fn from_f32(val: f32) -> Self {
        unsafe { c_long_double_from_float(val) }
    }

    pub fn from_f64(val: f64) -> Self {
        unsafe { c_long_double_from_double(val) }
    }

    pub fn to_f32(self) -> f32 {
        unsafe { c_long_double_to_float(self) }
    }

    pub fn to_f64(self) -> f64 {
        unsafe { c_long_double_to_double(self) }
    }

    pub fn sqrt(self) -> Self {
        unsafe { c_long_double_sqrt(self) }
    }

    pub fn cbrt(self) -> Self {
        unsafe { c_long_double_cbrt(self) }
    }

    pub fn cos(self) -> Self {
        unsafe { c_long_double_cos(self) }
    }

    pub fn acos(self) -> Self {
        unsafe { c_long_double_acos(self) }
    }

    pub fn tan(self) -> Self {
        unsafe { c_long_double_tan(self) }
    }

    pub fn exp(self) -> Self {
        unsafe { c_long_double_exp(self) }
    }

    pub fn abs(self) -> Self {
        unsafe { c_long_double_abs(self) }
    }

    pub fn pow(self, other: Self) -> Self {
        unsafe { c_long_double_pow(self, other) }
    }

    pub fn is_inf(self) -> bool {
        unsafe { c_long_double_isinf(self) }
    }

    pub fn is_nan(self) -> bool {
        unsafe { c_long_double_isnan(self) }
    }

    pub fn min(self, other: Self) -> Self {
        if self.is_nan() {
            other
        } else if other.is_nan() || self < other {
            self
        } else {
            other
        }
    }

    pub fn max(self, other: Self) -> Self {
        if self.is_nan() {
            other
        } else if other.is_nan() || self > other {
            self
        } else {
            other
        }
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }
}

impl Add for LongDouble {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        unsafe { c_long_double_add(self, rhs) }
    }
}

impl Sub for LongDouble {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        unsafe { c_long_double_sub(self, rhs) }
    }
}

impl Mul for LongDouble {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        unsafe { c_long_double_mul(self, rhs) }
    }
}

impl Div for LongDouble {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        unsafe { c_long_double_div(self, rhs) }
    }
}

impl Neg for LongDouble {
    type Output = Self;
    fn neg(self) -> Self {
        unsafe { c_long_double_neg(self) }
    }
}

impl Default for LongDouble {
    fn default() -> Self {
        LongDouble::from_f32(0.0)
    }
}

impl PartialEq for LongDouble {
    fn eq(&self, other: &Self) -> bool {
        unsafe { c_long_double_eq(*self, *other) }
    }

    fn ne(&self, other: &Self) -> bool {
        unsafe { c_long_double_ne(*self, *other) }
    }
}

impl PartialOrd for LongDouble {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe {
            if c_long_double_lt(*self, *other) {
                Some(Ordering::Less)
            } else if c_long_double_gt(*self, *other) {
                Some(Ordering::Greater)
            } else if c_long_double_eq(*self, *other) {
                Some(Ordering::Equal)
            } else {
                None
            }
        }
    }

    fn lt(&self, other: &Self) -> bool {
        unsafe { c_long_double_lt(*self, *other) }
    }

    fn le(&self, other: &Self) -> bool {
        unsafe { c_long_double_lte(*self, *other) }
    }

    fn gt(&self, other: &Self) -> bool {
        unsafe { c_long_double_gt(*self, *other) }
    }

    fn ge(&self, other: &Self) -> bool {
        unsafe { c_long_double_gte(*self, *other) }
    }
}

impl fmt::Debug for LongDouble {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:e} (long double)", self.to_f64())
    }
}
