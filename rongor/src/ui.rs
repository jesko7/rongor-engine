use crate::math::math::*;
use crate::vec::*;
use crate::color::*;
use crate::draw::*;
use crate::font::*;
use crate::image::*;
use crate::mouse::*;
use crate::hitbox::*;
use crate::key::*;



/*
pub struct Button {
    pub logo: Image,
    pub position: Option<Vec2>,
    pub percentage_pos: Option<Vec2>,
    pub curviture: f32,
    pub size: Vec2,
    pub inner_color: Color,
    pub rim_color: Color,
    pub rim_width: f32
}

impl Button {
    pub fn new(
        logo: Image,
        position: Option<Vec2>,
        percentage_pos: Option<Vec2>,
        curviture: f32,
        size: Vec2,
        inner_color: Color,
        rim_color: Color,
        rim_width: f32
    ) ->  
    Button
    {
        Button {
            logo,
            position,
            percentage_pos,
            curviture,
            size,
            inner_color,
            rim_color,
            rim_width
        }
    }

    fn get_shader(&self) -> GlslShader {
        let shader = GlslShader::new(
            //-----------------------------------------------------------------------------------
            "
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
            
            ", 
            //-----------------------------------------------------------------------------------
            "
            
            precision lowp float;

            varying vec2 uv;

            uniform sampler2D Texture;

            void main() {
                gl_FragColor = te;
            }
            
            "
            //-----------------------------------------------------------------------------------
            , 
            vec![
                ("position".to_string(), Uniform::Float2("10.0".to_string(), "10.0".to_string())),
                ("curviture".to_string(), Uniform::Float1("4.0".to_string())),
                ("size".to_string(), Uniform::Float2("10.0".to_string(), "10.0".to_string())),
                ("inner_color".to_string(), Uniform::Color(Vec3::new(0., 0., 0.))),
                ("rim_color".to_string(), Uniform::Color(Vec3::new(0., 0., 0.))),
                ("rim_width".to_string(), Uniform::Float1("4.0".to_string())),
    
            ], true, DepthTest::Always, CullFace::Nothing, FrontFaceOder::CounterClockwise);

        shader
    }

    pub fn draw(&self) {

        //let screen = prelude::get_screen_data();

        //let shader = self.get_shader();

        //shader.use_shader();
        //prelude::draw_texture(&prelude::Texture2D::from_image(&screen), 0., 0., prelude::WHITE);
        //use_default_shader()

        for x in 0..1920 {
            for y in 0..1080 {
                if (x as f32 * self.size.x + self.position.unwrap().x).abs().powf(self.curviture) +
                   (y as f32 * self.size.y + self.position.unwrap().y).abs().powf(self.curviture) <= 1.
                {
                    println!("{x},{y}");
                    draw2d::set_pixle(Vec2::new(x as f32, y as f32), self.inner_color.clone());
                }

                //if (x as f32 + self.position.unwrap().x).abs().powf(self.curviture) >= 1. - self.rim_width && 
                //   (x as f32 + self.position.unwrap().x).abs().powf(self.curviture) <= 1. + self.rim_width && 
                //   (y as f32 + self.position.unwrap().y).abs().powf(self.curviture) >= 1. - self.rim_width &&
                //   (y as f32 + self.position.unwrap().y).abs().powf(self.curviture) <= 1. + self.rim_width
                //{
                //    draw2d::set_pixle(Vec2::new(x as f32, y as f32), self.rim_color.clone());
                //}
            }
        }
    }
}
*/

#[derive(Debug, Clone)]
pub struct QuadraticBeziercurve {
    pub start_point: Vec2,
    pub end_point: Vec2,
    pub curve_point: Vec2,
    pub curviture: f32,
    pub thickness: f32,
    pub color: Color
}

impl QuadraticBeziercurve {
    pub fn new
    (
        start_point: Vec2,
        end_point: Vec2,
        curve_point: Vec2,
        curviture: f32,
        thickness: f32,
        color: Color
    ) 
    -> QuadraticBeziercurve
    {
        QuadraticBeziercurve {
            start_point,
            end_point,
            curve_point,
            curviture,
            thickness,
            color
        }
    }

    pub fn draw(&self, detail: i32) {
        let mut points = vec![];

        let start_point = lerp2d(self.start_point, self.curve_point, self.curviture);
        let end_point = lerp2d(self.end_point, self.curve_point, self.curviture);

        for t_upscaled in 0..detail {
            let t = t_upscaled as f32 / detail as f32;

            let point_start_curve = lerp2d(start_point, self.curve_point, t);
            let point_curve_end = lerp2d(self.curve_point, end_point, t);

            let final_point = lerp2d(point_start_curve, point_curve_end, t);

            points.push(final_point);
        }

        for x in 0..points.len() - 1 {
            draw2d::line::line(points[x], points[x + 1], self.thickness, self.color.clone());
            draw2d::circle::circle(points[x], self.thickness / 2., self.color.clone());
            draw2d::circle::circle(points[x + 1], self.thickness / 2., self.color.clone());
        }

        draw2d::line::line(end_point, points[points.len() - 1], self.thickness, self.color.clone());
        draw2d::line::line(self.start_point, start_point, self.thickness, self.color.clone());
        draw2d::line::line(self.end_point, end_point, self.thickness, self.color.clone());

        draw2d::circle::circle(self.end_point, self.thickness / 2., self.color.clone());
    }
}

