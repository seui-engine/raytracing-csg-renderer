use seui_engine_raytracing_csg_renderer_core::types::math::Direction;

pub fn enhance_normal(
    ray_direction: Direction,
    face_normal: Direction,
    is_front_face: bool,
) -> Direction {
    if (ray_direction.dot(face_normal) < 0.0) == is_front_face {
        face_normal
    } else {
        -face_normal
    }
}

pub fn one() -> f64 {
    1.0
}

pub fn zero() -> f64 {
    0.0
}
