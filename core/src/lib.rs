use seui_engine_raytracing_csg_renderer_types::HDRPixel;
use types::rt::Ray;

pub mod types;

pub fn sample(ray: Ray) -> HDRPixel {
    HDRPixel {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    }
}
