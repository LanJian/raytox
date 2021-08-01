use crate::algebra::Point3;
use crate::color::Color;


#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    pub position: Point3,
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub intensity: f64,
}

impl PointLight {
    pub fn new(
        position: Point3,
        ambient: Color,
        diffuse: Color,
        specular: Color,
        intensity: f64,
    ) -> Self {
        Self {
            position,
            ambient,
            diffuse,
            specular,
            intensity,
        }
    }

    pub fn intensity_at(&self, p: &Point3) -> f64 {
        let r2 = (*p - self.position).norm();
        self.intensity / r2
    }
}
