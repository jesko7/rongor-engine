use macroquad::prelude;
use crate::window::Window;
use std::future::Future;
use crate::antialiasing::AntialiasingType;


pub async fn start
<A>
(
run_func: fn(Window) -> A, 
title: String,
width: i32,
height: i32,
resizable: bool,
fullscreen: bool,
exit_on_esc: bool,
antialiasing_type: AntialiasingType
)
where
A: Future<Output = ()>  + 'static
{   
    let config: prelude::Conf = prelude::Conf {
        window_title: title.clone(),
        window_width: width,
        window_height: height,
        window_resizable: resizable,
        fullscreen: fullscreen,
        sample_count: 4,
        ..Default::default()
    };
    
    let window = Window::from_conf(&config, exit_on_esc, antialiasing_type);

    macroquad::Window::from_config(config, run_func(window));
    
}