#[derive(Debug, Clone)]
pub struct CubicBeziercurve {
    pub start_point: Vec2,
    pub end_point: Vec2,
    pub curve_point1: Vec2,
    pub curve_point2: Vec2,
    pub thickness: f32,
    pub color: Color
}

impl CubicBeziercurve {
    pub fn new
    (
        start_point: Vec2,
        end_point: Vec2,
        curve_point1: Vec2,
        curve_point2: Vec2,
        thickness: f32,
        color: Color
    ) 
    -> CubicBeziercurve
    {
        CubicBeziercurve {
            start_point,
            end_point,
            curve_point1,
            curve_point2,
            thickness,
            color
        }
    }

    pub fn draw(&self, detail: i32) {
        let mut points = vec![];


        for t_upscaled in 0..detail {
            let t = t_upscaled as f32 / detail as f32;

            let point_start_curve1 = lerp2d(self.start_point, self.curve_point1, t);
            let point_curve1_curve2 = lerp2d(self.curve_point1, self.curve_point2, t);
            let point_curve2_end = lerp2d(self.curve_point2, self.end_point, t);

            let start_curve1_curve2 = lerp2d(point_start_curve1, point_curve1_curve2, t);
            let curve1_curve2_end = lerp2d(point_curve1_curve2, point_curve2_end, t);

            let final_point = lerp2d(start_curve1_curve2, curve1_curve2_end, t);

            points.push(final_point);

            //draw2d::circle::circle(final_point, self.thickness, self.color.clone());
        }

        for x in 0..points.len() - 1 {
            draw2d::line::line(points[x], points[x + 1], self.thickness, self.color.clone());
            draw2d::circle::circle(points[x], self.thickness / 2., self.color.clone());
            draw2d::circle::circle(points[x + 1], self.thickness / 2., self.color.clone());
        }

        draw2d::line::line(self.end_point, points[points.len() - 1], self.thickness, self.color.clone());
    }
}




#[derive(Debug, Clone)]
pub struct Button {
    pub size: Vec2,
    pub pos: Vec2,
    pub boundary_thickness: f32,
    pub curvituture: f32,
    pub field_color: Color,
    pub boundary_color: Color,
    pub text: Option<String>,
    pub font: Option<Font>,
    pub font_size: Option<u16>,
    pub text_color: Option<Color>,
    pub text_rotation: Option<f32>,
    pub image: Option<Image>,
    pub scale_when_clicked: bool,
    pub scale_function_add: Option<fn(f32) -> f32>,
    pub time_since_clicked: f32,
    pub is_clicked: bool,
    pub just_clicked: bool,
    pub just_released: bool
}

impl Button {
    pub fn new(
        size: Vec2,
        pos: Vec2,
        boundary_thickness: f32,
        curvituture: f32,
        field_color: Color,
        boundary_color: Color,
        text: Option<String>,
        font: Option<Font>,
        font_size: Option<u16>,
        text_color: Option<Color>,
        text_rotation: Option<f32>,
        image: Option<Image>,
        scale_when_clicked: bool,
        scale_function_add: Option<fn(f32) -> f32>,
    )
    -> Button {
        Button {
            size,
            pos,
            boundary_thickness,
            curvituture,
            field_color,
            boundary_color,
            text,
            font,
            font_size,
            text_color,
            text_rotation,
            image,
            scale_when_clicked,
            scale_function_add,
            time_since_clicked: 9999999.,
            is_clicked: false,
            just_clicked: false,
            just_released: false
        }
    }

    pub fn get_rounded_field(&self) -> RoundedField {
        RoundedField::new(self.size, self.pos, self.boundary_thickness, self.curvituture, self.field_color.clone(), self.boundary_color.clone())
    }

    pub fn get_text_field(&self) -> Textfield {
        Textfield::new(self.size, self.pos, self.boundary_thickness, self.curvituture, self.field_color.clone(), self.boundary_color.clone(), 
            self.text.clone().unwrap(), self.font.clone(), self.font_size.unwrap(), self.text_color.clone().unwrap(), self.text_rotation.unwrap())
    }

    pub fn get_hitbox(&self) -> SquareHitbox {
        SquareHitbox::new(self.pos, self.size + Vec2::new(self.boundary_thickness, self.boundary_thickness))
    }

    pub fn update(&mut self) {
        let hitbox = self.get_hitbox();

        if mouse::is_pressed(mouse::LEFT) && hitbox.collision_point(mouse::get_position()) {
            self.time_since_clicked = 0.;
            self.is_clicked = true;
        }
        else {
            self.is_clicked = false;
        }

        if mouse::just_pressed(mouse::LEFT) && hitbox.collision_point(mouse::get_position()) {
            self.just_clicked = true;
        }
        else {
            self.just_clicked = false;
        }

        if mouse::just_released(mouse::LEFT) && hitbox.collision_point(mouse::get_position()) {
            self.just_released= true;
        }
        else {
            self.just_released = false;
        }
        

        self.time_since_clicked += 0.05
    }

    pub fn is_clicked(&self) -> bool {
        self.is_clicked
    }

    pub fn just_clicked(&self) -> bool {
        self.just_clicked
    }

    pub fn just_released(&self) -> bool {
        self.just_released
    }
 
