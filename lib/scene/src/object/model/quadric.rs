use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position, Vec3},
    rt::Ray,
};
use seui_engine_raytracing_csg_renderer_long_double::LongDouble;
use seui_engine_raytracing_csg_renderer_types::LDRColor;

use crate::{
    deserialize::{deserialize_ldr_color, deserialize_ldr_float, deserialize_position},
    json_schema::{LDRColorSchema, PositionSchema},
};

use super::{
    util::{enhance_normal, zero},
    Hit, RTModel,
};

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeserializableQuadric {
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

    #[serde(default = "zero")]
    c200: f64,
    #[serde(default = "zero")]
    c020: f64,
    #[serde(default = "zero")]
    c002: f64,
    #[serde(default = "zero")]
    c110: f64,
    #[serde(default = "zero")]
    c011: f64,
    #[serde(default = "zero")]
    c101: f64,
    #[serde(default = "zero")]
    c100: f64,
    #[serde(default = "zero")]
    c010: f64,
    #[serde(default = "zero")]
    c001: f64,
    #[serde(default = "zero")]
    c000: f64,

    #[serde(default, deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    inside: Position,
}

impl DeserializableQuadric {
    pub fn into_rt_model(self) -> Box<dyn RTModel + Send + Sync> {
        Box::new(Quadric {
            position: self.position,
            albedo: self.albedo,
            roughness: LongDouble::from_f64(self.roughness),
            metallic: LongDouble::from_f64(self.metallic),

            c200: LongDouble::from_f64(self.c200),
            c020: LongDouble::from_f64(self.c020),
            c002: LongDouble::from_f64(self.c002),
            c110: LongDouble::from_f64(self.c110),
            c011: LongDouble::from_f64(self.c011),
            c101: LongDouble::from_f64(self.c101),
            c100: LongDouble::from_f64(self.c100),
            c010: LongDouble::from_f64(self.c010),
            c001: LongDouble::from_f64(self.c001),
            c000: LongDouble::from_f64(self.c000),

            inside: self.inside,
        })
    }
}

struct Quadric {
    position: Position,
    albedo: LDRColor,
    roughness: LongDouble,
    metallic: LongDouble,

    c200: LongDouble,
    c020: LongDouble,
    c002: LongDouble,
    c110: LongDouble,
    c011: LongDouble,
    c101: LongDouble,
    c100: LongDouble,
    c010: LongDouble,
    c001: LongDouble,
    c000: LongDouble,

    inside: Position,
}

impl Quadric {
    fn internal_test(&self, ray: Ray) -> Option<(Hit, Hit)> {
        // Move the sphere to the origin for simplicity
        let origin: Position = (ray.origin - self.position).into();

        let (a, b, c) = {
            let p = ray.direction.x;
            let q = ray.direction.y;
            let r = ray.direction.z;
            let u = origin.x;
            let v = origin.y;
            let w = origin.z;
            let mut a = LongDouble::from_f64(0.0);
            let mut b = LongDouble::from_f64(0.0);
            let mut c = LongDouble::from_f64(0.0);
            // c200
            a += self.c200 * p * p;
            b += self.c200 * LongDouble::from_f64(2.0) * p * u;
            c += self.c200 * u * u;
            // c020
            a += self.c020 * q * q;
            b += self.c020 * LongDouble::from_f64(2.0) * q * v;
            c += self.c020 * v * v;
            // c002
            a += self.c002 * r * r;
            b += self.c002 * LongDouble::from_f64(2.0) * r * w;
            c += self.c002 * w * w;
            // c110
            a += self.c110 * p * q;
            b += self.c110 * p * v;
            b += self.c110 * u * q;
            c += self.c110 * u * v;
            // c011
            a += self.c011 * q * r;
            b += self.c011 * q * w;
            b += self.c011 * v * r;
            c += self.c011 * v * w;
            // c101
            a += self.c101 * p * r;
            b += self.c101 * p * w;
            b += self.c101 * u * r;
            c += self.c101 * u * w;
            // c100
            b += self.c100 * p;
            c += self.c100 * u;
            // c010
            b += self.c010 * q;
            c += self.c010 * v;
            // c001
            b += self.c001 * r;
            c += self.c001 * w;
            // c000
            c += self.c000;
            // done
            (a, b, c)
        };

        let discriminant = b * b - LongDouble::from_f64(4.0) * a * c;
        if discriminant < LongDouble::from_f64(0.0) {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let (t1, t2) = {
            let t1 = (-b - sqrt_d) / (LongDouble::from_f64(2.0) * a);
            let t2 = (-b + sqrt_d) / (LongDouble::from_f64(2.0) * a);
            if t1 < t2 {
                (t1, t2)
            } else {
                (t2, t1)
            }
        };
        if t2 < LongDouble::from_f64(0.0) {
            return None;
        }

        if t1 < LongDouble::from_f64(0.0) {
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
                    distance: LongDouble::infinity(),
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
            LongDouble::from_f64(2.0) * self.c200 * position.x
                + self.c110 * position.y
                + self.c101 * position.z
                + self.c100,
            LongDouble::from_f64(2.0) * self.c020 * position.y
                + self.c110 * position.x
                + self.c011 * position.z
                + self.c010,
            LongDouble::from_f64(2.0) * self.c002 * position.z
                + self.c011 * position.y
                + self.c101 * position.x
                + self.c001,
        ))
    }
}

impl RTModel for Quadric {
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
                    distance: LongDouble::from_f64(0.0),
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
                    distance: LongDouble::infinity(),
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
                distance: LongDouble::from_f64(0.0),
                normal: -ray.direction,
                albedo: self.albedo,
                is_front_face: true,
                roughness: self.roughness,
                metallic: self.metallic,
            });
            result.push(Hit {
                distance: LongDouble::infinity(),
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
