use std::ops::{Add, Deref, Mul, Neg, Sub};

use glam::Vec3;

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

    pub fn dot(self, rhs: Direction) -> f32 {
        (*self).dot(*rhs)
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction(Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        })
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

    pub fn direction_and_length(self) -> (Direction, f32) {
        let length = self.length();
        (Direction(*self / length), length)
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
        Position(self.0 + rhs.0)
    }
}

impl Sub<Position> for Position {
    type Output = Move;

    fn sub(self, rhs: Position) -> Self::Output {
        Move(self.0 - rhs.0)
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
        Direction(-self.0)
    }
}

impl Neg for Move {
    type Output = Move;

    fn neg(self) -> Self::Output {
        Move(-self.0)
    }
}

impl Mul<f32> for Direction {
    type Output = Move;

    fn mul(self, rhs: f32) -> Self::Output {
        Move(self.0 * rhs)
    }
}

impl From<Move> for Position {
    fn from(val: Move) -> Self {
        Position(val.0)
    }
}
