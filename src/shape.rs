use std::cmp::min;

use crate::algebra::{point3::Point3, ray::Ray};

//trait  {

//}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn intersection(&self, ray: &Ray) -> Option<(Point3, f64)> {
        let c = self.center;
        let r = self.radius;
        let o = *ray.origin;
        let u = ray.dir;

        let det = u.dot(&(o - c)).powi(2) - (o - c).magnitude().powi(2) + r * r;

        if det < 0.0 {
            return None;
        }

        let d = -(u.dot(&(o - c)));

        if det == 0.0 {
            if d < 0.0 {
                return None;
            }

            return Some((o + d * u, d));
        }

        let sqrt_det = det.sqrt();
        let t1 = d + sqrt_det;
        let t2 = d - sqrt_det;

        if t1 < 0.0 && t2 < 0.0 {
            return None;
        }

        if t1 < 0.0 {
            return Some((o + t2 * u, t2));
        }

        if t2 < 0.0 {
            return Some((o + t1 * u, t1));
        }

        let t = if t1 < t2 { t1 } else { t2 };

        Some((o + t * u, t))
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::vector3::Vector3;

    use super::*;

    #[test]
    fn intersection() {
        let ray = Ray::new(&Point3::O, Vector3::new(0.0, 0.0, 1.0).normalize());
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 10.0), 5.0);

        assert_eq!(
            sphere.intersection(&ray),
            Some((Point3::new(0.0, 0.0, 5.0), 5.0))
        );
    }
}
