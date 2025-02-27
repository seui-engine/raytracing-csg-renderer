use std::rc::Rc;

use super::super::deserialize::{
    deserialize_direction, deserialize_ldr_color, deserialize_position,
};
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position},
    rt::{Hit, RTObject, Ray},
};
use seui_engine_raytracing_csg_renderer_types::LDRColor;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plane {
    #[serde(deserialize_with = "deserialize_position")]
    position: Position,
    #[serde(deserialize_with = "deserialize_direction")]
    normal: Direction,
    #[serde(deserialize_with = "deserialize_ldr_color")]
    albedo: LDRColor,
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
                    brdf: Rc::new(|normal, direction| normal.dot(direction)),
                });
                result.push(Hit {
                    distance: f32::INFINITY,
                    normal: ray.direction,
                    albedo: self.albedo,
                    is_front_face: false,
                    brdf: Rc::new(|normal, direction| normal.dot(direction)),
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
                brdf: Rc::new(|normal, direction| normal.dot(direction)),
            });
            result.push(Hit {
                distance: f32::INFINITY,
                normal: ray.direction,
                albedo: self.albedo,
                is_front_face: false,
                brdf: Rc::new(|normal, direction| normal.dot(direction)),
            });
        } else {
            result.push(Hit {
                distance: 0.0,
                normal: -ray.direction,
                albedo: self.albedo,
                is_front_face: true,
                brdf: Rc::new(|normal, direction| normal.dot(direction)),
            });
            result.push(Hit {
                distance: t,
                normal: self.normal,
                albedo: self.albedo,
                is_front_face: false,
                brdf: Rc::new(|normal, direction| normal.dot(direction)),
            });
        }

        result
    }
}
