use crate::algebra::Point2;
use crate::color::Color;

pub use self::image::Image;
pub use checker::Checker;

pub mod checker;
pub mod image;

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

#[derive(Debug, Clone, PartialEq)]
pub enum TextureData {
    Color(Color),
    Checker(Checker),
    Image(Image),
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

impl From<Image> for TextureData {
    fn from(i: Image) -> Self {
        Self::Image(i)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    scale: f64,
    data: TextureData,
}

impl Texture {
    pub fn new(scale: f64, data: impl Into<TextureData>) -> Self {
        Self {
            scale,
            data: data.into(),
        }
    }

    pub fn color_at(&self, p: &Point2) -> Color {
        let uv = TextureCoordinate(Point2::new(
            ((*p).x / self.scale).rem_euclid(1.0),
            ((*p).y / self.scale).rem_euclid(1.0),
        ));

        match &self.data {
            TextureData::Color(x) => *x,
            TextureData::Checker(x) => x.color_at(uv),
            TextureData::Image(x) => x.color_at(uv),
        }
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

impl From<Image> for Texture {
    fn from(i: Image) -> Self {
        Self::new(1.0, TextureData::Image(i))
    }
}

pub trait ColoredTexture {
    fn color_at(&self, p: TextureCoordinate) -> Color;
}
