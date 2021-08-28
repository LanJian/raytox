use piston_window::{EventLoop, RenderEvent, UpdateEvent};
use raytox::algebra::{Point3, Vector3};
use raytox::camera::Camera;
use raytox::color::Color;
use raytox::entity::{Entity, Transformable};
use raytox::geometry::{Plane, Sphere};
use raytox::geometry::Cube;
use raytox::light::PointLight;
use raytox::material::Phong;
use raytox::scene::Scene;
use raytox::texture::Texture;
use raytox::texture::{Checker, Image};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 600;

fn main() {
    let mut camera = Camera::new(Point3::new(0.0, 3.0, 0.0));
    camera.look_at(Point3::new(0.0, 0.0, 30.0));

    let mut scene = Scene::new(
        WIDTH,
        HEIGHT,
        120.0_f64.to_radians(),
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
            .build(),
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
            .build(),
    );
    scene.add_entity(
        Entity::from(Sphere::default())
            .with_material(Phong::random_color().with_reflectance(0.1))
            .scale(Vector3::new(5.0, 5.0, 5.0))
            .translate(Vector3::new(-15.0, 0.0, 30.0))
            .build(),
    );
    scene.add_entity(
        Entity::from(Sphere::default())
            .with_material(Phong::random_color().with_reflectance(0.5))
            .scale(Vector3::new(5.0, 5.0, 5.0))
            .translate(Vector3::new(0.0, 0.0, 30.0))
            .build(),
    );
    scene.add_entity(
        Entity::from(Sphere::default())
            .with_material(Phong::random_color().with_reflectance(0.9))
            .scale(Vector3::new(5.0, 5.0, 5.0))
            .translate(Vector3::new(15.0, 0.0, 30.0))
            .build(),
    );

    scene.add_light(PointLight::new(
        Point3::new(0.0, 50.0, 0.0),
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        f64::INFINITY,
    ));

    let mut window: piston_window::PistonWindow =
    piston_window::WindowSettings::new("Raytracer", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|_e| { panic!("Could not create window!")});


    let mut events = piston_window::Events::new(piston_window::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            let rendered = scene.render();
            let frame_buffer = rendered.into_rgba8();

            let tex = piston_window::Texture::from_image(
                &mut window.create_texture_context(),
                &frame_buffer,
                &piston_window::TextureSettings::new())
                .unwrap();

            window.draw_2d(&e, |c, g, _| {
                piston_window::clear([1.0; 4], g);
                piston_window::image(&tex, c.transform, g)
            });
        }

        if let Some(args) = e.update_args() {
            let new_pos = scene.camera.position
                .translate(Vector3::new(0.0, 5.0 * args.dt, 0.0));
            scene.camera.position = new_pos;
            scene.camera.look_at(Point3::new(0.0, 0.0, 30.0));
        }
    }
}
