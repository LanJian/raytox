use std::convert::TryFrom;

use raytox::algebra::{Point3, Vector3};
use raytox::camera::Camera;
use raytox::color::Color;
use raytox::geometry::Face;
use raytox::geometry::Mesh;
use raytox::geometry::Plane;
use raytox::geometry::Sphere;
use raytox::geometry::Vertex;
use raytox::light::PointLight;
use raytox::material::Phong;
use raytox::scene::Scene;
use raytox::texture::Checker;
use raytox::texture::Image;
use raytox::texture::Texture;

fn main() {
    let camera = Camera::new(Point3::new(0.0, 0.0, -35.0), Vector3::K, Vector3::J);
    let mut scene = Scene::new(
        800,
        600,
        70.0_f64.to_radians(),
        camera,
        Color::new(0.00, 0.03, 0.03),
    );

    scene.add_object(Plane::new(
        Point3::new(0.0, -10.0, 0.0),
        Vector3::J,
        Phong::new(
            Color::WHITE * 0.03,
            Texture::new(5.0, Checker::new(Color::WHITE * 0.4, Color::WHITE * 0.03)),
            Color::WHITE,
            20.0,
        ),
    ));
    scene.add_object(Sphere::new(
        Point3::new(-5.0, 0.0, 5.0),
        5.0,
        Phong::new(Color::WHITE * 0.03, Color::GREEN, Color::WHITE, 20.0),
    ));
    scene.add_object(Sphere::new(
        Point3::new(5.0, 0.0, 0.0),
        6.0,
        Phong::new(
            Color::WHITE * 0.03,
            Texture::new(1.0, Image::from(image::open("assets/earth.jpg").unwrap())),
            Color::WHITE * 0.1,
            20.0,
        ),
    ));
    scene.add_object(
        Mesh::from(vec![
            Face::try_from(vec![
                // front face
                Vertex::from(Point3::new(-5.0, 3.0, -8.0)),
                Vertex::from(Point3::new(-11.0, 3.0, -8.0)),
                Vertex::from(Point3::new(-11.0, -3.0, -8.0)),
                Vertex::from(Point3::new(-5.0, -3.0, -8.0)),
            ])
            .expect("invalid polygon face"),
            Face::try_from(vec![
                // back face
                Vertex::from(Point3::new(-5.0, 3.0, -2.0)),
                Vertex::from(Point3::new(-5.0, -3.0, -2.0)),
                Vertex::from(Point3::new(-11.0, -3.0, -2.0)),
                Vertex::from(Point3::new(-11.0, 3.0, -2.0)),
            ])
            .expect("invalid polygon face"),
            Face::try_from(vec![
                // top face
                Vertex::from(Point3::new(-5.0, 3.0, -2.0)),
                Vertex::from(Point3::new(-11.0, 3.0, -2.0)),
                Vertex::from(Point3::new(-11.0, 3.0, -8.0)),
                Vertex::from(Point3::new(-5.0, 3.0, -8.0)),
            ])
            .expect("invalid polygon face"),
            Face::try_from(vec![
                // bottom face
                Vertex::from(Point3::new(-5.0, -3.0, -2.0)),
                Vertex::from(Point3::new(-5.0, -3.0, -8.0)),
                Vertex::from(Point3::new(-11.0, -3.0, -8.0)),
                Vertex::from(Point3::new(-11.0, -3.0, -2.0)),
            ])
            .expect("invalid polygon face"),
            Face::try_from(vec![
                // left face
                Vertex::from(Point3::new(-11.0, 3.0, -8.0)),
                Vertex::from(Point3::new(-11.0, 3.0, -2.0)),
                Vertex::from(Point3::new(-11.0, -3.0, -2.0)),
                Vertex::from(Point3::new(-11.0, -3.0, -8.0)),
            ])
            .expect("invalid polygon face"),
            Face::try_from(vec![
                // right face
                Vertex::from(Point3::new(-5.0, 3.0, -2.0)),
                Vertex::from(Point3::new(-5.0, 3.0, -8.0)),
                Vertex::from(Point3::new(-5.0, -3.0, -8.0)),
                Vertex::from(Point3::new(-5.0, -3.0, -2.0)),
            ])
            .expect("invalid polygon face"),
        ])
        .with_material(Phong::new(
            Color::WHITE * 0.03,
            Color::BLUE,
            Color::WHITE,
            20.0,
        )),
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

    scene.render(String::from("examples/basic.png"));
}
