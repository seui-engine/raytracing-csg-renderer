use crate::{
    deserialize::deserialize_ldr_float,
    json_schema::{DirectionSchema, LDRColorSchema, PositionSchema},
};

use super::super::deserialize::{
    deserialize_direction, deserialize_ldr_color, deserialize_position,
};
use glam::Vec3;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position},
    rt::{Hit, RTObject, Ray},
};
use seui_engine_raytracing_csg_renderer_types::LDRColor;

fn up() -> Direction {
    Direction::new(Vec3::Z)
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
    roughness: f32,
    #[serde(default, deserialize_with = "deserialize_ldr_float")]
    #[schemars(range(min = 0, max = 1))]
    metallic: f32,
}

impl RTObject for Plane {
    fn test(&self, ray: Ray) -> Vec<Hit> {
        let mut result = Vec::new();

        // Compute intersection distance t
        let denominator = self.normal.dot(ray.direction);
        if denominator.abs() < 1e-6 {
            return result; // Ray is parallel to the plane, no intersection
        }

        let t = -(ray.origin.dot(*self.normal) - self.position.dot(*self.normal)) / denominator;

        if t < 0.0 {
            // The intersection is behind the ray's origin
            if self.normal.dot(ray.direction) < 0.0 {
                result.push(Hit {
                    distance: 0.0,
                    normal: -ray.direction,
                    albedo: self.albedo,
                    is_front_face: true,
                    roughness: self.roughness,
                    metallic: self.metallic,
                });
                result.push(Hit {
                    distance: f32::INFINITY,
                    normal: ray.direction,
                    albedo: self.albedo,
                    is_front_face: false,
                    roughness: self.roughness,
                    metallic: self.metallic,
                });
            }
            return result;
        }

        if self.normal.dot(ray.direction) < 0.0 {
            result.push(Hit {
                distance: t,
                normal: self.normal,
                albedo: self.albedo,
                is_front_face: true,
                roughness: self.roughness,
                metallic: self.metallic,
            });
            result.push(Hit {
                distance: f32::INFINITY,
                normal: ray.direction,
                albedo: self.albedo,
                is_front_face: false,
                roughness: self.roughness,
                metallic: self.metallic,
            });
        } else {
            result.push(Hit {
                distance: 0.0,
                normal: -ray.direction,
                albedo: self.albedo,
                is_front_face: true,
                roughness: self.roughness,
                metallic: self.metallic,
            });
            result.push(Hit {
                distance: t,
                normal: self.normal,
                albedo: self.albedo,
                is_front_face: false,
                roughness: self.roughness,
                metallic: self.metallic,
            });
        }

        result
    }
}
