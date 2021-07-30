use image::DynamicImage;
use image::GenericImageView;

use super::ColoredTexture;
use super::TextureCoordinate;
use crate::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    image: DynamicImage,
}

impl From<DynamicImage> for Image {
    fn from(image: DynamicImage) -> Self {
        Self { image }
    }
}

impl ColoredTexture for Image {
    fn color_at(&self, p: TextureCoordinate) -> Color {
        let w = (self.image.width() - 1) as f64;
        let h = (self.image.height() - 1) as f64;
        let x = (p.u() * w).round() as u32;
        let y = ((1.0 - p.v()) * h).round() as u32;

        //dbg!(self.image.get_pixel(x, y).into())
        self.image.get_pixel(x, y).into()
    }
}
