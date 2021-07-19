use crate::algebra::EPSILON;
use crate::algebra::Point3;
use crate::algebra::Vector3;
use crate::algebra::Ray;
use crate::material::Phong;

use super::Intersect;
use super::Intersection;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub origin: Point3,
    pub normal: Vector3,

    pub material: Phong,
}

impl Plane {
    pub fn new(origin: Point3, normal: Vector3, material: Phong) -> Self {
        Self { origin, normal, material }
    }
}

impl Intersect for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let l0 = *ray.origin;
        let l = -ray.dir;
        let p0 = self.origin;
        let n = self.normal;
            
        let denom = l.dot(&n);

        if denom < EPSILON {
            return None;
        }

        let t = (l0 - p0).dot(&n) / denom;
        Some(Intersection::new(t, l0 + t * ray.dir, n))
    }
}
