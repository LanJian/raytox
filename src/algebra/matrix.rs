use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;

use crate::geometry::Axis;

use super::Point3;
use super::Vector3;
use super::EPSILON;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    data: [[f64; 4]; 4],
}

impl Matrix4 {
    pub const IDENTITY: Self = Self {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    pub fn new() -> Self {
        Self::IDENTITY
    }

    pub fn translation(translation: Vector3) -> Self {
        let mut t = Self::new();
        t[0][3] = translation.x;
        t[1][3] = translation.y;
        t[2][3] = translation.z;
        t
    }

    pub fn rotation(axis: Axis, degrees: f64) -> Self {
        let angle = degrees.to_radians();
        let mut r = Self::new();
        match axis {
            Axis::X => {
                r[1][1] = angle.cos();
                r[1][2] = -angle.sin();
                r[2][1] = angle.sin();
                r[2][2] = angle.cos();
            }
            Axis::Y => {
                r[0][0] = angle.cos();
                r[0][2] = angle.sin();
                r[2][0] = -angle.sin();
                r[2][2] = angle.cos();
            }
            Axis::Z => {
                r[0][0] = angle.cos();
                r[0][1] = -angle.sin();
                r[1][0] = angle.sin();
                r[1][1] = angle.cos();
            }
        }
        r
    }

    pub fn scaling(scale: Vector3) -> Self {
        let mut s = Self::new();
        s[0][0] = scale.x;
        s[1][1] = scale.y;
        s[2][2] = scale.z;
        s
    }

    pub fn transpose(&self) -> Self {
        let mut ret = Self::new();

        for i in 0..4 {
            for j in 0..4 {
                ret[i][j] = self[j][i];
            }
        }

        ret
    }

    pub fn invert(&self) -> Option<Self> {
        let mut inv = Self::new();

        inv[0][0] = self[1][1] * self[2][2] * self[3][3]
            - self[1][1] * self[2][3] * self[3][2]
            - self[2][1] * self[1][2] * self[3][3]
            + self[2][1] * self[1][3] * self[3][2]
            + self[3][1] * self[1][2] * self[2][3]
            - self[3][1] * self[1][3] * self[2][2];

        inv[1][0] = -self[1][0] * self[2][2] * self[3][3]
            + self[1][0] * self[2][3] * self[3][2]
            + self[2][0] * self[1][2] * self[3][3]
            - self[2][0] * self[1][3] * self[3][2]
            - self[3][0] * self[1][2] * self[2][3]
            + self[3][0] * self[1][3] * self[2][2];

        inv[2][0] = self[1][0] * self[2][1] * self[3][3]
            - self[1][0] * self[2][3] * self[3][1]
            - self[2][0] * self[1][1] * self[3][3]
            + self[2][0] * self[1][3] * self[3][1]
            + self[3][0] * self[1][1] * self[2][3]
            - self[3][0] * self[1][3] * self[2][1];

        inv[3][0] = -self[1][0] * self[2][1] * self[3][2]
            + self[1][0] * self[2][2] * self[3][1]
            + self[2][0] * self[1][1] * self[3][2]
            - self[2][0] * self[1][2] * self[3][1]
            - self[3][0] * self[1][1] * self[2][2]
            + self[3][0] * self[1][2] * self[2][1];

        inv[0][1] = -self[0][1] * self[2][2] * self[3][3]
            + self[0][1] * self[2][3] * self[3][2]
            + self[2][1] * self[0][2] * self[3][3]
            - self[2][1] * self[0][3] * self[3][2]
            - self[3][1] * self[0][2] * self[2][3]
            + self[3][1] * self[0][3] * self[2][2];

        inv[1][1] = self[0][0] * self[2][2] * self[3][3]
            - self[0][0] * self[2][3] * self[3][2]
            - self[2][0] * self[0][2] * self[3][3]
            + self[2][0] * self[0][3] * self[3][2]
            + self[3][0] * self[0][2] * self[2][3]
            - self[3][0] * self[0][3] * self[2][2];

        inv[2][1] = -self[0][0] * self[2][1] * self[3][3]
            + self[0][0] * self[2][3] * self[3][1]
            + self[2][0] * self[0][1] * self[3][3]
            - self[2][0] * self[0][3] * self[3][1]
            - self[3][0] * self[0][1] * self[2][3]
            + self[3][0] * self[0][3] * self[2][1];

        inv[3][1] = self[0][0] * self[2][1] * self[3][2]
            - self[0][0] * self[2][2] * self[3][1]
            - self[2][0] * self[0][1] * self[3][2]
            + self[2][0] * self[0][2] * self[3][1]
            + self[3][0] * self[0][1] * self[2][2]
            - self[3][0] * self[0][2] * self[2][1];

        inv[0][2] = self[0][1] * self[1][2] * self[3][3]
            - self[0][1] * self[1][3] * self[3][2]
            - self[1][1] * self[0][2] * self[3][3]
            + self[1][1] * self[0][3] * self[3][2]
            + self[3][1] * self[0][2] * self[1][3]
            - self[3][1] * self[0][3] * self[1][2];

        inv[1][2] = -self[0][0] * self[1][2] * self[3][3]
            + self[0][0] * self[1][3] * self[3][2]
            + self[1][0] * self[0][2] * self[3][3]
            - self[1][0] * self[0][3] * self[3][2]
            - self[3][0] * self[0][2] * self[1][3]
            + self[3][0] * self[0][3] * self[1][2];

        inv[2][2] = self[0][0] * self[1][1] * self[3][3]
            - self[0][0] * self[1][3] * self[3][1]
            - self[1][0] * self[0][1] * self[3][3]
            + self[1][0] * self[0][3] * self[3][1]
            + self[3][0] * self[0][1] * self[1][3]
            - self[3][0] * self[0][3] * self[1][1];

        inv[3][2] = -self[0][0] * self[1][1] * self[3][2]
            + self[0][0] * self[1][2] * self[3][1]
            + self[1][0] * self[0][1] * self[3][2]
            - self[1][0] * self[0][2] * self[3][1]
            - self[3][0] * self[0][1] * self[1][2]
            + self[3][0] * self[0][2] * self[1][1];

        inv[0][3] = -self[0][1] * self[1][2] * self[2][3]
            + self[0][1] * self[1][3] * self[2][2]
            + self[1][1] * self[0][2] * self[2][3]
            - self[1][1] * self[0][3] * self[2][2]
            - self[2][1] * self[0][2] * self[1][3]
            + self[2][1] * self[0][3] * self[1][2];

        inv[1][3] = self[0][0] * self[1][2] * self[2][3]
            - self[0][0] * self[1][3] * self[2][2]
            - self[1][0] * self[0][2] * self[2][3]
            + self[1][0] * self[0][3] * self[2][2]
            + self[2][0] * self[0][2] * self[1][3]
            - self[2][0] * self[0][3] * self[1][2];

        inv[2][3] = -self[0][0] * self[1][1] * self[2][3]
            + self[0][0] * self[1][3] * self[2][1]
            + self[1][0] * self[0][1] * self[2][3]
            - self[1][0] * self[0][3] * self[2][1]
            - self[2][0] * self[0][1] * self[1][3]
            + self[2][0] * self[0][3] * self[1][1];

        inv[3][3] = self[0][0] * self[1][1] * self[2][2]
            - self[0][0] * self[1][2] * self[2][1]
            - self[1][0] * self[0][1] * self[2][2]
            + self[1][0] * self[0][2] * self[2][1]
            + self[2][0] * self[0][1] * self[1][2]
            - self[2][0] * self[0][2] * self[1][1];

        let det = self[0][0] * inv[0][0]
            + self[0][1] * inv[1][0]
            + self[0][2] * inv[2][0]
            + self[0][3] * inv[3][0];

        if det == 0.0 {
            return None;
        }

        let inv_det = 1.0 / det;
        Some(inv * inv_det)
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<[[f64; 4]; 4]> for Matrix4 {
    fn from(data: [[f64; 4]; 4]) -> Self {
        Self { data }
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        let a = self.data.iter().flatten();
        let b = other.data.iter().flatten();
        a.zip(b).all(|(x, y)| (x - y).abs() <= EPSILON)
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut ret = Self::new();
        for i in 0..4 {
            for j in 0..4 {
                ret[i][j] = self[i][0] * rhs[0][j]
                    + self[i][1] * rhs[1][j]
                    + self[i][2] * rhs[2][j]
                    + self[i][3] * rhs[3][j];
            }
        }

        ret
    }
}

impl Mul<f64> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut ret = Self::new();
        for i in 0..4 {
            for j in 0..4 {
                ret[i][j] = self[i][j] * rhs
            }
        }

        ret
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z,
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z,
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z,
        )
    }
}

impl Mul<Point3> for Matrix4 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output {
        Point3::new(
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3],
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3],
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invert() {
        assert_eq!(Matrix4::IDENTITY.invert(), Some(Matrix4::IDENTITY));

        let actual = Matrix4::from([
            [6.0, 3.0, 65.0, 12.0],
            [3.0, 43.0, 5.0, 9.0],
            [5.0, 1.0, 8.0, 4.0],
            [2.0, 56.0, 11.0, 0.0],
        ])
        .invert();
        let expected = Some(Matrix4::from([
            [
                -4705.0 / 131479.0,
                -9184.0 / 131479.0,
                34779.0 / 131479.0,
                6683.0 / 131479.0,
            ],
            [
                -259.0 / 131479.0,
                724.0 / 131479.0,
                -852.0 / 131479.0,
                1821.0 / 131479.0,
            ],
            [
                2174.0 / 131479.0,
                -2016.0 / 131479.0,
                -1986.0 / 131479.0,
                1467.0 / 131479.0,
            ],
            [
                1598.0 / 131479.0,
                15331.0 / 131479.0,
                -6419.0 / 131479.0,
                -11743.0 / 131479.0,
            ],
        ]));
        assert_eq!(actual, expected);
    }
}
