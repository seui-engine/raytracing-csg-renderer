use std::rc::Rc;

use camera::DeserializableCamera;
use deserialize::deserialize_hdr_color;
use light::DeserializableLight;
use object::DeserializableRTObject;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::Scene;
use seui_engine_raytracing_csg_renderer_types::HDRColor;

pub mod camera;
pub mod deserialize;
pub mod light;
pub mod object;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeserializableScene {
    pub camera: DeserializableCamera,
    pub objects: Vec<DeserializableRTObject>,
    pub lights: Vec<DeserializableLight>,
    #[serde(deserialize_with = "deserialize_hdr_color")]
    pub sky_color: HDRColor,
    #[serde(deserialize_with = "deserialize_hdr_color")]
    pub ambient_light: HDRColor,
}

impl DeserializableScene {
    pub fn into_scene(self, screen_aspect_ratio: f32) -> Scene {
        Scene {
            camera: self.camera.into_camera(screen_aspect_ratio),
            objects: self
                .objects
                .into_iter()
                .map(DeserializableRTObject::into_rt_object)
                .collect(),
            lights: self
                .lights
                .into_iter()
                .map(DeserializableLight::into_light)
                .collect(),
            sky_color: Rc::new(move |_| self.sky_color),
            ambient_light: self.ambient_light,
        }
    }
}
