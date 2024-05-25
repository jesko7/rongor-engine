use macroquad::prelude;

#[derive(Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}


impl Color {
    pub const LIGHTGRAY:            Color = Color::from_0to1(0.78, 0.78, 0.78, 1.00);
    pub const GRAY:                 Color = Color::from_0to1(0.51, 0.51, 0.51, 1.00);
    pub const DARKGRAY:             Color = Color::from_0to1(0.31, 0.31, 0.31, 1.00);
    pub const YELLOW:               Color = Color::from_0to1(0.99, 0.98, 0.00, 1.00);
    pub const GOLD:                 Color = Color::from_0to1(1.00, 0.80, 0.00, 1.00);
    pub const ORANGE:               Color = Color::from_0to1(1.00, 0.63, 0.00, 1.00);
    pub const PINK:                 Color = Color::from_0to1(1.00, 0.43, 0.76, 1.00);
    pub const RED:                  Color = Color::from_0to1(0.90, 0.16, 0.22, 1.00);
    pub const MAROON:               Color = Color::from_0to1(0.75, 0.13, 0.22, 1.00);
    pub const GREEN:                Color = Color::from_0to1(0.00, 0.89, 0.19, 1.00);
    pub const LIME:                 Color = Color::from_0to1(0.00, 0.62, 0.18, 1.00);
    pub const DARKGREEN:            Color = Color::from_0to1(0.00, 0.46, 0.17, 1.00);
    pub const SKYBLUE:              Color = Color::from_0to1(0.40, 0.75, 1.00, 1.00);
    pub const BLUE:                 Color = Color::from_0to1(0.00, 0.47, 0.95, 1.00);
    pub const DARKBLUE:             Color = Color::from_0to1(0.00, 0.32, 0.67, 1.00);
    pub const PURPLE:               Color = Color::from_0to1(0.78, 0.48, 1.00, 1.00);
    pub const VIOLET:               Color = Color::from_0to1(0.53, 0.24, 0.75, 1.00);
    pub const DARKPURPLE:           Color = Color::from_0to1(0.44, 0.12, 0.49, 1.00);
    pub const BEIGE:                Color = Color::from_0to1(0.83, 0.69, 0.51, 1.00);
    pub const BROWN:                Color = Color::from_0to1(0.50, 0.42, 0.31, 1.00);
    pub const DARKBROWN:            Color = Color::from_0to1(0.30, 0.25, 0.18, 1.00);
    pub const WHITE:                Color = Color::from_0to1(1.00, 1.00, 1.00, 1.00);
    pub const BLACK:                Color = Color::from_0to1(0.00, 0.00, 0.00, 1.00);
    pub const BLANK:                Color = Color::from_0to1(0.00, 0.00, 0.00, 0.00);
    pub const MAGENTA:              Color = Color::from_0to1(1.00, 0.00, 1.00, 1.00);

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {r: r as f32 / 255., b: b as f32 / 255., g: g as f32 / 255., a: a as f32 / 255.}
    }

    pub const fn from_0to1(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {r, g, b, a}
    }


    pub fn to_macro_color(&self) -> prelude::Color {
        prelude::Color::new(self.r, self.g, self.b, self.a)
    }
}