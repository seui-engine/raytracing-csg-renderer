use glam::Vec3;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position},
    rt::Ray,
};
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
pub struct Quartic {
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
    c400: f32,
    #[serde(default = "zero")]
    c040: f32,
    #[serde(default = "zero")]
    c004: f32,
    #[serde(default = "zero")]
    c310: f32,
    #[serde(default = "zero")]
    c301: f32,
    #[serde(default = "zero")]
    c130: f32,
    #[serde(default = "zero")]
    c031: f32,
    #[serde(default = "zero")]
    c103: f32,
    #[serde(default = "zero")]
    c013: f32,
    #[serde(default = "zero")]
    c211: f32,
    #[serde(default = "zero")]
    c121: f32,
    #[serde(default = "zero")]
    c112: f32,
    #[serde(default = "zero")]
    c220: f32,
    #[serde(default = "zero")]
    c022: f32,
    #[serde(default = "zero")]
    c202: f32,
    #[serde(default = "zero")]
    c300: f32,
    #[serde(default = "zero")]
    c030: f32,
    #[serde(default = "zero")]
    c003: f32,
    #[serde(default = "zero")]
    c210: f32,
    #[serde(default = "zero")]
    c201: f32,
    #[serde(default = "zero")]
    c120: f32,
    #[serde(default = "zero")]
    c021: f32,
    #[serde(default = "zero")]
    c102: f32,
    #[serde(default = "zero")]
    c012: f32,
    #[serde(default = "zero")]
    c111: f32,
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

    #[serde(deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    inside: Position,
}

pub fn quartic_roots(a: f32, b: f32, c: f32, d: f32, e: f32) -> Vec<f32> {
    if a.abs() <= 0.000001 {
        if b.abs() <= 0.000001 {
            let mut roots = Vec::new();
            if c.abs() <= 0.000001 {
                if d.abs() <= 0.000001 {
                    return roots;
                } else {
                    roots.push(-e / d);
                }
            } else {
                let discriminant = d.powi(2) - 4.0 * c * e;
                if discriminant >= 0.0 {
                    roots.push((-d + discriminant.sqrt()) / (2.0 * c));
                    roots.push((-d - discriminant.sqrt()) / (2.0 * c));
                }
            }
            roots.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            return roots;
        }

        let a1 = c / b;
        let a2 = d / b;
        let a3 = e / b;

        let q = (3.0 * a2 - a1 * a1) / 9.0;
        let r = (9.0 * a1 * a2 - 27.0 * a3 - 2.0 * a1 * a1 * a1) / 54.0;
        let discriminant = q * q * q + r * r;
        let a_div_3 = a1 / 3.0;

        let mut roots = Vec::new();

        if discriminant > 0.0 {
            let s = (r + discriminant.sqrt()).cbrt();
            let t = (r - discriminant.sqrt()).cbrt();
            roots.push(s + t - a_div_3);
        } else {
            let theta = (r / (-q.powi(3)).sqrt()).acos();
            if theta.is_nan() {
                return roots;
            }
            let sqrt_q = (-q).sqrt();
            roots.push(2.0 * sqrt_q * (theta / 3.0).cos() - a_div_3);
            roots.push(2.0 * sqrt_q * ((theta + 2.0 * std::f32::consts::PI) / 3.0).cos() - a_div_3);
            roots.push(2.0 * sqrt_q * ((theta - 2.0 * std::f32::consts::PI) / 3.0).cos() - a_div_3);
        }

        roots.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        return roots;
    }

    // TODO: implement it
    vec![]
}