    pub fn draw(&mut self, detail: i32) {

        if  self.text.is_some() &&
            self.text_color.is_some() &&
            self.text_rotation.is_some() &&
            self.font_size.is_some()
        {
            let mut text_field = self.get_text_field();
            if let Some(func) = self.scale_function_add {
                let scale_add = func(self.time_since_clicked);
                text_field.size = self.size + Vec2::new(scale_add, scale_add);
                text_field.font_size = text_field.font_size + (scale_add / 2.) as u16;
            }
            text_field.draw(detail);

           
        }
        else if self.image.is_some() {
            let mut rounded_field = self.get_rounded_field();
            if let Some(func) = self.scale_function_add {
                let scale_add = func(self.time_since_clicked);
                rounded_field.size = self.size + Vec2::new(scale_add, scale_add);
            }
            rounded_field.draw(detail, true);

            draw2d::image::image_center(self.pos, &self.image.as_ref().unwrap())
        }   
        else {
            panic!("either the arguments for an image or the arguments for the text need to be set for button to be drawn")
        }

        self.update();
    }
}

#[derive(Debug, Clone)]
pub struct RoundedField {
    pub size: Vec2,
    pub pos: Vec2,
    pub boundary_thickness: f32,
    pub curviture: f32,
    pub field_color: Color,
    pub boundary_color: Color
}

impl RoundedField {
    pub fn new(
        size: Vec2,
        pos: Vec2,
        boundary_thickness: f32,
        curviture: f32,
        field_color: Color,
        boundary_color: Color
    )
    -> RoundedField
    {
        RoundedField {
            size,
            pos,
            boundary_thickness,
            curviture,
            field_color,
            boundary_color
        }
    }

    pub fn draw(&self, detail: i32, fill: bool) {

        let top_left = self.pos + Vec2::new(self.size.scale(-0.5).x, self.size.scale(-0.5).y);
        let top_right = self.pos + Vec2::new(self.size.scale(0.5).x, self.size.scale(-0.5).y);

        let bottom_left = self.pos + Vec2::new(self.size.scale(-0.5).x, self.size.scale(0.5).y);
        let bottom_right = self.pos + Vec2::new(self.size.scale(0.5).x, self.size.scale(0.5).y);


        let mid_point_left = lerp2d(bottom_left, top_left, 0.5);
        let mid_point_right = lerp2d(bottom_right, top_right, 0.5);

        let dist = (mid_point_left.y - bottom_left.y).abs();


        let point_top_left = top_left + Vec2::new(dist, 0.);
        let point_top_right = top_right - Vec2::new(dist, 0.);
        let point_bottom_left = bottom_left + Vec2::new(dist, 0.);
        let point_bottom_right = bottom_right - Vec2::new(dist, 0.);

        let bezier1 = QuadraticBeziercurve::new(mid_point_left, point_top_left, top_left, self.curviture, self.boundary_thickness, self.boundary_color.clone());
        let bezier2 = QuadraticBeziercurve::new(mid_point_left, point_bottom_left, bottom_left, self.curviture, self.boundary_thickness, self.boundary_color.clone());
        let bezier3 = QuadraticBeziercurve::new(mid_point_right, point_top_right, top_right, self.curviture, self.boundary_thickness, self.boundary_color.clone());
        let bezier4 = QuadraticBeziercurve::new(mid_point_right, point_bottom_right, bottom_right, self.curviture, self.boundary_thickness, self.boundary_color.clone());
        

        //draw2d::square::square_middle(self.pos, self.size.x / 0.5 / screen_width(), self.size.y / 0.5 / screen_width(), self.field_color.clone());
        //draw2d::square::square_middle(self.pos, self.size.x / 0.5 / screen_width(), dist * 2., self.field_color.clone());

        if fill {
            let start_point_top_left = lerp2d(mid_point_left, top_left, self.curviture);
            let end_point_top_left = lerp2d(point_top_left, top_left, self.curviture);
            let dist_radius = (start_point_top_left.y - end_point_top_left.y).abs();
            draw2d::circle::draw_smooth_circle(Vec2::new(end_point_top_left.x, start_point_top_left.y), dist_radius, detail as u8, self.field_color.clone());

            let start_point_bottom_left = lerp2d(mid_point_left, bottom_left, self.curviture);
            let end_point_bottom_left = lerp2d(point_bottom_left, bottom_left, self.curviture);
            let dist_radius = (start_point_bottom_left.y - end_point_bottom_left.y).abs();
            draw2d::circle::draw_smooth_circle(Vec2::new(end_point_bottom_left.x, start_point_bottom_left.y), dist_radius, detail as u8, self.field_color.clone());

            let start_point_top_right = lerp2d(mid_point_right, top_right, self.curviture);
            let end_point_top_right = lerp2d(point_top_right, top_right, self.curviture);
            let dist_radius = (start_point_top_left.y - end_point_top_right.y).abs();
            draw2d::circle::draw_smooth_circle(Vec2::new(end_point_top_right.x, start_point_top_right.y), dist_radius, detail as u8, self.field_color.clone());

            let start_point_bottom_right = lerp2d(mid_point_right, bottom_right, self.curviture);
            let end_point_bottom_right = lerp2d(point_bottom_right, bottom_right, self.curviture);
            let dist_radius = (start_point_bottom_right.y - end_point_bottom_right.y).abs();
            draw2d::circle::draw_smooth_circle(Vec2::new(end_point_bottom_right.x, start_point_bottom_right.y), dist_radius, detail as u8, self.field_color.clone());

            //bezier1.draw(detail, true, Some(self.pos), Some(detail), Some(self.field_color.clone()));
            //bezier2.draw(detail, true, Some(self.pos), Some(detail), Some(self.field_color.clone()));
            //bezier3.draw(detail, true, Some(self.pos), Some(detail), Some(self.field_color.clone()));
            //bezier4.draw(detail, true, Some(self.pos), Some(detail), Some(self.field_color.clone()));
            //draw2d::square::square_middle(self.pos, self.size.x, (point_top_right.y - point_bottom_right.y).abs(), self.field_color.clone());

            draw2d::square::square_middle(self.pos, (end_point_bottom_left.x - end_point_bottom_right.x).abs(), self.size.y, self.field_color.clone());
            draw2d::square::square_middle(self.pos, self.size.x, (start_point_bottom_left.y - start_point_top_left.y).abs(), self.field_color.clone());

        }

        

        bezier1.draw(detail);
        bezier2.draw(detail);
        bezier3.draw(detail);
        bezier4.draw(detail);

        draw2d::line::line(point_top_left, point_top_right, self.boundary_thickness, self.boundary_color.clone());
        draw2d::line::line(point_bottom_left, point_bottom_right, self.boundary_thickness, self.boundary_color.clone());


        //draw2d::circle::circle(bottom_left, 10., Color::new(255, 255, 0, 255));
        //draw2d::circle::circle(mid_point_left, 10., Color::new(255, 255, 255, 255));
        //draw2d::circle::circle(point_bottom_left, 10., Color::new(255, 0, 255, 255));

        //println!("dist: {:?}, {}", dist, (bottom_left.x - point_bottom_left.x).abs());

        //draw2d::square::square_middle(self.pos, 0.2, self.size.y * 2. / screen_height(), self.field_color.clone());

    }
}

