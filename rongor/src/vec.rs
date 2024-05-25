use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Neg;
use macroquad::prelude;



#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, other: Vec2) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}


impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0., y: 0. };
    pub const UP: Vec2 = Vec2 { x: 0., y: 1. };
    pub const DOWN: Vec2 = Vec2 { x: 0., y: -1. };
    pub const LEFT: Vec2 = Vec2 { x: -1., y: 0. };
    pub const RIGHT: Vec2 = Vec2 { x: 1., y: 0. };

    pub fn new(x: f32, y: f32) -> Self
    {
        Vec2 {
            x: x.into(), 
            y: y.into()
        }
    }
    pub fn newi(x: i32, y: i32) -> Self
    {
        Vec2 {
            x: x as f32, 
            y: y as f32
        }
    }
    pub fn newu(x: u32, y: u32) -> Self
    {
        Vec2 {
            x: x as f32, 
            y: y as f32
        }
    }
    pub fn newf64(x: f64, y: f64) -> Self
    {
        Vec2 {
            x: x as f32, 
            y: y as f32
        }
    }
    pub fn newi64(x: i64, y: i64) -> Self
    {
        Vec2 {
            x: x as f32, 
            y: y as f32
        }
    }
    pub fn newu64(x: u64, y: u64) -> Self
    {
        Vec2 {
            x: x as f32, 
            y: y as f32
        }
    }
    pub fn newi16(x: i16, y: i16) -> Self
    {
        Vec2 {
            x: x as f32, 
            y: y as f32
        }
    }
    pub fn newu16(x: u16, y: u16) -> Self
    {
        Vec2 {
            x: x as f32, 
            y: y as f32
        }
    }
    pub fn dist(&self, &other: &Vec2) -> f32 {
        (((other.x - self.x).powf(2.) + (other.y - self.y).powf(2.)) as f64).sqrt() as f32
    }
    pub fn length(&self) -> f32 {
        self.dist(&Vec2::new(0., 0.))
    }
    pub fn normalize(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
    }
    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
    pub fn to_array_u32(&self) -> [u32; 2] {
        [self.x as u32, self.y as u32]
    }
    pub fn to_array_f64(&self) -> [f64; 2] {
        [self.x as f64, self.y as f64]
    }
    pub fn to_array_f64_3(&self) -> [f64; 3] {
        [self.x as f64, self.y as f64, 0.]
    }
    pub fn to_macro_vec2(&self) -> prelude::Vec2 {
        prelude::Vec2::new(self.x, self.y)
    }
    pub fn scale(&self, other: f32) -> Vec2 {
        Vec2::new(self.x * other, self.y * other)
    }
}



impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}



#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0., y: 0., z: 0.};
    pub const UP: Vec3 = Vec3 { x: 0., y: 1., z: 0. };
    pub const DOWN: Vec3 = Vec3 { x: 0., y: -1., z: 0. };
    pub const LEFT: Vec3 = Vec3 { x: -1., y: 0., z: 0. };
    pub const RIGHT: Vec3 = Vec3 { x: 1., y: 0., z: 0. };
    pub const FRONT: Vec3 = Vec3 { x: 0., y: 0., z: -1. };
    pub const BACK: Vec3 = Vec3 { x: 0., y: 0., z: 1. };

    pub fn new(x: f32, y: f32, z: f32) -> Self
    {
        Vec3 {
            x: x.into(), 
            y: y.into(),
            z: z.into()
        }
    }
    pub fn newi(x: i32, y: i32, z: i32) -> Self
    {
        Vec3 {
            x: x as f32, 
            y: y as f32,
            z: z as f32
        }
    }
    pub fn newu(x: u32, y: u32, z: u32) -> Self
    {
        Vec3 {
            x: x as f32, 
            y: y as f32,
            z: z as f32
        }
    }
    pub fn newf64(x: f64, y: f64, z: f64) -> Self
    {
        Vec3 {
            x: x as f32, 
            y: y as f32,
            z: z as f32
        }
    }
    pub fn newi64(x: i64, y: i64, z: i64) -> Self
    {
        Vec3 {
            x: x as f32, 
            y: y as f32,
            z: z as f32
        }
    }
    pub fn newu64(x: u64, y: u64, z: u64) -> Self
    {
        Vec3 {
            x: x as f32, 
            y: y as f32,
            z: z as f32
        }
    }
    pub fn newi16(x: i16, y: i16, z: i16) -> Self
    {
        Vec3 {
            x: x as f32, 
            y: y as f32,
            z: z as f32
        }
    }
    pub fn newu16(x: u16, y: u16, z:u16) -> Self
    {
        Vec3 {
            x: x as f32, 
            y: y as f32,
            z: z as f32
        }
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let macro_vec = self.to_macro_vec3().cross(other.to_macro_vec3());
        Vec3::new(macro_vec.x, macro_vec.y, macro_vec.z)
    }
    pub fn dist(&self, &other: &Vec3) -> f32 {
        (((other.x - self.x).powf(2.) + (other.y - self.y).powf(2.) + (other.z - self.z).powf(2.)) as f64).sqrt() as f32
    }
    pub fn length(&self) -> f32 {
        self.dist(&Vec3::new(0., 0., 0.))
    }
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.to_macro_vec3().dot(other.to_macro_vec3())
    }
    pub fn normalize(&mut self) {
        let len = self.length();
        if self.x != 0. {
            self.x /= len;
        }
        if self.y != 0. {
            self.y /= len;
        }
        if self.z != 0. {
            self.z /= len;
        }
    }
    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    pub fn to_array_u32(&self) -> [u32; 3] {
        [self.x as u32, self.y as u32, self.z as u32]
    }
    pub fn to_array_f64(&self) -> [f64; 3] {
        [self.x as f64, self.y as f64, self.z as f64]
    }
    pub fn to_array_f64_3(&self) -> [f64; 3] {
        [self.x as f64, self.y as f64, self.z as f64]
    }
    pub fn to_macro_vec3(&self) -> prelude::Vec3 {
        prelude::Vec3::new(self.x, self.y, self.z)
    }
    pub fn scale(&self, other: f32) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}



impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}