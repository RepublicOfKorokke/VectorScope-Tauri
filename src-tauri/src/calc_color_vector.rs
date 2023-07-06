struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

// ITU-R BT.709 (1250/50/2:1)
struct YUV {
    y: u8,
    u: i8,
    v: i8,
}

impl RGB {
    fn convert_to_ybr(&self) -> YUV {
        let y = 0.299 * (self.r as f32) + 0.587 * (self.g as f32) + 0.114 * (self.b as f32);
        let u = -0.169 * (self.r as f32) - 0.331 * (self.g as f32) + 0.5 * (self.b as f32);
        let v = 0.5 * (self.r as f32) - 0.419 * (self.g as f32) - 0.081 * (self.b as f32);
        YUV {
            y: y.round() as u8,
            u: u.round() as i8,
            v: v.round() as i8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::calc_color_vector::RGB;

    #[test]
    fn check_rgb_black_to_yuv() {
        let rgb = RGB { r: 0, g: 0, b: 0 };
        let ybr = rgb.convert_to_ybr();
        assert_eq!(ybr.y, 0);
        assert_eq!(ybr.u, 0);
        assert_eq!(ybr.v, 0);
    }

    #[test]
    fn check_rgb_white_to_yuv() {
        let rgb = RGB {
            r: 255,
            g: 255,
            b: 255,
        };
        let ybr = rgb.convert_to_ybr();
        assert_eq!(ybr.y, 255);
        assert_eq!(ybr.u, 0);
        assert_eq!(ybr.v, 0);
    }

    #[test]
    fn check_rgb_gray_to_yuv() {
        let rgb = RGB {
            r: 128,
            g: 128,
            b: 128,
        };
        let ybr = rgb.convert_to_ybr();
        assert_eq!(ybr.y, 128);
        assert_eq!(ybr.u, 0);
        assert_eq!(ybr.v, 0);
    }

    #[test]
    fn check_rgb_red_to_yuv() {
        let rgb = RGB { r: 255, g: 0, b: 0 };
        let ybr = rgb.convert_to_ybr();
        assert_eq!(ybr.y, 76);
        assert_eq!(ybr.u, -43);
        assert_eq!(ybr.v, 127);
    }

    #[test]
    fn check_rgb_green_to_yuv() {
        let rgb = RGB { r: 0, g: 255, b: 0 };
        let ybr = rgb.convert_to_ybr();
        assert_eq!(ybr.y, 150);
        assert_eq!(ybr.u, -84);
        assert_eq!(ybr.v, -107);
    }

    #[test]
    fn check_rgb_blue_to_yuv() {
        let rgb = RGB { r: 0, g: 0, b: 255 };
        let ybr = rgb.convert_to_ybr();
        assert_eq!(ybr.y, 29);
        assert_eq!(ybr.u, 127);
        assert_eq!(ybr.v, -21);
    }

    #[test]
    fn check_rgb_cyan_to_yuv() {
        let rgb = RGB {
            r: 0,
            g: 255,
            b: 255,
        };
        let ybr = rgb.convert_to_ybr();
        assert_eq!(ybr.y, 179);
        assert_eq!(ybr.u, 43);
        assert_eq!(ybr.v, -128);
    }

    #[test]
    fn check_rgb_purple_to_yuv() {
        let rgb = RGB {
            r: 255,
            g: 0,
            b: 255,
        };
        let ybr = rgb.convert_to_ybr();
        assert_eq!(ybr.y, 105);
        assert_eq!(ybr.u, 84);
        assert_eq!(ybr.v, 107);
    }

    #[test]
    fn check_rgb_yello_to_yuv() {
        let rgb = RGB {
            r: 255,
            g: 255,
            b: 0,
        };
        let ybr = rgb.convert_to_ybr();
        assert_eq!(ybr.y, 226);
        assert_eq!(ybr.u, -128);
        assert_eq!(ybr.v, 21);
    }
}
