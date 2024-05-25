use macroquad::prelude::*;
use ndarray::arr2;

fn get_rotation_matrix(rotation: Vec2) -> [[f32; 3]; 3] {
    [[0., 0., 0.],
     [0., 0., 0.],
     [0., 0., 0.]]
}


struct Camera {
    position: Vec3,
    rotation: Vec2,
    focal_length: f32,
    far: f32,
    fov: Vec2
}


impl Camera {
    fn new(position: Vec3,rotation: Vec2,focal_length: f32,far: f32,fov: Vec2) -> Camera {
        Camera {
            position,
            rotation,
            focal_length,
            far,
            fov 
        }
    }

    fn get_orthographic_matrix(&self) -> [[f32; 4]; 4] {
        let f = self.far + self.position.z;
        let n = self.focal_length + self.position.z;
        let r = self.fov.x + self.position.x;
        let l = -self.fov.x + self.position.x;
        let t = self.fov.y + self.position.y;
        let b = -self.fov.y + self.position.y;

        println!("{}, {}, {}, {}, {}, {}", f, n, r, l, t, b);

        [[2./(r-l), 0.,       0.,        -((r+l)/(r-l))],
         [0.,       2./(b-t), 0.,        -((b+t)/(b-t))],
         [0.,       0.,       -2./(f-n), -((f+n)/(f-n))],
         [0.,       0.,       0.,        1.            ]]
    }
    
    fn get_perspective_matrix(&self) -> [[f32; 4]; 4] {
        let f = self.far + self.position.z;
        let n = self.focal_length + self.position.z;
        [[n,  0., 0.,  0.],
         [0., n,  0.,  0.],
         [0., 0., f+n, -(f*n)],
         [0., 0., 1.,  0.]]
    }
    fn get_camera_matrix(&self) -> ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>> {
        let om: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>> = arr2(&self.get_orthographic_matrix());
        let pm: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>> = arr2(&self.get_perspective_matrix());
        let cm: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>> = om.dot(&pm);
        cm
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


fn matrix_to_vec3(matrix: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>> ) -> Vec3{
    let mut elements = matrix.columns().into_iter().next().unwrap().into_iter().collect::<Vec<_>>();
    Vec3::new(elements[0] / elements[3], elements[1] / elements[3], elements[2] / elements[3])
}


fn vec3_to_matrix(vector: Vec3) -> ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>> {
    arr2(&[[vector.to_array()[0]], 
               [vector.to_array()[1]], 
               [vector.to_array()[2]],
               [1.]])
}

#[macroquad::main("3d engine")]
async fn main() {
    let (mut p1, mut p2, mut p3) = (Vec3::new(1., 1., 1.),
                                                      Vec3::new(1.5, 1., 1.),
                                                      Vec3::new(1., 1.5, 1.));

    let camera = Camera::new(Vec3::new(0., 10., 0.), Vec2::ZERO, 0.1, 100., Vec2::new(1., 1.));

    let om = camera.get_orthographic_matrix();
    
    for row  in om {
        println!("{:?}", row);
    }

    //let  mat1 = arr2(&[[2.],[4.],[6.],[2.]]);
    //println!("{}", matrix_to_vec3(mat1));


    loop {
        clear_background(RED);
        let projection_matrix: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::prelude::Dim<[usize; 2]>> = camera.get_camera_matrix();

        let (pp1, pp2, pp3) = (projection_matrix.dot(&vec3_to_matrix(p1)), projection_matrix.dot(&vec3_to_matrix(p2)), projection_matrix.dot(&vec3_to_matrix(p3)));
        let (pp1, pp2, pp3) = (matrix_to_vec3(pp1) * 1000., matrix_to_vec3(pp2) * 1000., matrix_to_vec3(pp3) * 1000.);
        println!("{:?}, {:?}, {:?}", pp1.xy(), pp2.xy(), pp3.xy());
        draw_triangle(pp1.xy(), pp2.xy(), pp3.xy(), BLUE);

        next_frame().await
    }
}