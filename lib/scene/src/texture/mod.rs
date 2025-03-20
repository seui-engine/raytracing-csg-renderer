use std::sync::Arc;

use plain::DeserializablePlainTexture;
use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_types::LDRColor;

use crate::{ImageCache, ImageLoader};

pub mod plain;

pub trait Texture {
    fn get(&self, u: f64, v: f64) -> LDRColor;
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum DeserializableTexture {
    Plain(DeserializablePlainTexture),
}

impl DeserializableTexture {
    pub fn into_texture<T: ImageLoader>(
        self,
        image_cache: &mut ImageCache<T>,
    ) -> Arc<dyn Texture + Send + Sync> {
        match self {
            DeserializableTexture::Plain(t) => t.into_texture(image_cache),
        }
    }
}
