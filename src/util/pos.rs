use std::hash::*;
use std::ops::*;

pub const ORIGIN: Pos = Pos::new(0, 0);
pub const UP: Pos     = Pos::new(0, -1);
pub const DOWN: Pos   = Pos::new(0, 1);
pub const LEFT: Pos   = Pos::new(-1, 0);
pub const RIGHT: Pos  = Pos::new(1, 0);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl Hash for Pos {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x as u32);
        state.write_u32(self.y as u32);
    }
}

impl AddAssign for Pos {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
