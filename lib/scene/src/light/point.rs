use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position},
    rt::Light,
};
use seui_engine_raytracing_csg_renderer_long_double::LongDouble;
use seui_engine_raytracing_csg_renderer_types::HDRColor;

use crate::{
    deserialize::{deserialize_hdr_color, deserialize_position},
    json_schema::{HDRColorSchema, PositionSchema},
};

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PointLight {
    #[serde(default, deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    position: Position,
    #[serde(default, deserialize_with = "deserialize_hdr_color")]
    #[schemars(with = "HDRColorSchema")]
    color: HDRColor,
}

impl Light for PointLight {
    fn test(&self, position: Position) -> Option<(HDRColor, Direction, LongDouble)> {
        // Compute the vector from the ray's origin to the light's position
        let to_light = self.position - position;
        let (direction, distance) = to_light.direction_and_length();
        if distance < LongDouble::from_f64(1e-3) {
            return Some((self.color, direction, distance));
        }

        // Compute attenuation using inverse square falloff
        let attenuation_factor = LongDouble::from_f64(1.0) / (distance * distance);
        let attenuated_color = self.color * attenuation_factor;

        Some((attenuated_color, direction, distance))
    }
}
