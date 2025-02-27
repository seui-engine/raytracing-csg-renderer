use plane::Plane;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::RTObject;
use sphere::Sphere;

pub mod plane;
pub mod sphere;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DeserializableRTObject {
    Sphere(Sphere),
    Plane(Plane),
}

impl DeserializableRTObject {
    pub fn into_rt_object(self) -> Box<dyn RTObject> {
        match self {
            DeserializableRTObject::Sphere(o) => Box::new(o),
            DeserializableRTObject::Plane(o) => Box::new(o),
        }
    }
}