#[derive(Debug, Clone)]
pub struct Textfield {
    pub size: Vec2,
    pub pos: Vec2,
    pub boundary_thickness: f32,
    pub curvituture: f32,
    pub field_color: Color,
    pub boundary_color: Color,
    pub text: String,
    pub font: Option<Font>,
    pub font_size: u16,
    pub text_color: Color,
    pub text_rotation: f32
}

impl Textfield {
    pub fn new(
        size: Vec2,
        pos: Vec2,
        boundary_thickness: f32,
        curvituture: f32,
        field_color: Color,
        boundary_color: Color,
        text: String,
        font: Option<Font>,
        font_size: u16,
        text_color: Color,
        text_rotation: f32
    )
    -> Self 
    {
        Textfield {
            size,
            pos,
            boundary_thickness,
            curvituture,
            field_color,
            boundary_color,
            text,
            font,
            font_size,
            text_color,
            text_rotation
        }
    }

    pub fn get_rounded_field(&self) -> RoundedField {
        RoundedField::new(self.size, self.pos, self.boundary_thickness, self.curvituture, self.field_color.clone(), self.boundary_color.clone())
    }

    pub fn draw(&self, detail: i32) {
        let rounded_field = self.get_rounded_field();
        rounded_field.draw(detail, true);

        //let x = (self.pos.x + 1.) / 2. * screen_width();
        //let y = (self.pos.y + 1.) / 2. * screen_height();


        let mut total_y = 0.;
        let mut y_offset = 0.;

        let text_self = self.text.strip_suffix("\n");
        let texts;
        
        if text_self.is_some() {
            texts = text_self.unwrap().split("\n");
        }
        else {
            texts = self.text.split("\n");
        }
        
        for text in texts.clone() {
            total_y += draw2d::text::measure_text(&text, self.font_size, self.font.clone()).y + (self.font_size / 5) as f32;
        }
        for text in texts.clone() {
            draw2d::text::draw_text_center(&text, self.pos + Vec2::new(0., y_offset + draw2d::text::measure_text(texts.clone().collect::<Vec<&str>>()[0], self.font_size, self.font.clone()).y / 2.) - Vec2::new(0., total_y / 2.), self.font_size as u16, self.text_color.clone(), self.text_rotation, self.font.clone());
            y_offset += draw2d::text::measure_text(&text, self.font_size, self.font.clone()).y + (self.font_size / 5) as f32;
        }
         //draw2d::text::draw_text_center("hallo", Vec2::new(20., 20.), 20, Color::new(255, 255, 255, 255), 0., None);
 
    }
}

#[derive(Debug, Clone)]
pub struct Bar {
    pub size: Vec2,
    pub pos: Vec2,
    pub boundary_thickness: f32,
    pub field_color_left: Color,
    pub field_color_right: Color,
    pub boundary_color: Color,
    pub fill_percent: f32,
}

impl Bar {
    pub fn new(
        size: Vec2,
        pos: Vec2,
        boundary_thickness: f32,
        field_color_left: Color,
        field_color_right: Color,
        boundary_color: Color,
        fill_percent: f32
    )
    ->  Self
    {
        Bar 
        { 
            size, 
            pos, 
            boundary_thickness, 
            field_color_left,
            field_color_right,
            boundary_color, 
            fill_percent 
        }
    }

    pub fn get_rounded_field(&self) -> RoundedField {
        RoundedField::new(self.size, self.pos, self.boundary_thickness, 0., Color::WHITE, self.boundary_color.clone())
    }

