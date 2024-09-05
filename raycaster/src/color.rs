#[derive(Debug, Clone, Copy)]
pub struct ColorRGB {
    r: u8,
    g: u8,
    b: u8,
}

pub const RED_RGB: ColorRGB = ColorRGB { r: 255, g: 0, b: 0 };
pub const GREEN_RGB: ColorRGB = ColorRGB { r: 0, g: 255, b: 0 };
pub const BLUE_RGB: ColorRGB = ColorRGB { r: 0, g: 0, b: 255 };
pub const WHITE_RGB: ColorRGB = ColorRGB { r: 255, g: 255, b: 255 };
pub const YELLOW_RGB: ColorRGB = ColorRGB { r: 255, g: 255, b: 0 };

impl ColorRGB {
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

impl std::ops::Div<u8> for ColorRGB {
    type Output = ColorRGB;

    fn div(self, rhs: u8) -> ColorRGB {
        ColorRGB {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}