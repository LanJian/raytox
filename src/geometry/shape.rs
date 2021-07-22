use std::cmp::Ordering;

use crate::algebra::Ray;
use crate::algebra::{Point3, Vector3};
use crate::material::Phong;
use crate::texture::TextureCoordinate;

use super::Sphere;
use super::Plane;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Geometry {
    Sphere(Sphere),
    Plane(Plane),
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub trait Textured {
    fn to_uv(&self, p: &Point3) -> TextureCoordinate;
}

impl Geometry {
    pub fn material(&self) -> Phong {
        match self {
            Self::Sphere(x) => x.material,
            Self::Plane(x) => x.material,
        }
    }
}

impl Intersect for Geometry {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Self::Sphere(x) => x.intersect(ray),
            Self::Plane(x) => x.intersect(ray),
        }
    }
}

impl Textured for Geometry {
    fn to_uv(&self, p: &Point3) -> TextureCoordinate {
        match self {
            Self::Sphere(x) => x.to_uv(p),
            Self::Plane(x) => x.to_uv(p),
        }
    }
}

impl From<Sphere> for Geometry {
    fn from(s: Sphere) -> Self {
        Geometry::Sphere(s)
    }
}

impl From<Plane> for Geometry {
    fn from(p: Plane) -> Self {
        Geometry::Plane(p)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub position: Point3,
    pub normal: Vector3,
}

impl Intersection {
    pub fn new(t: f64, position: Point3, normal: Vector3) -> Self {
        Self { t, position, normal }
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}
