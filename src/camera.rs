use crate::algebra::{EPSILON, Point3, Vector3};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Camera {
    pub position: Point3,
    pub view: Vector3,
    pub up: Vector3,
    pub side: Vector3,
}

impl Camera {
    pub fn new(position: Point3) -> Self {
        Self {
            position,
            view: Vector3::K,
            up: Vector3::J,
            side: Vector3::J.cross(&Vector3::K),
        }
    }

    pub fn look_at(&mut self, point: Point3) {
        // view vector
        self.view = (point - self.position).normalize();

        // side vector
        let mut candidate = Vector3::J.cross(&self.view);
        if candidate.magnitude() < EPSILON {
            candidate = Vector3::K.cross(&self.view);
        }
        self.side = candidate.normalize();

        // up vector
        self.up = self.view.cross(&self.side).normalize();
    }
}
