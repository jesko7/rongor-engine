use macroquad::prelude::*;


pub fn string_to_key_code(key: &str) -> KeyCode{
    match key.to_lowercase().as_str() {
        "space" => KeyCode::Space,
        "apostrophe" => KeyCode::Apostrophe,
        "comma" => KeyCode::Comma,
        "minus" => KeyCode::Minus,
        "period" => KeyCode::Period,
        "slash" => KeyCode::Slash,
        "key0" => KeyCode::Key0,
        "key1" => KeyCode::Key1,
        "key2" => KeyCode::Key2,
        "key3" => KeyCode::Key3,
        "key4" => KeyCode::Key4,
        "key5" => KeyCode::Key5,
        "key6" => KeyCode::Key6,
        "key7" => KeyCode::Key7,
        "key8" => KeyCode::Key8,
        "key9" => KeyCode::Key9,
        "semicolon" => KeyCode::Semicolon,
        "equal" => KeyCode::Equal,
        "a" => KeyCode::A,
        "b" => KeyCode::B,
        "c" => KeyCode::C,
        "d" => KeyCode::D,
        "e" => KeyCode::E,
        "f" => KeyCode::F,
        "g" => KeyCode::G,
        "h" => KeyCode::H,
        "i" => KeyCode::I,
        "j" => KeyCode::J,
        "k" => KeyCode::K,
        "l" => KeyCode::L,
        "m" => KeyCode::M,
        "n" => KeyCode::N,
        "o" => KeyCode::O,
        "p" => KeyCode::P,
        "q" => KeyCode::Q,
        "r" => KeyCode::R,
        "s" => KeyCode::S,
        "t" => KeyCode::T,
        "u" => KeyCode::U,
        "v" => KeyCode::V,
        "w" => KeyCode::W,
        "x" => KeyCode::X,
        "y" => KeyCode::Y,
        "z" => KeyCode::Z,
        "leftbracket" => KeyCode::LeftBracket,
        "backslash" => KeyCode::Backslash,
        "rightbracket" => KeyCode::RightBracket,
        "graveaccent" => KeyCode::GraveAccent,
        "world1" => KeyCode::World1,
        "world2" => KeyCode::World2,
        "escape" => KeyCode::Escape,
        "enter" => KeyCode::Enter,
        "tab" => KeyCode::Tab,
        "backspace" => KeyCode::Backspace,
        "insert" => KeyCode::Insert,
        "delete" => KeyCode::Delete,
        "right" => KeyCode::Right,
        "left" => KeyCode::Left,
        "down" => KeyCode::Down,
        "up" => KeyCode::Up,
        "pageup" => KeyCode::PageUp,
        "pagedown" => KeyCode::PageDown,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "capslock" => KeyCode::CapsLock,
        "scrolllock" => KeyCode::ScrollLock,
        "numlock" => KeyCode::NumLock,
        "printscreen" => KeyCode::PrintScreen,
        "pause" => KeyCode::Pause,
        "f1" => KeyCode::F1,
        "f2" => KeyCode::F2,
        "f3" => KeyCode::F3,
        "f4" => KeyCode::F4,
        "f5" => KeyCode::F5,
        "f6" => KeyCode::F6,
        "f7" => KeyCode::F7,
        "f8" => KeyCode::F8,
        "f9" => KeyCode::F9,
        "f10" => KeyCode::F10,
        "f11" => KeyCode::F11,
        "f12" => KeyCode::F12,
        "f13" => KeyCode::F13,
        "f14" => KeyCode::F14,
        "f15" => KeyCode::F15,
        "f16" => KeyCode::F16,
        "f17" => KeyCode::F17,
        "f18" => KeyCode::F18,
        "f19" => KeyCode::F19,
        "f20" => KeyCode::F20,
        "f21" => KeyCode::F21,
        "f22" => KeyCode::F22,
        "f23" => KeyCode::F23,
        "f24" => KeyCode::F24,
        "f25" => KeyCode::F25,
        "kp0" => KeyCode::Kp0,
        "kp1" => KeyCode::Kp1,
        "kp2" => KeyCode::Kp2,
        "kp3" => KeyCode::Kp3,
        "kp4" => KeyCode::Kp4,
        "kp5" => KeyCode::Kp5,
        "kp6" => KeyCode::Kp6,
        "kp7" => KeyCode::Kp7,
        "kp8" => KeyCode::Kp8,
        "kp9" => KeyCode::Kp9,
        "kpdecimal" => KeyCode::KpDecimal,
        "kpdivide" => KeyCode::KpDivide,
        "kpmultiply" => KeyCode::KpMultiply,
        "kpsubtract" => KeyCode::KpSubtract,
        "kpadd" => KeyCode::KpAdd,
        "kpenter" => KeyCode::KpEnter,
        "kpequal" => KeyCode::KpEqual,
        "leftshift" => KeyCode::LeftShift,
        "leftcontrol" => KeyCode::LeftControl,
        "leftalt" => KeyCode::LeftAlt,
        "leftsuper" => KeyCode::LeftSuper,
        "rightshift" => KeyCode::RightShift,
        "rightcontrol" => KeyCode::RightControl,
        "rightalt" => KeyCode::RightAlt,
        "rightsuper" => KeyCode::RightSuper,
        "menu" => KeyCode::Menu,
        "unknown" => KeyCode::Unknown,
        _other => panic!("notgood")
     }
}

