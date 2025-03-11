use csg::{DeserializableDifference, DeserializableIntersection, DeserializableUnion};
use cube::Cube;
use plane::Plane;
use quadratic::Quadratic;
use quadric::Quadric;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::{Hit, Ray};
use sphere::Sphere;

pub mod csg;
pub mod cube;
pub mod plane;
pub mod quadratic;
pub mod quadric;
pub mod sphere;
pub mod util;

pub trait RTModel {
    fn test(&self, ray: Ray) -> Vec<Hit>;
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum DeserializableRTModel {
    Union(DeserializableUnion),
    Intersection(DeserializableIntersection),
    Difference(DeserializableDifference),
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Quadric(Quadric),
    Quadratic(Quadratic),
}

impl DeserializableRTModel {
    pub fn into_rt_model(self) -> Box<dyn RTModel + Send + Sync> {
        match self {
            DeserializableRTModel::Union(o) => o.into_rt_model(),
            DeserializableRTModel::Intersection(o) => o.into_rt_model(),
            DeserializableRTModel::Difference(o) => o.into_rt_model(),
            DeserializableRTModel::Sphere(o) => Box::new(o),
            DeserializableRTModel::Plane(o) => Box::new(o),
            DeserializableRTModel::Cube(o) => Box::new(o),
            DeserializableRTModel::Quadric(o) => Box::new(o),
            DeserializableRTModel::Quadratic(o) => Box::new(o),
        }
    }
}
