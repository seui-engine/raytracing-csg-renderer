use std::sync::Arc;

use crate::{
    deserialize::{deserialize_ldr_float, deserialize_nonnegative_float},
    json_schema::{LDRColorSchema, PositionSchema},
    texture::{DeserializableTexture, Texture},
    ImageCache, ImageLoader,
};

use super::{
    super::super::deserialize::{deserialize_ldr_color, deserialize_position},
    util::one,
    Hit, RTModel,
};
use glam::Vec3;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position},
    rt::Ray,
};
use seui_engine_raytracing_csg_renderer_types::LDRColor;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeserializableSphere {
    #[serde(default = "one", deserialize_with = "deserialize_nonnegative_float")]
    #[schemars(range(min = 0))]
    radius: f32,
    #[serde(default, deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    position: Position,
    #[serde(default, deserialize_with = "deserialize_ldr_color")]
    #[schemars(with = "LDRColorSchema")]
    albedo: LDRColor,
    #[serde(default, deserialize_with = "deserialize_ldr_float")]
    #[schemars(range(min = 0, max = 1))]
    roughness: f32,
    #[serde(default, deserialize_with = "deserialize_ldr_float")]
    #[schemars(range(min = 0, max = 1))]
    metallic: f32,
    #[serde(default)]
    texture: Option<DeserializableTexture>,
}

impl DeserializableSphere {
    pub fn into_rt_model<T: ImageLoader>(
        self,
        image_cache: &mut ImageCache<T>,
    ) -> Box<dyn RTModel + Send + Sync> {
        Box::new(Sphere {
            radius: self.radius,
            position: self.position,
            albedo: self.albedo,
            roughness: self.roughness,
            metallic: self.metallic,
            texture: self.texture.map(|t| t.into_texture(image_cache)),
        })
    }
}

struct Sphere {
    radius: f32,
    position: Position,
    albedo: LDRColor,
    roughness: f32,
    metallic: f32,
    texture: Option<Arc<dyn Texture + Send + Sync>>,
}

impl Sphere {
    fn albedo(&self, position: Position) -> LDRColor {
        if let Some(texture) = &self.texture {
            let dir = Direction::new(*(position - self.position));

            let theta = dir.x.atan2(dir.y);
            let phi = dir.z.acos();

            let u = (theta + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
            let v = phi / std::f32::consts::PI;

            texture.get(u, v)
        } else {
            self.albedo
        }
    }
}

impl RTModel for Sphere {
    fn test(&self, ray: Ray) -> Vec<Hit> {
        let mut result = Vec::new();

        // Move the sphere to the origin for simplicity
        let origin: Position = (ray.origin - self.position).into();

        let a = ray.direction.x.powi(2) + ray.direction.y.powi(2) + ray.direction.z.powi(2);
        let b = 2.0
            * (origin.x * ray.direction.x
                + origin.y * ray.direction.y
                + origin.z * ray.direction.z);
        let c = origin.x.powi(2) + origin.y.powi(2) + origin.z.powi(2) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return result; // No intersection
        }

        let sqrt_d = discriminant.sqrt();
        let mut t1 = (-b - sqrt_d) / (2.0 * a);
        let mut t2 = (-b + sqrt_d) / (2.0 * a);
        if t1 > t2 {
            (t1, t2) = (t2, t1);
        }

        if t2 < 0.0 {
            return result; // No visible intersection
        }
        if t1.is_nan() {
            return result; // error
        }

        if t1 < 0.0 {
            // If t1 is negative, ray started inside the sphere
            result.push(Hit {
                distance: 0.0,
                normal: -ray.direction, // Opposite direction
                albedo: self.albedo,
                is_front_face: true,
                roughness: self.roughness,
                metallic: self.metallic,
            });
        } else {
            let normal: Vec3 = *(origin + ray.direction * t1) * 2.0;
            result.push(Hit {
                distance: t1,
                normal: Direction::new(normal),
                albedo: self.albedo(ray.origin + ray.direction * t1),
                is_front_face: true,
                roughness: self.roughness,
                metallic: self.metallic,
            });
        }

        let normal: Vec3 = *(origin + ray.direction * t2) * 2.0;
        result.push(Hit {
            distance: t2,
            normal: Direction::new(normal),
            albedo: self.albedo(ray.origin + ray.direction * t2),
            is_front_face: false,
            roughness: self.roughness,
            metallic: self.metallic,
        });

        result
    }
}
