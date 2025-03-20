use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position, Vec3},
    rt::Light,
};
use seui_engine_raytracing_csg_renderer_long_double::LongDouble;
use seui_engine_raytracing_csg_renderer_types::HDRColor;

use crate::{
    deserialize::{deserialize_direction, deserialize_hdr_color},
    json_schema::{DirectionSchema, HDRColorSchema},
};

fn down() -> Direction {
    Direction::new(Vec3::new(
        LongDouble::from_f32(0.0),
        LongDouble::from_f32(0.0),
        LongDouble::from_f32(-1.0),
    ))
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DirectionalLight {
    #[serde(default, deserialize_with = "deserialize_hdr_color")]
    #[schemars(with = "HDRColorSchema")]
    color: HDRColor,
    #[serde(default = "down", deserialize_with = "deserialize_direction")]
    #[schemars(with = "DirectionSchema")]
    direction: Direction,
}

impl Light for DirectionalLight {
    fn test(&self, _position: Position) -> Option<(HDRColor, Direction, LongDouble)> {
        Some((self.color, -self.direction, LongDouble::infinity()))
    }
}
