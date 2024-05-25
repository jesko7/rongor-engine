

pub mod draw2d {
    use macroquad::prelude;
    //use crate::vec::*;
    use crate::{color::*, prelude::Vec2};

    pub mod circle {
        use macroquad::prelude;
        use crate::vec::*;
        use crate::color::*;
        
        pub fn circle(position: Vec2, radius: f32, color: Color) {
            prelude::draw_circle(position.x, position.y, radius, color.to_macro_color());
        }
        pub fn circle_lines(position: Vec2, radius: f32, color: Color, thickness: f32) {
            prelude::draw_circle_lines(position.x, position.y, radius, thickness, color.to_macro_color());
        }
        pub fn circle_quick(position: (f32, f32), radius: f32, color: Color) {
            prelude::draw_circle(position.0, position.1, radius, color.to_macro_color());
        }
        pub fn circle_lines_quick(position: (f32, f32), radius: f32, color: Color, thickness: f32) {
            prelude::draw_circle_lines(position.1, position.0, radius, thickness, color.to_macro_color());
        }
        pub fn draw_smooth_circle(position: Vec2, radius: f32, sides: u8, color: Color) {
            prelude::draw_poly(position.x, position.y, sides, radius, 0., color.to_macro_color());
        }
    }

    pub mod triangle {
        use macroquad::prelude;
        use crate::vec::*;
        use crate::color::*;

