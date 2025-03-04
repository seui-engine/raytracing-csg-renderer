use std::rc::Rc;

use glam::Vec3;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position},
    rt::{Hit, RTObject, Ray},
};
use seui_engine_raytracing_csg_renderer_types::LDRColor;

use crate::{
    deserialize::{deserialize_ldr_color, deserialize_position},
    json_schema::{LDRColorSchema, PositionSchema},
};

use super::util::enhance_normal;
// Ax³ + By³ + Cz³ + Dx²y + Ey²z + Fz²x + Gx² + Hy² + Iz² + Jxyz + Kx + Ly + Mz + N = 0
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Cubic {
    #[serde(deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    position: Position,

    #[serde(deserialize_with = "deserialize_ldr_color")]
    #[schemars(with = "LDRColorSchema")]
    albedo: LDRColor,

    c300: f32,
    c030: f32,
    c003: f32,
    c210: f32,
    c021: f32,
    c102: f32,
    c200: f32,
    c020: f32,
    c002: f32,
    c111: f32,
    c100: f32,
    c010: f32,
    c001: f32,
    c000: f32,

    #[serde(deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    inside: Position,
}

impl Cubic {
    fn internal_test(&self, ray: Ray) -> Option<(Hit, Hit)> {
        let origin: Position = (ray.origin - self.position).into();

        // Calculate cubic coefficients
        let a = self.c300 * ray.direction.x.powi(3)
            + self.c030 * ray.direction.y.powi(3)
            + self.c003 * ray.direction.z.powi(3)
            + self.c210 * ray.direction.x.powi(2) * ray.direction.y
            + self.c021 * ray.direction.y.powi(2) * ray.direction.z
            + self.c102 * ray.direction.x * ray.direction.y.powi(2)
            + self.c200 * ray.direction.x.powi(2)
            + self.c020 * ray.direction.y.powi(2)
            + self.c002 * ray.direction.z.powi(2)
            + self.c111 * ray.direction.x * ray.direction.y * ray.direction.z
            + self.c100 * ray.direction.x
            + self.c010 * ray.direction.y
            + self.c001 * ray.direction.z;

        let b = 3.0 * self.c300 * origin.x.powi(2) * ray.direction.x
            + 3.0 * self.c030 * origin.y.powi(2) * ray.direction.y
            + 3.0 * self.c003 * origin.z.powi(2) * ray.direction.z
            + 2.0 * self.c210 * origin.x * ray.direction.y
            + 2.0 * self.c021 * origin.y * ray.direction.z
            + 2.0 * self.c102 * origin.x * ray.direction.z
            + 2.0 * self.c200 * origin.x * ray.direction.x
            + 2.0 * self.c020 * origin.y * ray.direction.y
            + 2.0 * self.c002 * origin.z * ray.direction.z
            + self.c111
                * (origin.x * ray.direction.y * ray.direction.z
                    + origin.y * ray.direction.x * ray.direction.z
                    + origin.z * ray.direction.x * ray.direction.y)
            + self.c100 * origin.x
            + self.c010 * origin.y
            + self.c001 * origin.z;

        let c = self.c300 * origin.x.powi(3)
            + self.c030 * origin.y.powi(3)
            + self.c003 * origin.z.powi(3)
            + self.c210 * origin.x.powi(2) * origin.y
            + self.c021 * origin.y.powi(2) * origin.z
            + self.c102 * origin.x * origin.y.powi(2)
            + self.c200 * origin.x.powi(2)
            + self.c020 * origin.y.powi(2)
            + self.c002 * origin.z.powi(2)
            + self.c111 * origin.x * origin.y * origin.z
            + self.c100 * origin.x
            + self.c010 * origin.y
            + self.c001 * origin.z
            + self.c000;

        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let (t1, t2) = {
            let t1 = (-b - sqrt_d) / (2.0 * a);
            let t2 = (-b + sqrt_d) / (2.0 * a);
            if t1 < t2 {
                (t1, t2)
            } else {
                (t2, t1)
            }
        };
        if t2 < 0.0 {
            return None;
        }

        if t1 < 0.0 {
            Some((
                Hit {
                    distance: t2,
                    normal: -self.normal(origin + ray.direction * t2),
                    albedo: self.albedo,
                    is_front_face: true,
                    brdf: Rc::new(|normal, direction| normal.dot(direction)),
                },
                Hit {
                    distance: f32::INFINITY,
                    normal: ray.direction,
                    albedo: self.albedo,
                    is_front_face: false,
                    brdf: Rc::new(|normal, direction| normal.dot(direction)),
                },
            ))
        } else {
            Some((
                Hit {
                    distance: t1,
                    normal: self.normal(origin + ray.direction * t1),
                    albedo: self.albedo,
                    is_front_face: true,
                    brdf: Rc::new(|normal, direction| normal.dot(direction)),
                },
                Hit {
                    distance: t2,
                    normal: self.normal(origin + ray.direction * t2),
                    albedo: self.albedo,
                    is_front_face: false,
                    brdf: Rc::new(|normal, direction| normal.dot(direction)),
                },
            ))
        }
    }

    fn normal(&self, position: Position) -> Direction {
        Direction::new(Vec3::new(
            3.0 * self.c300 * position.x.powi(2)
                + 2.0 * self.c210 * position.y
                + self.c102 * position.z
                + 2.0 * self.c200 * position.x
                + self.c111 * position.y * position.z
                + self.c100,
            3.0 * self.c030 * position.y.powi(2)
                + 2.0 * self.c210 * position.x
                + self.c021 * position.z
                + 2.0 * self.c020 * position.y
                + self.c111 * position.x * position.z
                + self.c010,
            3.0 * self.c003 * position.z.powi(2)
                + 2.0 * self.c021 * position.y
                + self.c102 * position.x
                + 2.0 * self.c002 * position.z
                + self.c111 * position.x * position.y
                + self.c001,
        ))
    }
}

impl RTObject for Cubic {
    fn test(&self, ray: Ray) -> Vec<Hit> {
        let (inside_direction, inside_length) = (ray.origin - self.inside).direction_and_length();
        let inside = if let Some((hit1, hit2)) = self.internal_test(Ray {
            origin: self.inside,
            direction: inside_direction,
        }) {
            (hit1.distance < inside_length) == (hit2.distance < inside_length)
        } else {
            true
        };

        let mut result = Vec::new();

        if let Some((hit1, hit2)) = self.internal_test(ray) {
            if inside {
                result.push(Hit {
                    distance: 0.0,
                    normal: -ray.direction,
                    albedo: self.albedo,
                    is_front_face: true,
                    brdf: Rc::new(|normal, direction| normal.dot(direction)),
                });
                result.push(Hit {
                    normal: enhance_normal(ray.direction, hit1.normal, false),
                    is_front_face: false,
                    ..hit1
                });
                result.push(Hit {
                    normal: enhance_normal(ray.direction, hit2.normal, true),
                    is_front_face: true,
                    ..hit2
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
                    normal: enhance_normal(ray.direction, hit1.normal, true),
                    ..hit1
                });
                result.push(Hit {
                    normal: enhance_normal(ray.direction, hit2.normal, false),
                    ..hit2
                });
            }
        } else if inside {
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

        result
    }
}
