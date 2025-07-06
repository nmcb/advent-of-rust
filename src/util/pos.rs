use std::ops::*;

pub const ZERO: Pos  = Pos::new(0, 0);

pub const UP: Pos    = Pos::new(0, -1);
pub const DOWN: Pos  = Pos::new(0, 1);
pub const LEFT: Pos  = Pos::new(-1, 0);
pub const RIGHT: Pos = Pos::new(1, 0);

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

}

impl From<u8> for Pos {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            b'^' => UP,
            b'v' => DOWN,
            b'<' => LEFT,
            b'>' => RIGHT,
            _ => unreachable!(),
        }
    }
}

impl AddAssign for Pos {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
