use macroquad::prelude::*;
use ndarray::arr2;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Mincroft"),
        fullscreen: true,
        ..Default::default()
    }
}

fn matrix_to_vec3(matrix: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>> ) -> Vec3{
    let mut elements = matrix.columns().into_iter().next().unwrap().into_iter().collect::<Vec<_>>();
    Vec3::new(*elements[0], *elements[1], *elements[2])
}


fn vec3_to_matrix(vector: Vec3) -> ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>> {
    arr2(&[[vector.to_array()[0]], 
               [vector.to_array()[1]], 
               [vector.to_array()[2]]])
}

fn rotate_point(point: Vec3, rotation: Vec3) -> Vec3 {
    println!("{:?}", point);


    let x_rot = &[[1., 0.,                         0.                        ],
                                  [0., f32::cos(rotation.x), -f32::sin(rotation.x)],
                                  [0., f32::sin(rotation.x), f32::cos(rotation.x)]];

    let y_rot = &[[f32::cos(rotation.y), 0.,f32::sin(rotation.y)  ],
                                  [0.,                         1.,                         0.],
                                  [-f32::sin(rotation.y),0., f32::cos(rotation.y)]];

    let z_rot = &[[f32::cos(rotation.z), -f32::sin(rotation.z), 0.],
                                  [f32::sin(rotation.z), f32::cos(rotation.z),  0.],
                                  [0.,                         0.,                         1.]];

                                  println!("{:?}, {:?}, {:?}", x_rot, y_rot, z_rot);

    let x_rot = &arr2(x_rot);
    let y_rot = &arr2(y_rot);
    let z_rot = &arr2(z_rot);

    let point = vec3_to_matrix(point);
    let point = x_rot.dot(&point);
    let point = y_rot.dot(&point);
    let point = z_rot.dot(&point);

   
    let point =  matrix_to_vec3(point);

    println!("{:?}, {:?}", point, rotation);
    println!("");
    point
}
    


struct Camera {
    position: Vec3,
    rotation: Vec3,
    focal_length: f32,
}


impl Camera {
    fn new(position: Vec3, rotation: Vec3, focal_length: f32) -> Camera {
        Camera {
            position,
            rotation,
            focal_length,
        }
    }

    fn project_point(&self, point: Vec3) -> Vec3 {

        let mut point = point.clone();

        point += self.position;
        point = rotate_point(point, self.rotation);

        let x_projected = (self.focal_length * point.x) / (self.focal_length + point.z);
        let y_projected = (self.focal_length * point.y) / (self.focal_length + point.z);
        
        Vec3::new(x_projected, y_projected, point.z)
    }
}

struct Triangle3d {
    v1: Vec2,
    v2: Vec2,
    v3: Vec2,
    color: Color
}

impl Triangle3d {
    fn new(v1: (f32, f32), v2: (f32, f32), v3: (f32, f32), color: Color) -> Triangle3d {
        Triangle3d {
            v1: Vec2::new(v1.0, v1.1),
            v2: Vec2::new(v2.0, v2.1),
            v3: Vec2::new(v3.0, v3.1),
            color
        }
    }

    fn draw() {

    }
}



#[macroquad::main(conf)]
async fn main() {
    let (p1, p2, p3) = (Vec3::new(1., 1., 1.),
                                          Vec3::new(1.5, 1., 1.),
                                          Vec3::new(1., 1.5, 1.));

    let movement_speed = 1.;

    let mut camera = Camera::new(Vec3::new(0., 0., 0.), Vec3::ZERO, 1.);

    set_cursor_grab(true);
    show_mouse(false);

    loop {
        let delta = get_frame_time();

        clear_background(RED);

        camera.rotation += -mouse_delta_position().extend(0.);

        let (pp1, pp2, pp3) = (camera.project_point(p1).xy() * 1000., camera.project_point(p2).xy() * 1000., camera.project_point(p3).xy() * 1000.);

        if is_key_down(KeyCode::A) {
            camera.position += Vec3::X * movement_speed * delta;
        }
        if is_key_down(KeyCode::D) {
            camera.position += -Vec3::X * movement_speed * delta;
        }
        if is_key_down(KeyCode::W) {
            camera.position += -Vec3::Z * movement_speed * delta;
        }
        if is_key_down(KeyCode::S) {
            camera.position += Vec3::Z * movement_speed * delta;
        }
        if is_key_down(KeyCode::Space) {
            camera.position += Vec3::Y * movement_speed * delta;
        }
        if is_key_down(KeyCode::LeftShift) {
            camera.position += -Vec3::Y * movement_speed * delta;
        }
        if is_key_down(KeyCode::Escape) {
            break;
        }

        draw_triangle(pp1, pp2, pp3, BLUE);

        next_frame().await
    }
}