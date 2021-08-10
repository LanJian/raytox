use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{entity::Transformable, geometry::Axis};

use super::Matrix4;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const I: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const J: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const K: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        let Self {
            x: ref bx,
            y: ref by,
            z: ref bz,
        } = self;
        let Self {
            x: ref cx,
            y: ref cy,
            z: ref cz,
        } = rhs;

        Self::new(by * cz - bz * cy, bz * cx - bx * cz, bx * cy - by * cx)
    }

    pub fn norm(&self) -> f64 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::ZERO - self
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self / rhs.x, self / rhs.y, self / rhs.z)
    }
}

impl Transformable for Vector3 {
    fn translate(self, translation: Vector3) -> Self {
        Matrix4::translation(translation) * self
    }

    fn rotate(self, axis: Axis, degrees: f64) -> Self {
        Matrix4::rotation(axis, degrees) * self
    }

    fn scale(self, scale: Vector3) -> Self {
        Matrix4::scaling(scale) * self
    }

    fn transform(self, transform: Matrix4) -> Self {
        transform * self
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl From<Vector3> for Vector4 {
    fn from(v: Vector3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vector3;

    #[test]
    fn add() {
        let expected = Vector3::new(1.0, 1.0, 0.0);
        let actual = Vector3::I + Vector3::J;
        assert_eq!(actual, expected);
    }

    #[test]
    fn subtract() {
        let expected = Vector3::new(1.0, -1.0, 0.0);
        let actual = Vector3::I - Vector3::J;
        assert_eq!(actual, expected);
    }

    #[test]
    fn negate() {
        let expected = Vector3::new(-1.0, 0.0, 0.0);
        let actual = -Vector3::I;
        assert_eq!(actual, expected);
    }

    #[test]
    fn scalar_times_vector() {
        let expected = Vector3::new(8.0, 0.0, 0.0);
        let actual = 8.0 * Vector3::I;
        assert_eq!(actual, expected);
    }

    #[test]
    fn vector_times_scalar() {
        let expected = Vector3::new(0.0, 6.0, 0.0);
        let actual = Vector3::J * 6.0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn vector_div_scalar() {
        assert_eq!(
            Vector3::new(4.0, 10.0, 0.0) / 2.0,
            Vector3::new(2.0, 5.0, 0.0)
        );
    }

    #[test]
    fn scalar_div_vector() {
        assert_eq!(
            16.0 / Vector3::new(4.0, 8.0, 2.0),
            Vector3::new(4.0, 2.0, 8.0)
        );
    }

    #[test]
    fn div_by_zero() {
        assert_eq!(
            Vector3::new(4.0, 10.0, 1.0) / 0.0,
            Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY)
        );

        assert_eq!(
            16.0 / Vector3::new(0.0, -0.0, 0.0),
            Vector3::new(f64::INFINITY, -f64::INFINITY, f64::INFINITY)
        );
    }
}
