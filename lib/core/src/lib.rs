use seui_engine_raytracing_csg_renderer_types::HDRColor;
use types::{
    math::Direction,
    rt::{Ray, Scene},
};

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

                let is_shadowed = if distance.is_finite() {
                    shadow_hit.map(|x| x.distance).unwrap_or(f32::INFINITY) < distance
                } else {
                    shadow_hit.is_some()
                };

                if !is_shadowed {
                    result = result
                        + hit.albedo
                            * color
                            * brdf(-ray.direction, direction, hit.normal, 0.5, 0.5).max(0.0)
                }
            }
        }
        result
    } else {
        (scene.sky_color)(ray.direction)
    }
}

fn brdf(
    surface_to_view: Direction,
    surface_to_light: Direction,
    surface_normal: Direction,
    roughness: f32,
    f0: f32,
) -> f32 {
    fn fresnel_schlick(cos_theta: f32, f0: f32) -> f32 {
        f0 + (1f32 - f0) * (1f32 - cos_theta).powf(5f32)
    }

    fn ggx_ndf(n: Direction, h: Direction, roughness: f32) -> f32 {
        let alpha = roughness * roughness;
        let alpha2 = alpha * alpha;
        let cos_n_h = n.dot(h);
        let cos_n_h2 = cos_n_h * cos_n_h;
        let denom = cos_n_h2 * alpha2 + (1f32 - cos_n_h2);
        alpha2 / (std::f32::consts::PI * denom * denom)
    }

    fn geometric_attenuation(n: Direction, v: Direction, l: Direction, roughness: f32) -> f32 {
        let k = (roughness + 1f32) * (roughness + 1f32) / 8f32;
        let cos_n_v = n.dot(v);
        let g_v = cos_n_v / (cos_n_v * (1f32 - k) + k);
        let cos_n_l = n.dot(l);
        let g_l = cos_n_l / (cos_n_l * (1f32 - k) + k);
        g_v * g_l
    }

    fn cook_torrance_specular(
        v: Direction,
        l: Direction,
        n: Direction,
        roughness: f32,
        f0: f32,
    ) -> f32 {
        let h = Direction::new(*v + *l);
        let d = ggx_ndf(n, h, roughness);
        let f = fresnel_schlick(h.dot(v), f0);
        let g = geometric_attenuation(n, v, l, roughness);
        let specular = (d * f * g) / (4f32 * n.dot(v) * n.dot(v));
        specular
    }

    cook_torrance_specular(
        surface_to_view,
        surface_to_light,
        surface_normal,
        roughness,
        f0,
    )
}
