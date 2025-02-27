use std::ops::{Add, Deref, Div, Mul, Neg, Sub};

use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Position(Vec3);
#[derive(Clone, Copy, Debug)]
pub struct Direction(Vec3);
#[derive(Clone, Copy, Debug)]
pub struct Move(Vec3);

#[derive(Clone, Copy, Debug)]
pub struct Size(Vec3);
#[derive(Clone, Copy, Debug)]
pub struct Scale(Vec3);

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

// Size
impl Size {
    pub fn new(value: Vec3) -> Self {
        Self(value)
    }
}

impl Mul<f32> for Size {
    type Output = Size;

    fn mul(self, rhs: f32) -> Self::Output {
        Size(self.0 * rhs)
    }
}

impl Div<f32> for Size {
    type Output = Size;

    fn div(self, rhs: f32) -> Self::Output {
        Size(self.0 / rhs)
    }
}

impl From<Move> for Size {
    fn from(val: Move) -> Self {
        Size(val.0)
    }
}

impl Deref for Size {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Size> for Vec3 {
    fn from(val: Size) -> Self {
        val.0
    }
}

impl Sub<Size> for Size {
    type Output = Move;

    fn sub(self, rhs: Size) -> Self::Output {
        Move(self.0 - rhs.0)
    }
}

// Scale
impl Scale {
    pub fn new(value: Vec3) -> Self {
        Self(value)
    }
}

impl Mul<f32> for Scale {
    type Output = Scale;

    fn mul(self, val: f32) -> Self::Output {
        Scale(self.0 * val)
    }
}

impl Div<f32> for Scale {
    type Output = Scale;

    fn div(self, rhs: f32) -> Self::Output {
        Scale(self.0 / rhs)
    }
}

impl From<Move> for Scale {
    fn from(val: Move) -> Self {
        Scale(val.0)
    }
}

impl Deref for Scale {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Scale> for Vec3 {
    fn from(val: Scale) -> Self {
        val.0
    }
}

impl Sub<Scale> for Scale {
    type Output = Move;

    fn sub(self, rhs: Scale) -> Self::Output {
        Move(self.0 - rhs.0)
    }
}
