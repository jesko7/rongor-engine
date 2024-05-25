use rongor::prelude::*;

async fn run(window: Window) {
    let default_cam = Camera2d::default();
    let rust_logo = Image::new("assets/rust.png", 0., false, false, Some(Vec2::new(100., 100.)), None).await;
    let cam3d = Camera3d::default();
    

    while window.update_screen().await {
        default_cam.set_as_current_cam();
        draw2d::clear(Color::new(0, 0, 0, 0));
        cam3d.set_as_current_cam();
        draw3d::cube::cube_center(Vec3::new(0., 0., 10.), Vec3::new(1., 1., 1.), Some(&rust_logo));
    }
}


#[rongor::main]
async fn main() {
    start(run, "test".into(), 1920, 1080, false, true, true, AntialiasingType::Lanczos3).await
}