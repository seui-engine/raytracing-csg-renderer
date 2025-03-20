use std::ops::{Add, Div, Mul};

use seui_engine_raytracing_csg_renderer_long_double::LongDouble;

#[derive(Clone, Copy, Debug)]
pub struct HDRColor {
    pub r: LongDouble,
    pub g: LongDouble,
    pub b: LongDouble,
}

#[derive(Clone, Copy, Debug)]
pub struct LDRColor {
    pub r: LongDouble,
    pub g: LongDouble,
    pub b: LongDouble,
}

impl Default for LDRColor {
    fn default() -> Self {
        Self {
            r: LongDouble::from_f64(1.0),
            g: LongDouble::from_f64(1.0),
            b: LongDouble::from_f64(1.0),
        }
    }
}

impl Default for HDRColor {
    fn default() -> Self {
        Self {
            r: LongDouble::from_f64(1.0),
            g: LongDouble::from_f64(1.0),
            b: LongDouble::from_f64(1.0),
        }
    }
}

impl LDRColor {
    pub fn new(r: LongDouble, g: LongDouble, b: LongDouble) -> LDRColor {
        LDRColor {
            r: r.min(LongDouble::from_f64(1.0)),
            g: g.min(LongDouble::from_f64(1.0)),
            b: b.min(LongDouble::from_f64(1.0)),
        }
    }
}

impl Mul<LDRColor> for HDRColor {
    type Output = HDRColor;

    fn mul(self, rhs: LDRColor) -> Self::Output {
        HDRColor {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<HDRColor> for HDRColor {
    type Output = HDRColor;

    fn mul(self, rhs: HDRColor) -> Self::Output {
        HDRColor {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<HDRColor> for LDRColor {
    type Output = HDRColor;

    fn mul(self, rhs: HDRColor) -> Self::Output {
        HDRColor {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Add for HDRColor {
    type Output = HDRColor;

    fn add(self, rhs: Self) -> Self::Output {
        HDRColor {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<LongDouble> for HDRColor {
    type Output = HDRColor;

    fn mul(self, rhs: LongDouble) -> Self::Output {
        HDRColor {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<LongDouble> for HDRColor {
    type Output = HDRColor;

    fn div(self, rhs: LongDouble) -> Self::Output {
        HDRColor {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}
