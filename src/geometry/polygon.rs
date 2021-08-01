use std::convert::TryFrom;
use std::fs::File;

use ply_rs::{
    parser::Parser,
    ply::{DefaultElement, Property},
};

use crate::{
    algebra::{Point2, Point3, Ray, Vector3, EPSILON},
    material::Phong,
};

use super::{Cube, Intersect, Intersection, Plane, Textured};

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
    bounding_box: Cube,

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

    fn calculate_bounding_box(faces: &Vec<Face>) -> Cube {
        let mut min_bounds = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max_bounds = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for face in faces {
            for vertex in &face.vertices {
                let p = vertex.point;

                if p.x < min_bounds.x {
                    min_bounds.x = p.x
                }
                if p.x > max_bounds.x {
                    max_bounds.x = p.x
                }

                if p.y < min_bounds.y {
                    min_bounds.y = p.y
                }
                if p.y > max_bounds.y {
                    max_bounds.y = p.y
                }

                if p.z < min_bounds.z {
                    min_bounds.z = p.z
                }
                if p.z > max_bounds.z {
                    max_bounds.z = p.z
                }
            }
        }

        Cube::new(min_bounds, max_bounds)
    }
}

impl From<Vec<Face>> for Mesh {
    fn from(faces: Vec<Face>) -> Self {
        let bounding_box = Self::calculate_bounding_box(&faces);

        Self {
            faces,
            bounding_box,
            material: Phong::default(),
        }
    }
}

impl Intersect for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.bounding_box.intersect(ray).is_none() {
            return None;
        }

        self.faces
            .iter()
            .filter_map(|x| x.intersect(ray))
            .min_by(|i1, i2| i1.partial_cmp(i2).unwrap())
    }
}

impl Textured for Mesh {
    fn to_texture_space(&self, _p: &Point3) -> Point2 {
        Point2::default()
    }
}
