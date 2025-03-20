use std::sync::Arc;

use seui_engine_raytracing_csg_renderer_types::{HDRColor, LDRColor};

use super::math::{Direction, Position};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Position,
    pub direction: Direction,
}

#[derive(Clone, Debug)]
pub struct Hit {
    pub is_front_face: bool,
    pub albedo: LDRColor,
    pub normal: Direction,
    pub distance: f64,
    pub roughness: f64,
    pub metallic: f64,
}

pub trait RTObject {
    fn test(&self, ray: Ray) -> Option<Hit>;
}

pub trait Light {
    fn test(&self, position: Position) -> Option<(HDRColor, Direction, f64)>;
}

pub trait Camera {
    fn ray(&self, x: f64, y: f64) -> Ray;
}

pub struct Scene {
    pub camera: Box<dyn Camera + Send + Sync>,
    pub objects: Vec<Box<dyn RTObject + Send + Sync>>,
    pub lights: Vec<Box<dyn Light + Send + Sync>>,
    pub sky_color: Arc<dyn Fn(Direction) -> HDRColor + Send + Sync>,
    pub ambient_light: HDRColor,
}

impl Scene {
    pub fn test(&self, ray: Ray) -> Option<Hit> {
        let mut result = None::<Hit>;
        for object in self.objects.iter() {
            result = match (result, object.test(ray)) {
                (None, current) => current,
                (previous, None) => previous,
                (Some(previous), Some(current)) => {
                    if previous.distance < current.distance {
                        Some(previous)
                    } else {
                        Some(current.clone())
                    }
                }
            }
        }
        result
    }
}
