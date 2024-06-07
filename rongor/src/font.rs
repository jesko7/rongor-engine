use macroquad::prelude;


#[derive(Debug, Clone)]
pub struct Font {
    pub __f: prelude::Font
}

impl Font {
    pub async fn load_new_font(path: &str) -> Self {
        let mf = prelude::load_ttf_font(path).await;

        if mf.is_err() {
            panic!("could not load font {path}, error: {:?}", mf.unwrap_err());
        }

        let f = mf.unwrap();

        Font {
            __f: f
        }
    }
}