    pub fn draw(&mut self, detail: i32) {
        let mut rounded_field = self.get_rounded_field();

        self.fill_percent = self.fill_percent.clamp(0., 1.);

        if self.fill_percent == 0. {
            rounded_field.field_color = self.field_color_right.clone();
            rounded_field.draw(detail, true);
        }

        else if self.fill_percent == 1. {
            rounded_field.field_color = self.field_color_left.clone();
            rounded_field.draw(detail, true);
        }

        else {   
            let top_left = self.pos + Vec2::new(self.size.scale(-0.5).x, self.size.scale(-0.5).y);
            let top_right = self.pos + Vec2::new(self.size.scale(0.5).x, self.size.scale(-0.5).y);

            let bottom_left = self.pos + Vec2::new(self.size.scale(-0.5).x, self.size.scale(0.5).y);
            let bottom_right = self.pos + Vec2::new(self.size.scale(0.5).x, self.size.scale(0.5).y);

            let mid_point_left = lerp2d(bottom_left, top_left, 0.5);
            let mid_point_right = lerp2d(bottom_right, top_right, 0.5);

            let dist = (mid_point_left.y - bottom_left.y).abs();

            let point_top_left = top_left + Vec2::new(dist, 0.);
            let point_top_right = top_right - Vec2::new(dist, 0.);

            let start_point_left = mid_point_left;
            let end_point_left = point_top_left;
            let dist_radius = (start_point_left.y - end_point_left.y).abs();
            draw2d::circle::draw_smooth_circle(Vec2::new(end_point_left.x, start_point_left.y), dist_radius, detail as u8, self.field_color_left.clone());

            let start_point_right = mid_point_right;
            let end_point_right = point_top_right;
            let dist_radius = (start_point_left.y - end_point_right.y).abs();
            draw2d::circle::draw_smooth_circle(Vec2::new(end_point_right.x, start_point_right.y), dist_radius, detail as u8, self.field_color_right.clone());


            let right_fill = lerp2d(end_point_left, end_point_right, self.fill_percent).x;
            let left_fill = end_point_left.x;
            let width = (right_fill - left_fill).abs();

            draw2d::square::square_middle(
                Vec2::new(lerp2d(Vec2::new(left_fill, 0.), Vec2::new(right_fill, 0.), 0.5).x, self.pos.y), 
                width, self.size.y, self.field_color_left.clone());
            
            let left_fill = right_fill;
            let right_fill = end_point_right.x;
            
            let width = (right_fill - left_fill).abs();
    
            draw2d::square::square_middle(
                Vec2::new(lerp2d(Vec2::new(left_fill, 0.), Vec2::new(right_fill, 0.), 0.5).x, self.pos.y), 
                width, self.size.y, self.field_color_right.clone());

            rounded_field.draw(detail, false);
        }    
    }
}

#[derive(Debug, Clone)]
pub struct Slider {
    pub size: Vec2,
    pub pos: Vec2,
    pub boundary_thickness: f32,
    pub field_color_left: Color,
    pub field_color_right: Color,
    pub boundary_color: Color,
    pub fill_percent: f32,
    pub circle_radius: f32,
    pub inner_circle_color: Color,
    pub rim_color: Color,
    pub rim_width: f32,
    selected: bool,
    offset: f32
}

impl Slider {
    pub fn new (
        size: Vec2,
        pos: Vec2,
        boundary_thickness: f32,
        field_color_left: Color,
        field_color_right: Color,
        boundary_color: Color,
        fill_percent: f32,
        circle_radius: f32,
        inner_circle_color: Color,
        rim_color: Color,
        rim_width: f32
    )
    ->  Self
    {
        Slider 
        { 
            size, 
            pos, 
            boundary_thickness, 
            field_color_left,
            field_color_right,
            boundary_color, 
            fill_percent,
            circle_radius,
            inner_circle_color,
            rim_color,
            rim_width,
            selected: false,
            offset: 0.
        }
    }
    
    pub fn get_circle_hitbox(&self, circle_pos: Vec2) ->  SquareHitbox  {
        SquareHitbox::new(circle_pos, Vec2::new((self.circle_radius + self.rim_width) * 2., (self.circle_radius + self.rim_width) * 2.))
    }

    pub fn update(&mut self, circle_pos: Vec2, left: f32, right: f32) {
        if mouse::just_pressed(mouse::LEFT) && self.get_circle_hitbox(circle_pos).collision_point(mouse::get_position()) {
            self.selected = true;
            self.offset = mouse::get_position().x - circle_pos.x;
        }

        if mouse::just_released(mouse::LEFT)  {
            self.selected = false;
        }

        if self.selected   
        {
            let pos = mouse::get_position().x - self.offset;
            let dist = (left - right).abs();
            let dist_left_pos = pos - left;
            self.fill_percent = dist_left_pos / dist
        }
    }
 
    pub fn get_rounded_field(&self) -> RoundedField {
        RoundedField::new(self.size, self.pos, self.boundary_thickness, 0., Color::WHITE, self.boundary_color.clone())
    }

    pub fn draw_circle(&self, x: f32) {
        draw2d::circle::circle(Vec2::new(x, self.pos.y), self.circle_radius  + self.rim_width, self.rim_color.clone());
        draw2d::circle::circle(Vec2::new(x, self.pos.y), self.circle_radius, self.inner_circle_color.clone());
    }

    pub fn get_value(&self) -> f32 {
        self.fill_percent
    }

