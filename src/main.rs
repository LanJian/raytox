use algebra::{Point3, Vector3};
use camera::Camera;
use color::Color;
use geometry::Plane;
use geometry::Sphere;
use light::PointLight;
use material::Phong;
use scene::Scene;

mod algebra;
mod camera;
mod color;
mod geometry;
mod light;
mod material;
mod scene;

fn main() {
    let camera = Camera::new(Point3::new(0.0, 0.0, -20.0), Vector3::K, Vector3::J);
    let mut scene = Scene::new(
        800,
        600,
        70.0_f64.to_radians(),
        camera,
        Color::new(0.00, 0.03, 0.03),
    );

    scene.add_object(
        Plane::new(
            Point3::new(0.0, -10.0, 0.0),
            Vector3::J,
            Phong::new(
                Color::WHITE * 0.03,
                Color::WHITE * 0.4,
                Color::WHITE,
                20.0,
            ),
        )
    );
    scene.add_object(
        Sphere::new(
            Point3::new(-5.0, 0.0, 5.0),
            5.0,
            Phong::new(
                Color::WHITE * 0.03,
                Color::GREEN,
                Color::WHITE,
                20.0,
            ),
        )
    );
    scene.add_object(
        Sphere::new(
            Point3::new(5.0, 0.0, 0.0),
            6.0,
            Phong::new(
                Color::WHITE * 0.03,
                Color::GREEN,
                Color::WHITE,
                20.0,
            ),
        )
    );

    let lights = [
        PointLight::new(
            Point3::new(50.0, 100.0, 0.0),
            Color::WHITE,
            Color::WHITE,
            Color::WHITE,
        ),
        PointLight::new(
            Point3::new(-100.0, 50.0, 0.0),
            Color::WHITE,
            Color::WHITE,
            Color::WHITE,
        ),
    ];

    lights.iter().for_each(|light| scene.add_light(*light));

    scene.render(String::from("out.png"));
}
