use rongor::prelude::*;

async fn run(window: Window) {
    let default_cam = Camera2d::default(&window.render_target);
    
    while window.update_screen().await {
        default_cam.set_as_current_cam();
        draw2d::clear(Color::new(255, 255, 255, 255));
        draw2d::square::quick_square_top_left((0., 0.), 100., 100., (0, 255, 0, 0))
    }
}


#[rongor::main]
async fn main() {
    start(run, "test".into(), 1920, 1080, false, true, true, AntialiasingType::Lanczos3).await
}