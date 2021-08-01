use crate::algebra::vector3::Vector3;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub const O: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Point3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Point3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Point3> for f64 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output {
        Point3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Point3;
    use super::Vector3;

    #[test]
    fn point_add_vector() {
        let expected = Point3::new(4.0, 3.0, 1.0);
        let actual = Point3::new(1.0, 1.0, 0.0) + Vector3::new(3.0, 2.0, 1.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn point_subtract_point() {
        let expected = Vector3::new(3.0, 4.0, -3.0);
        let actual = Point3::new(5.0, 5.0, 0.0) - Point3::new(2.0, 1.0, 3.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn negate() {
        let expected = Point3::new(-1.0, 0.0, 0.0);
        let actual = -Point3::new(1.0, 0.0, 0.0);
        assert_eq!(actual, expected);
    }
    #[test]
    fn scalar_times_point() {
        let expected = Point3::new(8.0, 0.0, 0.0);
        let actual = 8.0 * Point3::new(1.0, 0.0, 0.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn point_times_scalar() {
        let expected = Point3::new(0.0, 6.0, 0.0);
        let actual = Point3::new(0.0, 1.0, 0.0) * 6.0;
        assert_eq!(actual, expected);
    }
}
