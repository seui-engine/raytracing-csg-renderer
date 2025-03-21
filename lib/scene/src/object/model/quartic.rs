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
pub struct DeserializableQuartic {
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
    c400: f64,
    #[serde(default = "zero")]
    c040: f64,
    #[serde(default = "zero")]
    c004: f64,
    #[serde(default = "zero")]
    c310: f64,
    #[serde(default = "zero")]
    c301: f64,
    #[serde(default = "zero")]
    c130: f64,
    #[serde(default = "zero")]
    c031: f64,
    #[serde(default = "zero")]
    c103: f64,
    #[serde(default = "zero")]
    c013: f64,
    #[serde(default = "zero")]
    c211: f64,
    #[serde(default = "zero")]
    c121: f64,
    #[serde(default = "zero")]
    c112: f64,
    #[serde(default = "zero")]
    c220: f64,
    #[serde(default = "zero")]
    c022: f64,
    #[serde(default = "zero")]
    c202: f64,
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

impl DeserializableQuartic {
    pub fn into_rt_model(self) -> Box<dyn RTModel + Send + Sync> {
        Box::new(Quartic {
            position: self.position,
            albedo: self.albedo,
            roughness: LongDouble::from_f64(self.roughness),
            metallic: LongDouble::from_f64(self.metallic),

            c400: LongDouble::from_f64(self.c400),
            c040: LongDouble::from_f64(self.c040),
            c004: LongDouble::from_f64(self.c004),
            c310: LongDouble::from_f64(self.c310),
            c301: LongDouble::from_f64(self.c301),
            c130: LongDouble::from_f64(self.c130),
            c031: LongDouble::from_f64(self.c031),
            c103: LongDouble::from_f64(self.c103),
            c013: LongDouble::from_f64(self.c013),
            c211: LongDouble::from_f64(self.c211),
            c121: LongDouble::from_f64(self.c121),
            c112: LongDouble::from_f64(self.c112),
            c220: LongDouble::from_f64(self.c220),
            c022: LongDouble::from_f64(self.c022),
            c202: LongDouble::from_f64(self.c202),
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

pub struct Quartic {
    position: Position,
    albedo: LDRColor,
    roughness: LongDouble,
    metallic: LongDouble,

    c400: LongDouble,
    c040: LongDouble,
    c004: LongDouble,
    c310: LongDouble,
    c301: LongDouble,
    c130: LongDouble,
    c031: LongDouble,
    c103: LongDouble,
    c013: LongDouble,
    c211: LongDouble,
    c121: LongDouble,
    c112: LongDouble,
    c220: LongDouble,
    c022: LongDouble,
    c202: LongDouble,
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

fn linear_roots(a: LongDouble, b: LongDouble) -> Vec<LongDouble> {
    if b.abs() <= LongDouble::from_f64(1e-6) {
        vec![]
    } else {
        vec![-b / a]
    }
}

fn quadratic_roots(a: LongDouble, b: LongDouble, c: LongDouble) -> Vec<LongDouble> {
    if a.abs() <= LongDouble::from_f64(1e-6) {
        return linear_roots(b, c);
    }

    let mut roots = Vec::new();

    let discriminant = b * b - LongDouble::from_f64(4.0) * a * c;
    if discriminant >= LongDouble::from_f64(0.0) {
        roots.push((-b + discriminant.sqrt()) / (LongDouble::from_f64(2.0) * a));
        roots.push((-b - discriminant.sqrt()) / (LongDouble::from_f64(2.0) * a));
    }

    roots.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    roots
}

fn cubic_roots(a: LongDouble, b: LongDouble, c: LongDouble, d: LongDouble) -> Vec<LongDouble> {
    if a.abs() <= LongDouble::from_f64(1e-6) {
        return quadratic_roots(b, c, d);
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

fn quartic_roots(
    a: LongDouble,
    b: LongDouble,
    c: LongDouble,
    d: LongDouble,
    e: LongDouble,
) -> Vec<LongDouble> {
    if a.abs() <= LongDouble::from_f64(1e-6) {
        return cubic_roots(b, c, d, e);
    }

    let b = b / a;
    let c = c / a;
    let d = d / a;
    let e = e / a;

    let bb = b * b;
    let p = -LongDouble::from_f64(3.0) * bb / LongDouble::from_f64(8.0) + c;
    let q = bb * b / LongDouble::from_f64(8.0) - b * c / LongDouble::from_f64(2.0) + d;
    let r = -LongDouble::from_f64(3.0) * bb * bb / LongDouble::from_f64(256.0)
        + bb * c / LongDouble::from_f64(16.0)
        - b * d / LongDouble::from_f64(4.0)
        + e;

    let mut roots = Vec::new();

    if q.abs() < LongDouble::from_f64(1e-6) {
        let discriminant1 = p * p - LongDouble::from_f64(4.0) * r;
        if discriminant1 >= LongDouble::from_f64(0.0) {
            let y1 = (-p + discriminant1.sqrt()) / LongDouble::from_f64(2.0);
            let y2 = (-p - discriminant1.sqrt()) / LongDouble::from_f64(2.0);

            for y in [y1, y2].iter() {
                if *y >= LongDouble::from_f64(0.0) {
                    roots.push(y.sqrt() - b / LongDouble::from_f64(4.0));
                    roots.push(-y.sqrt() - b / LongDouble::from_f64(4.0));
                }
            }
        }
    } else {
        let cubic_a = LongDouble::from_f64(1.0);
        let cubic_b = -p / LongDouble::from_f64(2.0);
        let cubic_c = -r;
        let cubic_d = (p * r - q * q / LongDouble::from_f64(4.0)) / LongDouble::from_f64(2.0);

        let cubic_roots = cubic_roots(cubic_a, cubic_b, cubic_c, cubic_d);
        if cubic_roots.is_empty() {
            return roots;
        }

        let z = cubic_roots[0];
        let u = (LongDouble::from_f64(2.0) * z - p).sqrt();
        let v = if u.abs() > LongDouble::from_f64(1e-6) {
            q / (LongDouble::from_f64(2.0) * u)
        } else {
            LongDouble::from_f64(0.0)
        };

        let quadratic1_a = LongDouble::from_f64(1.0);
        let quadratic1_b = u;
        let quadratic1_c = z - v;

        let quadratic2_a = LongDouble::from_f64(1.0);
        let quadratic2_b = -u;
        let quadratic2_c = z + v;

        for (qa, qb, qc) in [
            (quadratic1_a, quadratic1_b, quadratic1_c),
            (quadratic2_a, quadratic2_b, quadratic2_c),
        ]
        .into_iter()
        {
            let disc = qb * qb - LongDouble::from_f64(4.0) * qa * qc;
            if disc >= LongDouble::from_f64(0.0) {
                roots.push(
                    (-qb + disc.sqrt()) / (LongDouble::from_f64(2.0) * qa)
                        - b / LongDouble::from_f64(4.0),
                );
                roots.push(
                    (-qb - disc.sqrt()) / (LongDouble::from_f64(2.0) * qa)
                        - b / LongDouble::from_f64(4.0),
                );
            }
        }
    }

    roots.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    roots
}

impl Quartic {
    fn internal_test(&self, ray: Ray) -> Vec<Hit> {
        let origin: Position = (ray.origin - self.position).into();

        let (a, b, c, d, e) = {
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
            let mut e = LongDouble::from_f64(0.0);
            // c400
            a += self.c400 * p * p * p * p;
            b += self.c400 * LongDouble::from_f64(4.0) * p * p * p * u;
            c += self.c400 * LongDouble::from_f64(6.0) * p * p * u * u;
            d += self.c400 * LongDouble::from_f64(4.0) * p * u * u * u;
            e += self.c400 * u * u * u * u;
            // c040
            a += self.c040 * q * q * q * q;
            b += self.c040 * LongDouble::from_f64(4.0) * q * q * q * v;
            c += self.c040 * LongDouble::from_f64(6.0) * q * q * v * v;
            d += self.c040 * LongDouble::from_f64(4.0) * q * v * v * v;
            e += self.c040 * v * v * v * v;
            // c004
            a += self.c004 * r * r * r * r;
            b += self.c004 * LongDouble::from_f64(4.0) * r * r * r * w;
            c += self.c004 * LongDouble::from_f64(6.0) * r * r * w * w;
            d += self.c004 * LongDouble::from_f64(4.0) * r * w * w * w;
            e += self.c004 * w * w * w * w;
            // c310
            a += self.c310 * p * p * p * q;
            b += self.c310 * LongDouble::from_f64(3.0) * p * p * p * v;
            b += self.c310 * p * p * q * u;
            c += self.c310 * LongDouble::from_f64(3.0) * p * p * u * v;
            c += self.c310 * LongDouble::from_f64(3.0) * p * q * u * u;
            d += self.c310 * LongDouble::from_f64(3.0) * p * u * u * v;
            d += self.c310 * q * u * u * u;
            e += self.c310 * u * u * u * v;
            // c301
            a += self.c301 * p * p * p * r;
            b += self.c301 * LongDouble::from_f64(3.0) * p * p * p * w;
            b += self.c301 * p * p * r * u;
            c += self.c301 * LongDouble::from_f64(3.0) * p * p * u * w;
            c += self.c301 * LongDouble::from_f64(3.0) * p * r * u * u;
            d += self.c301 * LongDouble::from_f64(3.0) * p * u * u * w;
            d += self.c301 * r * u * u * u;
            e += self.c301 * u * u * u * w;
            // c130
            a += self.c130 * q * q * q * p;
            b += self.c130 * LongDouble::from_f64(3.0) * q * q * q * u;
            b += self.c130 * q * q * p * v;
            c += self.c130 * LongDouble::from_f64(3.0) * q * q * v * u;
            c += self.c130 * LongDouble::from_f64(3.0) * q * p * v * v;
            d += self.c130 * LongDouble::from_f64(3.0) * q * v * v * u;
            d += self.c130 * p * v * v * v;
            e += self.c130 * v * v * v * u;
            // c031
            a += self.c031 * q * q * q * r;
            b += self.c031 * LongDouble::from_f64(3.0) * q * q * q * w;
            b += self.c031 * q * q * r * v;
            c += self.c031 * LongDouble::from_f64(3.0) * q * q * v * w;
            c += self.c031 * LongDouble::from_f64(3.0) * q * r * v * v;
            d += self.c031 * LongDouble::from_f64(3.0) * q * v * v * w;
            d += self.c031 * r * v * v * v;
            e += self.c031 * v * v * v * w;
            // c103
            a += self.c103 * r * r * r * p;
            b += self.c103 * LongDouble::from_f64(3.0) * r * r * r * u;
            b += self.c103 * r * r * p * w;
            c += self.c103 * LongDouble::from_f64(3.0) * r * r * w * u;
            c += self.c103 * LongDouble::from_f64(3.0) * r * p * w * w;
            d += self.c103 * LongDouble::from_f64(3.0) * r * w * w * u;
            d += self.c103 * p * w * w * w;
            e += self.c103 * w * w * w * u;
            // c013
            a += self.c013 * r * r * r * q;
            b += self.c013 * LongDouble::from_f64(3.0) * r * r * r * v;
            b += self.c013 * r * r * q * w;
            c += self.c013 * LongDouble::from_f64(3.0) * r * r * w * v;
            c += self.c013 * LongDouble::from_f64(3.0) * r * q * w * w;
            d += self.c013 * LongDouble::from_f64(3.0) * r * w * w * v;
            d += self.c013 * q * w * w * w;
            e += self.c013 * w * w * w * v;
            // c211
            a += self.c211 * p * p * q * r;
            b += self.c211 * LongDouble::from_f64(2.0) * p * q * r * u;
            b += self.c211 * p * p * r * v;
            b += self.c211 * p * p * q * w;
            c += self.c211 * p * p * v * w;
            c += self.c211 * LongDouble::from_f64(2.0) * p * q * u * w;
            c += self.c211 * LongDouble::from_f64(2.0) * p * r * u * v;
            c += self.c211 * q * r * u * u;
            d += self.c211 * LongDouble::from_f64(2.0) * p * u * v * w;
            d += self.c211 * q * u * u * w;
            d += self.c211 * r * u * u * v;
            e += self.c211 * u * u * v * w;
            // c121
            a += self.c121 * q * q * p * r;
            b += self.c121 * LongDouble::from_f64(2.0) * q * p * r * v;
            b += self.c121 * q * q * r * u;
            b += self.c121 * q * q * p * w;
            c += self.c121 * q * q * u * w;
            c += self.c121 * LongDouble::from_f64(2.0) * q * p * v * w;
            c += self.c121 * LongDouble::from_f64(2.0) * q * r * v * u;
            c += self.c121 * p * r * v * v;
            d += self.c121 * LongDouble::from_f64(2.0) * q * v * u * w;
            d += self.c121 * p * v * v * w;
            d += self.c121 * r * v * v * u;
            e += self.c121 * v * v * u * w;
            // c112
            a += self.c112 * r * r * p * q;
            b += self.c112 * LongDouble::from_f64(2.0) * r * p * q * w;
            b += self.c112 * r * r * q * u;
            b += self.c112 * r * r * p * v;
            c += self.c112 * r * r * u * v;
            c += self.c112 * LongDouble::from_f64(2.0) * r * p * w * v;
            c += self.c112 * LongDouble::from_f64(2.0) * r * q * w * u;
            c += self.c112 * p * q * w * w;
            d += self.c112 * LongDouble::from_f64(2.0) * r * w * u * v;
            d += self.c112 * p * w * w * v;
            d += self.c112 * q * w * w * u;
            e += self.c112 * w * w * u * v;
            // c220
            a += self.c220 * p * p * q * q;
            b += self.c220 * LongDouble::from_f64(2.0) * p * p * q * v;
            b += self.c220 * LongDouble::from_f64(2.0) * p * q * q * u;
            c += self.c220 * p * p * v * v;
            c += self.c220 * LongDouble::from_f64(4.0) * p * q * u * v;
            c += self.c220 * q * q * u * u;
            d += self.c220 * LongDouble::from_f64(2.0) * p * u * v * v;
            d += self.c220 * LongDouble::from_f64(2.0) * q * u * u * v;
            e += self.c220 * u * u * v * v;
            // c022
            a += self.c022 * r * r * q * q;
            b += self.c022 * LongDouble::from_f64(2.0) * r * r * q * v;
            b += self.c022 * LongDouble::from_f64(2.0) * r * q * q * w;
            c += self.c022 * r * r * v * v;
            c += self.c022 * LongDouble::from_f64(4.0) * r * q * w * v;
            c += self.c022 * q * q * w * w;
            d += self.c022 * LongDouble::from_f64(2.0) * r * w * v * v;
            d += self.c022 * LongDouble::from_f64(2.0) * q * w * w * v;
            e += self.c022 * w * w * v * v;
            // c202
            a += self.c202 * r * r * p * p;
            b += self.c202 * LongDouble::from_f64(2.0) * r * r * p * u;
            b += self.c202 * LongDouble::from_f64(2.0) * r * p * p * w;
            c += self.c202 * r * r * u * u;
            c += self.c202 * LongDouble::from_f64(4.0) * r * p * w * u;
            c += self.c202 * p * p * w * w;
            d += self.c202 * LongDouble::from_f64(2.0) * r * w * u * u;
            d += self.c202 * LongDouble::from_f64(2.0) * p * w * w * u;
            e += self.c202 * w * w * u * u;
            // c300
            b += self.c300 * p * p * p;
            c += self.c300 * LongDouble::from_f64(3.0) * p * p * u;
            d += self.c300 * LongDouble::from_f64(3.0) * p * u * u;
            e += self.c300 * u * u * u;
            // c030
            b += self.c030 * q * q * q;
            c += self.c030 * LongDouble::from_f64(3.0) * q * q * v;
            d += self.c030 * LongDouble::from_f64(3.0) * q * v * v;
            e += self.c030 * v * v * v;
            // c003
            b += self.c003 * r * r * r;
            c += self.c003 * LongDouble::from_f64(3.0) * r * r * w;
            d += self.c003 * LongDouble::from_f64(3.0) * r * w * w;
            e += self.c003 * w * w * w;
            // c210
            b += self.c210 * p * p * q;
            c += self.c210 * p * p * v;
            c += self.c210 * LongDouble::from_f64(2.0) * p * q * u;
            d += self.c210 * LongDouble::from_f64(2.0) * p * u * v;
            d += self.c210 * q * u * u;
            e += self.c210 * u * u * v;
            // c201
            b += self.c201 * p * p * r;
            c += self.c201 * p * p * w;
            c += self.c201 * LongDouble::from_f64(2.0) * p * r * u;
            d += self.c201 * LongDouble::from_f64(2.0) * p * u * w;
            d += self.c201 * r * u * u;
            e += self.c201 * u * u * w;
            // c120
            b += self.c120 * q * q * p;
            c += self.c120 * q * q * u;
            c += self.c120 * LongDouble::from_f64(2.0) * q * p * v;
            d += self.c120 * LongDouble::from_f64(2.0) * q * v * u;
            d += self.c120 * p * v * v;
            e += self.c120 * v * v * u;
            // c021
            b += self.c021 * q * q * r;
            c += self.c021 * q * q * w;
            c += self.c021 * LongDouble::from_f64(2.0) * q * r * v;
            d += self.c021 * LongDouble::from_f64(2.0) * q * v * w;
            d += self.c021 * r * v * v;
            e += self.c021 * v * v * w;
            // c102
            b += self.c102 * r * r * p;
            c += self.c102 * r * r * u;
            c += self.c102 * LongDouble::from_f64(2.0) * r * p * w;
            d += self.c102 * LongDouble::from_f64(2.0) * r * w * u;
            d += self.c102 * p * w * w;
            e += self.c102 * w * w * u;
            // c012
            b += self.c012 * r * r * q;
            c += self.c012 * r * r * v;
            c += self.c012 * LongDouble::from_f64(2.0) * r * q * w;
            d += self.c012 * LongDouble::from_f64(2.0) * r * w * v;
            d += self.c012 * q * w * w;
            e += self.c012 * w * w * v;
            // c111
            b += self.c111 * p * q * r;
            c += self.c111 * p * q * w;
            c += self.c111 * p * v * r;
            c += self.c111 * u * q * r;
            d += self.c111 * p * v * w;
            d += self.c111 * u * q * w;
            d += self.c111 * u * v * r;
            e += self.c111 * u * v * w;
            // c200
            c += self.c200 * p * p;
            d += self.c200 * LongDouble::from_f64(2.0) * p * u;
            e += self.c200 * u * u;
            // c020
            c += self.c020 * q * q;
            d += self.c020 * LongDouble::from_f64(2.0) * q * v;
            e += self.c020 * v * v;
            // c002
            c += self.c002 * r * r;
            d += self.c002 * LongDouble::from_f64(2.0) * r * w;
            e += self.c002 * w * w;
            // c110
            c += self.c110 * p * q;
            d += self.c110 * p * v;
            d += self.c110 * u * q;
            e += self.c110 * u * v;
            // c011
            c += self.c011 * q * r;
            d += self.c011 * q * w;
            d += self.c011 * v * r;
            e += self.c011 * v * w;
            // c101
            c += self.c101 * p * r;
            d += self.c101 * p * w;
            d += self.c101 * u * r;
            e += self.c101 * u * w;
            // c100
            d += self.c100 * p;
            e += self.c100 * u;
            // c010
            d += self.c010 * q;
            e += self.c010 * v;
            // c001
            d += self.c001 * r;
            e += self.c001 * w;
            // c000
            e += self.c000;
            // done
            (a, b, c, d, e)
        };

        quartic_roots(a, b, c, d, e)
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
            LongDouble::from_f64(4.0) * self.c400 * x * x * x
                + LongDouble::from_f64(3.0) * self.c310 * x * x * y
                + LongDouble::from_f64(3.0) * self.c301 * x * x * z
                + LongDouble::from_f64(3.0) * self.c300 * x * x
                + LongDouble::from_f64(2.0) * self.c220 * x * y * y
                + LongDouble::from_f64(2.0) * self.c202 * x * z * z
                + LongDouble::from_f64(2.0) * self.c211 * x * y * z
                + LongDouble::from_f64(2.0) * self.c210 * x * y
                + LongDouble::from_f64(2.0) * self.c201 * x * z
                + LongDouble::from_f64(2.0) * self.c200 * x
                + self.c130 * y * y * y
                + self.c103 * z * z * z
                + self.c121 * y * y * z
                + self.c112 * y * z * z
                + self.c120 * y * y
                + self.c102 * z * z
                + self.c111 * y * z
                + self.c110 * y
                + self.c101 * z
                + self.c100,
            LongDouble::from_f64(4.0) * self.c040 * y * y * y
                + LongDouble::from_f64(3.0) * self.c130 * y * y * x
                + LongDouble::from_f64(3.0) * self.c031 * y * y * z
                + LongDouble::from_f64(3.0) * self.c030 * y * y
                + LongDouble::from_f64(2.0) * self.c220 * y * x * x
                + LongDouble::from_f64(2.0) * self.c022 * y * z * z
                + LongDouble::from_f64(2.0) * self.c121 * y * x * z
                + LongDouble::from_f64(2.0) * self.c120 * y * x
                + LongDouble::from_f64(2.0) * self.c021 * y * z
                + LongDouble::from_f64(2.0) * self.c020 * y
                + self.c310 * x * x * x
                + self.c013 * z * z * z
                + self.c211 * x * x * z
                + self.c112 * x * z * z
                + self.c210 * x * x
                + self.c012 * z * z
                + self.c111 * x * z
                + self.c110 * x
                + self.c011 * z
                + self.c010,
            LongDouble::from_f64(4.0) * self.c004 * z * z * z
                + LongDouble::from_f64(3.0) * self.c013 * z * z * y
                + LongDouble::from_f64(3.0) * self.c103 * z * z * x
                + LongDouble::from_f64(3.0) * self.c003 * z * z
                + LongDouble::from_f64(2.0) * self.c022 * z * y * y
                + LongDouble::from_f64(2.0) * self.c202 * z * x * x
                + LongDouble::from_f64(2.0) * self.c112 * z * y * x
                + LongDouble::from_f64(2.0) * self.c012 * z * y
                + LongDouble::from_f64(2.0) * self.c102 * z * x
                + LongDouble::from_f64(2.0) * self.c002 * z
                + self.c031 * y * y * y
                + self.c301 * x * x * x
                + self.c121 * y * y * x
                + self.c211 * y * x * x
                + self.c021 * y * y
                + self.c201 * x * x
                + self.c111 * y * x
                + self.c011 * y
                + self.c101 * x
                + self.c001,
        ))
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
