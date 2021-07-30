use std::convert::TryFrom;
use std::fs::File;

use ply_rs::{
    parser::Parser,
    ply::{DefaultElement, Ply, Property},
};

use crate::{
    algebra::{Point2, Point3, Ray, Vector3, EPSILON},
    material::Phong,
};

use super::{Intersect, Intersection, Plane, Textured};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub point: Point3,
    pub normal: Vector3,
}

impl Vertex {
    pub fn with_normal(mut self, normal: Vector3) -> Self {
        self.normal = normal;
        self
    }
}

impl From<Point3> for Vertex {
    fn from(point: Point3) -> Self {
        Self {
            point,
            normal: Vector3::default(),
        }
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
        let plane = Plane::new(self.vertices[0].point, self.normal, Phong::default());
        let intersection = plane.intersect(ray);

        match intersection {
            Some(Intersection { position: c, .. }) => {
                let num_vertices = self.vertices.len();
                let inside = (0..num_vertices).all(|i| {
                    let a = self.vertices[i].point;
                    let b = self.vertices[(i + 1) % num_vertices].point;

                    (a - b).cross(&(c - b)).dot(&self.normal) > -EPSILON
                });

                if inside {
                    intersection
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub faces: Vec<Face>,

    pub material: Phong,
}

impl Mesh {
    pub fn with_material(mut self, material: Phong) -> Self {
        self.material = material;
        self
    }

    pub fn from_ply_file(path: &str) -> Result<Self, String> {
        let mut file = File::open(path).or_else(|_| Err("Cannot read file".to_string()))?;

        // create a parser
        let parser = Parser::<DefaultElement>::new();

        // use the parser: read the entire file
        let ply = parser
            .read_ply(&mut file)
            .or_else(|_| Err("Cannot parse into ply".to_string()))?;

        let faces = ply.payload["face"]
            .iter()
            .map(|f| {
                let vertex_indices = match &f["vertex_indices"] {
                    Property::ListInt(indices) => indices.clone(),
                    _ => vec![],
                };

                let vertices = vertex_indices
                    .iter()
                    .rev()
                    .map(|i| {
                        let vp = &ply.payload["vertex"][*i as usize];

                        let x: f64 = match vp["x"] {
                            Property::Float(val) => val.into(),
                            _ => 0.0,
                        };
                        let y: f64 = match vp["y"] {
                            Property::Float(val) => val.into(),
                            _ => 0.0,
                        };
                        let z: f64 = match vp["z"] {
                            Property::Float(val) => val.into(),
                            _ => 0.0,
                        };

                        Vertex::from(Point3::new(x, y, z))
                    })
                    .collect::<Vec<Vertex>>();

                Face::try_from(vertices).expect("Invalid polygon face")
            })
            .collect::<Vec<Face>>();

        Ok(Self::from(faces))
    }
}

impl From<Vec<Face>> for Mesh {
    fn from(faces: Vec<Face>) -> Self {
        Self {
            faces,
            material: Phong::default(),
        }
    }
}

impl Intersect for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.faces
            .iter()
            .filter_map(|x| x.intersect(ray))
            .min_by(|i1, i2| i1.partial_cmp(i2).unwrap())
    }
}

impl Textured for Mesh {
    fn to_texture_space(&self, p: &Point3) -> Point2 {
        Point2::default()
    }
}
