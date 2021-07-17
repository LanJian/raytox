use crate::algebra::{point3::Point3, vector3::Vector3};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Camera {
    pub position: Point3,
    pub view: Vector3,
    pub up: Vector3,
    pub side: Vector3,
}

impl Camera {
    pub fn new(position: Point3, view: Vector3, up: Vector3) -> Self {
        Self {
            position,
            view,
            up,
            side: up.cross(&view),
        }
    }
}
