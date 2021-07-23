use std::convert::TryFrom;

use crate::algebra::{Point3, Ray, Vector3};

use super::{Intersect, Intersection};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub point: Point3,
    pub normal: Vector3,
}

impl Vertex {
    pub fn new(point: Point3, normal: Vector3) -> Self {
        Self { point, normal }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    pub vertices: Vec<Vertex>,
    pub normal: Vector3,
}

impl TryFrom<Vec<Vertex>> for Face {
    type Error = String;

    fn try_from(vertices: Vec<Vertex>) -> Result<Self, Self::Error> {
        if vertices.len() < 3 {
            return Err("Not enough vertices".to_string());
        }

        let a = vertices[0].point;
        let b = vertices[1].point;
        let c = vertices[2].point;

        Ok(Self {
            vertices,
            normal: (a - b).cross(&(c - b)).normalize(),
        })
    }
}

impl Intersect for Face {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub faces: Vec<Face>,
}

impl From<Vec<Face>> for Mesh {
    fn from(faces: Vec<Face>) -> Self {
        Self { faces }
    }
}

impl Intersect for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        todo!()
    }
}

