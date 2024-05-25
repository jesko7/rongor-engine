use rongor::prelude::*;


async fn run(window: Window) {
    let cam = Camera2d::default();

    while window.update_screen().await {
        cam.set_as_current_cam();
        draw2d::clear(Color::new(123, 123, 123, 0));

        draw2d::circle::circle_quick((300., 300.), 100., Color::new(255, 0, 0, 255));
    }
}


#[rongor::main]
async fn main() {
    start(run, "test".into(), 1920, 1080, false, true, true, AntialiasingType::Lanczos3).await
}