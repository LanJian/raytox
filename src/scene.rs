use image::{DynamicImage, GenericImage};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::prelude::*;

use crate::{
    algebra::{Ray, EPSILON},
    camera::Camera,
    color::Color,
    entity::Entity,
    geometry::{Intersect, Intersection, Textured},
    light::PointLight,
};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub camera: Camera,
    pub entities: Vec<Entity>,
    pub lights: Vec<PointLight>,
    pub background: Color,
}

impl Scene {
    pub fn new(width: u32, height: u32, fov: f64, camera: Camera, background: Color) -> Scene {
        Self {
            width,
            height,
            fov,
            camera,
            entities: Vec::new(),
            lights: Vec::new(),
            background,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn ray_to_screen_space(&self, x: i32, y: i32) -> Ray {
        let width = self.width as i32;
        let height = self.height as i32;

        let d = (width as f64 / 2.0) / (self.fov / 2.0).tan();
        let raw = d * self.camera.view
            + (x - (width / 2)) as f64 * self.camera.side
            + ((height / 2) - y) as f64 * self.camera.up;
        Ray::new(self.camera.position, raw)
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn render(&self) -> DynamicImage {
        let mut img = DynamicImage::new_rgb8(self.width, self.height);

        let width = self.width as i32;
        let height = self.height as i32;
        let max_y = height - 1;

        let screen: Vec<(i32, i32)> = (0..width).cartesian_product(0..height).collect();

        //let pb = ProgressBar::new(screen.len() as u64);
        //pb.set_style(ProgressStyle::default_bar().template(
            //"{spinner:.cyan} {msg:.green} [{elapsed_precise}] \
            //{wide_bar:.magenta/white.dim} {percent}% ({eta})",
        //));
        //pb.set_message("Rendering");

        let pixels: Vec<(i32, i32, Color)> = screen
            .into_par_iter()
            .map(|(x, y)| {
                let ray = self.ray_to_screen_space(x, y);
                let ret = (x, y, self.trace(ray.normalize(), 5));

                //if y == max_y {
                    //pb.inc(height as u64);
                //}

                ret
            })
            .collect();

        for (x, y, color) in pixels {
            img.put_pixel(x as u32, y as u32, color.into());
        }

        //img.save(outfile).unwrap();

        //pb.finish();

        return img;
    }

    fn trace(&self, ray: Ray, depth: i32) -> Color {
        if depth == 0 {
            return self.background;
        }

        match self.closest_intersection(&ray) {
            Some((
                entity,
                Intersection {
                    position: intersect_point,
                    normal,
                    ..
                },
            )) => {
                let material = entity.material();
                let uv = entity.to_texture_space(&intersect_point);

                let ka = material.ambient.color_at(&uv);
                let kd = material.diffuse.color_at(&uv);
                let ks = material.specular.color_at(&uv);
                let alpha = material.shininess;
                let reflectance = material.reflectance;

                let n = normal;
                let v = -ray.dir;
                let offset_position = intersect_point + EPSILON * normal;

                let local_color = self
                    .lights
                    .iter()
                    .map(|light| {
                        let intensity = light.intensity_at(&intersect_point);
                        let ip = light.position;
                        let ia = light.ambient;
                        let id = light.diffuse;
                        let is = light.specular;

                        let l = (ip - intersect_point).normalize();
                        let r = 2.0 * l.dot(&n) * n - l;

                        let mut color = ka * ia * intensity;

                        let shadow_ray = Ray::new(offset_position, l);

                        let shadow_intersection = self.closest_intersection(&shadow_ray);
                        let obstructed = match shadow_intersection {
                            Some((_, Intersection { t, .. }))
                                if (ip - offset_position).magnitude() > t =>
                            {
                                true
                            }
                            _ => false,
                        };
                        if obstructed {
                            return color;
                        }

                        if l.dot(&n) > 0.0 {
                            color = color + kd * l.dot(&n) * id * intensity;
                        }

                        if r.dot(&v) > 0.0 {
                            color = color + ks * r.dot(&v).powf(alpha) * is * intensity;
                        }

                        color
                    })
                    .sum();

                if reflectance == 0.0 {
                    return local_color;
                }

                let r = 2.0 * (v.dot(&n)) * n - v;
                let reflected_ray = Ray::new(offset_position, r);
                let reflected_color = self.trace(reflected_ray, depth - 1);

                let blended_color =
                    reflected_color * reflectance + local_color * (1.0 - reflectance);
                blended_color
            }
            None => self.background,
        }
    }

    fn closest_intersection(&self, ray: &Ray) -> Option<(&Entity, Intersection)> {
        self.entities
            .iter()
            .filter_map(|x| x.intersect(ray).and_then(|i| Some((x, i))))
            .min_by(|(_, i1), (_, i2)| i1.partial_cmp(i2).unwrap())
    }
}
