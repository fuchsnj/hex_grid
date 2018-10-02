#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PixelOrientation {
    pub right_increasing: bool,
    pub up_increasing: bool,
}

impl PixelOrientation {
    /**
     * Convert coordinates from right_increasing/up_increasing to this orientation
     */
    pub(crate) fn orient(&self, (x, y): (f32, f32)) -> (f32, f32) {
        let x = if self.right_increasing {
            x
        } else {
            -x
        };
        let y = if self.up_increasing {
            y
        } else {
            -y
        };
        (x, y)
    }
}