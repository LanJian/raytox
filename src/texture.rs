use crate::{
    algebra::{Point2, Point3},
    color::Color,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Texture {
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
    pub fn color_at(&self, p: &TextureCoordinate) -> Color {
        match self {
            Self::Color(c) => *c,
            Self::Checker(c) => c.color_at(p),
        }
    }
}

impl From<Color> for Texture {
    fn from(c: Color) -> Self {
        Self::Color(c)
    }
}

impl From<Checker> for Texture {
    fn from(c: Checker) -> Self {
        Self::Checker(c)
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
            return self.primary
        }

        self.secondary
    }
}

impl Default for Checker {
    fn default() -> Self {
        Self::new(Color::WHITE, Color::WHITE * 0.2)
    }
}