    pub fn draw(&mut self, detail: i32) {
        let mut rounded_field = self.get_rounded_field();

        self.fill_percent = self.fill_percent.clamp(0., 1.);

        if self.fill_percent == 0. {

            rounded_field.field_color = self.field_color_right.clone();
            rounded_field.draw(detail, true);
        }

        else if self.fill_percent == 1. {
            rounded_field.field_color = self.field_color_left.clone();
            rounded_field.draw(detail, true);
        }

           
            let top_left = self.pos + Vec2::new(self.size.scale(-0.5).x, self.size.scale(-0.5).y);
            let top_right = self.pos + Vec2::new(self.size.scale(0.5).x, self.size.scale(-0.5).y);

            let bottom_left = self.pos + Vec2::new(self.size.scale(-0.5).x, self.size.scale(0.5).y);
            let bottom_right = self.pos + Vec2::new(self.size.scale(0.5).x, self.size.scale(0.5).y);

            let mid_point_left = lerp2d(bottom_left, top_left, 0.5);
            let mid_point_right = lerp2d(bottom_right, top_right, 0.5);

            let dist = (mid_point_left.y - bottom_left.y).abs();

            let point_top_left = top_left + Vec2::new(dist, 0.);
            let point_top_right = top_right - Vec2::new(dist, 0.);

            let start_point_left = mid_point_left;
            let end_point_left = point_top_left;
            let dist_radius = (start_point_left.y - end_point_left.y).abs();
            if self.fill_percent != 1. && self.fill_percent != 0.  
            {
                draw2d::circle::circle(Vec2::new(end_point_left.x, start_point_left.y), dist_radius, self.field_color_left.clone());
            }
            
            let start_point_right = mid_point_right;
            let end_point_right = point_top_right;
            let dist_radius = (start_point_left.y - end_point_right.y).abs();
            if self.fill_percent != 1. && self.fill_percent != 0.  
            {
                draw2d::circle::circle(Vec2::new(end_point_right.x, start_point_right.y), dist_radius, self.field_color_right.clone());

            }
            

            let right_fill = lerp2d(end_point_left, end_point_right, self.fill_percent).x;
            let left_fill = end_point_left.x;
            let width = (right_fill - left_fill).abs();

            if self.fill_percent != 1. && self.fill_percent != 0.  
            {
                draw2d::square::square_middle(
                Vec2::new(lerp2d(Vec2::new(left_fill, 0.), Vec2::new(right_fill, 0.), 0.5).x, self.pos.y), 
                width, self.size.y, self.field_color_left.clone());
            }
            
            
            let left_fill = right_fill;
            let right_fill = end_point_right.x;
            
            let width = (right_fill - left_fill).abs();
            
            if self.fill_percent != 1. && self.fill_percent != 0.  
            {
                draw2d::square::square_middle(
                Vec2::new(lerp2d(Vec2::new(left_fill, 0.), Vec2::new(right_fill, 0.), 0.5).x, self.pos.y), 
                width, self.size.y, self.field_color_right.clone());
            }
            
            
            if self.fill_percent != 1. && self.fill_percent != 0.  
            {
                    rounded_field.draw(detail, false);
            }
            

            self.draw_circle(left_fill);
        
        self.update(Vec2::new(left_fill, self.pos.y), end_point_left.x, end_point_right.x);
    }
}

#[derive(Debug, Clone)]
pub struct Switch {
    pub size: Vec2,
    pub pos: Vec2,
    pub boundary_thickness: f32,
    pub field_color_left: Color,
    pub field_color_right: Color,
    pub boundary_color: Color,
    pub set: bool,
    fill_percent: f32,
    pub circle_radius: f32,
    pub inner_circle_color: Color,
    pub rim_color: Color,
    pub rim_width: f32
}

impl Switch {
    pub fn new (
        size: Vec2,
        pos: Vec2,
        boundary_thickness: f32,
        field_color_left: Color,
        field_color_right: Color,
        boundary_color: Color,
        set: bool,
        circle_radius: f32,
        inner_circle_color: Color,
        rim_color: Color,
        rim_width: f32
    )
    ->  Self
    {
        Switch 
        { 
            size, 
            pos, 
            boundary_thickness, 
            field_color_left,
            field_color_right,
            boundary_color, 
            set,
            fill_percent: if set {1.} else {0.},
            circle_radius,
            inner_circle_color,
            rim_color,
            rim_width
        }
    }
    
    pub fn get_hitbox(&self) ->  SquareHitbox  {
        SquareHitbox::new(self.pos, self.size)
    }

    pub fn update(&mut self) {
        if mouse::just_pressed(mouse::LEFT) && self.get_hitbox().collision_point(mouse::get_position()) {
            self.set = !self.set;
            self.fill_percent = (1. - self.fill_percent).abs();
        }
    }
 
    pub fn get_rounded_field(&self) -> RoundedField {
        RoundedField::new(self.size, self.pos, self.boundary_thickness, 0., Color::WHITE, self.boundary_color.clone())
    }

    pub fn draw_circle(&self, x: f32) {
        draw2d::circle::circle(Vec2::new(x, self.pos.y), self.circle_radius + self.rim_width, self.rim_color.clone());
        draw2d::circle::circle(Vec2::new(x, self.pos.y), self.circle_radius, self.inner_circle_color.clone());
    }

    pub fn get_value(&self) -> f32 {
        self.fill_percent
    }

    pub fn draw(&mut self, detail: i32) {
        let mut rounded_field = self.get_rounded_field();

        if self.fill_percent == 0. {

            rounded_field.field_color = self.field_color_right.clone();
            rounded_field.draw(detail, true);
        }

        else if self.fill_percent == 1. {
            rounded_field.field_color = self.field_color_left.clone();
            rounded_field.draw(detail, true);
        }

           
        let top_left = self.pos + Vec2::new(self.size.scale(-0.5).x, self.size.scale(-0.5).y);
        let top_right = self.pos + Vec2::new(self.size.scale(0.5).x, self.size.scale(-0.5).y);

        let bottom_left = self.pos + Vec2::new(self.size.scale(-0.5).x, self.size.scale(0.5).y);

        let mid_point_left = lerp2d(bottom_left, top_left, 0.5);

        let dist = (mid_point_left.y - bottom_left.y).abs();

        let point_top_left = top_left + Vec2::new(dist, 0.);
        let point_top_right = top_right - Vec2::new(dist, 0.);

        let right_fill = (if !self.set { point_top_left } else { point_top_right }).x;

        self.draw_circle(right_fill);
        
        self.update();
    }
}