        pub fn triangle(p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
            prelude::draw_triangle(p1.to_macro_vec2(), p2.to_macro_vec2(), p3.to_macro_vec2(), color.to_macro_color())
        }
        pub fn quick_triangle(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), color: (u8, u8, u8, u8)) {
            prelude::draw_triangle(prelude::Vec2::new(p1.0, p1.1), prelude::Vec2::new(p2.0, p2.1), prelude::Vec2::new(p3.0, p3.1), prelude::Color::from_rgba(color.0, color.1, color.2, color.3))
        }
        pub fn triangle_lines(p1: Vec2, p2: Vec2, p3: Vec2, thickness: f32, color: Color) {
            prelude::draw_triangle_lines(p1.to_macro_vec2(), p2.to_macro_vec2(), p3.to_macro_vec2(), thickness, color.to_macro_color())
        }
        pub fn quick_triangle_lines(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32), thickness: f32, color: (u8, u8, u8, u8)) {
            prelude::draw_triangle_lines(prelude::Vec2::new(p1.0, p1.1), prelude::Vec2::new(p2.0, p2.1), prelude::Vec2::new(p3.0, p3.1), thickness, prelude::Color::from_rgba(color.0, color.1, color.2, color.3))
        }
    }
    

    pub mod square {
        use macroquad::prelude;
        use crate::vec::*;
        use crate::color::*;



        pub fn square_top_left(pos_top_left: Vec2, width: f32, height: f32, color: Color) {
            prelude::draw_rectangle(pos_top_left.x, pos_top_left.y, width, height, color.to_macro_color())
        }
        pub fn quick_square_top_left(pos_top_left: (f32, f32), width: f32, height: f32, color: (u8, u8, u8, u8)) {
            prelude::draw_rectangle(pos_top_left.0, pos_top_left.1, width, height, prelude::Color::from_rgba(color.0, color.1, color.2, color.3))
        }
        pub fn square_middle(pos_middle: Vec2, width: f32, height: f32, color: Color) {
            prelude::draw_rectangle(pos_middle.x - width / 2., pos_middle.y - height / 2., width, height, color.to_macro_color())
        }
        pub fn quick_square_middle(pos_middle: (f32, f32), width: f32, height: f32, color: (u8, u8, u8, u8)) {
            prelude::draw_rectangle(pos_middle.0 - width / 2., pos_middle.1 - height / 2., width, height, prelude::Color::from_rgba(color.0, color.1, color.2, color.3))
        }
        pub fn square_top_left_lines(pos_top_left: Vec2, width: f32, height: f32, thickness: f32, color: Color) {
            prelude::draw_rectangle_lines(pos_top_left.x, pos_top_left.y, width, height, thickness,  color.to_macro_color())
        }
        pub fn quick_square_top_left_lines(pos_top_left: (f32, f32), width: f32, height: f32, thickness: f32, color: (u8, u8, u8, u8)) {
            prelude::draw_rectangle_lines(pos_top_left.0, pos_top_left.1, width, height, thickness, prelude::Color::from_rgba(color.0, color.1, color.2, color.3))
        }
        pub fn square_middle_lines(pos_middle: Vec2, width: f32, height: f32, thickness: f32, color: Color) {
            prelude::draw_rectangle_lines(pos_middle.x - width / 2., pos_middle.y - height / 2., width, height, thickness, color.to_macro_color())
        }
        pub fn quick_square_middle_lines(pos_middle: (f32, f32), width: f32, height: f32, thickness: f32, color: (u8, u8, u8, u8)) {
            prelude::draw_rectangle_lines(pos_middle.0 - width / 2., pos_middle.1 - height / 2., width, height, thickness, prelude::Color::from_rgba(color.0, color.1, color.2, color.3))
        }
    }


    pub mod line {
        use macroquad::prelude;
        use crate::vec::*;
        use crate::color::*;

        pub fn line(p1: Vec2, p2: Vec2, thickness: f32, color: Color) {
            prelude::draw_line(p1.x, p1.y, p2.x, p2.y, thickness, color.to_macro_color())
        }
        pub fn quick_line(p1: (f32, f32), p2: (f32, f32), thickness: f32, color: Color) {
            prelude::draw_line(p1.0, p1.1, p2.0, p2.1, thickness, color.to_macro_color())
        }
    }


    pub mod image {
        use macroquad::prelude;
        use crate::vec::*;
        use crate::image::*;

        pub fn image_bottom_left(position: Vec2, image: &Image) {
            prelude::draw_texture_ex(&image.___macroquad_texture, position.x, position.y, prelude::WHITE, image.get_draw_params());
        }

        pub fn image_center(position: Vec2, image: &Image) {


            prelude::draw_texture_ex(&image.___macroquad_texture, position.x - image.size.x / 2., position.y - image.size.y / 2., prelude::WHITE, image.get_draw_params());
        }
    }


    pub fn set_pixle(xy: Vec2, color: Color) {
        square::square_top_left(xy, 1., 1., color);
    }

    pub fn clear(color: Color) {
        prelude::clear_background(color.to_macro_color());
    }

    pub mod text {
        use macroquad::prelude;
        use macroquad::text::TextParams;
        use crate::vec::*;
        use crate::font::*;
        use crate::color::*;

        pub fn draw_text_bottom_left(text: &str, position: Vec2, font_size: u16, color: Color, rotation: f32, font: Option<Font>) {
            //prelude::draw_text("Hello, World!", 20., 600.0, 20.0, prelude::WHITE);

            //return;

            if font.is_none() {
                
                prelude::draw_text_ex(text, position.x, position.y, TextParams {
                    font: None,
                    font_size,
                    rotation,
                    color: color.to_macro_color(),
                    ..Default::default()
                })
            }

            
            else if let Some(ftemp) = font {
                //let ftemp = font.unwrap().clone().__f.clone();
                let f = Some(&ftemp.__f);

                prelude::draw_text_ex(text, position.x, position.y, TextParams {
                    font: f,
                    font_size,
                    rotation,
                    color: color.to_macro_color(),
                    ..Default::default()
                })
            }

            ()    
        }

        pub fn draw_text_center(text: &str, position: Vec2, font_size: u16, color: Color, rotation: f32, font: Option<Font>) {
            if font.is_none() {
                
                let size = prelude::measure_text(text, None, font_size, 1.);

                prelude::draw_text_ex(text, position.x - size.width / 2., position.y + size.height / 2., TextParams {
                    font: None,
                    font_size,
                    rotation,
                    color: color.to_macro_color(),
                    ..Default::default()
                })
            }

            
            else if let Some(ftemp) = font {
                //let ftemp = font.unwrap().clone().__f.clone();
                let f = Some(&ftemp.__f);

                let size = prelude::measure_text(text, f, font_size, 1.);

                prelude::draw_text_ex(text, position.x - size.width / 2., position.y + size.height / 2., TextParams {
                    font: f,
                    font_size,
                    rotation,
                    color: color.to_macro_color(),
                    ..Default::default()
                })
            }

            ()    
        }

        pub fn measure_text(text: &str, font_size: u16, font: Option<Font>) -> Vec2 {
            let f;
            if font.is_none() {
                f = None;
            }
            else if let Some(ftemp) = font {
                f = Some(ftemp.__f);
            }
            else {
                panic!("not good")
            }
            let dim =prelude::measure_text(text, f.as_ref(), font_size, 1.);

            Vec2::new(dim.width, dim.height)
        }
    }
}


