use crate::{
    deserialize::deserialize_ldr_float,
    json_schema::{DirectionSchema, LDRColorSchema, PositionSchema},
};

use super::{
    super::super::deserialize::{
        deserialize_direction, deserialize_ldr_color, deserialize_position,
    },
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

fn up() -> Direction {
    Direction::new(Vec3::new(
        LongDouble::from_f64(0.0),
        LongDouble::from_f64(0.0),
        LongDouble::from_f64(1.0),
    ))
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Plane {
    #[serde(default, deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    position: Position,
    #[serde(default = "up", deserialize_with = "deserialize_direction")]
    #[schemars(with = "DirectionSchema")]
    normal: Direction,
    #[serde(default, deserialize_with = "deserialize_ldr_color")]
    #[schemars(with = "LDRColorSchema")]
    albedo: LDRColor,
    #[serde(default, deserialize_with = "deserialize_ldr_float")]
    #[schemars(range(min = 0, max = 1))]
    roughness: f64,
    #[serde(default, deserialize_with = "deserialize_ldr_float")]
    #[schemars(range(min = 0, max = 1))]
    metallic: f64,
}

impl RTModel for Plane {
    fn test(&self, ray: Ray) -> Vec<Hit> {
        let mut result = Vec::new();

        // Compute intersection distance t
        let denominator = self.normal.dot(ray.direction);
        if denominator.abs() < LongDouble::from_f64(1e-6) {
            return result; // Ray is parallel to the plane, no intersection
        }

        let t = -(ray.origin.dot(*self.normal) - self.position.dot(*self.normal)) / denominator;

        if t < LongDouble::from_f64(0.0) {
            // The intersection is behind the ray's origin
            if self.normal.dot(ray.direction) < LongDouble::from_f64(0.0) {
                result.push(Hit {
                    distance: LongDouble::from_f64(0.0),
                    normal: -ray.direction,
                    albedo: self.albedo,
                    is_front_face: true,
                    roughness: LongDouble::from_f64(self.roughness),
                    metallic: LongDouble::from_f64(self.metallic),
                });
                result.push(Hit {
                    distance: LongDouble::infinity(),
                    normal: ray.direction,
                    albedo: self.albedo,
                    is_front_face: false,
                    roughness: LongDouble::from_f64(self.roughness),
                    metallic: LongDouble::from_f64(self.metallic),
                });
            }
            return result;
        }

        if self.normal.dot(ray.direction) < LongDouble::from_f64(0.0) {
            result.push(Hit {
                distance: t,
                normal: self.normal,
                albedo: self.albedo,
                is_front_face: true,
                roughness: LongDouble::from_f64(self.roughness),
                metallic: LongDouble::from_f64(self.metallic),
            });
            result.push(Hit {
                distance: LongDouble::infinity(),
                normal: ray.direction,
                albedo: self.albedo,
                is_front_face: false,
                roughness: LongDouble::from_f64(self.roughness),
                metallic: LongDouble::from_f64(self.metallic),
            });
        } else {
            result.push(Hit {
                distance: LongDouble::from_f64(0.0),
                normal: -ray.direction,
                albedo: self.albedo,
                is_front_face: true,
                roughness: LongDouble::from_f64(self.roughness),
                metallic: LongDouble::from_f64(self.metallic),
            });
            result.push(Hit {
                distance: t,
                normal: self.normal,
                albedo: self.albedo,
                is_front_face: false,
                roughness: LongDouble::from_f64(self.roughness),
                metallic: LongDouble::from_f64(self.metallic),
            });
        }

        result
    }
}
