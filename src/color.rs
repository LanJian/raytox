use image::{Pixel, Rgba};
use std::{
    iter::Sum,
    ops::{Add, Mul, Sub},
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
        }
    }
}

impl From<Color> for Rgba<u8> {
    fn from(c: Color) -> Self {
        Rgba::from_channels(
            (c.r * 255.0).clamp(0.0, 255.0) as u8,
            (c.g * 255.0).clamp(0.0, 255.0) as u8,
            (c.b * 255.0).clamp(0.0, 255.0) as u8,
            255,
        )
    }
}

impl From<Rgba<u8>> for Color {
    fn from(c: Rgba<u8>) -> Self {
        Color::new(
            c[0] as f64 / 255.0,
            c[1] as f64 / 255.0,
            c[2] as f64 / 255.0,
        )
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let val = rhs.clamp(0.0, 1.0);
        Self::new(self.r * val, self.g * val, self.b * val)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        let val = self.clamp(0.0, 1.0);
        Color::new(val * rhs.r, val * rhs.g, val * rhs.b)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::BLACK, |a, e| a + e)
    }
}
