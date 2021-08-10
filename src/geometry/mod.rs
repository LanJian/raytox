pub use cube::Cube;
pub use geometry::Geometry;
pub use geometry::Intersect;
pub use geometry::Intersection;
pub use geometry::Textured;
pub use plane::Plane;
pub use polygon::Face;
pub use polygon::Mesh;
pub use polygon::Vertex;
pub use sphere::Sphere;

mod cube;
mod geometry;
mod plane;
mod polygon;
mod sphere;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}
