use mouse_position::mouse_position::Mouse;

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
