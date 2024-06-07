use macroquad::prelude::*;

//mac:                  brew install fontconfig freetype
//linux:                sudo apt-get install libfontconfig1 libfreetype6


#[macroquad::main("Load Font Example")]
async fn main() {
    let font_result = load_ttf_font(r"assets\MADEAwelierPERSONALUSE-Regular.otf").await;
    let font = match font_result {
        Ok(font) => font,
        Err(e) => {
            eprintln!("Failed to load font: {}", e);
            std::process::exit(1);
        }
    };

    loop {
        clear_background(BLACK);

        draw_text_ex(
            "FUFUFUFUFUFUFUFUFUFU",
            300.0,
            300.0,
            TextParams {
                font: Some(&font),
                font_size: 100,
                color: WHITE,
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
