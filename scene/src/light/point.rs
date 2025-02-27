use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position},
    rt::{Light, Ray},
};
use seui_engine_raytracing_csg_renderer_types::HDRColor;

use crate::deserialize::{deserialize_hdr_color, deserialize_position};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointLight {
    #[serde(deserialize_with = "deserialize_position")]
    position: Position,
    #[serde(deserialize_with = "deserialize_hdr_color")]
    color: HDRColor,
}

impl Light for PointLight {
    fn test(&self, position: Position) -> Option<(HDRColor, Direction, f32)> {
        // Compute the vector from the ray's origin to the light's position
        let to_light = self.position - position;
        let (direction, distance) = to_light.direction_and_length();
        if distance < 1e-3 {
            return Some((self.color, direction, distance));
        }

        // Compute attenuation using inverse square falloff
        let attenuation_factor = 1.0 / (distance * distance);
        let attenuated_color = self.color * attenuation_factor;

        Some((attenuated_color, direction, distance))
    }
}
