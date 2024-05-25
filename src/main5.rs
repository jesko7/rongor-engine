use rongor::prelude::*;


async fn run(window: Window){     
    let default_camera = Camera2d::default();
    let mut image = Image::new("assets/rust.png", 0., false, false, Some(Vec2::new(100., 100.)), None).await;
    while window.update_screen().await {
        let delta = window.get_delta_time();

        default_camera.set_as_current_cam();
        
        draw2d::image::image(Vec2::new(0., 0.), &image);


    }
}






#[rongor::main]
async fn main() {
    start
    (
        run, 
        "title".to_string(), 
        1920, 
        1080, 
        false, 
        true, 
        true,
        AntialiasingType::Lanczos3
    ).await;
}

