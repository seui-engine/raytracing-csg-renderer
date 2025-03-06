use directional::DirectionalLight;
use point::PointLight;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::Light;

pub mod directional;
pub mod point;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DeserializableLight {
    Point(PointLight),
    Directional(DirectionalLight),
}

impl DeserializableLight {
    pub fn into_light(self) -> Box<dyn Light> {
        match self {
            DeserializableLight::Point(c) => Box::new(c),
            DeserializableLight::Directional(c) => Box::new(c),
        }
    }
}
