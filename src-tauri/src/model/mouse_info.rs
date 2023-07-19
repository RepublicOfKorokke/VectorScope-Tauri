use mouse_position::mouse_position::Mouse;

pub fn get_mouse_position() -> (i32, i32) {
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => {
            #[cfg(debug_assertions)]
            println!("x: {}, y: {}", x, y);
            return (x, y);
        }
        Mouse::Error => {
            #[cfg(debug_assertions)]
            println!("Error getting mouse position");
            return (0, 0);
        }
    }
}
