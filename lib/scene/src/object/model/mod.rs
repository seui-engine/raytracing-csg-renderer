use csg::{DeserializableDifference, DeserializableIntersection, DeserializableUnion};
use cube::Cube;
use plane::Plane;
use quadratic::Quadratic;
use quadric::Quadric;
use quartic::Quartic;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{math::Direction, rt::Ray};
use seui_engine_raytracing_csg_renderer_types::LDRColor;
use sphere::DeserializableSphere;

use crate::{ImageCache, ImageLoader};

pub mod csg;
pub mod cube;
pub mod plane;
pub mod quadratic;
pub mod quadric;
pub mod quartic;
pub mod sphere;
pub mod util;

#[derive(Clone, Debug)]
pub struct Hit {
    pub is_front_face: bool,
    pub albedo: LDRColor,
    pub normal: Direction,
    pub distance: f64,
    pub roughness: f64,
    pub metallic: f64,
}

pub trait RTModel {
    fn test(&self, ray: Ray) -> Vec<Hit>;
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum DeserializableRTModel {
    Union(DeserializableUnion),
    Intersection(DeserializableIntersection),
    Difference(DeserializableDifference),
    Sphere(DeserializableSphere),
    Plane(Plane),
    Cube(Cube),
    Quadric(Quadric),
    Quadratic(Quadratic),
    Quartic(Quartic),
}

impl DeserializableRTModel {
    pub fn into_rt_model<T: ImageLoader>(
        self,
        image_cache: &mut ImageCache<T>,
    ) -> Box<dyn RTModel + Send + Sync> {
        match self {
            DeserializableRTModel::Union(o) => o.into_rt_model(image_cache),
            DeserializableRTModel::Intersection(o) => o.into_rt_model(image_cache),
            DeserializableRTModel::Difference(o) => o.into_rt_model(image_cache),
            DeserializableRTModel::Sphere(o) => o.into_rt_model(image_cache),
            DeserializableRTModel::Plane(o) => Box::new(o),
            DeserializableRTModel::Cube(o) => Box::new(o),
            DeserializableRTModel::Quadric(o) => Box::new(o),
            DeserializableRTModel::Quadratic(o) => Box::new(o),
            DeserializableRTModel::Quartic(o) => Box::new(o),
        }
    }
}
