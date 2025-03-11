use model::{
    csg::{DeserializableDifference, DeserializableIntersection, DeserializableUnion},
    cube::Cube,
    plane::Plane,
    quadratic::Quadratic,
    quadric::Quadric,
    sphere::Sphere,
    DeserializableRTModel, RTModel,
};
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::{Hit, RTObject, Ray};

pub mod model;

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
    Quadratic(Quadratic),
    Default(DeserializableDefaultRTObject),
}

impl DeserializableRTObject {
    pub fn into_rt_object(self) -> Box<dyn RTObject + Send + Sync> {
        match self {
            DeserializableRTObject::Union(o) => Box::new(ModelRTObject::new(o.into_rt_model())),
            DeserializableRTObject::Intersection(o) => {
                Box::new(ModelRTObject::new(o.into_rt_model()))
            }
            DeserializableRTObject::Difference(o) => {
                Box::new(ModelRTObject::new(o.into_rt_model()))
            }
            DeserializableRTObject::Sphere(o) => Box::new(ModelRTObject::new(Box::new(o))),
            DeserializableRTObject::Plane(o) => Box::new(ModelRTObject::new(Box::new(o))),
            DeserializableRTObject::Cube(o) => Box::new(ModelRTObject::new(Box::new(o))),
            DeserializableRTObject::Quadric(o) => Box::new(ModelRTObject::new(Box::new(o))),
            DeserializableRTObject::Quadratic(o) => Box::new(ModelRTObject::new(Box::new(o))),
            DeserializableRTObject::Default(o) => o.into_rt_object(),
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
        self.model.test(ray).first().cloned()
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeserializableDefaultRTObject {
    model: DeserializableRTModel,
}

impl DeserializableDefaultRTObject {
    pub fn into_rt_object(self) -> Box<dyn RTObject + Send + Sync> {
        Box::new(ModelRTObject::new(self.model.into_rt_model()))
    }
}
