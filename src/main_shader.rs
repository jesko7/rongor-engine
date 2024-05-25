use macroquad::prelude::*;

#[macroquad::main("Shadertoy")]
async fn main() {
    let rust_logo = load_texture("assets/rust.png").await.unwrap();

    let fragment_shader = DEFAULT_FRAGMENT_SHADER.to_string();
    let vertex_shader = DEFAULT_VERTEX_SHADER.to_string();

    let pipeline_params = PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    };

    let camera = Camera3D {
        position: vec3(-15., 15., -5.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 5., -5.),
        ..Default::default()
    };

    let mut uniforms: Vec<(String, UniformType)> = vec![("1".to_string(), UniformType::Float1)];


    loop {
        clear_background(BLUE);

        let material = load_material(
            ShaderSource::Glsl {
                vertex: &vertex_shader,
                fragment: &fragment_shader,
            },
            MaterialParams {
                pipeline_params,
                uniforms: uniforms.clone(),
                ..Default::default()
            },
        )
        .unwrap();

        set_camera(&camera);

        gl_use_material(&material);
        draw_cube(vec3(0., 5., 0.), vec3(10., 10., 10.), Some(&rust_logo), WHITE);
        gl_use_default_material();

        next_frame().await
    }
}

const DEFAULT_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;
uniform float hallo;

void main() {
    gl_FragColor = vec4(hallo, 0., 0., 0.);
}
";

const DEFAULT_VERTEX_SHADER: &'static str = "#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";