use seui_engine_raytracing_csg_renderer_types::{HDRColor, LDRColor};
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
                        + color * brdf(-ray.direction, direction, hit.normal, 0.5, 0.5, hit.albedo)
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
    metallic: f32,    // Metallic factor (0 = non-metal, 1 = full metal)
    albedo: LDRColor, // Now using RGB instead of a single float
) -> HDRColor {
    fn fresnel_schlick(cos_theta: f32, f0: f32) -> f32 {
        f0 + (1.0 - f0) * (1.0 - cos_theta).powf(5.0)
    }

    fn ggx_ndf(n: Direction, h: Direction, roughness: f32) -> f32 {
        let alpha = roughness * roughness;
        let alpha2 = alpha * alpha;
        let cos_n_h = n.dot(h);
        let cos_n_h2 = cos_n_h * cos_n_h;
        let denom = cos_n_h2 * alpha2 + (1.0 - cos_n_h2);
        alpha2 / (std::f32::consts::PI * denom * denom)
    }

    fn geometric_attenuation(n: Direction, v: Direction, l: Direction, roughness: f32) -> f32 {
        let k = (roughness + 1.0) * (roughness + 1.0) / 8.0;
        let cos_n_v = n.dot(v);
        let g_v = cos_n_v / (cos_n_v * (1.0 - k) + k);
        let cos_n_l = n.dot(l);
        let g_l = cos_n_l / (cos_n_l * (1.0 - k) + k);
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
        (d * f * g) / (4.0 * n.dot(v) * n.dot(l))
    }

    let n_dot_l = surface_normal.dot(surface_to_light).max(0.0);

    let f0_r = albedo.r * metallic + (1.0 - metallic) * 0.04;
    let f0_g = albedo.g * metallic + (1.0 - metallic) * 0.04;
    let f0_b = albedo.b * metallic + (1.0 - metallic) * 0.04;

    let specular_r = cook_torrance_specular(
        surface_to_view,
        surface_to_light,
        surface_normal,
        roughness,
        f0_r,
    );
    let specular_g = cook_torrance_specular(
        surface_to_view,
        surface_to_light,
        surface_normal,
        roughness,
        f0_g,
    );
    let specular_b = cook_torrance_specular(
        surface_to_view,
        surface_to_light,
        surface_normal,
        roughness,
        f0_b,
    );

    let fresnel_r = fresnel_schlick(n_dot_l, f0_r);
    let fresnel_g = fresnel_schlick(n_dot_l, f0_g);
    let fresnel_b = fresnel_schlick(n_dot_l, f0_b);

    let diffuse_r =
        (1.0 - fresnel_r) * (1.0 - metallic) * (albedo.r / std::f32::consts::PI) * n_dot_l;
    let diffuse_g =
        (1.0 - fresnel_g) * (1.0 - metallic) * (albedo.g / std::f32::consts::PI) * n_dot_l;
    let diffuse_b =
        (1.0 - fresnel_b) * (1.0 - metallic) * (albedo.b / std::f32::consts::PI) * n_dot_l;

    HDRColor {
        r: (diffuse_r + specular_r).max(0.0),
        g: (diffuse_g + specular_g).max(0.0),
        b: (diffuse_b + specular_b).max(0.0),
    }
}
