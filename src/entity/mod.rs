pub use entity::Entity;

use crate::{
    algebra::{Matrix4, Vector3},
    geometry::Axis,
};

mod entity;

pub trait Transformable {
    fn translate(self, translation: Vector3) -> Self;
    fn rotate(self, axis: Axis, degrees: f64) -> Self;
    fn scale(self, scale: Vector3) -> Self;
    fn transform(self, transform: Matrix4) -> Self;
}
