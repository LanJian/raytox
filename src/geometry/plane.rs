use crate::algebra::EPSILON;
use crate::algebra::Point2;
use crate::algebra::Point3;
use crate::algebra::Vector3;
use crate::algebra::Ray;
use crate::material::Phong;

use super::Intersect;
use super::Intersection;
use super::shape::Textured;

#[derive(Debug, Clone, PartialEq)]
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
        if t < EPSILON {
            return None
        }

        Some(Intersection::new(t, l0 + t * ray.dir, n))
    }
}

impl Textured for Plane {
    fn to_texture_space(&self, p: &Point3) -> Point2 {
        let mut candidate = self.normal.cross(&Vector3::K);
        if candidate.magnitude() < EPSILON {
            candidate = self.normal.cross(&(-Vector3::J));
        }
        let u_hat = candidate.normalize();
        let v_hat = u_hat.cross(&self.normal);
        let l = *p - self.origin;

        Point2::new(
            l.dot(&u_hat),
            l.dot(&v_hat),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_hit() {
        let p = Point3::new(0.0, 10.0, 0.0);
        let ray = Ray::new(&p, -Vector3::J);
        let plane = Plane::new(Point3::O, Vector3::J, Phong::default());

        assert_eq!(
            plane.intersect(&ray),
            Some(Intersection::new(
                10.0,
                Point3::O,
                Vector3::J,
            )),
        );
    }

    #[test]
    fn intersect_miss() {
        let p = Point3::new(0.0, 10.0, 0.0);
        let ray = Ray::new(&p, Vector3::J);
        let plane = Plane::new(Point3::O, Vector3::J, Phong::default());

        assert_eq!(
            plane.intersect(&ray),
            None,
        );
    }

    #[test]
    fn intersect_parallel() {
        let p = Point3::new(0.0, 10.0, 0.0);
        let ray = Ray::new(&p, Vector3::K);
        let plane = Plane::new(Point3::O, Vector3::J, Phong::default());

        assert_eq!(
            plane.intersect(&ray),
            None,
        );
    }

    #[test]
    fn intersect_negative_t() {
        let p = Point3::new(0.0, 10.0, 0.0);
        let ray = Ray::new(&p, Vector3::J);
        let plane = Plane::new(Point3::O, -Vector3::J, Phong::default());

        assert_eq!(
            plane.intersect(&ray),
            None,
        );
    }
}
