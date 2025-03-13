use csg::{DeserializableDifference, DeserializableIntersection, DeserializableUnion};
use cube::Cube;
use plane::Plane;
use quadratic::Quadratic;
use quadric::Quadric;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::RTObject;
use sphere::DeserializableSphere;

use crate::{ImageCache, ImageLoader};

pub mod csg;
pub mod cube;
pub mod plane;
pub mod quadratic;
pub mod quadric;
pub mod sphere;
pub mod util;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum DeserializableRTObject {
    Union(DeserializableUnion),
    Intersection(DeserializableIntersection),
    Difference(DeserializableDifference),
    Sphere(DeserializableSphere),
    Plane(Plane),
    Cube(Cube),
    Quadric(Quadric),
    Quadratic(Quadratic),
}

impl DeserializableRTObject {
    pub fn into_rt_object<T: ImageLoader>(
        self,
        image_cache: &mut ImageCache<T>,
    ) -> Box<dyn RTObject + Send + Sync> {
        match self {
            DeserializableRTObject::Union(o) => o.into_rt_object(image_cache),
            DeserializableRTObject::Intersection(o) => o.into_rt_object(image_cache),
            DeserializableRTObject::Difference(o) => o.into_rt_object(image_cache),
            DeserializableRTObject::Sphere(o) => o.into_rt_object(image_cache),
            DeserializableRTObject::Plane(o) => Box::new(o),
            DeserializableRTObject::Cube(o) => Box::new(o),
            DeserializableRTObject::Quadric(o) => Box::new(o),
            DeserializableRTObject::Quadratic(o) => Box::new(o),
        }
    }
}
