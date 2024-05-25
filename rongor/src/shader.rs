use macroquad::{miniquad::ShaderSource, prelude};
use crate::vec::*;


pub fn use_default_shader() {
    prelude::gl_use_default_material()
}
#[derive(Clone, Debug)]
pub enum Uniform {
    Float1(String),
    Float2(String, String),
    Float3(String, String, String),
    Color(Vec3),
}

impl Uniform {
    fn uniform_type(&self) -> prelude::UniformType {
        match self {
            Uniform::Float1(_) =>  prelude::UniformType::Float1,
            Uniform::Float2(_, _) =>  prelude::UniformType::Float2,
            Uniform::Float3(_, _, _) =>  prelude::UniformType::Float3,
            Uniform::Color(_) =>  prelude::UniformType::Float3,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DepthTest {
    Never,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
    Always
}

impl DepthTest {
    fn to_macroquad_comparison(&self) -> prelude::Comparison {
        match self {
            DepthTest::Never => prelude::Comparison::Never,
            DepthTest::Less => prelude::Comparison::Less,
            DepthTest::LessOrEqual => prelude::Comparison::LessOrEqual,
            DepthTest::Greater => prelude::Comparison::Greater,
            DepthTest::GreaterOrEqual => prelude::Comparison::GreaterOrEqual,
            DepthTest::Equal => prelude::Comparison::Equal,
            DepthTest::NotEqual => prelude::Comparison::NotEqual,
            DepthTest::Always => prelude::Comparison::Always,
        }
    }
}
#[derive(Clone, Debug)]
pub enum CullFace {
    Front,
    Back,
    Nothing
}

impl CullFace {
    fn to_macroquad_cullface(&self) -> prelude::miniquad::graphics::CullFace {
        match self {
            CullFace::Front => prelude::miniquad::graphics::CullFace::Front,
            CullFace::Back => prelude::miniquad::graphics::CullFace::Back,
            CullFace::Nothing => prelude::miniquad::graphics::CullFace::Nothing,
        }
    }
}
#[derive(Clone, Debug)]
pub enum FrontFaceOder {
    Clockwise,
    CounterClockwise
}

impl FrontFaceOder {
    fn to_macroquad_face_oder(&self) -> macroquad::miniquad::FrontFaceOrder {
        match self {
            FrontFaceOder::Clockwise => macroquad::miniquad::FrontFaceOrder::Clockwise,
            FrontFaceOder::CounterClockwise => macroquad::miniquad::FrontFaceOrder::CounterClockwise
        }
    }
}
#[derive(Clone, Debug)]
pub struct GlslShader<'a>  {
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str,
    pub uniforms: Vec<(String, Uniform)>,
    pub depth_write: bool,
    pub depth_test: DepthTest,
    pub cull_face: CullFace,
    pub front_face_oder: FrontFaceOder
}


impl <'a>GlslShader<'a> {
    pub fn new(vertex: &'a str, fragment: &'a str, uniforms: Vec<(String, Uniform)>, depth_write: bool, depth_test: DepthTest, cull_face: CullFace, front_face_oder: FrontFaceOder) -> GlslShader<'a> {
        GlslShader {
            vertex_shader: vertex,
            fragment_shader: fragment,
            uniforms,
            depth_write,
            depth_test,
            cull_face,
            front_face_oder
        }
    }

    fn get_material(&self) -> prelude::Material {
        let shader = ShaderSource::Glsl { 
            vertex: self.vertex_shader, 
            fragment: self.fragment_shader 
        };
        let pipeline_parameters = prelude::PipelineParams {
            depth_write: self.depth_write,
            depth_test: self.depth_test.to_macroquad_comparison(),
            cull_face: self.cull_face.to_macroquad_cullface(),
            front_face_order: self.front_face_oder.to_macroquad_face_oder(),
            ..Default::default()
        };

        let uniforms = self.uniforms
        .iter()
        .map(|(name, uniform)| (name.clone(), uniform.uniform_type()))
        .collect::<Vec<_>>();


        let material = prelude::load_material(
            shader
            , prelude::MaterialParams {
                pipeline_params: pipeline_parameters,
                uniforms,
                ..Default::default()
            }).expect("couldnt load material");

        
        for (name, uniform) in self.uniforms.iter() {
            match uniform {
                Uniform::Float1(x) => {
                    if let Ok(x) = x.parse::<f32>() {
                        material.set_uniform(&name, x);
                    }
                },
                Uniform::Float2(x, y) => {
                    if let (Ok(x), Ok(y)) = (x.parse::<f32>(), y.parse::<f32>()) {
                        material.set_uniform(&name, (x, y));
                    }
                },
                Uniform::Float3(x, y, z) => {
                    if let (Ok(x), Ok(y), Ok(z)) =
                    (x.parse::<f32>(), y.parse::<f32>(), z.parse::<f32>())
                    {
                        material.set_uniform(&name, (x, y, z));
                    }
                },
                Uniform::Color(color) => {
                    material.set_uniform(&name, (color.x, color.y, color.z));
                },
            }
        }

        material
    }

    pub fn use_shader(&self) {
        let material = self.get_material();
        prelude::gl_use_material(&material);
    }
}