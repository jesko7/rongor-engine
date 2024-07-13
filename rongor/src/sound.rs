use macroquad;

#[derive(Debug, Clone)]
pub struct Sound {
    inner_sound: macroquad::audio::Sound,
    pub looped: bool,
    pub volume: f32
}


impl Sound {
    pub async fn new(path: &str, looped: bool, volume: f32) -> Sound {
        let audio = macroquad::audio::load_sound(path).await;
        
        if audio.is_err() {
            panic!("could not load audio path: {}", path)
        }

        Sound { inner_sound: audio.unwrap(), looped, volume}
    }

    pub fn get_sound_params(&self) -> macroquad::audio::PlaySoundParams {
        macroquad::audio::PlaySoundParams {
            looped: self.looped,
            volume: self.volume
        }
    }

    pub fn play(&self) {
        macroquad::audio::play_sound(&self.inner_sound, self.get_sound_params())
    }
}