#[derive(Debug, Clone)]
pub struct TickBox {
    pub size: f32,
    pub pos: Vec2,
    pub boundary_thickness: f32,
    pub curviture: f32,
    pub field_color: Color,
    pub boundary_color: Color,
    pub tick_color: Color,
    pub ticked: bool
}

impl TickBox {
    pub fn new(
        size: f32,
        pos: Vec2,
        boundary_thickness: f32,
        curviture: f32,
        field_color: Color,
        boundary_color: Color,
        tick_color: Color,
        ticked: bool
    )
    -> TickBox
    {
        TickBox {
            size,
            pos,
            boundary_thickness,
            curviture,
            field_color,
            boundary_color,
            tick_color,
            ticked
        }
    }

    pub fn get_rounded_field(&self) -> RoundedField {
        RoundedField::new(Vec2::new(self.size, self.size), self.pos, self.boundary_thickness, self.curviture, self.field_color.clone(), self.boundary_color.clone())
    }

    pub fn get_hitbox(&self) -> SquareHitbox {
        SquareHitbox::new(self.pos, Vec2::new(self.size, self.size))
    }

    pub fn update(&mut self) {
        if mouse::just_pressed(mouse::LEFT) && self.get_hitbox().collision_point(mouse::get_position()) {
            self.ticked = !self.ticked;
        }
    }

    pub fn draw_tick(&self) {
        let left_point = self.pos - Vec2::new(self.size / 4.5, 0.);
        let bottom_mid = self.pos + Vec2::new(0., self.size / 4.5);
        let top_right = self.pos + Vec2::new(self.size / 4.5, -self.size / 4.5);

        draw2d::line::line(left_point, bottom_mid, self.size / 4.5, self.tick_color.clone());
        draw2d::line::line(bottom_mid, top_right, self.size / 4.5, self.tick_color.clone());
        draw2d::circle::circle(left_point, self.size / 9., self.tick_color.clone());
        draw2d::circle::circle(bottom_mid, self.size / 9., self.tick_color.clone());
        draw2d::circle::circle(top_right, self.size / 9., self.tick_color.clone());
    }

    pub fn draw(&mut self, detail: i32) {
        let rounded_field = self.get_rounded_field();

        rounded_field.draw(detail, self.field_color.a != 0.);

        if self.ticked {
            self.draw_tick();
        }
        
        self.update();
    }
}



#[derive(Debug, Clone)]
pub struct EntranceTextfield {
    pub minsize: Vec2,
    pub pos: Vec2,
    pub boundary_thickness: f32,
    pub curvituture: f32,
    pub field_color: Color,
    pub boundary_color: Color,
    pub text: String,
    pub font: Option<Font>,
    pub font_size: u16,
    pub text_color: Color,
    pub text_rotation: f32,
    pub scale_left_and_right: bool,
    pub scale_up_and_down: bool,
    blink_time: i32,
    pub cursor: String
}

impl EntranceTextfield {
    pub fn new(
        minsize: Vec2,
        pos: Vec2,
        boundary_thickness: f32,
        curvituture: f32,
        field_color: Color,
        boundary_color: Color,
        text: String,
        font: Option<Font>,
        font_size: u16,
        text_color: Color,
        text_rotation: f32,
        scale_left_and_right: bool,
        scale_up_and_down: bool,
        cursor: String
    )
    -> Self 
    {
        EntranceTextfield {
            minsize,
            pos,
            boundary_thickness,
            curvituture,
            field_color,
            boundary_color,
            text,
            font,
            font_size,
            text_color,
            text_rotation,
            scale_left_and_right,
            scale_up_and_down,
            blink_time: 0,
            cursor,
        }
    }

    pub fn update(&mut self) {
        let keys = key::get_keys_just_pressed();

        if keys.len() != 0 {
            let mut new_keys = vec![];

            for key in keys {
                if key == "space" {
                    new_keys.push(" ".to_string());
                }
                else if key == "leftshift" || key == "rightshift" {

                }
                else if key == "backspace" {
                    let deleted_letter = new_keys.pop();
                    if deleted_letter.is_none() {
                        let _ = self.text.split_off(self.text.len() - 1);
                    }
                }
                else if key.starts_with("key") {
                    self.text += key.strip_prefix("key").unwrap();
                }
                else if key == "enter" {
                    self.text += "\n";
                    
                }
                else {
                    if key::is_pressed("leftshift") || key::is_pressed("rightshift") {
                        new_keys.push(key.to_uppercase());
                    }
                    else {
                        new_keys.push(key);
                    }
                }
            }
            self.text += &new_keys.join("");
        }
    }

