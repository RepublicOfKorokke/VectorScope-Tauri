use display_info::DisplayInfo;
use screenshots::{Image, Screen};

#[inline(always)]
pub fn capture_entire_sreen() -> Image {
    let display_info = DisplayInfo::from_point(0, 0).unwrap();
    let screen = Screen::new(&display_info);

    let image = screen.capture().unwrap();
    return image;
}

#[inline(always)]
pub fn capture_area(area_top_left: (i32, i32), area_bottom_right: (i32, i32)) -> Image {
    let display_info = DisplayInfo::from_point(area_top_left.0, area_top_left.1).unwrap();
    let screen = Screen::new(&display_info);

    // for multi screen support
    // screen.capture_area() requires the axis from target screen's top left
    // so I need to re-calculate capture area position
    let display_top_left: (i32, i32) = (display_info.x, display_info.y);

    let width: u32 = (area_bottom_right.0 - area_top_left.0).try_into().unwrap();
    let height: u32 = (area_bottom_right.1 - area_top_left.1).try_into().unwrap();
    let image = screen
        .capture_area(
            area_top_left.0 - display_top_left.0,
            area_top_left.1 - display_top_left.1,
            width,
            height,
        )
        .unwrap();
    return image;
}
