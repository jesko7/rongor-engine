pub mod math {
    use crate::vec::*;
    use perlin_noise::PerlinNoise;
    use simple_simplex;

    pub fn rotate2d(point: Vec2, pivot: Option<Vec2>, rotation: f32) -> Vec2 {
        let point = point - pivot.unwrap_or(Vec2::new(0., 0.));

        let cos = rotation.cos();
        let sin = rotation.sin();

        let x = point.x * cos - point.y * sin;
        let y = point.x * sin + point.y * cos;

        Vec2::new(x, y) + pivot.unwrap_or(Vec2::new(0., 0.))
    }

    pub fn rotate3d(point: Vec3, pivot: Option<Vec3>, rotation: Vec3) -> Vec3 {
        let mut point = point - pivot.unwrap_or(Vec3::new(0., 0., 0.));

        let yz = rotate2d(Vec2::new(point.y, point.z), None, rotation.x);
        point.y = yz.x;
        point.z = yz.y;

        let xz = rotate2d(Vec2::new(point.x, point.z), None, rotation.y);
        point.x = xz.x;
        point.z = xz.y;

        let xy = rotate2d(Vec2::new(point.x, point.y), None, rotation.z);
        point.x = xy.x;
        point.y = xy.y;

        point + pivot.unwrap_or(Vec3::new(0., 0., 0.))
    }

    pub fn simplex_noise(x: f32, y: f32, octaves:  i32, x_frequency: f32, y_frequency: f32, amplitude: f32, seed: u64) -> f32 {
        let config = simple_simplex::NoiseConfig::new(octaves, x_frequency, y_frequency, amplitude, 2.5, 0.5, (0.0, 255.0), seed);
        config.generate_rangeless(x, y)
    }
    pub fn perlin_noise(x: f32, y: f32, seed: i32) -> f32 {
        let noise = PerlinNoise::new();
        noise.get2d([(x + seed as f32 * 3235.) as f64, (y + seed as f32 * 3235.) as f64]) as f32
    }

    pub fn fast_acos(x: f32, iterations: i32) -> f32 {
        let mut x = x;
        let mut angle = 0.0;
        let mut power_of_two = 1.0;
    
        for _ in 0..iterations {
            let sign = if x < 0.0 { 1.0 } else { -1.0 };
            let angle_increment = sign * power_of_two;
            let arccos_increment = angle_increment / ((1.0 + x.powi(2)).sqrt());
            angle += arccos_increment;
            x -= sign * (power_of_two / 2.0);
            power_of_two /= 2.0;
        }
    
        angle
    }
    pub fn lerp1d(p0: f32, p1: f32, t: f32) -> f32 {
        p0 * (1. - t) + p1 * t
    }
    pub fn lerp2d(p0: Vec2, p1: Vec2, t: f32) -> Vec2 {
        p0.scale(1. - t) + p1.scale(t)
    }
    pub fn lerp3d(p0: Vec3, p1: Vec3, t: f32) -> Vec3 {
        p0.scale(1. - t) + p1.scale(t)
    }
}