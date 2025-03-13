use std::{collections::HashMap, sync::Arc};

use camera::DeserializableCamera;
use deserialize::deserialize_hdr_color;
use json_schema::HDRColorSchema;
use light::DeserializableLight;
use object::DeserializableRTObject;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::Scene;
use seui_engine_raytracing_csg_renderer_types::HDRColor;

pub mod camera;
pub mod deserialize;
pub mod json_schema;
pub mod light;
pub mod object;
pub mod texture;

pub trait Image {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get(&self, x: usize, y: usize) -> [f32; 3];
}

pub trait ImageLoader {
    fn load(&self, path: &str) -> Arc<dyn Image + Send + Sync>;
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeserializableScene {
    pub camera: DeserializableCamera,
    pub objects: Vec<DeserializableRTObject>,
    pub lights: Vec<DeserializableLight>,
    #[serde(deserialize_with = "deserialize_hdr_color")]
    #[schemars(with = "HDRColorSchema")]
    pub sky_color: HDRColor,
    #[serde(deserialize_with = "deserialize_hdr_color")]
    #[schemars(with = "HDRColorSchema")]
    pub ambient_light: HDRColor,
}

impl DeserializableScene {
    pub fn into_scene<T: ImageLoader>(self, screen_aspect_ratio: f32, image_loader: &T) -> Scene {
        let mut cache = ImageCache::new(image_loader);
        Scene {
            camera: self.camera.into_camera(screen_aspect_ratio),
            objects: self
                .objects
                .into_iter()
                .map(|o| o.into_rt_object(&mut cache))
                .collect(),
            lights: self
                .lights
                .into_iter()
                .map(DeserializableLight::into_light)
                .collect(),
            sky_color: Arc::new(move |_| self.sky_color),
            ambient_light: self.ambient_light,
        }
    }
}

pub struct ImageCache<'a, T: ImageLoader> {
    loader: &'a T,
    cache: HashMap<String, Arc<dyn Image + Send + Sync>>,
}

impl<'a, T: ImageLoader> ImageCache<'a, T> {
    pub fn new(loader: &'a T) -> ImageCache<'a, T> {
        ImageCache {
            loader,
            cache: HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &str) -> Arc<dyn Image + Send + Sync> {
        if let Some(image) = self.cache.get(path) {
            return image.clone();
        }

        let loaded_image: Arc<dyn Image + Send + Sync> = self.loader.load(path);
        self.cache.insert(path.to_string(), loaded_image.clone());

        loaded_image
    }
}
