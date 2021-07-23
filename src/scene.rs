use image::{DynamicImage, GenericImage, Pixel, Rgb, Rgba};

use crate::{
    algebra::{Point3, Ray, Vector3, EPSILON},
    camera::Camera,
    color::Color,
    geometry::Geometry,
    geometry::Sphere,
    geometry::{Intersect, Intersection, Textured},
    light::PointLight,
    material::Phong,
};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub camera: Camera,
    pub objects: Vec<Geometry>,
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
            objects: Vec::new(),
            lights: Vec::new(),
            background,
        }
    }

    pub fn add_object(&mut self, obj: impl Into<Geometry>) {
        self.objects.push(obj.into());
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn render(&self, outfile: String) {
        let mut img = DynamicImage::new_rgb8(self.width, self.height);

        let d = (self.width as f64 / 2.0) / (self.fov / 2.0).tan();

        let width = self.width as i32;
        let height = self.height as i32;

        for i in 0..width {
            for j in 0..height {
                let raw = d * self.camera.view
                    + (i - (width / 2)) as f64 * self.camera.side
                    + ((height / 2) - j) as f64 * self.camera.up;
                let v = raw.normalize();

                let ray = Ray::new(&self.camera.position, v);

                let color = self.trace(ray);
                img.put_pixel(i as u32, j as u32, color.into());
            }
        }

        img.save(outfile).unwrap();
    }

    fn trace(&self, ray: Ray) -> Color {
        match self.closest_intersection(&ray) {
            Some((
                obj,
                Intersection {
                    position: intersect_point, normal, ..
                },
            )) => self
                .lights
                .iter()
                .map(|light| {
                    let material = obj.material();
                    let uv = obj.to_texture_space(&intersect_point);

                    let ka = material.ambient.color_at(&uv);
                    let kd = material.diffuse.color_at(&uv);
                    let ks = material.specular.color_at(&uv);
                    let alpha = material.shininess;

                    let ip = light.position;
                    let ia = light.ambient;
                    let id = light.diffuse;
                    let is = light.specular;

                    let l = (ip - intersect_point).normalize();
                    let n = normal;
                    let r = 2.0 * l.dot(&n) * n - l;
                    let v = -ray.dir;

                    let mut color = ka * ia;

                    let offset_position = intersect_point + EPSILON * normal;
                    let shadow_ray =
                        Ray::new(&offset_position, l);

                    if self.closest_intersection(&shadow_ray).is_some() {
                        return color;
                    }

                    if l.dot(&n) > 0.0 {
                        color = color + kd * l.dot(&n) * id
                    }

                    if r.dot(&v) > 0.0 {
                        color = color + ks * r.dot(&v).powf(alpha) * is;
                    }

                    color
                })
                .sum(),
            None => self.background,
        }
    }

    fn closest_intersection(&self, ray: &Ray) -> Option<(&Geometry, Intersection)> {
        self.objects
            .iter()
            .filter_map(|x| x.intersect(ray).and_then(|i| Some((x, i))))
            .min_by(|(_, i1), (_, i2)| i1.partial_cmp(i2).unwrap())
    }
}
