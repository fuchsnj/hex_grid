use std::ops::{Add, Sub};

pub use super::offset::*;

pub const CENTER: Coordinate = Coordinate { x: 0, y: 0 };

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Coordinate {
    fn from((x, y): (i32, i32)) -> Self {
        Coordinate { x, y }
    }
}

impl<O: Into<Offset>> Add<O> for Coordinate {
    type Output = Coordinate;

    fn add(self, offset: O) -> Coordinate {
        let offset = offset.into();
        Coordinate {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}

impl Add<Vec<Offset>> for Coordinate {
    type Output = Vec<Coordinate>;

    fn add(self, offsets: Vec<Offset>) -> Self::Output {
        offsets.iter().map(|offset| *offset + self).collect()
    }
}

impl Add<Coordinate> for Vec<Offset> {
    type Output = Vec<Coordinate>;

    fn add(self, coord: Coordinate) -> Self::Output {
        coord + self
    }
}

impl<C: Into<Coordinate>> Sub<C> for Coordinate {
    type Output = Offset;

    fn sub(self, other: C) -> Offset {
        let other = other.into();
        Offset {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[test]
fn test_coord_plus_offset_list() {
    let coord = Coordinate::from(CENTER + UP_RIGHT);
    let offsets = vec!(LEFT, ZERO_OFFSET, RIGHT);
    let coords = coord + offsets.clone();
    assert_eq!(coords, vec!(coord + LEFT, coord + ZERO_OFFSET, RIGHT + coord));
    assert_eq!(coord + offsets.clone(), offsets.clone() + coord);
}

#[test]
fn test_sub() {
    let offset = (CENTER + RIGHT) - (0, 0);
    assert_eq!(offset, Offset::from((1, 0)));
}