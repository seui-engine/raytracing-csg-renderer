use std::ops::{Add, Sub};

use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Position(Vec3);
#[derive(Clone, Copy, Debug)]
pub struct Direction(Vec3);
#[derive(Clone, Copy, Debug)]
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
