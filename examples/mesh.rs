use raytox::algebra::{Point3, Vector3};
use raytox::camera::Camera;
use raytox::color::Color;
use raytox::entity::Entity;
use raytox::geometry::Mesh;
use raytox::geometry::Plane;
use raytox::light::PointLight;
use raytox::material::Phong;
use raytox::scene::Scene;
use raytox::texture::Checker;
use raytox::texture::Texture;

fn main() {
    let mut camera = Camera::new(Point3::new(0.0, 0.0, 20.0));
    camera.look_at(Point3::O);
    let mut scene = Scene::new(
        800,
        600,
        70.0_f64.to_radians(),
        camera,
        Color::new(0.00, 0.03, 0.03),
    );

    scene.add_entity(
        Entity::from(Plane::new(Point3::new(0.0, -10.0, 0.0), Vector3::J))
            .with_material(Phong::new(
                Color::WHITE * 0.03,
                Texture::new(5.0, Checker::new(Color::WHITE * 0.4, Color::WHITE * 0.03)),
                Color::WHITE,
                20.0,
            ))
            .build(),
    );

    scene.add_entity(
        Entity::from(Mesh::from_ply_file("assets/beethoven.ply").expect("Failed to load ply"))
            .with_material(Phong::new(
                Color::WHITE * 0.03,
                Color::BLUE,
                Color::WHITE,
                20.0,
            ))
            .build(),
    );

    scene.add_light(PointLight::new(
        Point3::new(0.0, 10.0, 10.0),
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        200.0,
    ));

    scene.render().save("output/mesh.png").unwrap();
}
