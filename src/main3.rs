use std::f32::consts::PI;
use rongor::prelude::*;


static MOVEMENT_SPEED: f32 = 6.;
static LOOK_SPEED: f32 = 100.;


async fn load_textures() -> Vec<Image> {
    let mut textures = vec![];
    let rust_logo = Image::new("assets/rust.png", 0., false, false, Some(Vec2::new(100., 100.)), None).await;
    textures.push(rust_logo);
    textures
}

fn get_cams() -> (Camera2d, Camera3d) {
    let default_cam = Camera2d::default();
    let cam: Camera3d = Camera3d::default();
    (default_cam, cam)
}






fn movment(key: &str, cam3d: &mut Camera3d, delta: f32) {
    let offset3d = match key {
        "w" => cam3d.get_front(),
        "a" => cam3d.get_left(),
        "s" => cam3d.get_back(),
        "d" => cam3d.get_right(),
        "space" => Vec3::new(0., 1., 0.),
        "leftshift" => Vec3::new(0., -1., 0.),
        other => panic!("{} isnt a valid movment key", other)
    };
    if ["w", "a", "s", "d"].contains(&key) {
        if key::is_pressed(key) {
            let mut offset2d = Vec2::new(offset3d.x, offset3d.z);
            offset2d.normalize();
            let offset3d = Vec3::new(offset2d.x, 0., offset2d.y).scale(MOVEMENT_SPEED).scale(delta);

            cam3d.position = cam3d.position + offset3d;
        }
    }
    else {
        if key::is_pressed(key) {
            cam3d.position = cam3d.position + offset3d.scale(MOVEMENT_SPEED).scale(delta);
        }
    }
}








fn update_cam(cam3d: &mut Camera3d, delta: f32) {

    for key in ["w", "a", "s", "d", "space", "leftshift"] {
        movment(key, cam3d, delta);
    }
    
    cam3d.rotation = cam3d.rotation + mouse::get_delta_position().scale(delta).scale(LOOK_SPEED);
    if cam3d.rotation.y < -PI / 2. + 0.001  {
        cam3d.rotation.y = -PI / 2. + 0.001;
    }
    if cam3d.rotation.y > PI / 2. - 0.001 {
        cam3d.rotation.y = PI / 2. - 0.001;
    }

    cam3d.update_all_rotation();
}


async fn run(window: Window){   
    let textures = load_textures().await;
    let (default_cam, mut cam3d) = get_cams();
    
    mouse::set_grab(true);
    mouse::set_visivle(false);

    while window.update_screen().await {
        let delta = window.get_delta_time();
        let rust_logo = &textures[0];
    
        default_cam.set_as_current_cam();
        draw2d::clear(Color::new(0, 0, 0, 255));
    
        update_cam(&mut cam3d, delta);
        draw3d::cube::cube_center(Vec3::new(0., 0., 6.), Vec3::new(1.2, 1.2, 1.2), Some(&rust_logo));
        draw3d::cube::cube_center(Vec3::new(0., 0., 10.), Vec3::new(1., 1., 1.), Some(&rust_logo));
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
        true
    ).await;
}





































