impl Quartic {
    fn internal_test(&self, ray: Ray) -> Vec<Hit> {
        let origin: Position = (ray.origin - self.position).into();

        let (a, b, c, d, e) = {
            let mut a = 0.0;
            let mut b = 0.0;
            let mut c = 0.0;
            let mut d = 0.0;
            let mut e = 0.0;
            // c400
            a += self.c400 * ray.direction.x.powi(4);
            b += self.c400 * 4.0 * ray.direction.x.powi(3) * origin.x;
            c += self.c400 * 6.0 * ray.direction.x.powi(2) * origin.x.powi(2);
            d += self.c400 * 4.0 * ray.direction.x * origin.x.powi(3);
            e += self.c400 * origin.x.powi(4);
            // c040
            a += self.c040 * ray.direction.y.powi(4);
            b += self.c040 * 4.0 * ray.direction.y.powi(3) * origin.y;
            c += self.c040 * 6.0 * ray.direction.y.powi(2) * origin.y.powi(2);
            d += self.c040 * 4.0 * ray.direction.y * origin.y.powi(3);
            e += self.c040 * origin.y.powi(4);
            // c004
            a += self.c004 * ray.direction.z.powi(4);
            b += self.c004 * 4.0 * ray.direction.z.powi(3) * origin.z;
            c += self.c004 * 6.0 * ray.direction.z.powi(2) * origin.z.powi(2);
            d += self.c004 * 4.0 * ray.direction.z * origin.z.powi(3);
            e += self.c004 * origin.z.powi(4);
            // c310
            a += self.c310 * ray.direction.x.powi(3) * ray.direction.y;
            b += self.c310 * 3.0 * ray.direction.x.powi(3) * ray.origin.y;
            b += self.c310 * ray.direction.x.powi(2) * ray.direction.y * ray.origin.x;
            c += self.c310 * 3.0 * ray.direction.x.powi(2) * ray.origin.x * ray.origin.y;
            c += self.c310 * 3.0 * ray.direction.x * ray.direction.y * ray.origin.x.powi(2);
            d += self.c310 * 3.0 * ray.direction.x * ray.origin.x.powi(2) * ray.origin.y;
            d += self.c310 * ray.direction.y * ray.origin.x.powi(3);
            e += self.c310 * ray.origin.x.powi(3) * ray.origin.y;
            // c301
            a += self.c301 * ray.direction.x.powi(3) * ray.direction.z;
            b += self.c301 * 3.0 * ray.direction.x.powi(3) * ray.origin.z;
            b += self.c301 * ray.direction.x.powi(2) * ray.direction.z * ray.origin.x;
            c += self.c301 * 3.0 * ray.direction.x.powi(2) * ray.origin.x * ray.origin.z;
            c += self.c301 * 3.0 * ray.direction.x * ray.direction.z * ray.origin.x.powi(2);
            d += self.c301 * 3.0 * ray.direction.x * ray.origin.x.powi(2) * ray.origin.z;
            d += self.c301 * ray.direction.z * ray.origin.x.powi(3);
            e += self.c301 * ray.origin.x.powi(3) * ray.origin.z;
            // c130
            a += self.c130 * ray.direction.y.powi(3) * ray.direction.x;
            b += self.c130 * 3.0 * ray.direction.y.powi(3) * ray.origin.x;
            b += self.c130 * ray.direction.y.powi(2) * ray.direction.x * ray.origin.y;
            c += self.c130 * 3.0 * ray.direction.y.powi(2) * ray.origin.y * ray.origin.x;
            c += self.c130 * 3.0 * ray.direction.y * ray.direction.x * ray.origin.y.powi(2);
            d += self.c130 * 3.0 * ray.direction.y * ray.origin.y.powi(2) * ray.origin.x;
            d += self.c130 * ray.direction.x * ray.origin.y.powi(3);
            e += self.c130 * ray.origin.y.powi(3) * ray.origin.x;
            // c031
            a += self.c031 * ray.direction.y.powi(3) * ray.direction.z;
            b += self.c031 * 3.0 * ray.direction.y.powi(3) * ray.origin.z;
            b += self.c031 * ray.direction.y.powi(2) * ray.direction.z * ray.origin.y;
            c += self.c031 * 3.0 * ray.direction.y.powi(2) * ray.origin.y * ray.origin.z;
            c += self.c031 * 3.0 * ray.direction.y * ray.direction.z * ray.origin.y.powi(2);
            d += self.c031 * 3.0 * ray.direction.y * ray.origin.y.powi(2) * ray.origin.z;
            d += self.c031 * ray.direction.z * ray.origin.y.powi(3);
            e += self.c031 * ray.origin.y.powi(3) * ray.origin.z;
            // c103
            a += self.c103 * ray.direction.z.powi(3) * ray.direction.x;
            b += self.c103 * 3.0 * ray.direction.z.powi(3) * ray.origin.x;
            b += self.c103 * ray.direction.z.powi(2) * ray.direction.x * ray.origin.z;
            c += self.c103 * 3.0 * ray.direction.z.powi(2) * ray.origin.z * ray.origin.x;
            c += self.c103 * 3.0 * ray.direction.z * ray.direction.x * ray.origin.z.powi(2);
            d += self.c103 * 3.0 * ray.direction.z * ray.origin.z.powi(2) * ray.origin.x;
            d += self.c103 * ray.direction.x * ray.origin.z.powi(3);
            e += self.c103 * ray.origin.z.powi(3) * ray.origin.x;
            // c013
            a += self.c013 * ray.direction.z.powi(3) * ray.direction.y;
            b += self.c013 * 3.0 * ray.direction.z.powi(3) * ray.origin.y;
            b += self.c013 * ray.direction.z.powi(2) * ray.direction.y * ray.origin.z;
            c += self.c013 * 3.0 * ray.direction.z.powi(2) * ray.origin.z * ray.origin.y;
            c += self.c013 * 3.0 * ray.direction.z * ray.direction.y * ray.origin.z.powi(2);
            d += self.c013 * 3.0 * ray.direction.z * ray.origin.z.powi(2) * ray.origin.y;
            d += self.c013 * ray.direction.y * ray.origin.z.powi(3);
            e += self.c013 * ray.origin.z.powi(3) * ray.origin.y;
            // c211
            a += self.c211 * ray.direction.x.powi(2) * ray.direction.y * ray.direction.z;
            b += self.c211
                * 2.0
                * ray.direction.x
                * ray.direction.y
                * ray.direction.z
                * ray.origin.x;
            b += self.c211 * ray.direction.x.powi(2) * ray.direction.z * ray.origin.y;
            b += self.c211 * ray.direction.x.powi(2) * ray.direction.y * ray.origin.z;
            c += self.c211 * ray.direction.x.powi(2) * ray.origin.y * ray.origin.z;
            c += self.c211 * 2.0 * ray.direction.x * ray.direction.y * ray.origin.x * ray.origin.z;
            c += self.c211 * 2.0 * ray.direction.x * ray.direction.z * ray.origin.x * ray.origin.y;
            c += self.c211 * ray.direction.y * ray.direction.z * ray.origin.x.powi(2);
            d += self.c211 * 2.0 * ray.direction.x * ray.origin.x * ray.origin.y * ray.origin.z;
            d += self.c211 * ray.direction.y * ray.origin.x.powi(2) * ray.origin.z;
            d += self.c211 * ray.direction.z * ray.origin.x.powi(2) * ray.origin.y;
            e += self.c211 * ray.origin.x.powi(2) * ray.origin.y + ray.origin.z;
            // c121
            a += self.c121 * ray.direction.y.powi(2) * ray.direction.x * ray.direction.z;
            b += self.c121
                * 2.0
                * ray.direction.y
                * ray.direction.x
                * ray.direction.z
                * ray.origin.y;
            b += self.c121 * ray.direction.y.powi(2) * ray.direction.z * ray.origin.x;
            b += self.c121 * ray.direction.y.powi(2) * ray.direction.x * ray.origin.z;
            c += self.c121 * ray.direction.y.powi(2) * ray.origin.x * ray.origin.z;
            c += self.c121 * 2.0 * ray.direction.y * ray.direction.x * ray.origin.y * ray.origin.z;
            c += self.c121 * 2.0 * ray.direction.y * ray.direction.z * ray.origin.y * ray.origin.x;
            c += self.c121 * ray.direction.x * ray.direction.z * ray.origin.y.powi(2);
            d += self.c121 * 2.0 * ray.direction.y * ray.origin.y * ray.origin.x * ray.origin.z;
            d += self.c121 * ray.direction.x * ray.origin.y.powi(2) * ray.origin.z;
            d += self.c121 * ray.direction.z * ray.origin.y.powi(2) * ray.origin.x;
            e += self.c121 * ray.origin.y.powi(2) * ray.origin.x + ray.origin.z;
            // c112
            a += self.c112 * ray.direction.z.powi(2) * ray.direction.x * ray.direction.y;
            b += self.c112
                * 2.0
                * ray.direction.z
                * ray.direction.x
                * ray.direction.y
                * ray.origin.z;
            b += self.c112 * ray.direction.z.powi(2) * ray.direction.y * ray.origin.x;
            b += self.c112 * ray.direction.z.powi(2) * ray.direction.x * ray.origin.y;
            c += self.c112 * ray.direction.z.powi(2) * ray.origin.x * ray.origin.y;
            c += self.c112 * 2.0 * ray.direction.z * ray.direction.x * ray.origin.z * ray.origin.y;
            c += self.c112 * 2.0 * ray.direction.z * ray.direction.y * ray.origin.z * ray.origin.x;
            c += self.c112 * ray.direction.x * ray.direction.y * ray.origin.z.powi(2);
            d += self.c112 * 2.0 * ray.direction.z * ray.origin.z * ray.origin.x * ray.origin.y;
            d += self.c112 * ray.direction.x * ray.origin.z.powi(2) * ray.origin.y;
            d += self.c112 * ray.direction.y * ray.origin.z.powi(2) * ray.origin.x;
            e += self.c112 * ray.origin.z.powi(2) * ray.origin.x + ray.origin.y;
            // c220
            a += self.c220 * ray.direction.x.powi(2) * ray.direction.y.powi(2);
            b += self.c220 * 2.0 * ray.direction.x.powi(2) * ray.direction.y * ray.origin.y;
            b += self.c220 * 2.0 * ray.direction.x * ray.direction.y.powi(2) * ray.origin.x;
            c += self.c220 * ray.direction.x.powi(2) * ray.origin.y.powi(2);
            c += self.c220 * 4.0 * ray.direction.x * ray.direction.y * ray.origin.x * ray.origin.y;
            c += self.c220 * ray.direction.y.powi(2) * ray.origin.x.powi(2);
            d += self.c220 * 2.0 * ray.direction.x * ray.origin.x * ray.origin.y.powi(2);
            d += self.c220 * 2.0 * ray.direction.y * ray.origin.x.powi(2) * ray.origin.y;
            e += self.c220 * ray.origin.x.powi(2) * ray.origin.y.powi(2);
            // c022
            a += self.c022 * ray.direction.z.powi(2) * ray.direction.y.powi(2);
            b += self.c022 * 2.0 * ray.direction.z.powi(2) * ray.direction.y * ray.origin.y;
            b += self.c022 * 2.0 * ray.direction.z * ray.direction.y.powi(2) * ray.origin.z;
            c += self.c022 * ray.direction.z.powi(2) * ray.origin.y.powi(2);
            c += self.c022 * 4.0 * ray.direction.z * ray.direction.y * ray.origin.z * ray.origin.y;
            c += self.c022 * ray.direction.y.powi(2) * ray.origin.z.powi(2);
            d += self.c022 * 2.0 * ray.direction.z * ray.origin.z * ray.origin.y.powi(2);
            d += self.c022 * 2.0 * ray.direction.y * ray.origin.z.powi(2) * ray.origin.y;
            e += self.c022 * ray.origin.z.powi(2) * ray.origin.y.powi(2);
            // c202
            a += self.c202 * ray.direction.z.powi(2) * ray.direction.x.powi(2);
            b += self.c202 * 2.0 * ray.direction.z.powi(2) * ray.direction.x * ray.origin.x;
            b += self.c202 * 2.0 * ray.direction.z * ray.direction.x.powi(2) * ray.origin.z;
            c += self.c202 * ray.direction.z.powi(2) * ray.origin.x.powi(2);
            c += self.c202 * 4.0 * ray.direction.z * ray.direction.x * ray.origin.z * ray.origin.x;
            c += self.c202 * ray.direction.x.powi(2) * ray.origin.z.powi(2);
            d += self.c202 * 2.0 * ray.direction.z * ray.origin.z * ray.origin.x.powi(2);
            d += self.c202 * 2.0 * ray.direction.x * ray.origin.z.powi(2) * ray.origin.x;
            e += self.c202 * ray.origin.z.powi(2) * ray.origin.x.powi(2);
            // c300
            b += self.c300 * ray.direction.x.powi(3);
            c += self.c300 * 3.0 * ray.direction.x.powi(2) * origin.x;
            d += self.c300 * 3.0 * ray.direction.x * origin.x.powi(2);
            e += self.c300 * origin.x.powi(3);
            // c030
            b += self.c030 * ray.direction.y.powi(3);
            c += self.c030 * 3.0 * ray.direction.y.powi(2) * origin.y;
            d += self.c030 * 3.0 * ray.direction.y * origin.y.powi(2);
            e += self.c030 * origin.y.powi(3);
            // c003
            b += self.c003 * ray.direction.z.powi(3);
            c += self.c003 * 3.0 * ray.direction.z.powi(2) * origin.z;
            d += self.c003 * 3.0 * ray.direction.z * origin.z.powi(2);
            e += self.c003 * origin.z.powi(3);
            // c210
            b += self.c210 * ray.direction.x.powi(2) * ray.direction.y;
            c += self.c210 * ray.direction.x.powi(2) * origin.y;
            c += self.c210 * 2.0 * ray.direction.x * ray.direction.y * origin.x;
            d += self.c210 * 2.0 * ray.direction.x * origin.x * origin.y;
            d += self.c210 * ray.direction.y * origin.x.powi(2);
            e += self.c210 * origin.x.powi(2) * origin.y;
            // c201
            b += self.c201 * ray.direction.x.powi(2) * ray.direction.z;
            c += self.c201 * ray.direction.x.powi(2) * origin.z;
            c += self.c201 * 2.0 * ray.direction.x * ray.direction.z * origin.x;
            d += self.c201 * 2.0 * ray.direction.x * origin.x * origin.z;
            d += self.c201 * ray.direction.z * origin.x.powi(2);
            e += self.c201 * origin.x.powi(2) * origin.z;
            // c120
            b += self.c120 * ray.direction.y.powi(2) * ray.direction.x;
            c += self.c120 * ray.direction.y.powi(2) * origin.x;
            c += self.c120 * 2.0 * ray.direction.y * ray.direction.x * origin.y;
            d += self.c120 * 2.0 * ray.direction.y * origin.y * origin.x;
            d += self.c120 * ray.direction.x * origin.y.powi(2);
            e += self.c120 * origin.y.powi(2) * origin.x;
            // c021
            b += self.c021 * ray.direction.y.powi(2) * ray.direction.z;
            c += self.c021 * ray.direction.y.powi(2) * origin.z;
            c += self.c021 * 2.0 * ray.direction.y * ray.direction.z * origin.y;
            d += self.c021 * 2.0 * ray.direction.y * origin.y * origin.z;
            d += self.c021 * ray.direction.z * origin.y.powi(2);
            e += self.c021 * origin.y.powi(2) * origin.z;
            // c102
            b += self.c102 * ray.direction.z.powi(2) * ray.direction.x;
            c += self.c102 * ray.direction.z.powi(2) * origin.x;
            c += self.c102 * 2.0 * ray.direction.z * ray.direction.x * origin.z;
            d += self.c102 * 2.0 * ray.direction.z * origin.z * origin.x;
            d += self.c102 * ray.direction.x * origin.z.powi(2);
            e += self.c102 * origin.z.powi(2) * origin.x;
            // c012
            b += self.c012 * ray.direction.z.powi(2) * ray.direction.y;
            c += self.c012 * ray.direction.z.powi(2) * origin.y;
            c += self.c012 * 2.0 * ray.direction.z * ray.direction.y * origin.z;
            d += self.c012 * 2.0 * ray.direction.z * origin.z * origin.y;
            d += self.c012 * ray.direction.y * origin.z.powi(2);
            e += self.c012 * origin.z.powi(2) * origin.y;
            // c111
            b += self.c111 * ray.direction.x * ray.direction.y * ray.direction.z;
            c += self.c111 * ray.direction.x * ray.direction.y * origin.z;
            c += self.c111 * ray.direction.x * origin.y * ray.direction.z;
            c += self.c111 * origin.x * ray.direction.y * ray.direction.z;
            d += self.c111 * ray.direction.x * origin.y * origin.z;
            d += self.c111 * origin.x * ray.direction.y * origin.z;
            d += self.c111 * origin.x * origin.y * ray.direction.z;
            e += self.c111 * origin.x * origin.y * origin.z;
            // c200
            c += self.c200 * ray.direction.x.powi(2);
            d += self.c200 * 2.0 * ray.direction.x * origin.x;
            e += self.c200 * origin.x.powi(2);
            // c020
            c += self.c020 * ray.direction.y.powi(2);
            d += self.c020 * 2.0 * ray.direction.y * origin.y;
            e += self.c020 * origin.y.powi(2);
            // c002
            c += self.c002 * ray.direction.z.powi(2);
            d += self.c002 * 2.0 * ray.direction.z * origin.z;
            e += self.c002 * origin.z.powi(2);
            // c110
            c += self.c110 * ray.direction.x * ray.direction.y;
            d += self.c110 * ray.direction.x * origin.y;
            d += self.c110 * origin.x * ray.direction.y;
            e += self.c110 * origin.x * origin.y;
            // c011
            c += self.c011 * ray.direction.y * ray.direction.z;
            d += self.c011 * ray.direction.y * origin.z;
            d += self.c011 * origin.y * ray.direction.z;
            e += self.c011 * origin.y * origin.z;
            // c101
            c += self.c101 * ray.direction.x * ray.direction.z;
            d += self.c101 * ray.direction.x * origin.z;
            d += self.c101 * origin.x * ray.direction.z;
            e += self.c101 * origin.x * origin.z;
            // c100
            d += self.c100 * ray.direction.x;
            e += self.c100 * origin.x;
            // c010
            d += self.c010 * ray.direction.y;
            e += self.c010 * origin.y;
            // c001
            d += self.c001 * ray.direction.z;
            e += self.c001 * origin.z;
            // c000
            e += self.c000;
            // done
            (a, b, c, d, e)
        };

        quartic_roots(a, b, c, d, e)
            .into_iter()
            .filter(|t| *t >= 0.0)
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
        Direction::new(Vec3::new(0.0, 0.0, 0.0))
    }
}

impl RTModel for Quartic {
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
                distance: 0.0,
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
