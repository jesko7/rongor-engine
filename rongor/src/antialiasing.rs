use core::panic;

use crate::image::Image;
use image::{DynamicImage, Rgba};

pub fn gaussian_blur(image: &DynamicImage, sigma: f32) -> DynamicImage {
    let rgba_image: image::ImageBuffer<Rgba<u8>, _> = image.to_rgba8();


    let kernel_size = (3.0 * sigma).ceil() as usize;


    let mut kernel: Vec<f32> = Vec::with_capacity(2 * kernel_size + 1);
    let scale: f32 = 1.0 / (2.0 * std::f32::consts::PI * sigma * sigma);
    let mut sum: f32 = 0.0;

    for x in (-(kernel_size as i32))..=(kernel_size as i32) {
        let weight: f32 = scale * (-((x as f32 * x as f32) / (2.0 * sigma * sigma)).exp());
        sum += weight;
        kernel.push(weight);
    }


    for weight in &mut kernel {
        *weight /= sum;
    }


    let width: usize = rgba_image.width() as usize;
    let height: usize = rgba_image.height() as usize;

    let mut blurred_image: image::ImageBuffer<Rgba<u8>, _> = rgba_image.clone();


    for y in 0..height {
        for x in kernel_size..(width - kernel_size) {
            let mut r: f32 = 0.0;
            let mut g: f32 = 0.0;
            let mut b: f32 = 0.0;
            let mut a: f32 = 0.0;

            for i in 0..=(2 * kernel_size) {
                let pixel: Rgba<u8> = *rgba_image.get_pixel((x + i - kernel_size) as u32, y as u32);
                let weight: f32 = kernel[i];
                r += weight * pixel[0] as f32;
                g += weight * pixel[1] as f32;
                b += weight * pixel[2] as f32;
                a += weight * pixel[3] as f32;
            }

            let blurred_pixel: Rgba<u8> = Rgba([
                r.round() as u8,
                g.round() as u8,
                b.round() as u8,
                a.round() as u8,
            ]);

            blurred_image.put_pixel(x as u32, y as u32, blurred_pixel);
        }
    }


    for x in 0..width {
        for y in kernel_size..(height - kernel_size) {
            let mut r: f32 = 0.0;
            let mut g: f32 = 0.0;
            let mut b: f32 = 0.0;
            let mut a: f32 = 0.0;

            for i in 0..=(2 * kernel_size) {
                let pixel: Rgba<u8> = *blurred_image.get_pixel(x as u32, (y + i - kernel_size) as u32);
                let weight: f32 = kernel[i];
                r += weight * pixel[0] as f32;
                g += weight * pixel[1] as f32;
                b += weight * pixel[2] as f32;
                a += weight * pixel[3] as f32;
            }

            let blurred_pixel: Rgba<u8> = Rgba([
                r.round() as u8,
                g.round() as u8,
                b.round() as u8,
                a.round() as u8,
            ]);

            blurred_image.put_pixel(x as u32, y as u32, blurred_pixel);
        }
    }

    DynamicImage::ImageRgba8(blurred_image)
}


#[derive(Clone, Debug)]
pub enum AntialiasingType {
    Gaussian(f32),
    Nearest,
    Lanczos3,
    Triangle,
    CatmullRom
}

pub fn anti_alias_dynamic_image(image: &DynamicImage, antialiasing_type: AntialiasingType) -> Image{
    let mut input_texture = image.clone();

    if let AntialiasingType::Gaussian(blur_radius) = antialiasing_type 
    {
        input_texture = gaussian_blur(&input_texture, blur_radius);
    }
    else {
        let filter_type = match antialiasing_type {
            AntialiasingType::CatmullRom => image::imageops::FilterType::CatmullRom,
            AntialiasingType::Nearest => image::imageops::FilterType::Nearest,
            AntialiasingType::Triangle => image::imageops::FilterType::Triangle,
            AntialiasingType::Lanczos3 => image::imageops::FilterType::Lanczos3,
            _ohter => panic!("not good")
        };
    
        input_texture.resize_exact(input_texture.width(), input_texture.height(), filter_type);
    
    }

    Image::new_from_dynamic_image(input_texture, 0., None, false, false, image.width() as u16, image.height() as u16)
}


pub fn anti_alias_texture(image: &Image, antialiasing_type: AntialiasingType) -> Image{
    let mut input_texture = image.to_dynamic_image();
    //let input_texture = gaussian_blur(&input_texture, blur_radius);
    //let input_texture = input_texture.resize_exact(input_texture.width(), input_texture.height(), image::imageops::Gaussian);
    //let input_texture = input_texture.grayscale();

    if let AntialiasingType::Gaussian(blur_radius) = antialiasing_type 
    {
        input_texture = gaussian_blur(&input_texture, blur_radius);
    }
    else {
        let filter_type = match antialiasing_type {
            AntialiasingType::CatmullRom => image::imageops::FilterType::CatmullRom,
            AntialiasingType::Nearest => image::imageops::FilterType::Nearest,
            AntialiasingType::Triangle => image::imageops::FilterType::Triangle,
            AntialiasingType::Lanczos3 => image::imageops::FilterType::Lanczos3,
            _ohter => panic!("not good")
        };
    
        input_texture.resize_exact(input_texture.width(), input_texture.height(), filter_type);
    
    }

    Image::new_from_dynamic_image(input_texture, image.rotation, image.pivot, image.flip_x, image.flip_y, image.size.x as u16, image.size.y as u16)
}