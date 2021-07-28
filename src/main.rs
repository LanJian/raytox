use algebra::{Point3, Vector3};
use camera::Camera;
use color::Color;
use geometry::Mesh;
use geometry::Plane;
use geometry::Sphere;
use light::PointLight;
use material::Phong;
use scene::Scene;
use std::convert::TryFrom;
use texture::Checker;
use texture::Texture;

use crate::geometry::Face;
use crate::geometry::Vertex;
use crate::texture::Image;

mod algebra;
mod camera;
mod color;
mod geometry;
mod light;
mod material;
mod scene;
mod texture;

fn main() {
    let camera = Camera::new(Point3::new(0.0, 0.0, -35.0), Vector3::K, Vector3::J);
    let mut scene = Scene::new(
        1920,
        1080,
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
            //Texture::new(5.0, Image::from(image::open("resized_test.png").unwrap())),
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
            Texture::new(1.0, Image::from(image::open("resized_earth.jpg").unwrap())),
            Color::WHITE,
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
            Color::GREEN,
            Color::WHITE,
            20.0,
        )),
    );

    let lights = [
        PointLight::new(
            Point3::new(50.0, 100.0, 0.0),
            Color::WHITE,
            Color::WHITE,
            Color::WHITE,
        ),
        PointLight::new(
            Point3::new(-100.0, 50.0, -50.0),
            Color::WHITE,
            Color::WHITE,
            Color::WHITE,
        ),
    ];

    lights.iter().for_each(|light| scene.add_light(*light));

    scene.render(String::from("out.png"));
}
