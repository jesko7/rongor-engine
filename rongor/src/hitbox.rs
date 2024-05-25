use crate::vec::*;

pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back
}

#[derive(Clone, Debug)]
pub struct SquareHitbox {
    pub position: Vec2,
    pub size: Vec2
}

impl SquareHitbox {
    pub fn new(position: Vec2, size: Vec2) -> SquareHitbox {
        SquareHitbox {
            position,
            size
        }
    }

    pub fn get_min_max(&self) -> ((f32, f32), (f32, f32)) {
        ((self.position.x - self.size.x / 2.,
          self.position.y - self.size.y / 2.),
  
         (self.position.x + self.size.x / 2.,
          self.position.y + self.size.y / 2.))
    }

    pub fn collision_square(&self, ohter: SquareHitbox) -> bool {
        let (min, max) = self.get_min_max();
        let (other_min, other_max) = ohter.get_min_max();

        if max.0 < other_min.0 || min.0 > other_max.0 {
            return false;
        }
        if max.1 < other_min.1 || min.1 > other_max.1 {
            return false;
        }
        true
    }

    pub fn collision_point(&self, other: Vec2) -> bool {
        let (min, max) = self.get_min_max();

        println!("{:?}, {:?},  {:?}, {:?}", min, max, other, self.position);

        if min.0 > other.x || max.0 < other.x {
            return false;
        }
        if min.1 > other.y || max.1 < other.y {
            return false;
        }
        true
    }
}
#[derive(Clone, Debug)]
pub struct CubeHitbox {
    pub position: Vec3,
    pub size: Vec3
}

impl CubeHitbox {
    pub fn new(position: Vec3, size: Vec3) -> CubeHitbox {
        CubeHitbox {
            position,
            size
        }
    }

    pub fn get_min_max(&self) -> ((f32, f32, f32), (f32, f32, f32)) {
        ((self.position.x - self.size.x / 2.,
          self.position.y - self.size.y / 2.,
          self.position.z - self.size.z / 2.),

        ( self.position.x + self.size.x / 2.,
          self.position.y + self.size.y / 2.,
          self.position.z + self.size.z / 2.))
    }

    pub fn collision_cube(&self, ohter: CubeHitbox) -> bool {
        let (min, max) = self.get_min_max();
        let (other_min, other_max) = ohter.get_min_max();
        if max.0 < other_min.0 || min.0 > other_max.0 {
            return false;
        }
        if max.1 < other_min.1 || min.1 > other_max.1 {
            return false;
        }
        if max.2 < other_min.2 || min.2 > other_max.2 {
            return false;
        }
        true
    }

    pub fn get_collision_quads(&self, direction: Direction) -> QuadHitbox {
        match direction {
            Direction::Top => QuadHitbox::new(self.position + Vec3::new(0., self.size.y / 2., 0.), Vec2::new(self.size.x, self.size.z), QuadHitboxAxis::Y),
            Direction::Bottom => QuadHitbox::new(self.position - Vec3::new(0., self.size.y / 2., 0.), Vec2::new(self.size.x, self.size.z), QuadHitboxAxis::Y),
            Direction::Right => QuadHitbox::new(self.position + Vec3::new(self.size.x / 2., 0., 0.), Vec2::new(self.size.z, self.size.y), QuadHitboxAxis::Z),
            Direction::Left =>  QuadHitbox::new(self.position - Vec3::new(self.size.x / 2., 0., 0.), Vec2::new(self.size.z, self.size.y), QuadHitboxAxis::Z),
            Direction::Back => QuadHitbox::new(self.position + Vec3::new(0., 0., self.size.z / 2.), Vec2::new(self.size.x, self.size.y), QuadHitboxAxis::X),
            Direction::Front => QuadHitbox::new(self.position + Vec3::new(0., 0., self.size.z / 2.), Vec2::new(self.size.x, self.size.y), QuadHitboxAxis::X),
        }
    }

    pub fn collision_point(&self, other: Vec3) -> bool {
        let (min, max) = self.get_min_max();

        if min.0 > other.x || max.0 < other.x {
            return false;
        }
        if min.1 > other.y || max.1 < other.y {
            return false;
        }
        if min.2 > other.z || max.2 < other.z {
            return false;
        }

        true
    }
}

#[derive(Clone, Debug)]
pub enum QuadHitboxAxis {
    X,
    Y,
    Z
}

#[derive(Clone, Debug)]
pub struct QuadHitbox {
    pub position: Vec3,
    pub size: Vec2,
    pub axis: QuadHitboxAxis
}

impl QuadHitbox {
    pub fn new(position: Vec3, size: Vec2, axis: QuadHitboxAxis) -> QuadHitbox {
        QuadHitbox {
            position,
            size,
            axis
        }
    }

    pub fn collision_cube(&self, ohter: CubeHitbox) -> bool {
        let size = match self.axis {
            QuadHitboxAxis::X => Vec3::new(0., self.size.x, self.size.y),
            QuadHitboxAxis::Y => Vec3::new(self.size.x, 0., self.size.y),
            QuadHitboxAxis::Z => Vec3::new(self.size.x, self.size.y, 0.)
        };
        CubeHitbox::new(self.position, size).collision_cube(ohter)
    }
}