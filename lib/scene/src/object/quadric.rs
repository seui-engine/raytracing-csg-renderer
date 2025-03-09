use glam::Vec3;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position},
    rt::{Hit, RTObject, Ray},
};
use seui_engine_raytracing_csg_renderer_types::LDRColor;

use crate::{
    deserialize::{deserialize_ldr_color, deserialize_ldr_float, deserialize_position},
    json_schema::{LDRColorSchema, PositionSchema},
};

use super::util::{enhance_normal, zero};

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Quadric {
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

    #[serde(default = "zero")]
    c200: f32,
    #[serde(default = "zero")]
    c020: f32,
    #[serde(default = "zero")]
    c002: f32,
    #[serde(default = "zero")]
    c110: f32,
    #[serde(default = "zero")]
    c011: f32,
    #[serde(default = "zero")]
    c101: f32,
    #[serde(default = "zero")]
    c100: f32,
    #[serde(default = "zero")]
    c010: f32,
    #[serde(default = "zero")]
    c001: f32,
    #[serde(default = "zero")]
    c000: f32,

    #[serde(default, deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    inside: Position,
}

impl Quadric {
    fn internal_test(&self, ray: Ray) -> Option<(Hit, Hit)> {
        // Move the sphere to the origin for simplicity
        let origin: Position = (ray.origin - self.position).into();

        let (a, b, c) = {
            let mut a = 0.0;
            let mut b = 0.0;
            let mut c = 0.0;
            // c200
            a += self.c200 * ray.direction.x.powi(2);
            b += self.c200 * 2.0 * ray.direction.x * origin.x;
            c += self.c200 * origin.x.powi(2);
            // c020
            a += self.c020 * ray.direction.y.powi(2);
            b += self.c020 * 2.0 * ray.direction.y * origin.y;
            c += self.c020 * origin.y.powi(2);
            // c002
            a += self.c002 * ray.direction.z.powi(2);
            b += self.c002 * 2.0 * ray.direction.z * origin.z;
            c += self.c002 * origin.z.powi(2);
            // c110
            a += self.c110 * ray.direction.x * ray.direction.y;
            b += self.c110 * ray.direction.x * origin.y;
            b += self.c110 * origin.x * ray.direction.y;
            c += self.c110 * origin.x * origin.y;
            // c011
            a += self.c011 * ray.direction.y * ray.direction.z;
            b += self.c011 * ray.direction.y * origin.z;
            b += self.c011 * origin.y * ray.direction.z;
            c += self.c011 * origin.y * origin.z;
            // c101
            a += self.c101 * ray.direction.x * ray.direction.z;
            b += self.c101 * ray.direction.x * origin.z;
            b += self.c101 * origin.x * ray.direction.z;
            c += self.c101 * origin.x * origin.z;
            // c100
            b += self.c100 * ray.direction.x;
            c += self.c100 * origin.x;
            // c010
            b += self.c010 * ray.direction.y;
            c += self.c010 * origin.y;
            // c001
            b += self.c001 * ray.direction.z;
            c += self.c001 * origin.z;
            // c000
            c += self.c000;
            // done
            (a, b, c)
        };

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
                    normal: self.normal(origin + ray.direction * t2),
                    albedo: self.albedo,
                    is_front_face: true,
                    roughness: self.roughness,
                    metallic: self.metallic,
                },
                Hit {
                    distance: f32::INFINITY,
                    normal: ray.direction,
                    albedo: self.albedo,
                    is_front_face: false,
                    roughness: self.roughness,
                    metallic: self.metallic,
                },
            ))
        } else {
            Some((
                Hit {
                    distance: t1,
                    normal: self.normal(origin + ray.direction * t1),
                    albedo: self.albedo,
                    is_front_face: true,
                    roughness: self.roughness,
                    metallic: self.metallic,
                },
                Hit {
                    distance: t2,
                    normal: self.normal(origin + ray.direction * t2),
                    albedo: self.albedo,
                    is_front_face: false,
                    roughness: self.roughness,
                    metallic: self.metallic,
                },
            ))
        }
    }

    fn normal(&self, position: Position) -> Direction {
        Direction::new(Vec3::new(
            2.0 * self.c200 * position.x
                + self.c110 * position.y
                + self.c101 * position.z
                + self.c100,
            2.0 * self.c020 * position.y
                + self.c110 * position.x
                + self.c011 * position.z
                + self.c010,
            2.0 * self.c002 * position.z
                + self.c011 * position.y
                + self.c101 * position.x
                + self.c001,
        ))
    }
}

impl RTObject for Quadric {
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
                    roughness: self.roughness,
                    metallic: self.metallic,
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
                    roughness: self.roughness,
                    metallic: self.metallic,
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

        result
    }
}