    pub fn get_text_field(&mut self) -> Textfield {
        let mut y_offset = 0.;

        let mut x_offset = 0.;

        self.blink_time += 1;
        let text_self = self.text.strip_suffix("\n");
        let texts;
        
        if text_self.is_some() {
            texts = text_self.unwrap().split("\n");
        }
        else {
            texts = self.text.split("\n");
        }
        
        for (x, text) in texts.enumerate() {
            let size;
            if x + 1 == text.len() && self.blink_time % 100 <= 50 {
                size = draw2d::text::measure_text(&(text.to_owned() + &self.cursor), self.font_size, self.font.clone());
            }
            else {
                size = draw2d::text::measure_text(&text, self.font_size, self.font.clone());
            }
            x_offset = size.x.max(x_offset);
            y_offset += size.y + (self.font_size / 5) as f32;
        }

        let size_raw = Vec2::new(x_offset, y_offset);

        let size = Vec2::new(
            if self.scale_left_and_right {
                self.minsize.x.max(size_raw.x) + 70.
            } 
            else {
                self.minsize.x
            } 
            ,
            if self.scale_up_and_down {
                self.minsize.y.max(size_raw.y)
            } 
            else {
                self.minsize.y
            } 
        );

        if self.blink_time % 100 <= 50 {
            return Textfield::new(size, self.pos, self.boundary_thickness, self.curvituture, self.field_color.clone(), self.boundary_color.clone(), self.text.clone() + &self.cursor, self.font.clone(), self.font_size, self.text_color.clone(), self.text_rotation);
        }
        else {
            Textfield::new(size, self.pos, self.boundary_thickness, self.curvituture, self.field_color.clone(), self.boundary_color.clone(), self.text.clone(), self.font.clone(), self.font_size, self.text_color.clone(), self.text_rotation)
        }

        }

    pub fn draw(&mut self, detail: i32) {
        self.update();

        let text_field = self.get_text_field();
        text_field.draw(detail);
    }
}

#[derive(Debug, Clone)]
pub struct DottedQuadraticBeziercurve {
    pub start_point: Vec2,
    pub end_point: Vec2,
    pub curve_point: Vec2,
    pub curviture: f32,
    pub radius: f32,
    pub color: Color
}

impl DottedQuadraticBeziercurve {
    pub fn new
    (
        start_point: Vec2,
        end_point: Vec2,
        curve_point: Vec2,
        curviture: f32,
        radius: f32,
        color: Color
    ) 
    -> DottedQuadraticBeziercurve
    {
        DottedQuadraticBeziercurve {
            start_point,
            end_point,
            curve_point,
            curviture,
            radius,
            color
        }
    }

    pub fn draw(&self, number_of_dots: i32) {
        let mut points = vec![];

        let start_point = lerp2d(self.start_point, self.curve_point, self.curviture);
        let end_point = lerp2d(self.end_point, self.curve_point, self.curviture);

        for t_upscaled in 0..number_of_dots {
            let t = t_upscaled as f32 / number_of_dots as f32;

            let point_start_curve = lerp2d(start_point, self.curve_point, t);
            let point_curve_end = lerp2d(self.curve_point, end_point, t);

            let final_point = lerp2d(point_start_curve, point_curve_end, t);

            points.push(final_point);
        }

        for x in 0..points.len() - 1 {
            //draw2d::line::line(points[x], points[x + 1], self.thickness, self.color.clone());
            //draw2d::circle::circle(points[x], self.thickness / 2., self.color.clone());
            draw2d::circle::circle(points[x], self.radius, self.color.clone());
        }

        draw2d::circle::circle(self.end_point, self.radius, self.color.clone());
        draw2d::circle::circle(self.start_point, self.radius, self.color.clone())

        //draw2d::line::line(end_point, points[points.len() - 1], self.thickness, self.color.clone());
        //draw2d::line::line(self.start_point, start_point, self.thickness, self.color.clone());
        //draw2d::line::line(self.end_point, end_point, self.thickness, self.color.clone());
    }
}

#[derive(Debug, Clone)]
pub struct DottedCubicBeziercurve {
    pub start_point: Vec2,
    pub end_point: Vec2,
    pub curve_point1: Vec2,
    pub curve_point2: Vec2,
    pub radius: f32,
    pub color: Color
}

impl DottedCubicBeziercurve {
    pub fn new
    (
        start_point: Vec2,
        end_point: Vec2,
        curve_point1: Vec2,
        curve_point2: Vec2,
        radius: f32,
        color: Color
    ) 
    -> DottedCubicBeziercurve
    {
        DottedCubicBeziercurve {
            start_point,
            end_point,
            curve_point1,
            curve_point2,
            radius,
            color
        }
    }

    pub fn draw(&self, number_of_dots: i32) {
        let mut points = vec![];


        for t_upscaled in 0..number_of_dots {
            let t = t_upscaled as f32 / number_of_dots as f32;

            let point_start_curve1 = lerp2d(self.start_point, self.curve_point1, t);
            let point_curve1_curve2 = lerp2d(self.curve_point1, self.curve_point2, t);
            let point_curve2_end = lerp2d(self.curve_point2, self.end_point, t);

            let start_curve1_curve2 = lerp2d(point_start_curve1, point_curve1_curve2, t);
            let curve1_curve2_end = lerp2d(point_curve1_curve2, point_curve2_end, t);

            let final_point = lerp2d(start_curve1_curve2, curve1_curve2_end, t);

            points.push(final_point);

            //draw2d::circle::circle(final_point, self.thickness, self.color.clone());
        }

        for x in 0..points.len() - 1 {
            //draw2d::line::line(points[x], points[x + 1], self.thickness, self.color.clone());
            draw2d::circle::circle(points[x], self.radius, self.color.clone());
            //draw2d::circle::circle(points[x + 1], self.thickness / 2., self.color.clone());
        }

        draw2d::circle::circle(self.end_point, self.radius, self.color.clone());
        draw2d::circle::circle(self.start_point, self.radius, self.color.clone())
        
        //draw2d::line::line(self.end_point, points[points.len() - 1], self.thickness, self.color.clone());
    }
}
