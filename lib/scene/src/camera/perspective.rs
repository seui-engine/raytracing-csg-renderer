use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::{
    math::{Direction, Position, Vec3},
    rt::{Camera, Ray},
};
use seui_engine_raytracing_csg_renderer_long_double::LongDouble;

use crate::{
    deserialize::{deserialize_direction, deserialize_position},
    json_schema::{DirectionSchema, PositionSchema},
};

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AspectRatio {
    aspect_ratio: f64,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
pub enum FovMode {
    X,
    Y,
    Cover(AspectRatio),
    Contain(AspectRatio),
}

fn forward() -> Direction {
    Direction::new(Vec3::new(
        LongDouble::from_f64(0.0),
        LongDouble::from_f64(1.0),
        LongDouble::from_f64(0.0),
    ))
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeserializablePerspectiveCamera {
    fov: f64,
    fov_mode: FovMode,
    #[serde(default, deserialize_with = "deserialize_position")]
    #[schemars(with = "PositionSchema")]
    position: Position,
    #[serde(default = "forward", deserialize_with = "deserialize_direction")]
    #[schemars(with = "DirectionSchema")]
    direction: Direction,
}

impl DeserializablePerspectiveCamera {
    pub fn into_camera(self, screen_aspect_ratio: LongDouble) -> Box<dyn Camera + Send + Sync> {
        let (tan_half_fov_x, tan_half_fov_y) = match self.fov_mode {
            FovMode::X => {
                let tan_half_fov_x = LongDouble::from_f64((self.fov.to_radians() / 2.0).tan());
                let tan_half_fov_y = tan_half_fov_x / screen_aspect_ratio;
                (tan_half_fov_x, tan_half_fov_y)
            }
            FovMode::Y => {
                let tan_half_fov_y = LongDouble::from_f64((self.fov.to_radians() / 2.0).tan());
                let tan_half_fov_x = tan_half_fov_y * screen_aspect_ratio;
                (tan_half_fov_x, tan_half_fov_y)
            }
            FovMode::Cover(aspect_ratio) => {
                let aspect_ratio = LongDouble::from_f64(aspect_ratio.aspect_ratio);
                let tan_half_fov_x = LongDouble::from_f64((self.fov.to_radians() / 2.0).tan());
                let tan_half_fov_y = tan_half_fov_x / aspect_ratio;
                let scale = (screen_aspect_ratio / aspect_ratio).max(LongDouble::from_f64(1.0));
                (tan_half_fov_x * scale, tan_half_fov_y * scale)
            }
            FovMode::Contain(aspect_ratio) => {
                let aspect_ratio = LongDouble::from_f64(aspect_ratio.aspect_ratio);
                let tan_half_fov_x = LongDouble::from_f64((self.fov.to_radians() / 2.0).tan());
                let tan_half_fov_y = tan_half_fov_x / aspect_ratio;
                let scale = (screen_aspect_ratio / aspect_ratio).min(LongDouble::from_f64(1.0));
                (tan_half_fov_x * scale, tan_half_fov_y * scale)
            }
        };

        let right = self
            .direction
            .cross(Vec3::new(
                LongDouble::from_f64(0.0),
                LongDouble::from_f64(0.0),
                LongDouble::from_f64(1.0),
            ))
            .normalize();
        let up = right.cross(*self.direction).normalize();
        Box::new(PerspectiveCamera {
            tan_half_fov_x,
            tan_half_fov_y,
            position: self.position,
            direction: self.direction,
            right,
            up,
        })
    }
}

struct PerspectiveCamera {
    tan_half_fov_x: LongDouble,
    tan_half_fov_y: LongDouble,
    position: Position,
    direction: Direction,
    right: Vec3,
    up: Vec3,
}

impl Camera for PerspectiveCamera {
    fn ray(&self, x: LongDouble, y: LongDouble) -> Ray {
        let dir_x =
            (LongDouble::from_f64(2.0) * x - LongDouble::from_f64(1.0)) * self.tan_half_fov_x;
        let dir_z =
            (LongDouble::from_f64(1.0) - LongDouble::from_f64(2.0) * y) * self.tan_half_fov_y;

        let direction = Direction::new(*self.direction + dir_x * self.right + self.up * dir_z);

        Ray {
            origin: self.position,
            direction,
        }
    }
}
