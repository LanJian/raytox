use crate::Color;
use super::{ColoredTexture, TextureCoordinate};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Checker {
    primary: Color,
    secondary: Color,
}

impl Checker {
    pub fn new(primary: Color, secondary: Color) -> Self {
        Self { primary, secondary }
    }
}

impl ColoredTexture for Checker {
    fn color_at(&self, p: TextureCoordinate) -> Color {
        if (p.u() < 0.5) ^ (p.v() < 0.5) {
            return self.primary;
        }

        self.secondary
    }
}

impl Default for Checker {
    fn default() -> Self {
        Self::new(Color::WHITE, Color::WHITE * 0.1)
    }
}
