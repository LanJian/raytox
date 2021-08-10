mod matrix;
mod point;
mod ray;
mod vector;

pub use matrix::Matrix4;
pub use point::Point2;
pub use point::Point3;
pub use ray::Ray;
pub use vector::Vector3;
pub use vector::Vector4;

pub const EPSILON: f64 = 1e-6;
