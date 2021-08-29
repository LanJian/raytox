use crate::{
    algebra::{Matrix4, Point2, Point3, Ray, Vector3},
    geometry::{Axis, Cube, Geometry, Intersect, Intersection, Mesh, Plane, Sphere, Textured},
    material::Phong,
};

use super::Transformable;

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    geometry: Geometry,
    material: Phong,

    translation: Matrix4,
    rotation: Matrix4,
    scaling: Matrix4,
    ad_hoc_transform: Matrix4,

    transform: Matrix4,
    inv_transform: Matrix4,
}

impl Entity {
    pub fn with_material(mut self, material: Phong) -> Self {
        self.material = material;
        self
    }

    pub fn material(&self) -> &Phong {
        &self.material
    }

    pub fn build(mut self) -> Self {
        // apply eveything except translation
        self.transform = self.rotation * self.scaling * self.ad_hoc_transform;

        // for intuitiveness, apply translation in world space instead of object space
        self.transform[0][3] += self.translation[0][3];
        self.transform[1][3] += self.translation[1][3];
        self.transform[2][3] += self.translation[2][3];

        // set inverse transform
        self.inv_transform = self.transform.invert().expect("could not invert transform");

        self
    }
}

impl From<Geometry> for Entity {
    fn from(geometry: Geometry) -> Self {
        Self {
            geometry,
            material: Phong::default(),
            translation: Matrix4::default(),
            rotation: Matrix4::default(),
            scaling: Matrix4::default(),
            ad_hoc_transform: Matrix4::default(),
            transform: Matrix4::default(),
            inv_transform: Matrix4::default(),
        }
    }
}

impl From<Plane> for Entity {
    fn from(plane: Plane) -> Self {
        let geometry: Geometry = plane.into();
        Self::from(geometry)
    }
}

impl From<Sphere> for Entity {
    fn from(sphere: Sphere) -> Self {
        let geometry: Geometry = sphere.into();
        Self::from(geometry)
    }
}

impl From<Cube> for Entity {
    fn from(cube: Cube) -> Self {
        let geometry: Geometry = cube.into();
        Self::from(geometry)
    }
}

impl From<Mesh> for Entity {
    fn from(mesh: Mesh) -> Self {
        let geometry: Geometry = mesh.into();
        Self::from(geometry)
    }
}

impl Intersect for Entity {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // transform the ray to object space
        let new_ray = ray.transform(self.inv_transform);

        // do the intersection
        let intersection = self.geometry.intersect(&new_ray);

        // transform the intersection back to world space
        match intersection {
            Some(Intersection {
                position, normal, ..
            }) => {
                let new_position = position.transform(self.transform);
                let new_normal = normal.transform(self.inv_transform.transpose());
                let new_t = ray.distance_to(new_position);
                Some(Intersection::new(
                    new_t,
                    new_position,
                    new_normal.normalize(),
                ))
            }
            None => None,
        }
    }
}

impl Textured for Entity {
    fn to_texture_space(&self, p: &Point3) -> Point2 {
        self.geometry
            .to_texture_space(&p.transform(self.inv_transform))
    }
}

impl Transformable for Entity {
    fn translate(mut self, translation: Vector3) -> Self {
        self.translation = self.translation * Matrix4::translation(translation);
        self
    }

    fn rotate(mut self, axis: Axis, degrees: f64) -> Self {
        self.rotation = self.rotation * Matrix4::rotation(axis, degrees);
        self
    }

    fn scale(mut self, scale: Vector3) -> Self {
        self.scaling = self.scaling * Matrix4::scaling(scale);
        self
    }

    fn transform(mut self, transform: Matrix4) -> Self {
        self.ad_hoc_transform = self.ad_hoc_transform * transform;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale() {
        let entity = Entity::from(Sphere::default())
            .scale(Vector3::new(2.0, 2.0, 2.0))
            .build();
        let ray = Ray::new(Point3::new(0.0, 0.0, -10.0), Vector3::K);

        assert_eq!(
            entity.intersect(&ray),
            Some(Intersection::new(
                8.0,
                Point3::new(0.0, 0.0, -2.0),
                -Vector3::K
            ))
        );
    }
}