fn key_code_to_string(key_code: KeyCode) -> String {
    match key_code {
        KeyCode::Space => "space".to_string(),
        KeyCode::Apostrophe => "apostrophe".to_string(),
        KeyCode::Comma => "comma".to_string(),
        KeyCode::Minus => "minus".to_string(),
        KeyCode::Period => "period".to_string(),
        KeyCode::Slash => "slash".to_string(),
        KeyCode::Key0 => "key0".to_string(),
        KeyCode::Key1 => "key1".to_string(),
        KeyCode::Key2 => "key2".to_string(),
        KeyCode::Key3 => "key3".to_string(),
        KeyCode::Key4 => "key4".to_string(),
        KeyCode::Key5 => "key5".to_string(),
        KeyCode::Key6 => "key6".to_string(),
        KeyCode::Key7 => "key7".to_string(),
        KeyCode::Key8 => "key8".to_string(),
        KeyCode::Key9 => "key9".to_string(),
        KeyCode::Semicolon => "semicolon".to_string(),
        KeyCode::Equal => "equal".to_string(),
        KeyCode::A => "a".to_string(),
        KeyCode::B => "b".to_string(),
        KeyCode::C => "c".to_string(),
        KeyCode::D => "d".to_string(),
        KeyCode::E => "e".to_string(),
        KeyCode::F => "f".to_string(),
        KeyCode::G => "g".to_string(),
        KeyCode::H => "h".to_string(),
        KeyCode::I => "i".to_string(),
        KeyCode::J => "j".to_string(),
        KeyCode::K => "k".to_string(),
        KeyCode::L => "l".to_string(),
        KeyCode::M => "m".to_string(),
        KeyCode::N => "n".to_string(),
        KeyCode::O => "o".to_string(),
        KeyCode::P => "p".to_string(),
        KeyCode::Q => "q".to_string(),
        KeyCode::R => "r".to_string(),
        KeyCode::S => "s".to_string(),
        KeyCode::T => "t".to_string(),
        KeyCode::U => "u".to_string(),
        KeyCode::V => "v".to_string(),
        KeyCode::W => "w".to_string(),
        KeyCode::X => "x".to_string(),
        KeyCode::Y => "y".to_string(),
        KeyCode::Z => "z".to_string(),
        KeyCode::LeftBracket => "leftbracket".to_string(),
        KeyCode::Backslash => "backslash".to_string(),
        KeyCode::RightBracket => "rightbracket".to_string(),
        KeyCode::GraveAccent => "graveaccent".to_string(),
        KeyCode::World1 => "world1".to_string(),
        KeyCode::World2 => "world2".to_string(),
        KeyCode::Escape => "escape".to_string(),
        KeyCode::Enter => "enter".to_string(),
        KeyCode::Tab => "tab".to_string(),
        KeyCode::Backspace => "backspace".to_string(),
        KeyCode::Insert => "insert".to_string(),
        KeyCode::Delete => "delete".to_string(),
        KeyCode::Right => "right".to_string(),
        KeyCode::Left => "left".to_string(),
        KeyCode::Down => "down".to_string(),
        KeyCode::Up => "up".to_string(),
        KeyCode::PageUp => "pageup".to_string(),
        KeyCode::PageDown => "pagedown".to_string(),
        KeyCode::Home => "home".to_string(),
        KeyCode::End => "end".to_string(),
        KeyCode::CapsLock => "capslock".to_string(),
        KeyCode::ScrollLock => "scrolllock".to_string(),
        KeyCode::NumLock => "numlock".to_string(),
        KeyCode::PrintScreen => "printscreen".to_string(),
        KeyCode::Pause => "pause".to_string(),
        KeyCode::F1 => "f1".to_string(),
        KeyCode::F2 => "f2".to_string(),
        KeyCode::F3 => "f3".to_string(),
        KeyCode::F4 => "f4".to_string(),
        KeyCode::F5 => "f5".to_string(),
        KeyCode::F6 => "f6".to_string(),
        KeyCode::F7 => "f7".to_string(),
        KeyCode::F8 => "f8".to_string(),
        KeyCode::F9 => "f9".to_string(),
        KeyCode::F10 => "f10".to_string(),
        KeyCode::F11 => "f11".to_string(),
        KeyCode::F12 => "f12".to_string(),
        KeyCode::F13 => "f13".to_string(),
        KeyCode::F14 => "f14".to_string(),
        KeyCode::F15 => "f15".to_string(),
        KeyCode::F16 => "f16".to_string(),
        KeyCode::F17 => "f17".to_string(),
        KeyCode::F18 => "f18".to_string(),
        KeyCode::F19 => "f19".to_string(),
        KeyCode::F20 => "f20".to_string(),
        KeyCode::F21 => "f21".to_string(),
        KeyCode::F22 => "f22".to_string(),
        KeyCode::F23 => "f23".to_string(),
        KeyCode::F24 => "f24".to_string(),
        KeyCode::F25 => "f25".to_string(),
        KeyCode::Kp0 => "kp0".to_string(),
        KeyCode::Kp1 => "kp1".to_string(),
        KeyCode::Kp2 => "kp2".to_string(),
        KeyCode::Kp3 => "kp3".to_string(),
        KeyCode::Kp4 => "kp4".to_string(),
        KeyCode::Kp5 => "kp5".to_string(),
        KeyCode::Kp6 => "kp6".to_string(),
        KeyCode::Kp7 => "kp7".to_string(),
        KeyCode::Kp8 => "kp8".to_string(),
        KeyCode::Kp9 => "kp9".to_string(),
        KeyCode::KpDecimal => "kpdecimal".to_string(),
        KeyCode::KpDivide => "kpdivide".to_string(),
        KeyCode::KpMultiply => "kpmultiply".to_string(),
        KeyCode::KpSubtract => "kpsubtract".to_string(),
        KeyCode::KpAdd => "kpadd".to_string(),
        KeyCode::KpEnter => "kpenter".to_string(),
        KeyCode::KpEqual => "kpequal".to_string(),
        KeyCode::LeftShift => "leftshift".to_string(),
        KeyCode::LeftControl => "leftcontrol".to_string(),
        KeyCode::LeftAlt => "leftalt".to_string(),
        KeyCode::LeftSuper => "leftsuper".to_string(),
        KeyCode::RightShift => "rightshift".to_string(),
        KeyCode::RightControl => "rightcontrol".to_string(),
        KeyCode::RightAlt => "rightalt".to_string(),
        KeyCode::RightSuper => "rightsuper".to_string(),
        KeyCode::Menu => "menu".to_string(),
        KeyCode::Unknown => "unknown".to_string()
    }
}


