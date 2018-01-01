use std::i32;
use std::ops::{Add, Mul, Neg, Sub};
use super::Coordinate;

pub const ZERO_OFFSET: Offset = Offset { x: 0, y: 0 };
pub const RIGHT: Offset = Offset { x: 1, y: 0 };
pub const LEFT: Offset = Offset { x: -1, y: 0 };
pub const UP_RIGHT: Offset = Offset { x: 1, y: -1 };
pub const UP_LEFT: Offset = Offset { x: 0, y: -1 };
pub const DOWN_RIGHT: Offset = Offset { x: 0, y: 1 };
pub const DOWN_LEFT: Offset = Offset { x: -1, y: 1 };

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Offset {
    pub x: i32,
    pub y: i32
}

impl Offset {
    pub fn fill_hex(radius: u16) -> Vec<Offset> {
        if radius == 0 {
            vec!(ZERO_OFFSET)
        } else {
            let vertices = vec![
                DOWN_RIGHT * radius,
                DOWN_LEFT * radius,
                LEFT * radius,
                UP_LEFT * radius,
                UP_RIGHT * radius,
                RIGHT * radius
            ];
            let mut current = RIGHT * radius;
            let mut output = vec!();
            for vertex in vertices {
                while current != vertex {
                    current = current + (vertex - current).direction();
                    output.push(current);
                }
            }
            output.append(&mut Self::fill_hex(radius - 1));
            output
        }
    }

    /**
     * truncates the x/y coordinates to be -1, 0, 1
     */
    pub fn direction(&self) -> Offset {
        Offset {
            x: i32::max(i32::min(self.x, 1), -1),
            y: i32::max(i32::min(self.y, 1), -1),
        }
    }
}


impl From<(i32, i32)> for Offset {
    fn from((x, y): (i32, i32)) -> Self {
        Offset { x, y }
    }
}

impl<O: Into<Offset>> Add<O> for Offset {
    type Output = Offset;

    fn add(self, offset: O) -> Offset {
        let offset = offset.into();
        Offset {
            x: self.x + offset.x,
            y: self.y + offset.y
        }
    }
}

impl<O: Into<Offset>> Sub<O> for Offset {
    type Output = Offset;

    fn sub(self, offset: O) -> Offset {
        let offset = offset.into();
        Offset {
            x: self.x - offset.x,
            y: self.y - offset.y
        }
    }
}


impl Add<Coordinate> for Offset {
    type Output = Coordinate;

    fn add(self, coord: Coordinate) -> Coordinate {
        coord + self
    }
}


impl<I: Into<i32>> Mul<I> for Offset {
    type Output = Offset;

    fn mul(self, value: I) -> Offset {
        let value = value.into();
        Offset {
            x: self.x * value,
            y: self.y * value
        }
    }
}

impl Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Offset {
        self * -1
    }
}

#[test]
fn test_offset_math() {
    assert_eq!(LEFT + RIGHT, ZERO_OFFSET);
    assert_eq!(UP_RIGHT + DOWN_LEFT, ZERO_OFFSET);
    assert_eq!(UP_RIGHT + DOWN_RIGHT, RIGHT);
    assert_eq!(UP_LEFT + DOWN_LEFT, LEFT);
    assert_eq!(UP_RIGHT + UP_LEFT, (1, -2).into());
    assert_eq!(DOWN_RIGHT + DOWN_LEFT, (-1, 2).into());
    assert_eq!(RIGHT + RIGHT, RIGHT * 2);
    assert_eq!(RIGHT * -1, LEFT);
    assert_eq!(-RIGHT, LEFT);
    assert_eq!(UP_RIGHT + DOWN_RIGHT + LEFT, ZERO_OFFSET);
    assert_eq!(RIGHT - LEFT, RIGHT + RIGHT);
}

#[test]
fn test_direction() {
    assert_eq!(ZERO_OFFSET.direction(), ZERO_OFFSET);
    assert_eq!(RIGHT.direction(), RIGHT);
    assert_eq!(Offset::from((2, 0)).direction(), RIGHT);
    assert_eq!(Offset::from((1, -2)).direction(), UP_RIGHT);
    assert_eq!(Offset::from((1, -3)).direction(), UP_RIGHT);
}

#[test]
fn test_fill_hex() {
    assert_eq!(Offset::fill_hex(0), vec![ZERO_OFFSET]);
    assert_eq!(Offset::fill_hex(1), vec![
        DOWN_RIGHT, DOWN_LEFT, LEFT, UP_LEFT, UP_RIGHT, RIGHT, ZERO_OFFSET
    ]);
    assert_eq!(Offset::fill_hex(2).len(), 1 + 6 * 3);
    assert_eq!(Offset::fill_hex(3).len(), 1 + 6 * 6);
}