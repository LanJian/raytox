use crate::algebra::point3::Point3;
use crate::algebra::vector3::Vector3;

#[derive(Debug, Clone, PartialEq)]
pub struct Ray<'a> {
    pub origin: &'a Point3,
    pub dir: Vector3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point3, dir: Vector3) -> Self {
        Self { origin, dir }
    }
}
