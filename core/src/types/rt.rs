use super::math::{Direction, Position};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Position,
    pub direction: Direction,
}

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    //
}

pub trait RTObject {
    fn test(&self, ray: Ray) -> Vec<Hit>;
}
