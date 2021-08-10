use std::f64::consts::PI;

use crate::algebra::{Point2, Point3, Ray};

use super::{Intersect, Intersection, Textured};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Point3::O, 1.0)
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let center = self.center;
        let r = self.radius;
        let o = ray.origin;
        let u = ray.dir;

        let a = u.norm();
        let b = 2.0 * u.dot(&(o - center));
        let c = (o - center).norm() - r * r;

        let discrim = b * b - 4.0 * a * c;

        // no solutions
        if discrim < 0.0 {
            return None;
        }

        let t;

        if discrim == 0.0 {
            // 1 potential solution, pick it if its positive
            t = -b / (2.0 * a);

            if t < 0.0 {
                return None;
            }
        } else {
            // 2 potential solution, pick the min positive solution
            let sqrt_discrim = discrim.sqrt();
            let t1 = (-b + sqrt_discrim) / (2.0 * a);
            let t2 = (-b - sqrt_discrim) / (2.0 * a);

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
        let n = (p - center) / r;
        Some(Intersection::new(t, p, n))
    }
}

impl Textured for Sphere {
    fn to_texture_space(&self, p: &Point3) -> Point2 {
        let d = (self.center - *p).normalize();
        Point2::new(0.5 + d.z.atan2(d.x) / (2.0 * PI), 0.5 - d.y.asin() / PI)
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::Vector3;

    use super::*;

    #[test]
    fn intersect() {
        let ray = Ray::new(Point3::O, Vector3::new(0.0, 0.0, 1.0).normalize());
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 10.0), 5.0);

        assert_eq!(
            sphere.intersect(&ray),
            Some(Intersection::new(
                5.0,
                Point3::new(0.0, 0.0, 5.0),
                Vector3::new(0.0, 0.0, -1.0)
            )),
        );
    }

    #[test]
    fn to_texture_space() {}
}
