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
#[serde(rename_all = "camelCase")]
pub struct DeserializableQuadratic {
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
    c300: f64,
    #[serde(default = "zero")]
    c030: f64,
    #[serde(default = "zero")]
    c003: f64,
    #[serde(default = "zero")]
    c210: f64,
    #[serde(default = "zero")]
    c201: f64,
    #[serde(default = "zero")]
    c120: f64,
    #[serde(default = "zero")]
    c021: f64,
    #[serde(default = "zero")]
    c102: f64,
    #[serde(default = "zero")]
    c012: f64,
    #[serde(default = "zero")]
    c111: f64,
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

    #[serde(deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    inside: Position,
}

impl DeserializableQuadratic {
    pub fn into_rt_model(self) -> Box<dyn RTModel + Send + Sync> {
        Box::new(Quadratic {
            position: self.position,
            albedo: self.albedo,
            roughness: LongDouble::from_f64(self.roughness),
            metallic: LongDouble::from_f64(self.metallic),

            c300: LongDouble::from_f64(self.c300),
            c030: LongDouble::from_f64(self.c030),
            c003: LongDouble::from_f64(self.c003),
            c210: LongDouble::from_f64(self.c210),
            c201: LongDouble::from_f64(self.c201),
            c120: LongDouble::from_f64(self.c120),
            c021: LongDouble::from_f64(self.c021),
            c102: LongDouble::from_f64(self.c102),
            c012: LongDouble::from_f64(self.c012),
            c111: LongDouble::from_f64(self.c111),
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

pub struct Quadratic {
    position: Position,
    albedo: LDRColor,
    roughness: LongDouble,
    metallic: LongDouble,

    c300: LongDouble,
    c030: LongDouble,
    c003: LongDouble,
    c210: LongDouble,
    c201: LongDouble,
    c120: LongDouble,
    c021: LongDouble,
    c102: LongDouble,
    c012: LongDouble,
    c111: LongDouble,
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

fn cubic_roots(a: LongDouble, b: LongDouble, c: LongDouble, d: LongDouble) -> Vec<LongDouble> {
    if a.abs() <= LongDouble::from_f64(1e-6) {
        let mut roots = Vec::new();
        if b.abs() <= LongDouble::from_f64(1e-6) {
            if c.abs() <= LongDouble::from_f64(1e-6) {
                return roots;
            } else {
                roots.push(-d / c);
            }
        } else {
            let discriminant = c * c - LongDouble::from_f64(4.0) * b * d;
            if discriminant >= LongDouble::from_f64(0.0) {
                roots.push((-c + discriminant.sqrt()) / (LongDouble::from_f64(2.0) * b));
                roots.push((-c - discriminant.sqrt()) / (LongDouble::from_f64(2.0) * b));
            }
        }
        roots.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        return roots;
    }

    let a1 = b / a;
    let a2 = c / a;
    let a3 = d / a;

    let q = (LongDouble::from_f64(3.0) * a2 - a1 * a1) / LongDouble::from_f64(9.0);
    let r = (LongDouble::from_f64(9.0) * a1 * a2
        - LongDouble::from_f64(27.0) * a3
        - LongDouble::from_f64(2.0) * a1 * a1 * a1)
        / LongDouble::from_f64(54.0);
    let discriminant = q * q * q + r * r;
    let a_div_3 = a1 / LongDouble::from_f64(3.0);

    let mut roots = Vec::new();

    if discriminant > LongDouble::from_f64(0.0) {
        let s = (r + discriminant.sqrt()).cbrt();
        let t = (r - discriminant.sqrt()).cbrt();
        roots.push(s + t - a_div_3);
    } else {
        let theta = (r / (-q * q * q).sqrt()).acos();
        if theta.is_nan() {
            return roots;
        }
        let sqrt_q = (-q).sqrt();
        roots.push(
            LongDouble::from_f64(2.0) * sqrt_q * (theta / LongDouble::from_f64(3.0)).cos()
                - a_div_3,
        );
        roots.push(
            LongDouble::from_f64(2.0)
                * sqrt_q
                * ((theta + LongDouble::from_f64(2.0) * LongDouble::pi())
                    / LongDouble::from_f64(3.0))
                .cos()
                - a_div_3,
        );
        roots.push(
            LongDouble::from_f64(2.0)
                * sqrt_q
                * ((theta - LongDouble::from_f64(2.0) * LongDouble::pi())
                    / LongDouble::from_f64(3.0))
                .cos()
                - a_div_3,
        );
    }

    roots.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    roots
}

impl Quadratic {
    fn internal_test(&self, ray: Ray) -> Vec<Hit> {
        let origin: Position = (ray.origin - self.position).into();

        let (a, b, c, d) = {
            let p = ray.direction.x;
            let q = ray.direction.y;
            let r = ray.direction.z;
            let u = origin.x;
            let v = origin.y;
            let w = origin.z;
            let mut a = LongDouble::from_f64(0.0);
            let mut b = LongDouble::from_f64(0.0);
            let mut c = LongDouble::from_f64(0.0);
            let mut d = LongDouble::from_f64(0.0);
            // c300
            a += self.c300 * p * p * p;
            b += self.c300 * LongDouble::from_f64(3.0) * p * p * u;
            c += self.c300 * LongDouble::from_f64(3.0) * p * u * u;
            d += self.c300 * u * u * u;
            // c030
            a += self.c030 * q * q * q;
            b += self.c030 * LongDouble::from_f64(3.0) * q * q * v;
            c += self.c030 * LongDouble::from_f64(3.0) * q * v * v;
            d += self.c030 * v * v * v;
            // c003
            a += self.c003 * r * r * r;
            b += self.c003 * LongDouble::from_f64(3.0) * r * r * w;
            c += self.c003 * LongDouble::from_f64(3.0) * r * w * w;
            d += self.c003 * w * w * w;
            // c210
            a += self.c210 * p * p * q;
            b += self.c210 * p * p * v;
            b += self.c210 * LongDouble::from_f64(2.0) * p * q * u;
            c += self.c210 * LongDouble::from_f64(2.0) * p * u * v;
            c += self.c210 * q * u * u;
            d += self.c210 * u * u * v;
            // c201
            a += self.c201 * p * p * r;
            b += self.c201 * p * p * w;
            b += self.c201 * LongDouble::from_f64(2.0) * p * r * u;
            c += self.c201 * LongDouble::from_f64(2.0) * p * u * w;
            c += self.c201 * r * u * u;
            d += self.c201 * u * u * w;
            // c120
            a += self.c120 * q * q * p;
            b += self.c120 * q * q * u;
            b += self.c120 * LongDouble::from_f64(2.0) * q * p * v;
            c += self.c120 * LongDouble::from_f64(2.0) * q * v * u;
            c += self.c120 * p * v * v;
            d += self.c120 * v * v * u;
            // c021
            a += self.c021 * q * q * r;
            b += self.c021 * q * q * w;
            b += self.c021 * LongDouble::from_f64(2.0) * q * r * v;
            c += self.c021 * LongDouble::from_f64(2.0) * q * v * w;
            c += self.c021 * r * v * v;
            d += self.c021 * v * v * w;
            // c102
            a += self.c102 * r * r * p;
            b += self.c102 * r * r * u;
            b += self.c102 * LongDouble::from_f64(2.0) * r * p * w;
            c += self.c102 * LongDouble::from_f64(2.0) * r * w * u;
            c += self.c102 * p * w * w;
            d += self.c102 * w * w * u;
            // c012
            a += self.c012 * r * r * q;
            b += self.c012 * r * r * v;
            b += self.c012 * LongDouble::from_f64(2.0) * r * q * w;
            c += self.c012 * LongDouble::from_f64(2.0) * r * w * v;
            c += self.c012 * q * w * w;
            d += self.c012 * w * w * v;
            // c111
            a += self.c111 * p * q * r;
            b += self.c111 * p * q * w;
            b += self.c111 * p * v * r;
            b += self.c111 * u * q * r;
            c += self.c111 * p * v * w;
            c += self.c111 * u * q * w;
            c += self.c111 * u * v * r;
            d += self.c111 * u * v * w;
            // c200
            b += self.c200 * p * p;
            c += self.c200 * LongDouble::from_f64(2.0) * p * u;
            d += self.c200 * u * u;
            // c020
            b += self.c020 * q * q;
            c += self.c020 * LongDouble::from_f64(2.0) * q * v;
            d += self.c020 * v * v;
            // c002
            b += self.c002 * r * r;
            c += self.c002 * LongDouble::from_f64(2.0) * r * w;
            d += self.c002 * w * w;
            // c110
            b += self.c110 * p * q;
            c += self.c110 * p * v;
            c += self.c110 * u * q;
            d += self.c110 * u * v;
            // c011
            b += self.c011 * q * r;
            c += self.c011 * q * w;
            c += self.c011 * v * r;
            d += self.c011 * v * w;
            // c101
            b += self.c101 * p * r;
            c += self.c101 * p * w;
            c += self.c101 * u * r;
            d += self.c101 * u * w;
            // c100
            c += self.c100 * p;
            d += self.c100 * u;
            // c010
            c += self.c010 * q;
            d += self.c010 * v;
            // c001
            c += self.c001 * r;
            d += self.c001 * w;
            // c000
            d += self.c000;
            // done
            (a, b, c, d)
        };

        cubic_roots(a, b, c, d)
            .into_iter()
            .filter(|t| *t >= LongDouble::from_f64(0.0))
            .map(|distance| Hit {
                distance,
                normal: self.normal(origin + ray.direction * distance),
                albedo: self.albedo,
                is_front_face: true, // decided later
                roughness: self.roughness,
                metallic: self.metallic,
            })
            .collect()
    }

    fn normal(&self, position: Position) -> Direction {
        let x = position.x;
        let y = position.y;
        let z = position.z;
        Direction::new(Vec3::new(
            LongDouble::from_f64(3.0) * self.c300 * x * x
                + LongDouble::from_f64(2.0) * self.c210 * x * y
                + LongDouble::from_f64(2.0) * self.c201 * x * z
                + LongDouble::from_f64(2.0) * self.c200 * x
                + self.c120 * y * y
                + self.c102 * z * z
                + self.c111 * y * z
                + self.c110 * y
                + self.c101 * z
                + self.c100,
            LongDouble::from_f64(3.0) * self.c030 * y * y
                + LongDouble::from_f64(2.0) * self.c120 * y * x
                + LongDouble::from_f64(2.0) * self.c021 * y * z
                + LongDouble::from_f64(2.0) * self.c020 * y
                + self.c210 * x * x
                + self.c012 * z * z
                + self.c111 * x * z
                + self.c110 * x
                + self.c011 * z
                + self.c010,
            LongDouble::from_f64(3.0) * self.c003 * z * z
                + LongDouble::from_f64(2.0) * self.c102 * z * x
                + LongDouble::from_f64(2.0) * self.c012 * z * y
                + LongDouble::from_f64(2.0) * self.c002 * z
                + self.c201 * x * x
                + self.c021 * y * y
                + self.c111 * x * y
                + self.c101 * x
                + self.c011 * y
                + self.c001,
        ))
    }
}

impl RTModel for Quadratic {
    fn test(&self, ray: Ray) -> Vec<Hit> {
        let (inside_direction, inside_length) = (ray.origin - self.inside).direction_and_length();
        let internal = self.internal_test(Ray {
            origin: self.inside,
            direction: inside_direction,
        });
        let inside = internal
            .into_iter()
            .filter(|hit| hit.distance < inside_length)
            .count()
            % 2
            == 0;

        let mut result = Vec::new();

        let mut is_front_face = false;
        if inside {
            is_front_face = true;
            result.push(Hit {
                distance: LongDouble::from_f64(0.0),
                normal: -ray.direction,
                albedo: self.albedo,
                is_front_face,
                roughness: self.roughness,
                metallic: self.metallic,
            });
        }
        for hit in self.internal_test(ray).into_iter() {
            is_front_face = !is_front_face;
            result.push(Hit {
                normal: enhance_normal(ray.direction, hit.normal, is_front_face),
                is_front_face,
                ..hit
            });
        }
        if is_front_face {
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
