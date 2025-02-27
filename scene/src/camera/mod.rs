use perspective::DeserializablePerspectiveCamera;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::Camera;

pub mod perspective;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DeserializableCamera {
    Perspective(DeserializablePerspectiveCamera),
}

impl DeserializableCamera {
    pub fn into_camera(self, screen_aspect_ratio: f32) -> Box<dyn Camera> {
        match self {
            DeserializableCamera::Perspective(c) => c.into_camera(screen_aspect_ratio),
        }
    }
}
