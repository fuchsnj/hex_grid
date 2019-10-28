use std::ops::{Add, Sub, Neg};

pub use super::offset::*;
use PixelOrientation;
use HexSize;

pub const CENTER: Coordinate = Coordinate { x: 0, y: 0 };

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn z(&self) -> i32 {
        Self::calculate_3rd((self.x, self.y))
    }

    fn x32(&self) -> f32 {
        self.x as f32
    }
    fn y32(&self) -> f32 {
        self.y as f32
    }

    pub fn to_pixel(&self, hex_size: HexSize, pixel_orientation: PixelOrientation) -> (f32, f32) {
        let px = hex_size.width() * (self.x32() + self.y32() / 2.0);
        let py = hex_size.height() * -0.75 * self.y32();
        pixel_orientation.orient((px, py))
    }

    pub fn from_pixel((px, py): (f32, f32), hex_size: HexSize, pixel_orientation: PixelOrientation) -> Coordinate {
        let (px, py) = pixel_orientation.orient((px, py));
        let hy = py / (hex_size.height() * -0.75);
        let hx = px / hex_size.width() - hy / 2.0;
        Self::round_to_nearest((hx, hy))
    }

    fn round_to_nearest((x, y): (f32, f32)) -> Coordinate {
        let z = Self::calculate_3rd((x, y));

        let mut rx = x.round();
        let mut ry = y.round();
        let rz = z.round();

        let delta_x = (rx - x).abs();
        let delta_y = (ry - y).abs();
        let delta_z = (rz - z).abs();

        if delta_x > delta_y && delta_x > delta_z {
            rx = Self::calculate_3rd((ry, rz));
        } else if delta_y > delta_z {
            ry = Self::calculate_3rd((rx, rz));
        }
        Coordinate {
            x: rx as i32,
            y: ry as i32,
        }
    }

    fn calculate_3rd<T: Neg<Output=T> + Sub<Output=T>>((x, y): (T, T)) -> T {
        -x - y
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
    let coord = CENTER + UP_RIGHT;
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

#[test]
fn test_from_pixel() {
    let hex_size = HexSize::from_regular_height(1.0);
    let pixel_orientation = PixelOrientation { right_increasing: true, up_increasing: true };

    assert_eq!(Coordinate::from_pixel((0.0, 0.0), hex_size, pixel_orientation), CENTER);
    assert_eq!(Coordinate::from_pixel((0.8660254, 0.0), hex_size, pixel_orientation), CENTER + RIGHT);
    assert_eq!(Coordinate::from_pixel((-0.8660254, 0.0), hex_size, pixel_orientation), CENTER + LEFT);
    assert_eq!(Coordinate::from_pixel((0.4330127, 0.75), hex_size, pixel_orientation), CENTER + UP_RIGHT);
    assert_eq!(Coordinate::from_pixel((-0.8660254, -1.5), hex_size, pixel_orientation), CENTER + DOWN_LEFT * 2);
}