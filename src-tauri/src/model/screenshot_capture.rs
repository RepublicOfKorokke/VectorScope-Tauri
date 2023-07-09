use display_info::DisplayInfo;
use screenshots::Screen;

#[allow(dead_code)]
pub fn capture_entire_sreen() -> Vec<u8> {
    let display_info = DisplayInfo::from_point(0, 0).unwrap();
    let screen = Screen::new(&display_info);

    let image = screen.capture().unwrap();
    return image.rgba().clone();
}

#[allow(dead_code)]
pub fn capture_area(area_top_left: (i32, i32), area_bottom_right: (i32, i32)) -> Vec<u8> {
    let display_info = DisplayInfo::from_point(0, 0).unwrap();
    let screen = Screen::new(&display_info);

    let width: u32 = (area_bottom_right.0 - area_top_left.0).try_into().unwrap();
    let height: u32 = (area_bottom_right.1 - area_top_left.1).try_into().unwrap();
    println!("range width {} /range height {}", width, height);
    let image = screen
        .capture_area(area_top_left.0, area_top_left.1, width, height)
        .unwrap();
    println!(
        "image width {} / image height {}",
        image.width(),
        image.height()
    );
    return image.rgba().clone();
}