pub mod draw3d {
    use crate::color::Color;
    use macroquad::prelude;

    pub mod cube {
        use macroquad::prelude;
        use crate::vec::*;
        use crate::color::*;
        use crate::image::*;

        pub fn cube_top_left_front(position: Vec3, size: Vec3, texture: Option<Image>, color: Color) {
            let position_center = Vec3::new(position.x - size.x / 2., position.y - size.y / 2., position.z - size.z / 2.);
            if texture.is_some() {
                prelude::draw_cube(position_center.to_macro_vec3(), size.to_macro_vec3(), Some(&texture.unwrap().___macroquad_texture), color.to_macro_color())
            }
            else {
                prelude::draw_cube(position_center.to_macro_vec3(), size.to_macro_vec3(), None, prelude::WHITE)
            }
        }
        pub fn cube_center(position: Vec3, size: Vec3, texture: Option<&Image>, color: Color) {
            if texture.is_some() {
                prelude::draw_cube(position.to_macro_vec3(), size.to_macro_vec3(), Some(&texture.unwrap().___macroquad_texture), color.to_macro_color())
            }
            else {
                prelude::draw_cube(position.to_macro_vec3(), size.to_macro_vec3(), None, color.to_macro_color())
            }
        }
        pub fn wire_cube_top_left_front(position: Vec3, size: Vec3, color: Color) {
            let position = Vec3::new(position.x - size.x / 2., position.y - size.y / 2., position.z - size.z / 2.);
            prelude::draw_cube_wires(position.to_macro_vec3(), size.to_macro_vec3(), color.to_macro_color());
        }
        pub fn wire_cube_center(position: Vec3, size: Vec3, color: Color) {
            prelude::draw_cube_wires(position.to_macro_vec3(), size.to_macro_vec3(), color.to_macro_color());
        }
    }

    pub mod sphere {
        use macroquad::prelude;
        use crate::vec::*;
        use crate::color::*;
        use crate::image::*;

        pub fn sphere_center(position: Vec3, radius: f32, texture: Option<&Image>, color: Color) {
            if texture.is_some() {
                prelude::draw_sphere(position.to_macro_vec3(), radius, Some(&texture.unwrap().___macroquad_texture), color.to_macro_color())
            }
            else {
                prelude::draw_sphere(position.to_macro_vec3(), radius, None, color.to_macro_color())
            }
        }
        pub fn wire_sphere_center(position: Vec3, radius: f32, color: Color) {
            prelude::draw_sphere_wires(position.to_macro_vec3(), radius, None, color.to_macro_color());
        }
    }

    pub mod grid {
        use macroquad::prelude;
        use crate::vec::*;
        use crate::color::*;

        pub fn grid_center(number_slices: u32, spacing: f32, line_color: Color, other_color: Color, center: Vec3, rotation: Vec3) {
            let rotx = prelude::Quat::from_rotation_x(rotation.x);
            let roty = prelude::Quat::from_rotation_y(rotation.y);
            let rotz = prelude::Quat::from_rotation_z(rotation.z);
            prelude::draw_grid_ex(number_slices, spacing, line_color.to_macro_color(), other_color.to_macro_color(), center.to_macro_vec3(), rotx * roty * rotz)
        }
    }

    pub fn clear(color: Color) {
        prelude::clear_background(color.to_macro_color());
    }

    
}