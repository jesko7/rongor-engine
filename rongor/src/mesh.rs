use macroquad::material;
use macroquad::math::Vec2;
use macroquad::prelude;
use crate::vec::*;
use crate::image::*;
use crate::math::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use tobj;

#[derive(Clone, Debug)]
pub struct Uv {
    pub u: f32,
    pub v: f32
}


impl Uv {
    pub fn new(u: f32, v: f32) -> Uv {
        Uv {
            u,
            v
        }
    }
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub uv: Uv 
}

impl Vertex {
    pub fn new(position: Vec3, uv: Uv) -> Vertex {
        Vertex { position, uv }
    }

    fn to_macroquad_vertex(&self) -> macroquad::models::Vertex {
        macroquad::models::Vertex {
            position: self.position.to_macro_vec3(),
            uv: Vec2::new(self.uv.u, self.uv.v),
            color: prelude::WHITE
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertecies: Vec<Vertex>,
    pub triangle_indecies: Vec<(u16, u16, u16)>, 
    pub texture: Image,

    pub position: Vec3,
    pub size: Vec3,
    pub rotation: Vec3,
    pub pivot: Option<Vec3>,
}

impl Mesh {
    pub fn new(vertecies: Vec<Vertex>, triangle_indecies: Vec<(u16, u16, u16)>, texture: Image, position: Vec3, size: Vec3, rotation: Vec3, pivot: Option<Vec3>) -> Self {
        Mesh { vertecies, triangle_indecies, texture, position, size, rotation, pivot }
    }

    pub fn load_obj(obj_path: &str, texture: Image, position: Vec3, size: Vec3, rotation: Vec3, pivot: Option<Vec3>) -> Mesh {
        let (models, materials) = tobj::load_obj(obj_path, true).expect("couldnt load mesh");
        let mesh = models[0].mesh.clone();
        let material = materials[0].clone();
        let positions = mesh.positions;
        let texcord = mesh.texcoords;
        let mesh_indecies = mesh.indices;

        let mut triangle_indecies: Vec<(u16, u16, u16)> = vec![];
        let mut vertecies = vec![];


        for i in 0..positions.len() / 3 {
            let vertex = Vertex::new(Vec3::new(positions[i * 3], positions[i * 2 + 1], positions[i * 3 + 2]), Uv::new(texcord[((i / 3) as f32).floor() as usize * 2 - 1], texcord[i  / 3 * 2]));
            vertecies.push(vertex);
            if i + 2 < mesh_indecies.len() {
                triangle_indecies.push((mesh_indecies[i * 3] as u16, mesh_indecies[i * 3 + 1] as u16, mesh_indecies[i * 3 + 2] as u16));
            }
           
        }

        Mesh {
            vertecies,
            triangle_indecies,
            texture,
            position,
            size,
            rotation,
            pivot
        }
    }

    pub fn __new_from_path(path: &str, texture: Image, position: Vec3, size: Vec3, rotation: Vec3, pivot: Option<Vec3>) -> Self {
        let mut vertex_positions = vec![];
        let mut vertex_uvs = vec![];
        let mut triangle_indecies_positions = vec![];
        let mut triangle_indecies_uvs = vec![];

        let file = File::open(path);

        if file.is_err() {
            panic!("couldnt load file with path: {}", path)
        }

        let file = file.unwrap();

        let reader = BufReader::new(file);
    
        for (x, line) in reader.lines().enumerate() {
            if line.is_err() {
                panic!("couldnt read lines of file with path: {}", path)
            }

            let mut line = line.unwrap();
            let _ = line.split_off(line.find('#').unwrap_or(line.len()));
            line = line.trim().to_string();

            if line.len() == 0 {
                continue;
            }

            let mut first_key_word = line.trim().to_string();
            let index = first_key_word.find(' ');
            if index.is_none() {
                panic!("syntax error in file {path} in line {x}:
                {line}")
            }
            let index = index.unwrap();
            let contense = first_key_word.split_off(index).trim().to_string();

            if first_key_word == "mtllib".to_string() {
                continue;
            }
            if first_key_word == "o".to_string() {
                continue;
            }
            if first_key_word == "g".to_string() {
                continue;
            }
            if first_key_word == "s".to_string() {
                continue;
            }
            if first_key_word == "vn".to_string() {
                continue;
            }
            if first_key_word == "v".to_string() {
                let xyz = contense.split(" ");
                let mut xyz_f32 = vec![];
                for coordinate in xyz {
                    if coordinate.trim() == "" {
                        continue;
                    }
                    
                    let parse = coordinate.trim().parse::<f32>();

                    if parse.is_err() {
                        panic!("syntax error in file {path} in line {x}:
                        {line}")
                    }

                    xyz_f32.push(parse.unwrap() as f32)
                }
                vertex_positions.push((xyz_f32[0], xyz_f32[1], xyz_f32[2]));
            }
            if first_key_word == "vt".to_string() {
                let xyz = contense.split(" ");
                let mut xyz_f32 = vec![];
                for coordinate in xyz {
                    let parse = coordinate.trim().parse::<f32>();

                    if parse.is_err() {
                        panic!("syntax error in file {path} in line {x}:
                        {line}")
                    }

                    xyz_f32.push(parse.unwrap() as f32)
                }

                vertex_uvs.push(Uv::new(xyz_f32[0], xyz_f32[1]));
            }
            if first_key_word == "f".to_string() {
                let indecies = contense.split(" ");
                
                for index in indecies {
                    let mut pos_uv = index.split("/");

                    let pos = pos_uv.next();
                    if pos.is_none() {
                        panic!("position index error in file {path} in line {x}:
                        {line}
                        position: {:?}", pos)
                    }
                    triangle_indecies_positions.push(pos.unwrap().trim().parse::<i32>().unwrap() as i32);

                    let uv = pos_uv.next();
                    if uv.is_none() {
                        panic!("uv index error in file {path} in line {x}:
                        {line}
                        uv: {:?}", uv)
                    }
                    triangle_indecies_uvs.push(uv.unwrap().trim().parse::<i32>().unwrap() as i32)
                }
            } 
        }

        let mut vertecies = vec![];

        let mut triangle_indecies = vec![];

        for x in (0..triangle_indecies_positions.len()).step_by(3) {
            triangle_indecies.push((x as u16, x  as u16 + 1, x  as u16 + 2))
        }

        for (index_pos, index_uv) in zip(triangle_indecies_positions.clone(), triangle_indecies_uvs.clone()) {
            let vertex = Vertex::new(
                Vec3::new(
                    vertex_positions[index_pos as usize - 1].0, 
                    vertex_positions[index_pos as usize - 1].1, 
                    vertex_positions[index_pos as usize - 1].2), 
                    vertex_uvs[index_uv as usize - 1].clone());
            
            

            vertecies.push(vertex);
        }

        Mesh { vertecies, triangle_indecies, texture, position, size, rotation, pivot }
    }

    pub fn to_macroquad_mesh(&self) -> prelude::Mesh {
        let mut macroquad_vertecies = vec![];

        for vertex in self.vertecies.iter() {
            let mut transformed_vertex = vertex.clone();
            transformed_vertex.position = transformed_vertex.position * self.size;
            transformed_vertex.position = transformed_vertex.position + self.position;
            transformed_vertex.position = math::rotate3d(transformed_vertex.position, self.pivot, self.rotation);
            macroquad_vertecies.push(transformed_vertex.to_macroquad_vertex());
        }

        let mut macroquad_triangle_indecies = vec![];

        for triangle_index in self.triangle_indecies.iter() {
            macroquad_triangle_indecies.push(triangle_index.0);
            macroquad_triangle_indecies.push(triangle_index.1);
            macroquad_triangle_indecies.push(triangle_index.2);
        }

        prelude::Mesh {
            vertices: macroquad_vertecies,
            indices: macroquad_triangle_indecies,
            texture: Some(self.texture.___macroquad_texture.clone())
        }
    }

    pub fn draw(&self) {
        prelude::draw_mesh(&self.to_macroquad_mesh())
    }
}