use std::cmp::Ordering;

use crate::algebra::{Point2, Ray};
use crate::algebra::{Point3, Vector3};

use super::Plane;
use super::Sphere;
use super::{Cube, Mesh};

#[derive(Debug, Clone, PartialEq)]
pub enum Geometry {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Mesh(Mesh),
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub trait Textured {
    fn to_texture_space(&self, p: &Point3) -> Point2;
}

impl Intersect for Geometry {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Self::Sphere(x) => x.intersect(ray),
            Self::Plane(x) => x.intersect(ray),
            Self::Cube(x) => x.intersect(ray),
            Self::Mesh(x) => x.intersect(ray),
        }
    }
}

impl Textured for Geometry {
    fn to_texture_space(&self, p: &Point3) -> Point2 {
        match self {
            Self::Sphere(x) => x.to_texture_space(p),
            Self::Plane(x) => x.to_texture_space(p),
            Self::Cube(x) => x.to_texture_space(p),
            Self::Mesh(x) => x.to_texture_space(p),
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

impl From<Cube> for Geometry {
    fn from(c: Cube) -> Self {
        Geometry::Cube(c)
    }
}

impl From<Mesh> for Geometry {
    fn from(m: Mesh) -> Self {
        Geometry::Mesh(m)
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
        Self {
            t,
            position,
            normal,
        }
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}
