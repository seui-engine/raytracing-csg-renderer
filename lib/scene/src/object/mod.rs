use csg::{DeserializableDifference, DeserializableIntersection, DeserializableUnion};
use cube::Cube;
use plane::Plane;
use quadric::Quadric;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::RTObject;
use sphere::Sphere;

pub mod csg;
pub mod cube;
pub mod plane;
pub mod quadric;
pub mod sphere;
pub mod util;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum DeserializableRTObject {
    Union(DeserializableUnion),
    Intersection(DeserializableIntersection),
    Difference(DeserializableDifference),
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Quadric(Quadric),
}

impl DeserializableRTObject {
    pub fn into_rt_object(self) -> Box<dyn RTObject + Send + Sync> {
        match self {
            DeserializableRTObject::Union(o) => o.into_rt_object(),
            DeserializableRTObject::Intersection(o) => o.into_rt_object(),
            DeserializableRTObject::Difference(o) => o.into_rt_object(),
            DeserializableRTObject::Sphere(o) => Box::new(o),
            DeserializableRTObject::Plane(o) => Box::new(o),
            DeserializableRTObject::Cube(o) => Box::new(o),
            DeserializableRTObject::Quadric(o) => Box::new(o),
        }
    }
}
