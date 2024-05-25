use macroquad::{prelude, texture::Texture2D};
use crate::vec::*;
use image::{DynamicImage, ImageBuffer};


#[derive(Clone, Debug)]

pub struct Image {
    pub ___macroquad_texture: prelude::Texture2D,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub size: Vec2,
    pub pivot: Option<Vec2>,
}


impl Image {
    pub async fn new(path: &str, rotation: f32, flip_x: bool, flip_y: bool, size: Vec2, pivot: Option<Vec2>) -> Image {
        let texture = prelude::load_texture(path).await;

        if texture.is_err() {
            panic!("couldnt load file with path: {}", path)
        }

        Image {
            ___macroquad_texture: texture.unwrap(),
            rotation,
            flip_x,
            flip_y,
            size,
            pivot,
        }
    }

    pub fn new_from_bytes(width: u16, height: u16, bytes: Vec<u8>) -> Self {
        Image { ___macroquad_texture: Texture2D::from_rgba8(width, height, &bytes), rotation: 0., flip_x: false, flip_y: false, size: Vec2::new(width as f32, height as f32), pivot: None }
    }

    pub fn get_draw_params(&self) -> prelude::DrawTextureParams {
        let pivot;

        if self.pivot.is_none(){
            pivot = None;
        }
        else {
            pivot = Some(self.pivot.unwrap().to_macro_vec2());
        }

        prelude::DrawTextureParams {
            dest_size: Some(self.size.to_macro_vec2()),
            rotation: self.rotation,
            flip_x: self.flip_x,
            flip_y: self.flip_y,
            pivot,
            ..Default::default()
        }
    }


    pub fn to_dynamic_image(&self) -> DynamicImage {
        let image_data = self.___macroquad_texture.get_texture_data().bytes;
 
        let big_image = DynamicImage::ImageRgba8(ImageBuffer::from_raw(
            self.___macroquad_texture.width() as u32, 
            self.___macroquad_texture.height() as u32, 
        image_data)
        .expect("not good"));
        let smol_image = big_image.resize_exact(self.size.x as u32, self.size.y as u32, image::imageops::FilterType::Lanczos3);

        DynamicImage::ImageRgba8(
            ImageBuffer::from_raw(
            self.size.x as u32, 
            self.size.y as u32, 
        smol_image.as_bytes().to_vec()).expect("not good"))
        

        //image::load_from_memory_with_format(
        //   &self.___macroquad_texture.get_texture_data().bytes,
        //  image::ImageFormat::Png
        // ).unwrap()
        //.resize_exact(self.size_pixles.unwrap().x as u32, 
        //self.size_pixles.unwrap().y as u32, 
        //image::imageops::FilterType::Lanczos3)



        //DynamicImage::ImageRgba8(ImageBuffer::from_fn(self.size_pixles.unwrap().x as u32, self.size_pixles.unwrap().y as u32, |x, y| {
        //    let color_at_pixel = self.___macroquad_texture.get_texture_data().get_pixel(x, y);
        //    Rgba([color_at_pixel.r as u8, 
        //          color_at_pixel.g as u8, 
        //          color_at_pixel.b as u8, 
        //          color_at_pixel.a as u8])
        //}))
    }

    pub fn new_from_dynamic_image(image: DynamicImage, rotation: f32, pivot: Option<Vec2>, flip_x: bool, flip_y: bool, width: u16, height: u16) -> Image {
        Image {
            ___macroquad_texture: prelude::Texture2D::from_rgba8(width, height, image.as_bytes()),
            rotation,
            flip_x,
            flip_y,
            size: Vec2::new(width as f32, height as f32),
            pivot,
        }
    }
}







