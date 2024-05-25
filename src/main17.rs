use std::os::windows::raw::SOCKET;

use macroquad::ui::Ui;
use rongor::prelude::*;

async fn run(window: Window) {
    let cam = Camera2d::default();

    let image = Image::new("assets/rust.png", 0., false, false, Vec2::new(100., 100.), None).await;

    let curve = CubicBeziercurve::new(Vec2::new(300., 300.), Vec2::new(500., 300.), Vec2::new(300., 500.), Vec2::new(500., 100.), 10., Color::new(255, 0, 0, 255));


    while window.update_screen().await {
        cam.set_as_current_cam();
        draw2d::clear(Color::new(0, 0, 0, 0));
        
        draw2d::image::image_center(Vec2::new(100., 100.), &image);
        draw2d::square::quick_square_middle((400., 400.), 10., 10., (255,255,255,255));
        curve.draw(100)
    }
}


#[rongor::main]
async fn main() {
    start(run, "test".into(), 1920, 1080, false, true, true, AntialiasingType::Lanczos3).await
}