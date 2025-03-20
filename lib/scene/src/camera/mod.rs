use perspective::DeserializablePerspectiveCamera;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::Camera;
use seui_engine_raytracing_csg_renderer_long_double::LongDouble;

pub mod perspective;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum DeserializableCamera {
    Perspective(DeserializablePerspectiveCamera),
}

impl DeserializableCamera {
    pub fn into_camera(self, screen_aspect_ratio: LongDouble) -> Box<dyn Camera + Send + Sync> {
        match self {
            DeserializableCamera::Perspective(c) => c.into_camera(screen_aspect_ratio),
        }
    }
}
