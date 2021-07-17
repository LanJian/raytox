use algebra::{point3::Point3, vector3::Vector3};
use camera::Camera;
use scene::Scene;

use crate::shape::Sphere;

mod algebra;
mod camera;
mod scene;
mod shape;
mod color;

fn main() {
    let camera = Camera::new(Point3::new(0.0, 0.0, -20.0), Vector3::K, Vector3::J);
    println!("{:?}", camera);

    let mut scene = Scene::new(800, 600, 70.0_f64.to_radians(), camera);
    scene.add_object(Sphere::new(Point3::new(0.0, 0.0, 5.0), 5.0));

    scene.render();
}
