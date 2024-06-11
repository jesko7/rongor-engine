pub mod mouse {
    use macroquad::prelude;
    use crate::vec::*;
    use crossterm::cursor::*;

    pub const LEFT: u8 = 1;
    pub const RIGHT: u8 = 2;
    pub const WHEEL: u8 = 3;

    pub fn to_macro_button(button: u8) -> prelude::MouseButton {
        match button {
            LEFT => prelude::MouseButton::Left,
            RIGHT => prelude::MouseButton::Right,
            WHEEL => prelude::MouseButton::Middle,
            other => panic!("{} is not a valid mouse button", other)
        }
    }

    pub fn get_delta_position() -> Vec2 {
        let macro_delta = prelude::mouse_delta_position();
        Vec2::new(macro_delta.x, macro_delta.y)
    }
    pub fn set_position(pos: Vec2) {
        MoveTo(pos.x as u16, pos.y as u16);
    }
    pub fn get_position() -> Vec2 {
        let macro_pos = prelude::mouse_position();
        Vec2::new(macro_pos.0, macro_pos.1)
    }
    pub fn is_pressed(button: u8) -> bool {
        prelude::is_mouse_button_down(to_macro_button(button))
    }
    pub fn just_pressed(button: u8) -> bool {
        prelude::is_mouse_button_pressed(to_macro_button(button))
    }
    pub fn just_released(button: u8) -> bool {
        prelude::is_mouse_button_released(to_macro_button(button))
    }
    pub fn set_grab(grab: bool) {
        prelude::set_cursor_grab(grab);
    }
    pub fn set_visivle(visible: bool) {
        prelude::show_mouse(visible)
    }
}