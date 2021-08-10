use crate::algebra::Point3;
use crate::algebra::Vector3;
use crate::entity::Transformable;
use crate::geometry::Axis;

use super::Matrix4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vector3) -> Self {
        Self { origin, dir }
    }

    pub fn distance_to(&self, point: Point3) -> f64 {
        (point - self.origin).dot(&self.dir)
    }
}

impl Transformable for Ray {
    fn translate(self, translation: Vector3) -> Self {
        Self::new(
            self.origin.translate(translation),
            self.dir.translate(translation),
        )
    }

    fn rotate(self, axis: Axis, degrees: f64) -> Self {
        Self::new(
            self.origin.rotate(axis, degrees),
            self.dir.rotate(axis, degrees),
        )
    }

    fn scale(self, scale: Vector3) -> Self {
        Self::new(self.origin.scale(scale), self.dir.scale(scale))
    }

    fn transform(self, transform: Matrix4) -> Self {
        Self::new(
            self.origin.transform(transform),
            self.dir.transform(transform),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_to() {
        assert_eq!(
            Ray::new(Point3::O, Vector3::I).distance_to(Point3::new(4.0, 0.0, 0.0)),
            4.0
        );
    }
}
