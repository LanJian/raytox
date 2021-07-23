use crate::{
    algebra::{Point2, Point3},
    color::Color,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Texture {
    scale: f64,
    data: TextureData,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureData {
    Color(Color),
    Checker(Checker),
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct TextureCoordinate(Point2);

impl TextureCoordinate {
    pub fn new(u: f64, v: f64) -> Self {
        Self(Point2::new(u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)))
    }

    pub fn u(&self) -> f64 {
        self.0.x
    }

    pub fn v(&self) -> f64 {
        self.0.y
    }
}

impl Texture {
    pub fn new(scale: f64, data: impl Into<TextureData>) -> Self {
        Self { scale, data: data.into() }
    }

    pub fn color_at(&self, p: &Point2) -> Color {
        let uv = TextureCoordinate(Point2::new(
            ((*p).x / self.scale).rem_euclid(1.0),
            ((*p).y / self.scale).rem_euclid(1.0),
        ));

        match self.data {
            TextureData::Color(c) => c,
            TextureData::Checker(c) => c.color_at(&uv),
        }
    }
}

impl From<Color> for TextureData {
    fn from(c: Color) -> Self {
        Self::Color(c)
    }
}

impl From<Checker> for TextureData {
    fn from(c: Checker) -> Self {
        Self::Checker(c)
    }
}

impl From<Color> for Texture {
    fn from(c: Color) -> Self {
        Self::new(1.0, TextureData::Color(c))
    }
}

impl From<Checker> for Texture {
    fn from(c: Checker) -> Self {
        Self::new(1.0, TextureData::Checker(c))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Checker {
    primary: Color,
    secondary: Color,
}

impl Checker {
    pub fn new(primary: Color, secondary: Color) -> Self {
        Self { primary, secondary }
    }

    pub fn color_at(&self, p: &TextureCoordinate) -> Color {
        if (p.u() < 0.5) ^ (p.v() < 0.5) {
            return self.primary;
        }

        self.secondary
    }
}

impl Default for Checker {
    fn default() -> Self {
        Self::new(Color::WHITE, Color::WHITE * 0.2)
    }
}
