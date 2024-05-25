use std::f32::consts::PI;
use rongor::prelude::*;
use std::collections::HashMap;

static MOVEMENT_SPEED: f32 = 4.5;
static MOVEMENT_INCREASE: f32 = 1.1;
static MOVEMENT_STRAFE: f32 = 0.3;
static LOOK_SPEED: f32 = 0.8;
static JUMP_HEIGHT: f32 = 4.;
static GRAVITY: f32 = 10.;
static RENDER_DISTANCE: f32 = 2.;


async fn init() -> (Vec<Image>, Camera2d, Camera3d, Player, Vec<Boden>) {
    mouse::set_grab(true);
    mouse::set_visivle(false);

    let mut textures = vec![];
    let rust_logo = Image::new("assets/rust.png", 0., false, false, Vec2::new(100., 100.), None).await;
    textures.push(rust_logo.clone());


    let default_cam = Camera2d::default();
    let cam: Camera3d = Camera3d::default();
    


    let player = Player {
        position: Vec3::new(0., 0., 0.),
        size: Vec3::new(1., 2., 1.),
        velocity: 0.
    };


    let bigness = 100;

    let mut xz_ys: HashMap<(i32, i32), f32> = HashMap::new();

    let mut böden = vec![];

    for x in -bigness..=bigness {
        for z in -bigness..=bigness {
            let y = math::simplex_noise((x + bigness) as f32, (z + bigness) as f32, 1, 0.05, 0.05, 5., 234563456).ceil();
            
            xz_ys.insert((x, z), y);
        }
    }

    for x in -bigness..=bigness {
        for z in -bigness..=bigness {
            let y = xz_ys.get(&(x, z)).unwrap();

            let top_y = xz_ys.get(&(x, z - 1)).unwrap_or(y);
            let bottom_y = xz_ys.get(&(x, z + 1)).unwrap_or(y);
            let right_y = xz_ys.get(&(x + 1, z)).unwrap_or(y);
            let left_y = xz_ys.get(&(x - 1, z)).unwrap_or(y);

            let diff1 = (y - top_y).clamp(0., f32::MAX);
            let diff2 = (y - bottom_y).clamp(0., f32::MAX);
            let diff3 = (y - right_y).clamp(0., f32::MAX);
            let diff4 = (y - left_y).clamp(0., f32::MAX);

            let max_diff = diff1.max(diff2).max(diff3).max(diff4) as i32;

            for new_y in *y as i32 - max_diff..*y as i32 + 1 {
                böden.push(Boden::new(Vec3::new(x as f32 - 0.5, new_y as f32 - 10., z as f32 - 0.5), Vec3::new(1., 1., 1.), rust_logo.clone()));
            }
        }
    }



    (textures, default_cam, cam, player, böden)
}   


struct Boden {
    position: Vec3,
    size: Vec3,
    texture: Image
}

impl Boden {
    fn new(position: Vec3, size: Vec3, texture: Image) -> Boden {
        Boden {
            position,
            size,
            texture
        }
    }

    fn update(&self, cam3d: &mut Camera3d, front: Vec3) {
        let distancex = (cam3d.position.x - self.position.x).abs();
        let distancez = (cam3d.position.z - self.position.z).abs();

        if distancex + distancez > RENDER_DISTANCE * 16. {
            return;
        }
        
        let mut dir_to_self: Vec3 = cam3d.position - self.position;
        dir_to_self.normalize();

        let dir = dir_to_self.dot(&front).acos();

        if dir.is_nan() {
            return;
        }
        if dir < PI - cam3d.fovy  {
            return;
        }

        draw3d::cube::cube_center(self.position, self.size, Some(&self.texture));
    }

    fn get_hitbox(&self) -> CubeHitbox {
        CubeHitbox::new(self.position, self.size)
    }
}

struct Player {
    position: Vec3,
    size: Vec3,
    velocity: f32
}


impl Player {
    fn update(&mut self, cam3d: &mut Camera3d, delta: f32, böden: &Vec<Boden>) {        
        self.look(cam3d);
        let touching = self.gravity(delta, böden);
        self.movement(delta, cam3d, touching);

        cam3d.position = self.position;
        cam3d.update_all_rotation();
    }

    fn movement(&mut self, delta: f32, cam3d: &Camera3d, touching: bool) {
        let mut offset = Vec3::new(0., 0., 0.);
        if key::is_pressed("w") {
            let mut front = cam3d.get_front();
            front.y = 0.;
            front.normalize();
            offset = offset + front;
        }
        if key::is_pressed("a") {
            let mut left = cam3d.get_left();
            left.y = 0.;
            left.normalize();
            offset = offset + left;
        }
        if key::is_pressed("s") {
            let mut back = cam3d.get_back();
            back.y = 0.;
            back.normalize();
            offset = offset + back;
        }
        if key::is_pressed("d") {
            let mut right = cam3d.get_right();
            right.y = 0.;
            right.normalize();
            offset = offset + right;
        }

        
        offset.normalize();

        if (key::is_pressed("a") ^ key::is_pressed("d")) && key::is_pressed("w") {
            let mut front = cam3d.get_front();
            front.y = 0.;
            front.normalize();
            offset = offset + front.scale(MOVEMENT_STRAFE);
        }

        offset = offset.scale(delta).scale(MOVEMENT_SPEED);

        if !touching {
            offset = offset.scale(MOVEMENT_INCREASE);
        }

        self.position = self.position + offset;
    }

    fn gravity(&mut self, delta: f32, böden: &Vec<Boden>) -> bool{
        let mut touching = false;
        let mut böden_touching = vec![];


        for boden in böden {
            let distancex = (boden.position.x - self.position.x).abs();
            let distancey = (boden.position.y - self.position.y).abs();
            let distancez = (boden.position.z - self.position.z).abs();
            if distancex + distancey + distancez > 5. {
                continue;
            }
            if self.get_hitbox().collision_cube(boden.get_hitbox()) {
                touching = true;
                böden_touching.push(boden);
                break;
            }
        }
        if touching {
            if key::is_pressed("space") {
                self.velocity = -JUMP_HEIGHT;
                touching = false;
            } else {
                self.position.y = böden_touching[0].position.y + böden_touching[0].size.y / 2. + self.size.y / 2.;
            }
        }
        if !touching {
            self.position.y -= self.velocity * delta;
            self.velocity += GRAVITY * delta;
        }
        touching
    }

    fn look(&self, cam3d: &mut Camera3d) {
        cam3d.rotation = cam3d.rotation + mouse::get_delta_position().scale(LOOK_SPEED);
        if cam3d.rotation.y < -PI / 2. + 0.001  {
            cam3d.rotation.y = -PI / 2. + 0.001;
        }
        if cam3d.rotation.y > PI / 2. - 0.001 {
            cam3d.rotation.y = PI / 2. - 0.001;
        }

    }


    fn get_hitbox(&self) -> CubeHitbox {
        CubeHitbox::new(self.position, self.size)
    }
}



async fn run(window: Window){   
    let (
        _textures, 
        default_cam, 
        mut cam3d, 
        mut player, 
        böden
    ) = init().await;
    


    while window.update_screen().await {
        let delta = window.get_delta_time();

        default_cam.set_as_current_cam();
        draw2d::clear(Color::new(0, 0, 0, 255));
        cam3d.set_as_current_cam();
        


        player.update(&mut cam3d, delta, &böden);

        let mut front = cam3d.get_front();
        front.normalize();


        for boden in &böden {
            boden.update(&mut cam3d, front);
        }

        println!("{}", window.get_fps());
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






















