use seui_engine_raytracing_csg_renderer_long_double::LongDouble;
use seui_engine_raytracing_csg_renderer_types::{HDRColor, LDRColor};
use types::{
    math::Direction,
    rt::{Ray, Scene},
};

pub mod types;

pub fn sample(scene: &Scene, x: LongDouble, y: LongDouble) -> HDRColor {
    let ray = scene.camera.ray(x, y);
    if let Some(hit) = scene.test(ray) {
        let position =
            ray.origin + ray.direction * hit.distance + hit.normal * LongDouble::from_f64(1e-3);
        let mut result = scene.ambient_light * hit.albedo;
        for light in scene.lights.iter() {
            if let Some((color, direction, distance)) = light.test(position) {
                let shadow_ray = Ray {
                    origin: position,
                    direction,
                };

                let shadow_hit = scene.test(shadow_ray);

                let is_shadowed = if !distance.is_inf() {
                    shadow_hit
                        .map(|x| x.distance)
                        .unwrap_or(LongDouble::infinity())
                        < distance
                } else {
                    shadow_hit.is_some()
                };

                if !is_shadowed {
                    result = result
                        + brdf(
                            -ray.direction,
                            direction,
                            hit.normal,
                            hit.roughness,
                            hit.metallic,
                            hit.albedo,
                            color,
                        )
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
    roughness: LongDouble,
    metallic: LongDouble,
    albedo: LDRColor,
    light_color: HDRColor,
) -> HDRColor {
    fn fresnel_schlick(cos_theta: LongDouble, f0: LongDouble) -> LongDouble {
        let cos_theta = cos_theta.clamp(LongDouble::from_f64(0.0), LongDouble::from_f64(1.0));
        f0 + (LongDouble::from_f64(1.0) - f0)
            * (LongDouble::from_f64(1.0) - cos_theta).pow(LongDouble::from_f64(5.0))
    }

    fn ggx_ndf(n: Direction, h: Direction, roughness: LongDouble) -> LongDouble {
        let alpha = roughness * roughness;
        let alpha2 = alpha * alpha;
        let cos_n_h = n
            .dot(h)
            .clamp(LongDouble::from_f64(0.0), LongDouble::from_f64(1.0));
        let cos_n_h2 = cos_n_h * cos_n_h;
        let denom = cos_n_h2 * alpha2 + (LongDouble::from_f64(1.0) - cos_n_h2);
        alpha2 / (LongDouble::pi() * denom * denom)
    }

    fn geometric_attenuation(
        n: Direction,
        v: Direction,
        l: Direction,
        roughness: LongDouble,
    ) -> LongDouble {
        let k = (roughness + LongDouble::from_f64(1.0)) * (roughness + LongDouble::from_f64(1.0))
            / LongDouble::from_f64(8.0);
        let cos_n_v = n.dot(v).max(LongDouble::from_f64(1e-5));
        let g_v = cos_n_v / (cos_n_v * (LongDouble::from_f64(1.0) - k) + k);
        let cos_n_l = n.dot(l).max(LongDouble::from_f64(1e-5));
        let g_l = cos_n_l / (cos_n_l * (LongDouble::from_f64(1.0) - k) + k);
        g_v * g_l
    }

    fn cook_torrance_specular(
        v: Direction,
        l: Direction,
        n: Direction,
        roughness: LongDouble,
        f0: LongDouble,
    ) -> LongDouble {
        let h = Direction::new(*v + *l);
        let d = ggx_ndf(n, h, roughness);
        let f = fresnel_schlick(
            h.dot(v)
                .clamp(LongDouble::from_f64(0.0), LongDouble::from_f64(1.0)),
            f0,
        );
        let g = geometric_attenuation(n, v, l, roughness);
        (d * f * g)
            / (LongDouble::from_f64(4.0)
                * n.dot(v).max(LongDouble::from_f64(1e-5))
                * n.dot(l).max(LongDouble::from_f64(1e-5)))
    }

    let n_dot_l = surface_normal
        .dot(surface_to_light)
        .max(LongDouble::from_f64(0.0));

    let f0 = LDRColor {
        r: albedo.r * metallic
            + (LongDouble::from_f64(1.0) - metallic) * LongDouble::from_f64(0.04),
        g: albedo.g * metallic
            + (LongDouble::from_f64(1.0) - metallic) * LongDouble::from_f64(0.04),
        b: albedo.b * metallic
            + (LongDouble::from_f64(1.0) - metallic) * LongDouble::from_f64(0.04),
    };

    let specular = LDRColor {
        r: cook_torrance_specular(
            surface_to_view,
            surface_to_light,
            surface_normal,
            roughness,
            f0.r,
        ),
        g: cook_torrance_specular(
            surface_to_view,
            surface_to_light,
            surface_normal,
            roughness,
            f0.g,
        ),
        b: cook_torrance_specular(
            surface_to_view,
            surface_to_light,
            surface_normal,
            roughness,
            f0.b,
        ),
    };

    let fresnel = LDRColor {
        r: fresnel_schlick(n_dot_l, f0.r),
        g: fresnel_schlick(n_dot_l, f0.g),
        b: fresnel_schlick(n_dot_l, f0.b),
    };

    let diffuse = LDRColor {
        r: (LongDouble::from_f64(1.0) - fresnel.r)
            * (LongDouble::from_f64(1.0) - metallic)
            * (albedo.r / LongDouble::pi())
            * n_dot_l,
        g: (LongDouble::from_f64(1.0) - fresnel.g)
            * (LongDouble::from_f64(1.0) - metallic)
            * (albedo.g / LongDouble::pi())
            * n_dot_l,
        b: (LongDouble::from_f64(1.0) - fresnel.b)
            * (LongDouble::from_f64(1.0) - metallic)
            * (albedo.b / LongDouble::pi())
            * n_dot_l,
    };

    HDRColor {
        r: (diffuse.r + specular.r) * light_color.r,
        g: (diffuse.g + specular.g) * light_color.g,
        b: (diffuse.b + specular.b) * light_color.b,
    }
}
