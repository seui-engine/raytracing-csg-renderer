use std::ops::{Add, Deref, Mul, Neg, Sub};

use seui_engine_raytracing_csg_renderer_long_double::LongDouble;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: LongDouble,
    pub y: LongDouble,
    pub z: LongDouble,
}

impl Vec3 {
    pub const fn new(x: LongDouble, y: LongDouble, z: LongDouble) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(self, other: Vec3) -> LongDouble {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length_square(self) -> LongDouble {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> LongDouble {
        self.length_square().sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let length = self.length();
        if length.abs() < LongDouble::from_f64(1e-6) {
            Vec3::new(
                LongDouble::from_f64(1.0),
                LongDouble::from_f64(0.0),
                LongDouble::from_f64(0.0),
            )
        } else {
            Vec3::new(self.x / length, self.y / length, self.z / length)
        }
    }
}

impl Mul<LongDouble> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: LongDouble) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for LongDouble {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Position(Vec3);
#[derive(Clone, Copy, Debug)]
pub struct Direction(Vec3);
#[derive(Clone, Copy, Debug, Default)]
pub struct Move(Vec3);

impl Position {
    pub fn new(value: Vec3) -> Self {
        Self(value)
    }
}

impl From<Position> for Vec3 {
    fn from(val: Position) -> Self {
        val.0
    }
}

impl Direction {
    pub fn new(value: Vec3) -> Self {
        Self(value.normalize())
    }

    pub fn dot(self, rhs: Direction) -> LongDouble {
        (*self).dot(*rhs)
    }
}

impl From<Direction> for Vec3 {
    fn from(val: Direction) -> Self {
        val.0
    }
}

impl Move {
    pub fn new(value: Vec3) -> Self {
        Self(value)
    }

    pub fn direction_and_length(self) -> (Direction, LongDouble) {
        let length = self.length();
        (
            Direction(Vec3 {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            }),
            length,
        )
    }
}

impl From<Move> for Vec3 {
    fn from(val: Move) -> Self {
        val.0
    }
}

impl Add<Move> for Position {
    type Output = Position;

    fn add(self, rhs: Move) -> Self::Output {
        Position(*self + *rhs)
    }
}

impl Sub<Position> for Position {
    type Output = Move;

    fn sub(self, rhs: Position) -> Self::Output {
        Move(*self - *rhs)
    }
}

impl Deref for Position {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Direction {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Move {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        Direction(-*self)
    }
}

impl Neg for Move {
    type Output = Move;

    fn neg(self) -> Self::Output {
        Move(-*self)
    }
}

impl Mul<LongDouble> for Direction {
    type Output = Move;

    fn mul(self, rhs: LongDouble) -> Self::Output {
        Move(*self * rhs)
    }
}

impl From<Move> for Position {
    fn from(val: Move) -> Self {
        Position(val.0)
    }
}
