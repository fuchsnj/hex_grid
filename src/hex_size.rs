use std::f32;

/**
 * Width/Height of a pointy top hexagon
 */
#[derive(Copy, Clone, Debug)]
pub struct HexSize {
    width: f32,
    height: f32,
}

impl HexSize {
    /**
     * Calculate the size of a regular hexagon from the width
     */
    pub fn from_regular_width(width: f32) -> HexSize {
        HexSize {
            width,
            height: 2.0 * width / 3.0_f32.sqrt(),
        }
    }
    /**
     * Calculate the size of a regular hexagon from the height
     */
    pub fn from_regular_height(height: f32) -> HexSize {
        HexSize {
            width: (height / 2.0) * 3.0_f32.sqrt(),
            height,
        }
    }

    pub fn new_irregular(width: f32, height: f32) -> HexSize {
        HexSize { width, height }
    }

    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_from_width() {
        let hex_size = HexSize::from_regular_width(1.0);
        assert_eq!(hex_size.width(), 1.0);
        assert_eq!(hex_size.height(), 1.1547005);
    }

    #[test]
    pub fn test_from_height() {
        let hex_size = HexSize::from_regular_height(1.0);
        assert_eq!(hex_size.width(), 0.8660254);
        assert_eq!(hex_size.height(), 1.0);
    }
}