use seui_engine_raytracing_csg_renderer_types::HDRColor;
use types::rt::{Ray, Scene};

pub mod types;

pub fn sample(scene: &Scene, x: f32, y: f32) -> HDRColor {
    let ray = scene.camera.ray(x, y);
    if let Some(hit) = scene.test(ray) {
        let position = ray.origin + ray.direction * hit.distance + hit.normal * 1e-3;
        let mut result = scene.ambient_light * hit.albedo;
        for light in scene.lights.iter() {
            if let Some((color, direction, distance)) = light.test(position) {
                let shadow_ray = Ray {
                    origin: position,
                    direction,
                };
                let shadow_hit = scene.test(shadow_ray);
                if shadow_hit.map(|x| x.distance).unwrap_or(f32::INFINITY) > distance {
                    result =
                        result + hit.albedo * color * (hit.brdf)(hit.normal, direction).max(0.0)
                }
            }
        }
        result
    } else {
        (scene.sky_color)(ray.direction)
    }
}