pub mod key {
    use macroquad::prelude;
    use super::string_to_key_code;
    use super::key_code_to_string;

    pub fn just_pressed(key: &str) -> bool {
        prelude::is_key_pressed(string_to_key_code(key))
    }
    pub fn just_released(key: &str) -> bool {
        prelude::is_key_released(string_to_key_code(key))
    }
    pub fn is_pressed(key: &str) -> bool {
        prelude::is_key_down(string_to_key_code(key))
    }
    pub fn is_released(key: &str) -> bool {
        !prelude::is_key_down(string_to_key_code(key))
    }
    pub fn get_keys_just_pressed() -> Vec<String> {
        let key_codes = prelude::get_keys_pressed();

        let mut keys: Vec<String> = vec![];

        for key_code in key_codes {
            keys.push(key_code_to_string(key_code));
        }

        keys
    }
    pub fn get_keys_pressed() -> Vec<String> {
        let key_codes = prelude::get_keys_down();

        let mut keys: Vec<String> = vec![];

        for key_code in key_codes {
            keys.push(key_code_to_string(key_code));
        }

        keys
    }
    pub fn get_keys_just_released() -> Vec<String> {
        let key_codes = prelude::get_keys_released();

        let mut keys: Vec<String> = vec![];

        for key_code in key_codes {
            keys.push(key_code_to_string(key_code));
        }

        keys
    }
}