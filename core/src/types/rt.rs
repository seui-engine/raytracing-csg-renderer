use std::rc::Rc;

use seui_engine_raytracing_csg_renderer_types::{HDRColor, LDRColor};

use super::math::{Direction, Position};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Position,
    pub direction: Direction,
}

#[derive(Clone)]
pub struct Hit {
    pub is_front_face: bool,
    pub albedo: LDRColor,
    pub normal: Direction,
    pub brdf: Rc<dyn Fn(Direction, Direction) -> f32>,
    pub distance: f32,
}

pub trait RTObject {
    fn test(&self, ray: Ray) -> Vec<Hit>;
}

pub trait Light {
    fn test(&self, position: Position) -> Option<(HDRColor, Direction, f32)>;
}

pub trait Camera {
    fn ray(&self, x: f32, y: f32) -> Ray;
}

pub struct Scene {
    pub camera: Box<dyn Camera>,
    pub objects: Vec<Box<dyn RTObject>>,
    pub lights: Vec<Box<dyn Light>>,
    pub sky_color: Rc<dyn Fn(Direction) -> HDRColor>,
    pub ambient_light: HDRColor,
}

impl Scene {
    pub fn test(&self, ray: Ray) -> Option<Hit> {
        let mut result = None::<Hit>;
        for object in self.objects.iter() {
            result = match (result, object.test(ray).first()) {
                (None, current) => current.cloned(),
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
