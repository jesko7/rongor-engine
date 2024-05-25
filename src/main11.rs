use macroquad::prelude::*;

#[macroquad::main("Shadertoy")]
async fn main() {
    
    loop {
        set_default_camera();
        draw_text_ex("Custom font size:", 20.0, 20.0, TextParams::default());
        next_frame().await
    }
}
