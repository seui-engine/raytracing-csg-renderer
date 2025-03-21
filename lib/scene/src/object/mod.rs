use model::{
    csg::{DeserializableDifference, DeserializableIntersection, DeserializableUnion},
    cube::Cube,
    plane::Plane,
    quadratic::DeserializableQuadratic,
    quadric::DeserializableQuadric,
    quartic::DeserializableQuartic,
    sphere::DeserializableSphere,
    DeserializableRTModel, RTModel,
};
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::{Hit, RTObject, Ray};

use crate::{ImageCache, ImageLoader};

pub mod model;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum DeserializableRTObject {
    Union(DeserializableUnion),
    Intersection(DeserializableIntersection),
    Difference(DeserializableDifference),
    Sphere(DeserializableSphere),
    Plane(Plane),
    Cube(Cube),
    Quadric(DeserializableQuadric),
    Quadratic(DeserializableQuadratic),
    Quartic(DeserializableQuartic),
    Default(DeserializableDefaultRTObject),
}

impl DeserializableRTObject {
    pub fn into_rt_object<T: ImageLoader>(
        self,
        image_cache: &mut ImageCache<T>,
    ) -> Box<dyn RTObject + Send + Sync> {
        match self {
            DeserializableRTObject::Union(o) => {
                Box::new(ModelRTObject::new(o.into_rt_model(image_cache)))
            }
            DeserializableRTObject::Intersection(o) => {
                Box::new(ModelRTObject::new(o.into_rt_model(image_cache)))
            }
            DeserializableRTObject::Difference(o) => {
                Box::new(ModelRTObject::new(o.into_rt_model(image_cache)))
            }
            DeserializableRTObject::Sphere(o) => {
                Box::new(ModelRTObject::new(o.into_rt_model(image_cache)))
            }
            DeserializableRTObject::Plane(o) => Box::new(ModelRTObject::new(Box::new(o))),
            DeserializableRTObject::Cube(o) => Box::new(ModelRTObject::new(Box::new(o))),
            DeserializableRTObject::Quadric(o) => Box::new(ModelRTObject::new(o.into_rt_model())),
            DeserializableRTObject::Quadratic(o) => Box::new(ModelRTObject::new(o.into_rt_model())),
            DeserializableRTObject::Quartic(o) => Box::new(ModelRTObject::new(o.into_rt_model())),
            DeserializableRTObject::Default(o) => o.into_rt_object(image_cache),
        }
    }
}

struct ModelRTObject {
    model: Box<dyn RTModel + Send + Sync>,
}

impl ModelRTObject {
    pub fn new(model: Box<dyn RTModel + Send + Sync>) -> ModelRTObject {
        ModelRTObject { model }
    }
}

impl RTObject for ModelRTObject {
    fn test(&self, ray: Ray) -> Option<Hit> {
        self.model.test(ray).first().map(|hit| Hit {
            distance: hit.distance,
            albedo: hit.albedo,
            normal: hit.normal,
            is_front_face: hit.is_front_face,
            roughness: hit.roughness,
            metallic: hit.metallic,
        })
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeserializableDefaultRTObject {
    model: DeserializableRTModel,
}

impl DeserializableDefaultRTObject {
    pub fn into_rt_object<T: ImageLoader>(
        self,
        image_cache: &mut ImageCache<T>,
    ) -> Box<dyn RTObject + Send + Sync> {
        Box::new(ModelRTObject::new(self.model.into_rt_model(image_cache)))
    }
}
