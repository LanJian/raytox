use crate::algebra::{Point3, Ray, Vector3};
use crate::material::Phong;
use crate::texture::TextureCoordinate;

use super::Intersect;
use super::shape::{Intersection, Textured};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,

    pub material: Phong,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Phong) -> Self {
        Self { center, radius, material }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let c = self.center;
        let r = self.radius;
        let o = *ray.origin;
        let u = ray.dir;

        let det = u.dot(&(o - c)).powi(2) - (o - c).magnitude().powi(2) + r * r;

        // no solutions
        if det < 0.0 {
            return None;
        }

        let d = -(u.dot(&(o - c)));
        let t;

        if det == 0.0 {
            // 1 potential solution, pick it if its positive
            if d < 0.0 {
                return None;
            }

            t = d;
        } else {
            // 2 potential solution, pick the min positive solution
            let sqrt_det = det.sqrt();
            let t1 = d + sqrt_det;
            let t2 = d - sqrt_det;

            if t1 < 0.0 && t2 < 0.0 {
                return None;
            }

            if t1 < 0.0 {
                t = t2;
            } else if t2 < 0.0 {
                t = t1;
            } else if t1 < t2 {
                t = t1;
            } else {
                t = t2;
            }
        }


        let p = o + t * u;
        let n = (p - c) / r;
        Some(Intersection::new(t, p, n))
    }
}

impl Textured for Sphere {
    fn to_uv(&self, p: &Point3) -> TextureCoordinate {
        TextureCoordinate::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::Vector3;

    use super::*;

    #[test]
    fn intersect() {
        let ray = Ray::new(&Point3::O, Vector3::new(0.0, 0.0, 1.0).normalize());
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 10.0), 5.0, Phong::default());

        assert_eq!(
            sphere.intersect(&ray),
            Some(Intersection::new(
                5.0,
                Point3::new(0.0, 0.0, 5.0),
                Vector3::new(0.0, 0.0, -1.0)
            )),
        );
    }
}
