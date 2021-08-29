use raytox::algebra::{Point3, Vector3};
use raytox::camera::Camera;
use raytox::color::Color;
use raytox::entity::{Entity, Transformable};
use raytox::geometry::Cube;
use raytox::geometry::{Plane, Sphere};
use raytox::interactive::InteractiveWindow;
use raytox::light::PointLight;
use raytox::material::Phong;
use raytox::scene::Scene;
use raytox::texture::Texture;
use raytox::texture::{Checker, Image};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 400;

fn main() {
    let mut camera = Camera::new(Point3::new(0.0, 3.0, 0.0));
    camera.look_at(Point3::new(0.0, 0.0, 30.0));

    let mut scene = Scene::new(
        WIDTH,
        HEIGHT,
        70.0_f64.to_radians(),
        camera,
        Color::new(0.00, 0.03, 0.03),
    );

    scene.add_entity(
        Entity::from(Plane::default())
            .with_material(Phong::new(
                Color::WHITE * 0.03,
                Texture::new(7.0, Checker::new(Color::WHITE * 0.4, Color::WHITE * 0.03)),
                Color::BLACK,
                20.0,
            ))
            .translate(Vector3::new(0.0, -15.0, 0.0))
    );
    scene.add_entity(
        Entity::from(Cube::default().with_flipped_normals())
            .with_material(Phong::new(
                Color::WHITE * 0.03,
                Image::from(image::open("assets/space_cubemap.jpg").unwrap()),
                Color::WHITE * 0.01,
                20.0,
            ))
            .scale(Vector3::new(2000000.0, 2000000.0, 2000000.0))
    );
    scene.add_entity(
        Entity::from(Sphere::default())
            .with_material(Phong::random_color().with_reflectance(0.1))
            .scale(Vector3::new(5.0, 5.0, 5.0))
            .translate(Vector3::new(-15.0, 0.0, 30.0))
    );
    scene.add_entity(
        Entity::from(Sphere::default())
            .with_material(Phong::random_color().with_reflectance(0.5))
            .scale(Vector3::new(5.0, 5.0, 5.0))
            .translate(Vector3::new(0.0, 0.0, 30.0))
    );
    scene.add_entity(
        Entity::from(Sphere::default())
            .with_material(Phong::random_color().with_reflectance(0.9))
            .scale(Vector3::new(5.0, 5.0, 5.0))
            .translate(Vector3::new(15.0, 0.0, 30.0))
    );

    scene.add_light(PointLight::new(
        Point3::new(0.0, 50.0, 0.0),
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        f64::INFINITY,
    ));

    let mut window = InteractiveWindow::new(scene);
    window.start();
}
