use macroquad::prelude::*;

const MOVE_SPEED: f32 = 15.;
const LOOK_SPEED: f32 = 140.;
const GRAVITY: f32 = 0.3;
const JUMP_HIGHT: f32 = 0.13;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Mincroft"),
        fullscreen: true,
        ..Default::default()
    }
}


fn get_offset(delta: f32, front: Vec3, right: Vec3) -> Vec3 {
    let mut offset = Vec3::ZERO;

    if is_key_down(KeyCode::W) {
        let temp_offset = front * MOVE_SPEED * delta;
        offset += Vec3::new(temp_offset.x, 0., temp_offset.z);
    }
    if is_key_down(KeyCode::S) {
        let temp_offset = -front * MOVE_SPEED * delta;
        offset += Vec3::new(temp_offset.x, 0., temp_offset.z);
    }
    if is_key_down(KeyCode::A) {
        let temp_offset = -right * MOVE_SPEED * delta;
        offset += Vec3::new(temp_offset.x, 0., temp_offset.z);
    }
    if is_key_down(KeyCode::D) {
        let temp_offset = right * MOVE_SPEED * delta;
        offset += Vec3::new(temp_offset.x, 0., temp_offset.z);
    }




    offset
}  


fn get_on_ground(position: Vec3, player_height: f32) -> bool {
    position.y - player_height / 2. <= 1.
}

fn get_gravity_offset(delta: f32, velocity: f32, position: Vec3, player_height: f32) -> f32 {
    if get_on_ground(position, player_height) {
        if is_key_down(KeyCode::Space) {
            return JUMP_HIGHT - GRAVITY * delta;
        }
        else {
            return 0.;
        }
    } else {
        velocity - GRAVITY * delta
    }
    
}


fn mouse_movement(mut yaw:  f32, mut pitch: f32, world_up: Vec3, delta: f32) -> (f32, f32, Vec3, Vec3, Vec3){
    let mouse_delta = mouse_delta_position();

    yaw += mouse_delta.x * delta * -LOOK_SPEED;
    pitch += mouse_delta.y * delta * LOOK_SPEED;

    pitch = if pitch > 1.5 { 1.5 } else { pitch };
    pitch = if pitch < -1.5 { -1.5 } else { pitch };

    let front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();

    let right = front.cross(world_up).normalize();
    let up = right.cross(front).normalize();

    (yaw, pitch, front, right, up)
}


fn draw(position: Vec3, front: Vec3, up: Vec3, textures: &Vec<Texture2D>) {
    //let rust_logo = &textures[0];
    let crosshair = &textures[1];
    let mat = &textures[2];

    clear_background(LIGHTGRAY);

    set_camera(&Camera3D {
        position: position,
        up: up,
        fovy:  1.9,
        target: position + front,
        ..Default::default()
    });

    //draw_grid(20, 1., BLACK, GRAY);
    draw_cube(Vec3::new(0., 0., 0.), vec3(20., 1., 20.,), Some(mat), WHITE);
    //draw_cube(Vec3::ZERO, Vec3::new(4., 4., 4.), Some(&rust_logo), WHITE);
    //draw_sphere(Vec3::ZERO, 4., Some(&rust_logo), WHITE);


    set_default_camera();

    draw_texture(crosshair, 1920. / 2. - crosshair.width() / 2., 1080. / 2.  - crosshair.height() / 2., WHITE);

    draw_text("cooles fufu game", 10.0, 20.0, 30.0, BLACK);

    draw_text(
        format!("FPS: {}", get_fps()).as_str(),
        10.0,
        48.0,
        30.0,
        BLACK,
    );
    
    draw_text(
        format!("FOV: {}", 1.9).as_str(),
        10.0,
        48.0 + 18.0,
        30.0,
        BLACK,
    );
}


fn init() -> (Vec3, Vec3, Vec3, f32, f32, Vec3) {
    let world_up = vec3(0.0, 1.0, 0.0);
    let yaw: f32 = 1.18;
    let pitch: f32 = 0.0;

    let front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();

    let right = front.cross(world_up).normalize();

    let position = vec3(0.0, 1.0, 0.0);

    set_cursor_grab(true);
    show_mouse(false);

    (position, front, right, yaw, pitch, world_up)
}


async fn load_textures() -> Vec<Texture2D> {
    let mut textures: Vec<Texture2D> = vec![];

    textures.push(load_texture("assets/rust.png").await.unwrap());
    textures.push(load_texture("assets/crosshair.png").await.unwrap());
    textures.push(load_texture("assets/material.png").await.unwrap());

    textures
}


#[macroquad::main(conf)]
async fn main() 
{
    let (
        mut position, 
        mut front, 
        mut right, 
        mut yaw, 
        mut pitch, 
        world_up) = init();
    let mut up;
    let textures = load_textures().await;
    let mut velocity = 0.;

    loop {
        let delta = get_frame_time();
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        position += get_offset(delta, front, right);
        (yaw, pitch, front, right, up) = mouse_movement(yaw, pitch, world_up, delta);
        
        velocity = get_gravity_offset(delta, velocity, position, 6.);
        position.y += velocity;

        draw(position, front, up, &textures);
        next_frame().await
    }
}
