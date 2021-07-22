use crate::algebra::EPSILON;
use crate::algebra::Point3;
use crate::algebra::Vector3;
use crate::algebra::Ray;
use crate::material::Phong;
use crate::texture::TextureCoordinate;

use super::Intersect;
use super::Intersection;
use super::shape::Textured;

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

impl Textured for Plane {
    fn to_uv(&self, p: &Point3) -> TextureCoordinate {
        let mut candidate = self.normal.cross(&Vector3::K);
        if candidate.magnitude() < EPSILON {
            candidate = self.normal.cross(&(-Vector3::J));
        }
        let u_hat = candidate.normalize();
        let v_hat = u_hat.cross(&self.normal);
        let l = *p - self.origin;

        TextureCoordinate::new(
            l.dot(&u_hat).rem_euclid(1.0),
            l.dot(&v_hat).rem_euclid(1.0)
        )
    }
}
