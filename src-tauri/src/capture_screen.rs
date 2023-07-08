use display_info::DisplayInfo;
use mouse_position::mouse_position::Mouse;
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

#[allow(dead_code)]
pub fn get_mouse_position() -> (i32, i32) {
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => {
            println!("x: {}, y: {}", x, y);
            return (x, y);
        }
        Mouse::Error => {
            println!("Error getting mouse position");
            return (0, 0);
        }
    }
}
