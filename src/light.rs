use crate::algebra::Point3;
use crate::color::Color;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    pub position: Point3,
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
}

impl PointLight {
    pub fn new(position: Point3, ambient: Color, diffuse: Color, specular: Color) -> Self {
        Self { position, ambient, diffuse, specular }
    }
}
