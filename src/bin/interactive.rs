use piston_window::{
    AdvancedWindow, Button, FocusEvent, Key, MouseRelativeEvent, PressEvent,
    ReleaseEvent, RenderEvent, ResizeEvent, UpdateEvent,
};
use raytox::algebra::{Point3, Vector3};
use raytox::camera::Camera;
use raytox::color::Color;
use raytox::entity::{Entity, Transformable};
use raytox::geometry::Cube;
use raytox::geometry::{Plane, Sphere};
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
            .unwrap_or_else(|_e| panic!("Could not create window!"));

    let speed = 150.0;

    let mut is_focused = false;
    let mut cursor_pos = [0.0, 0.0];
    let mut key_states = (false, false, false, false);
    let mut events = piston_window::Events::new(piston_window::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.resize_args() {
            let window_size = args.window_size;
            scene.set_size(window_size[0] as u32, window_size[1] as u32);
        }

        if let Some(focus) = e.focus_args() {
            is_focused = focus;
            window.set_capture_cursor(is_focused);
            cursor_pos = [scene.width as f64 / 2.0, scene.height as f64 / 2.0];
        }

        if is_focused {
            if let Some(pos) = e.mouse_relative_args() {
                //dbg!(pos);
                let coord = [
                    (cursor_pos[0] + pos[0]) as i32,
                    (cursor_pos[1] + pos[1]) as i32,
                ];
                let ray = scene.ray_to_screen_space(coord[0], coord[1]);
                scene.camera.look_at(ray.origin + ray.dir);
            }

            if let Some(Button::Keyboard(key)) = e.press_args() {
                match key {
                    Key::W => key_states.0 = true,
                    Key::A => key_states.1 = true,
                    Key::S => key_states.2 = true,
                    Key::D => key_states.3 = true,
                    _ => {}
                }
            }

            if let Some(Button::Keyboard(key)) = e.release_args() {
                match key {
                    Key::W => key_states.0 = false,
                    Key::A => key_states.1 = false,
                    Key::S => key_states.2 = false,
                    Key::D => key_states.3 = false,
                    _ => {}
                }
            }
        }

        if let Some(args) = e.update_args() {
            let mut translation = Vector3::ZERO;
            if key_states.0 {
                // W
                translation = translation + scene.camera.view * speed;
            }
            if key_states.1 {
                // A
                translation = translation - scene.camera.side * speed;
            }
            if key_states.2 {
                // S
                translation = translation - scene.camera.view * speed;
            }
            if key_states.3 {
                // D
                translation = translation + scene.camera.side * speed;
            }

            let new_pos = scene.camera.position.translate(translation * args.dt);
            scene.camera.position = new_pos;
        }

        if let Some(_) = e.render_args() {
            let rendered = scene.render();
            let frame_buffer = rendered.into_rgba8();

            let tex = piston_window::Texture::from_image(
                &mut window.create_texture_context(),
                &frame_buffer,
                &piston_window::TextureSettings::new(),
            )
            .unwrap();

            window.draw_2d(&e, |c, g, _| {
                piston_window::clear([0.0; 4], g);
                piston_window::image(&tex, c.transform, g)
            });
        }
    }
}
