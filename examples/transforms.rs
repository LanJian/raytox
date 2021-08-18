use raytox::algebra::{Point3, Vector3};
use raytox::camera::Camera;
use raytox::color::Color;
use raytox::entity::{Entity, Transformable};
use raytox::geometry::Plane;
use raytox::geometry::{Axis, Cube};
use raytox::light::PointLight;
use raytox::material::Phong;
use raytox::scene::Scene;
use raytox::texture::Checker;
use raytox::texture::Texture;

fn main() {
    let mut camera = Camera::new(Point3::new(0.0, 0.0, -35.0));
    camera.look_at(Point3::O);

    let mut scene = Scene::new(
        800,
        600,
        70.0_f64.to_radians(),
        camera,
        Color::new(0.00, 0.03, 0.03),
    );

    scene.add_entity(
        Entity::from(Plane::default())
            .with_material(Phong::new(
                Color::WHITE * 0.03,
                Texture::new(5.0, Checker::new(Color::WHITE * 0.4, Color::WHITE * 0.03)),
                Color::WHITE,
                20.0,
            ))
            .translate(Vector3::new(0.0, -10.0, 0.0))
            .build(),
    );
    scene.add_entity(
        Entity::from(Cube::default())
            .with_material(Phong::random_color())
            .scale(Vector3::new(5.0, 5.0, 5.0))
            .rotate(Axis::X, -45.0)
            .rotate(Axis::Y, 45.0)
            .translate(Vector3::new(5.0, 0.0, 0.0))
            .build(),
    );
    scene.add_entity(
        Entity::from(Cube::default())
            .with_material(Phong::random_color())
            .scale(Vector3::new(3.0, 10.0, 5.0))
            .rotate(Axis::X, -22.0)
            .rotate(Axis::Y, 30.0)
            .rotate(Axis::Z, 60.0)
            .translate(Vector3::new(-7.0, 4.0, 0.0))
            .build(),
    );

    scene.add_light(PointLight::new(
        Point3::new(-2.0, -4.0, -3.0),
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        50.0,
    ));
    scene.add_light(PointLight::new(
        Point3::new(0.0, 20.0, -12.0),
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        300.0,
    ));
    scene.add_light(PointLight::new(
        Point3::new(20.0, 10.0, -5.0),
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        300.0,
    ));

    scene.render(String::from("examples/transforms.png"));
}
