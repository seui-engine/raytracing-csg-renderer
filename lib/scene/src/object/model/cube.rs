use crate::{
    deserialize::deserialize_ldr_float,
    json_schema::{LDRColorSchema, PositionSchema, Scale},
};

use super::{
    super::super::deserialize::{deserialize_ldr_color, deserialize_position, deserialize_scale},
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

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Cube {
    #[serde(default, deserialize_with = "deserialize_position")]
    #[schemars(default, with = "PositionSchema")]
    position: Position,
    #[serde(default, deserialize_with = "deserialize_ldr_color")]
    #[schemars(default, with = "LDRColorSchema")]
    albedo: LDRColor,
    #[serde(default, deserialize_with = "deserialize_scale")]
    scale: Scale,
    #[serde(default, deserialize_with = "deserialize_ldr_float")]
    #[schemars(range(min = 0, max = 1))]
    roughness: f64,
    #[serde(default, deserialize_with = "deserialize_ldr_float")]
    #[schemars(range(min = 0, max = 1))]
    metallic: f64,
}

impl RTModel for Cube {
    fn test(&self, ray: Ray) -> Vec<Hit> {
        let mut result = Vec::new();

        let min = Position::new(Vec3::new(
            self.position.x - self.scale.x,
            self.position.y - self.scale.y,
            self.position.z - self.scale.z,
        ));
        let max = Position::new(Vec3::new(
            self.position.x + self.scale.x,
            self.position.y + self.scale.y,
            self.position.z + self.scale.z,
        ));

        let mut t_min = -LongDouble::infinity();
        let mut t_max = LongDouble::infinity();
        let mut normal_min = Vec3::default();
        let mut normal_max = Vec3::default();

        for (i, (o, d, min, max)) in [
            (ray.origin.x, ray.direction.x, min.x, max.x),
            (ray.origin.y, ray.direction.y, min.y, max.y),
            (ray.origin.z, ray.direction.z, min.z, max.z),
        ]
        .iter()
        .enumerate()
        {
            if *d != LongDouble::from_f64(0.0) {
                let t1 = (*min - *o) / *d;
                let t2 = (*max - *o) / *d;
                let (t1, t2, normal1, normal2) = if t1 < t2 {
                    (
                        t1,
                        t2,
                        -Vec3::new(
                            LongDouble::from_f64(1.0),
                            LongDouble::from_f64(0.0),
                            LongDouble::from_f64(0.0),
                        ) * LongDouble::from_f64((i == 0) as i32 as f64)
                            - Vec3::new(
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(1.0),
                                LongDouble::from_f64(0.0),
                            ) * LongDouble::from_f64((i == 1) as i32 as f64)
                            - Vec3::new(
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(1.0),
                            ) * LongDouble::from_f64((i == 2) as i32 as f64),
                        Vec3::new(
                            LongDouble::from_f64(1.0),
                            LongDouble::from_f64(0.0),
                            LongDouble::from_f64(0.0),
                        ) * LongDouble::from_f64((i == 0) as i32 as f64)
                            + Vec3::new(
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(1.0),
                                LongDouble::from_f64(0.0),
                            ) * LongDouble::from_f64((i == 1) as i32 as f64)
                            + Vec3::new(
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(1.0),
                            ) * LongDouble::from_f64((i == 2) as i32 as f64),
                    )
                } else {
                    (
                        t2,
                        t1,
                        Vec3::new(
                            LongDouble::from_f64(1.0),
                            LongDouble::from_f64(0.0),
                            LongDouble::from_f64(0.0),
                        ) * LongDouble::from_f64((i == 0) as i32 as f64)
                            + Vec3::new(
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(1.0),
                                LongDouble::from_f64(0.0),
                            ) * LongDouble::from_f64((i == 1) as i32 as f64)
                            + Vec3::new(
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(1.0),
                            ) * LongDouble::from_f64((i == 2) as i32 as f64),
                        -Vec3::new(
                            LongDouble::from_f64(1.0),
                            LongDouble::from_f64(0.0),
                            LongDouble::from_f64(0.0),
                        ) * LongDouble::from_f64((i == 0) as i32 as f64)
                            - Vec3::new(
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(1.0),
                                LongDouble::from_f64(0.0),
                            ) * LongDouble::from_f64((i == 1) as i32 as f64)
                            - Vec3::new(
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(0.0),
                                LongDouble::from_f64(1.0),
                            ) * LongDouble::from_f64((i == 2) as i32 as f64),
                    )
                };

                if t1 > t_min {
                    t_min = t1;
                    normal_min = normal1;
                }
                if t2 < t_max {
                    t_max = t2;
                    normal_max = normal2;
                }
                if t_min > t_max {
                    return result;
                }
            } else if *o < *min || *o > *max {
                return result;
            }
        }

        if t_min < LongDouble::from_f64(0.0) && t_max < LongDouble::from_f64(0.0) {
            return result;
        }

        if t_min < LongDouble::from_f64(0.0) {
            t_min = LongDouble::from_f64(0.0);
        }

        if t_min <= t_max {
            if t_min >= LongDouble::from_f64(0.0) {
                result.push(Hit {
                    normal: Direction::new(normal_min),
                    distance: t_min,
                    is_front_face: true,
                    albedo: self.albedo,
                    roughness: LongDouble::from_f64(self.roughness),
                    metallic: LongDouble::from_f64(self.metallic),
                });
            }
            if t_max >= LongDouble::from_f64(0.0) {
                result.push(Hit {
                    normal: Direction::new(normal_max),
                    distance: t_max,
                    is_front_face: false,
                    albedo: self.albedo,
                    roughness: LongDouble::from_f64(self.roughness),
                    metallic: LongDouble::from_f64(self.metallic),
                });
            }
        }

        result
    }
}
