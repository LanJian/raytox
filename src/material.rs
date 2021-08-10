use crate::{color::Color, texture::Texture};

#[derive(Debug, Clone, PartialEq)]
pub struct Phong {
    pub ambient: Texture,
    pub diffuse: Texture,
    pub specular: Texture,
    pub shininess: f64,
}

impl Phong {
    pub fn new(
        ambient: impl Into<Texture>,
        diffuse: impl Into<Texture>,
        specular: impl Into<Texture>,
        shininess: f64,
    ) -> Self {
        Self {
            ambient: ambient.into(),
            diffuse: diffuse.into(),
            specular: specular.into(),
            shininess,
        }
    }

    pub fn random_color() -> Self {
        Self::new(Color::WHITE * 0.03, Color::random(), Color::WHITE, 20.0)
    }
}

impl Default for Phong {
    fn default() -> Self {
        Self::new(Color::WHITE * 0.03, Color::BLUE, Color::WHITE, 20.0)
    }
}
