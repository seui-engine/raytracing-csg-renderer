use std::sync::Arc;

use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_types::LDRColor;

use crate::{Image, ImageCache, ImageLoader};

use super::Texture;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeserializablePlainTexture {
    path: String,
    #[serde(default)]
    smooth: bool,
}

impl DeserializablePlainTexture {
    pub fn into_texture<T: ImageLoader>(
        self,
        image_cache: &mut ImageCache<T>,
    ) -> Arc<dyn Texture + Send + Sync> {
        if self.smooth {
            Arc::new(PlainLinearTexture {
                image: image_cache.load(&self.path),
            })
        } else {
            Arc::new(PlainNearestTexture {
                image: image_cache.load(&self.path),
            })
        }
    }
}

struct PlainNearestTexture {
    image: Arc<dyn Image + Send + Sync>,
}

impl Texture for PlainNearestTexture {
    fn get(&self, u: f32, v: f32) -> LDRColor {
        let width = self.image.width();
        let height = self.image.height();

        let x = (u * width as f32).round() as usize % width;
        let y = (v * height as f32).round() as usize % height;

        let (r, g, b) = self.image.get(y, x).into();

        LDRColor::new(r, g, b)
    }
}

struct PlainLinearTexture {
    image: Arc<dyn Image + Send + Sync>,
}

impl Texture for PlainLinearTexture {
    fn get(&self, u: f32, v: f32) -> LDRColor {
        let width = self.image.width() as f32;
        let height = self.image.height() as f32;

        let x = u * (width - 1.0);
        let y = v * (height - 1.0);

        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;

        let dx = x - x.floor();
        let dy = y - y.floor();

        let x1 = (x0 + 1).min(self.image.width() - 1);
        let y1 = (y0 + 1).min(self.image.height() - 1);

        let c00 = self.image.get(y0, x0);
        let c10 = self.image.get(y0, x1);
        let c01 = self.image.get(y1, x0);
        let c11 = self.image.get(y1, x1);

        let r = lerp(lerp(c00[0], c10[0], dx), lerp(c01[0], c11[0], dx), dy);
        let g = lerp(lerp(c00[1], c10[1], dx), lerp(c01[1], c11[1], dx), dy);
        let b = lerp(lerp(c00[2], c10[2], dx), lerp(c01[2], c11[2], dx), dy);

        LDRColor::new(r, g, b)
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}
