
use macroquad::prelude::*;

#[macroquad::main("Draw Text with 2D Camera Example")]
async fn main() {
    let mut camera = Camera2D::from_display_rect(Rect::new(0., screen_height(), screen_width(), -screen_height()));

    let rust_logo = load_texture("assets/rust.png").await.unwrap();

    loop {
        clear_background(BLACK);

        set_camera(&camera);

        draw_texture(&rust_logo, 100., 100., WHITE);
        draw_text("Hello, World!", 20., 600.0, 20.0, WHITE);
        draw_circle(100., 100., 10., WHITE);
        
        next_frame().await;

    }
}
