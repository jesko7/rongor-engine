use rongor::prelude::*;

async fn run(window: Window) {
    let default_cam = Camera2d::default();
    let rust_logo = Image::new("assets/rust.png", 0., false, false, None, None).await;
    let cam3d = Camera3d::default();
    let mut mesh = Mesh::new_from_path("assets/cube.obj", rust_logo, Vec3::new(0., 0., 6.), Vec3::new(1., 1., 1.), Vec3::new(0., 0., 0.), None);  

    while window.update_screen().await {
        default_cam.set_as_current_cam();
        draw2d::clear(Color::new(123, 123, 123, 0));
        cam3d.set_as_current_cam();
        mesh.position = mesh.position - Vec3::new(-0.01, -0.01, 0.);
        mesh.pivot = Some(mesh.position);
        mesh.rotation = mesh.rotation - Vec3::new(-0.01, 0., 0.);
        mesh.draw();
    }
}


#[rongor::main]
async fn main() {
    start(run, "test".into(), 1920, 1080, false, true, true, AntialiasingType::Lanczos3).await
}