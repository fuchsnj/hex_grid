use std::ops::{Add, Sub};

pub use super::offset::*;
use PixelOrientation;
use HexSize;

pub const CENTER: Coordinate = Coordinate { x: 0, y: 0 };

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn z(&self) -> i32 {
        -self.x - self.y
    }
    fn x32(&self) -> f32 {
        self.x as f32
    }
    fn y32(&self) -> f32 {
        self.y as f32
    }
    fn z32(&self) -> f32 {
        self.z() as f32
    }

    pub fn to_pixel(&self, hex_size: HexSize, pixel_orientation: PixelOrientation) -> (f32, f32) {
        let x = hex_size.width() * (self.x32() + self.y32() / 2.0);
        let y = hex_size.height() * -0.75 * self.y32();
        pixel_orientation.orient((x, y))
    }
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

#[test]
fn test_to_pixel() {
    let hex_size = HexSize::from_regular_height(1.0);
    let pixel_orientation = PixelOrientation { right_increasing: true, up_increasing: true };

    assert_eq!(CENTER.to_pixel(hex_size, pixel_orientation), (0.0, 0.0));
    assert_eq!((CENTER + RIGHT).to_pixel(hex_size, pixel_orientation), ((0.8660254, 0.0)));
    assert_eq!((CENTER + LEFT).to_pixel(hex_size, pixel_orientation), ((-0.8660254, 0.0)));
    assert_eq!((CENTER + UP_RIGHT).to_pixel(hex_size, pixel_orientation), ((0.4330127, 0.75)));
    assert_eq!((CENTER + DOWN_LEFT * 2).to_pixel(hex_size, pixel_orientation), ((-0.8660254, -1.5)));
}