use macroquad::prelude::*;
use crate::key::*;
use crate::antialiasing::*;

#[derive(Clone, Debug)]
pub struct Window {
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
    pub title: String,
    pub resizable: bool,
    pub exit_on_esc: bool,
    pub antialiasing_type: AntialiasingType,
}

impl Window {
    pub fn new(title: &str, width: i32, height: i32, fullscreen: bool, resizable: bool, exit_on_esc: bool, antialiasing_type: AntialiasingType) -> Window {
        Window {
            width,
            height,
            fullscreen,
            title: title.to_string(),
            resizable,
            exit_on_esc,
            antialiasing_type
        }
    }

    pub fn from_conf(config: &Conf, exit_on_esc: bool, antialiasing_type: AntialiasingType) -> Window {
        Window {
            width: config.window_width,
            height: config.window_height,
            fullscreen: config.fullscreen,
            title: config.window_title.clone(),
            resizable: config.window_resizable,
            exit_on_esc,
            antialiasing_type
        }
    }


    pub fn get_delta_time(&self) -> f32 {
        get_frame_time()
    }

    pub fn get_fps(&self) -> i32 {
        get_fps()
    }

    pub async fn update_screen(&self) -> bool {
        next_frame().await;
        !key::is_pressed("escape")
    }
}