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
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position, Vec3},
    rt::Ray,
};
use seui_engine_raytracing_csg_renderer_long_double::LongDouble;
use seui_engine_raytracing_csg_renderer_types::LDRColor;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeserializableSphere {
    #[serde(default = "one", deserialize_with = "deserialize_nonnegative_float")]
    #[schemars(range(min = 0))]
    radius: f64,
    #[serde(default, deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    position: Position,
    #[serde(default, deserialize_with = "deserialize_ldr_color")]
    #[schemars(with = "LDRColorSchema")]
    albedo: LDRColor,
    #[serde(default, deserialize_with = "deserialize_ldr_float")]
    #[schemars(range(min = 0, max = 1))]
    roughness: f64,
    #[serde(default, deserialize_with = "deserialize_ldr_float")]
    #[schemars(range(min = 0, max = 1))]
    metallic: f64,
    #[serde(default)]
    texture: Option<DeserializableTexture>,
}

impl DeserializableSphere {
    pub fn into_rt_model<T: ImageLoader>(
        self,
        image_cache: &mut ImageCache<T>,
    ) -> Box<dyn RTModel + Send + Sync> {
        Box::new(Sphere {
            radius: LongDouble::from_f64(self.radius),
            position: self.position,
            albedo: self.albedo,
            roughness: LongDouble::from_f64(self.roughness),
            metallic: LongDouble::from_f64(self.metallic),
            texture: self.texture.map(|t| t.into_texture(image_cache)),
        })
    }
}

struct Sphere {
    radius: LongDouble,
    position: Position,
    albedo: LDRColor,
    roughness: LongDouble,
    metallic: LongDouble,
    texture: Option<Arc<dyn Texture + Send + Sync>>,
}

impl Sphere {
    fn albedo(&self, position: Position) -> LDRColor {
        if let Some(texture) = &self.texture {
            let dir = Direction::new(*(position - self.position));

            let theta = LongDouble::from_f64(dir.x.to_f64().atan2(dir.y.to_f64()));
            let phi = dir.z.acos();

            let u = (theta + LongDouble::pi()) / (LongDouble::from_f64(2.0) * LongDouble::pi());
            let v = phi / LongDouble::pi();

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

        let p = ray.direction.x;
        let q = ray.direction.y;
        let r = ray.direction.z;
        let u = origin.x;
        let v = origin.y;
        let w = origin.z;

        let a = p * p + q * q + r * r;
        let b = LongDouble::from_f64(2.0) * (u * p + v * q + w * r);
        let c = u * u + v * v + w * w - self.radius * self.radius;
        let discriminant = b * b - LongDouble::from_f64(4.0) * a * c;

        if discriminant < LongDouble::from_f64(0.0) {
            return result; // No intersection
        }

        let sqrt_d = discriminant.sqrt();
        let mut t1 = (-b - sqrt_d) / (LongDouble::from_f64(2.0) * a);
        let mut t2 = (-b + sqrt_d) / (LongDouble::from_f64(2.0) * a);
        if t1 > t2 {
            (t1, t2) = (t2, t1);
        }

        if t2 < LongDouble::from_f64(0.0) {
            return result; // No visible intersection
        }
        if t1.is_nan() {
            return result; // error
        }

        if t1 < LongDouble::from_f64(0.0) {
            // If t1 is negative, ray started inside the sphere
            result.push(Hit {
                distance: LongDouble::from_f64(0.0),
                normal: -ray.direction, // Opposite direction
                albedo: self.albedo,
                is_front_face: true,
                roughness: self.roughness,
                metallic: self.metallic,
            });
        } else {
            let normal: Vec3 = *(origin + ray.direction * t1) * LongDouble::from_f64(2.0);
            result.push(Hit {
                distance: t1,
                normal: Direction::new(normal),
                albedo: self.albedo(ray.origin + ray.direction * t1),
                is_front_face: true,
                roughness: self.roughness,
                metallic: self.metallic,
            });
        }

        let normal: Vec3 = *(origin + ray.direction * t2) * LongDouble::from_f64(2.0);
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
