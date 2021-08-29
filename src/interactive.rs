use piston_window::{
    AdvancedWindow, Button, Event, EventSettings, Events, FocusEvent, Key, MouseRelativeEvent,
    PistonWindow, PressEvent, ReleaseEvent, RenderEvent, ResizeArgs, ResizeEvent, UpdateEvent,
};

use crate::{
    algebra::{Point2, Vector3},
    entity::Transformable,
    scene::Scene,
};

pub struct InteractiveWindow {
    window: PistonWindow,
    scene: Scene,

    is_focused: bool,
    cursor_pos: Point2,
    key_states: (bool, bool, bool, bool),
    events: Events,

    move_speed: f64,
}

impl InteractiveWindow {
    pub fn new(scene: Scene) -> Self {
        let window: piston_window::PistonWindow =
            piston_window::WindowSettings::new("Raytracer", [scene.width, scene.height])
                .exit_on_esc(true)
                .build()
                .expect("Could not create window!");

        Self {
            window,
            scene,

            is_focused: false,
            cursor_pos: Point2::default(),
            key_states: (false, false, false, false),
            events: Events::new(EventSettings::new()),

            move_speed: 150.0,
        }
    }

    pub fn with_move_speed(mut self, move_speed: f64) -> Self {
        self.move_speed = move_speed;
        self
    }

    pub fn start(&mut self) {
        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.resize_args() {
                self.handle_resize(args)
            }

            if let Some(focus) = e.focus_args() {
                self.handle_focus(focus);
            }

            if self.is_focused {
                if let Some(pos) = e.mouse_relative_args() {
                    self.handle_mouse_relative(pos.into());
                }

                if let Some(Button::Keyboard(key)) = e.press_args() {
                    self.handle_key_press(key);
                }

                if let Some(Button::Keyboard(key)) = e.release_args() {
                    self.handle_key_release(key);
                }
            }

            if let Some(args) = e.update_args() {
                self.handle_update(args.dt);
            }

            if let Some(_) = e.render_args() {
                self.handle_render(e);
            }
        }
    }

    // event handlers
    fn handle_resize(&mut self, args: ResizeArgs) {
        let window_size = args.window_size;
        self.scene
            .set_size(window_size[0] as u32, window_size[1] as u32);
    }

    fn handle_focus(&mut self, is_focused: bool) {
        self.is_focused = is_focused;
        self.window.set_capture_cursor(is_focused);
        self.cursor_pos.x = self.scene.width as f64 / 2.0;
        self.cursor_pos.y = self.scene.height as f64 / 2.0;
    }

    fn handle_mouse_relative(&mut self, pos: Point2) {
        let coord = [
            (self.cursor_pos.x + pos.x) as i32,
            (self.cursor_pos.y + pos.y) as i32,
        ];
        let ray = self.scene.ray_to_screen_space(coord[0], coord[1]);
        self.scene.camera.look_at(ray.origin + ray.dir);
    }

    fn handle_key_press(&mut self, key: Key) {
        match key {
            Key::W => self.key_states.0 = true,
            Key::A => self.key_states.1 = true,
            Key::S => self.key_states.2 = true,
            Key::D => self.key_states.3 = true,
            _ => {}
        }
    }

    fn handle_key_release(&mut self, key: Key) {
        match key {
            Key::W => self.key_states.0 = false,
            Key::A => self.key_states.1 = false,
            Key::S => self.key_states.2 = false,
            Key::D => self.key_states.3 = false,
            _ => {}
        }
    }

    fn handle_update(&mut self, dt: f64) {
        let mut translation = Vector3::ZERO;
        if self.key_states.0 {
            // W
            translation = translation + self.scene.camera.view * self.move_speed;
        }
        if self.key_states.1 {
            // A
            translation = translation - self.scene.camera.side * self.move_speed;
        }
        if self.key_states.2 {
            // S
            translation = translation - self.scene.camera.view * self.move_speed;
        }
        if self.key_states.3 {
            // D
            translation = translation + self.scene.camera.side * self.move_speed;
        }

        let new_pos = self.scene.camera.position.translate(translation * dt);
        self.scene.camera.position = new_pos;
    }

    fn handle_render(&mut self, e: Event) {
        let rendered = self.scene.render();
        let frame_buffer = rendered.into_rgba8();

        let tex = piston_window::Texture::from_image(
            &mut self.window.create_texture_context(),
            &frame_buffer,
            &piston_window::TextureSettings::new(),
        )
        .unwrap();

        self.window.draw_2d(&e, |c, g, _| {
            piston_window::clear([0.0; 4], g);
            piston_window::image(&tex, c.transform, g)
        });
    }
}
