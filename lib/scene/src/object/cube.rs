use std::rc::Rc;

use crate::json_schema::{LDRColorSchema, PositionSchema, ScaleSchema, SizeSchema};

use super::super::deserialize::{
    deserialize_ldr_color, deserialize_position, deserialize_scale, deserialize_size,
};

use glam::Vec3;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position, Scale, Size},
    rt::{Hit, RTObject, Ray},
};
use seui_engine_raytracing_csg_renderer_types::LDRColor;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Cube {
    #[serde(deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    position: Position,

    #[serde(deserialize_with = "deserialize_ldr_color")]
    #[schemars(with = "LDRColorSchema")]
    albedo: LDRColor,

    #[serde(deserialize_with = "deserialize_size")]
    #[schemars(with = "SizeSchema")]
    size: Size, // Cube의 한 변의 크기

    #[serde(deserialize_with = "deserialize_scale")]
    #[schemars(with = "ScaleSchema")]
    scale: Scale, // 크기를 조정할 스케일
}

impl RTObject for Cube {
    fn test(&self, ray: Ray) -> Vec<Hit> {
        let mut result = Vec::new();

        let origin: Position = (ray.origin - self.position).into();

        let half_size = *self.size / 2.0 * *self.scale;

        // Cube 한쪽 모서리
        let min: Position = Position::new(Vec3::new(
            self.position.x - half_size.x,
            self.position.y - half_size.y,
            self.position.z - half_size.z,
        ));

        // Cube의 다른 한쪽 모서리
        let max = Position::new(Vec3::new(
            self.position.x + half_size.x,
            self.position.y + half_size.y,
            self.position.z + half_size.z,
        ));

        let (mut tmin, mut tmax) = (f32::NEG_INFINITY, f32::INFINITY);

        // X-axis
        if ray.direction.x != 0.0 {
            let t1 = (min.x - origin.x) / ray.direction.x;
            let t2 = (max.x - origin.x) / ray.direction.x;
            let (t1, t2) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

            tmin = tmin.max(t1);
            tmax = tmax.min(t2);

            if tmin > tmax {
                return result;
            }
        } else if origin.x < min.x || origin.x > max.x {
            return result;
        }

        // Y-axis
        if ray.direction.y != 0.0 {
            let t1 = (min.y - origin.y) / ray.direction.y;
            let t2 = (max.y - origin.y) / ray.direction.y;
            let (t1, t2) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

            tmin = tmin.max(t1);
            tmax = tmax.min(t2);

            if tmin > tmax {
                return result;
            }
        } else if origin.y < min.y || origin.y > max.y {
            return result;
        }

        // Z-axis
        if ray.direction.z != 0.0 {
            let t1 = (min.z - origin.z) / ray.direction.z;
            let t2 = (max.z - origin.z) / ray.direction.z;
            let (t1, t2) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

            tmin = tmin.max(t1);
            tmax = tmax.min(t2);

            if tmin > tmax {
                return result;
            }
        } else if origin.z < min.z || origin.z > max.z {
            return result;
        }

        if tmin <= tmax {
            let hit_normal = if tmin == (min.x - origin.x) / ray.direction.x
                || tmin == (max.x - origin.x) / ray.direction.x
            {
                Vec3::new(1.0, 0.0, 0.0)
            } else if tmin == (min.y - origin.y) / ray.direction.y
                || tmin == (max.y - origin.y) / ray.direction.y
            {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(0.0, 0.0, 1.0)
            };

            result.push(Hit {
                normal: Direction::new(hit_normal),
                distance: tmin,
                is_front_face: false,
                albedo: self.albedo,
                brdf: Rc::new(|normal, direction| normal.dot(direction)),
            });
        }

        result
    }
}
