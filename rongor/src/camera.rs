use crate::vec::*;
use macroquad::{math::Rect, prelude, window::{screen_height, screen_width}};
use std::f32::consts::PI;
#[derive(Clone, Debug)]
pub struct Camera2d {
    pub rotation_degrees: f32,
    pub position: Vec2,
    pub zoom: Vec2
}

impl Camera2d {
    pub fn default() -> Camera2d {
        Camera2d {
            rotation_degrees: 0.,
            position: Vec2::new(0., 0.),
            zoom: Vec2::new(1., 1.)
        }
    }



    pub fn to_macro_cam(&self) -> prelude::Camera2D {

        let mut cam = prelude::Camera2D::from_display_rect(Rect::new(0., prelude::screen_height(), screen_width(), -screen_height()));

        cam.rotation = self.rotation_degrees;
        cam.offset = self.position.to_macro_vec2();

        cam.zoom = cam.zoom * self.zoom.to_macro_vec2();

        cam
    }

    pub fn set_as_current_cam(&self) {
        prelude::set_camera(&self.to_macro_cam());
    }
}






#[derive(Clone, Debug)]
pub struct Camera3d {
    pub position: Vec3,
    pub target: Vec3,
    pub rotation: Vec2,
    pub up: Vec3,
    pub fovy: f32,
    pub aspect: Option<f32>
}

impl Camera3d {
    pub fn default() -> Camera3d {
        Camera3d {
            position: Vec3::new(0., 0., 0.),
            target: Vec3::new(0., 0., 1.),
            rotation: Vec2::new(0., 0.),
            up: Vec3::new(0., 1., 0.),
            fovy: 1.4,
            aspect: None
        }
    }

    pub fn to_macro_cam(&self) -> prelude::Camera3D {
        prelude::Camera3D {
            position: self.position.to_macro_vec3(),
            target: self.target.to_macro_vec3(),
            up: self.up.to_macro_vec3(),
            fovy: self.fovy,
            aspect: self.aspect,
            ..Default::default()
        }
    }







    pub fn get_front_rotation(&self) -> Vec3 {
        let yaw = self.rotation.x;
        let pitch = self.rotation.y;
        let mut front = Vec3::new(
        yaw.sin() * pitch.cos(),
        pitch.sin(),
        yaw.cos() * pitch.cos());
        front.normalize();
        front
    }
    pub fn get_target_rotation(&self) -> Vec3 {
        self.get_front_rotation() + self.position
    }
    pub fn set_target_rotation(&mut self)  {
        self.target = self.get_target_rotation();
    }







    
    pub fn get_front(&self) -> Vec3 {
        let mut front = Vec3::new(self.target.x - self.position.x, self.target.y - self.position.y, self.target.z - self.position.z);
        front.normalize();
        front
    }
    pub fn get_back(&self) -> Vec3 {
        -self.get_front()
    }
    pub fn get_right(&self) -> Vec3 {
        self.get_front().cross(&Vec3::new(0., 1., 0.))
    }
    pub fn get_left(&self) -> Vec3 {
        -self.get_right()
    }
    pub fn get_up(&self) -> Vec3 {
        self.get_right().cross(&self.get_front())
    }
    pub fn get_down(&self) -> Vec3 {
        -self.get_up()
    }
    pub fn update_up(&mut  self) {
        self.up = self.get_up();
    }




    pub fn update_all_rotation(&mut self) { 
        self.set_target_rotation();
        self.update_up();
        self.set_as_current_cam();
    }

    pub fn look_at(&mut self, target: Vec3) {
        self.target = target;
    }

    pub fn point_visible_from_camera(&self, point: Vec3, render_distance: f32, front: Option<Vec3>) -> bool {
        let distancex = (self.position.x - point.x).abs();
        let distancez = (self.position.z - point.z).abs();

        if distancex + distancez > render_distance * 16. {
            return false;
        }
        
        let mut dir_to_self: Vec3 = self.position - point;
        dir_to_self.normalize();

        let frnt;
        if front.is_none() {
            frnt = self.get_front();
        } else {
            frnt = front.unwrap();
        }

        let dir = dir_to_self.dot(&frnt).acos();

        if dir.is_nan() {
            return false;
        }
        if dir < PI - self.fovy + 0.8 {
            return false;
        }

        true
    }

    pub fn update_all(&mut self) {
        self.update_up();
        self.set_as_current_cam();
    }


    pub fn set_as_current_cam(&self) {
        prelude::set_camera(&self.to_macro_cam());
    }
}