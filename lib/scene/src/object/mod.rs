use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::RTObject;

use cube::Cube;
use plane::Plane;
use sphere::Sphere;

pub mod cube;
pub mod plane;
pub mod sphere;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DeserializableRTObject {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
}

impl DeserializableRTObject {
    pub fn into_rt_object(self) -> Box<dyn RTObject> {
        match self {
            DeserializableRTObject::Sphere(o) => Box::new(o),
            DeserializableRTObject::Plane(o) => Box::new(o),
            DeserializableRTObject::Cube(o) => Box::new(o),
        }
    }
}
