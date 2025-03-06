use core::f32;

use glam::Vec3;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position},
    rt::Light,
};
use seui_engine_raytracing_csg_renderer_types::HDRColor;

use crate::{
    deserialize::{deserialize_direction, deserialize_hdr_color, deserialize_position},
    json_schema::{DirectionSchema, HDRColorSchema, PositionSchema},
};

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DirectionalLight {
    #[serde(default, deserialize_with = "deserialize_hdr_color")]
    #[schemars(with = "HDRColorSchema")]
    color: HDRColor,
    #[serde(default, deserialize_with = "deserialize_direction")]
    #[schemars(with = "DirectionSchema")]
    direction: Direction,
}

impl Light for DirectionalLight {
    fn test(&self, _position: Position) -> Option<(HDRColor, Direction, f32)> {
        Some((self.color, self.direction, f32::INFINITY))
    }
}
