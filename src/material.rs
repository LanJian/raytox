use crate::{color::Color, texture::Texture};

#[derive(Debug, Clone, Copy, PartialEq)]
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
}

impl Default for Phong {
    fn default() -> Self {
        Self::new(Color::default(), Color::default(), Color::default(), 0.0)
    }
}
