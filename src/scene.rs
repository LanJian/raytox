use image::{DynamicImage, GenericImage, Pixel, Rgba};

use crate::{
    algebra::{point3::Point3, ray::Ray, vector3::Vector3},
    camera::Camera,
    shape::Sphere,
};

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub camera: Camera,
    pub objects: Vec<Sphere>,
}

impl Scene {
    pub fn new(width: u32, height: u32, fov: f64, camera: Camera) -> Scene {
        Self {
            width,
            height,
            fov,
            camera,
            objects: Vec::new(),
        }
    }

    pub fn render(&self, outfile: String) {
        let mut img = DynamicImage::new_rgb8(self.width, self.height);
        let bg = Rgba::from_channels(10, 10, 10, 255);

        let d = (self.width as f64 / 2.0) / (self.fov / 2.0).tan();

        let width = self.width as i32;
        let height = self.height as i32;

        let near = 5.0;
        let far = 30.0;

        for i in 0..width {
            for j in 0..height {
                let raw = d * self.camera.view
                    + (i - (width / 2)) as f64 * self.camera.side
                    + ((height / 2) - j) as f64 * self.camera.up;
                let v = raw.normalize();

                let ray = Ray::new(&self.camera.position, v);

                let color = match self.closest_intersection(&ray) {
                    //Some(_) => Color::new(1.0, 1.0, 1.0),
                    //None => Color::new(0.0, 0.0, 0.0),
                    Some((_, t)) => {
                        //println!("{}", t);
                        let c = ((1.0 - (t - near) / (far - near)) * 255.0).clamp(0.0, 255.0) as u8;
                        Rgba::from_channels(c, c, c, 255)
                    }
                    None => bg,
                };

                img.put_pixel(i as u32, j as u32, color);
            }
        }

        img.save(outfile).unwrap();
    }

    pub fn add_object(&mut self, obj: Sphere) {
        self.objects.push(obj);
    }

    fn closest_intersection(&self, ray: &Ray) -> Option<(Point3, f64)> {
        self.objects
            .iter()
            .filter_map(|x| x.intersection(ray))
            .min_by(|(_, t1), (_, t2)| t1.partial_cmp(t2).unwrap())
    }
}
