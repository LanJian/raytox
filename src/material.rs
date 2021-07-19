use crate::color::Color;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Phong {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub shininess: f64,
}

impl Phong {
    pub fn new(ambient: Color, diffuse: Color, specular: Color, shininess: f64) -> Self {
        Self { ambient, diffuse, specular, shininess }
    }
}
