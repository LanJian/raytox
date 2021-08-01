use crate::algebra::{Point3, Ray, Vector3};

use super::{Intersect, Intersection};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cube {
    min_bounds: Point3,
    max_bounds: Point3,
}

impl Cube {
    pub fn new(min_bounds: Point3, max_bounds: Point3) -> Self {
        Self {
            min_bounds,
            max_bounds,
        }
    }
}

impl Default for Cube {
    // unit cube with the min corner at (0, 0, 0)
    fn default() -> Self {
        Self {
            min_bounds: Point3::new(0.0, 0.0, 0.0),
            max_bounds: Point3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Intersect for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let bounds = [self.min_bounds, self.max_bounds];
        let normals = [
            (-Vector3::I, -Vector3::J, -Vector3::K),
            (Vector3::I, Vector3::J, Vector3::K),
        ];
        let inv_dir = 1.0 / ray.dir;
        let sign = (
            (inv_dir.x < 0.0) as usize,
            (inv_dir.y < 0.0) as usize,
            (inv_dir.z < 0.0) as usize,
        );

        let mut tmin = (bounds[sign.0].x - ray.origin.x) * inv_dir.x;
        let mut tmax = (bounds[1 - sign.0].x - ray.origin.x) * inv_dir.x;
        let mut normal_min = normals[sign.0].0;
        let mut normal_max = normals[1 - sign.0].0;
        let tymin = (bounds[sign.1].y - ray.origin.y) * inv_dir.y;
        let tymax = (bounds[1 - sign.1].y - ray.origin.y) * inv_dir.y;

        if (tmin > tymax) || (tymin > tmax) {
            return None;
        }
        if tymin > tmin {
            tmin = tymin;
            normal_min = normals[sign.1].1;
        }
        if tymax < tmax {
            tmax = tymax;
            normal_max = normals[1 - sign.1].1;
        }

        let tzmin = (bounds[sign.2].z - ray.origin.z) * inv_dir.z;
        let tzmax = (bounds[1 - sign.2].z - ray.origin.z) * inv_dir.z;

        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }
        if tzmin > tmin {
            tmin = tzmin;
            normal_min = normals[sign.2].2;
        }
        if tzmax < tmax {
            tmax = tzmax; 
            normal_max = normals[1 - sign.2].2;
        }

        if tmax < 0.0 {
            return None;
        }
        if tmin < 0.0 {
            return Some(Intersection::new(
                    tmax,
                    *ray.origin + tmax * ray.dir,
                    normal_max,
            ));
        }

        Some(Intersection::new(
            tmin,
            *ray.origin + tmin * ray.dir,
            normal_min,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
        let cube = Cube::new(Point3::new(-0.5, -0.5, -0.5), Point3::new(0.5, 0.5, 0.5));

        // hit from left
        let mut origin = Point3::O - Vector3::I;
        let mut ray = Ray::new(&origin, Vector3::I);
        assert_eq!(
            cube.intersect(&ray),
            Some(Intersection::new(
                0.5,
                Point3::new(-0.5, 0.0, 0.0),
                -Vector3::I
            ))
        );

        // hit from right
        origin = Point3::O + Vector3::I;
        ray = Ray::new(&origin, -Vector3::I);
        assert_eq!(
            cube.intersect(&ray),
            Some(Intersection::new(
                0.5,
                Point3::new(0.5, 0.0, 0.0),
                Vector3::I
            ))
        );

        // hit from bottom
        origin = Point3::O - Vector3::J;
        ray = Ray::new(&origin, Vector3::J);
        assert_eq!(
            cube.intersect(&ray),
            Some(Intersection::new(
                0.5,
                Point3::new(0.0, -0.5, 0.0),
                -Vector3::J
            ))
        );

        // hit from top
        origin = Point3::O + Vector3::J;
        ray = Ray::new(&origin, -Vector3::J);
        assert_eq!(
            cube.intersect(&ray),
            Some(Intersection::new(
                0.5,
                Point3::new(0.0, 0.5, 0.0),
                Vector3::J
            ))
        );

        // hit from front
        origin = Point3::O - Vector3::K;
        ray = Ray::new(&origin, Vector3::K);
        assert_eq!(
            cube.intersect(&ray),
            Some(Intersection::new(
                0.5,
                Point3::new(0.0, 0.0, -0.5),
                -Vector3::K
            ))
        );

        // hit from back
        origin = Point3::O + Vector3::K;
        ray = Ray::new(&origin, -Vector3::K);
        assert_eq!(
            cube.intersect(&ray),
            Some(Intersection::new(
                0.5,
                Point3::new(0.0, 0.0, 0.5),
                Vector3::K
            ))
        );

        // miss
        origin = Point3::O + Vector3::I;
        ray = Ray::new(&origin, Vector3::J);
        assert_eq!(cube.intersect(&ray), None);
    }
}
