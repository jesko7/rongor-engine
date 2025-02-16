use macroquad::telemetry::textures_count;
use rongor::prelude::*;

pub fn scale_function_add(x: f32) -> f32 {
    (0 as f32).max(1. - (x - 0.).powf(2.)) * 10.
}

async fn run(window: Window) {
    let cam = Camera2d::default();

    let mut slider = Slider::new(Vec2::newi(600, 70), Vec2::newi(600, 530), 5., Color::RED, Color::BLUE, Color::GREEN, 0., 40., Color::GRAY, Color::DARKGRAY, 5.);
    let mut bar = Bar::new(Vec2::newi(600, 77), Vec2::newi(600, 300), 5., Color::PURPLE, Color::YELLOW, Color::BROWN, 0.5);
    let text_field = Textfield::new(Vec2::newi(200, 120), Vec2::newi(1450, 530), 10., 0.3, Color::DARKPURPLE, Color::MAGENTA, "hallo".to_string(), None, 80, Color::DARKBROWN, 0.);
    let mut button = Button::new(Vec2::newi(200, 90), Vec2::newi(1450, 760), 10., 0.7, Color::BLUE, Color::MAROON, Some("click".to_string()), None, Some(80), Some(Color::PINK), Some(0.), None, true, Some(scale_function_add));
    let mut switch = Switch::new(Vec2::newi(140, 70), Vec2::newi(670, 760), 5., Color::BROWN, Color::MAROON, Color::BEIGE, false, 40., Color::VIOLET, Color::SKYBLUE, 5.);
    let mut tick_box = TickBox::new(70., Vec2::newi(440, 760), 10., 0.5, Color::BLANK, Color::MAGENTA, Color::GREEN, false);
    let mut entrance_text_field = EntranceTextfield::new(Vec2::newi(200, 120), Vec2::newi(1450, 300), 10., 0.3, Color::DARKPURPLE, Color::MAGENTA, "".to_string(), None, 80, Color::DARKBROWN, 0., true, true, "_".to_string());
    

    let mut left = false;


    while window.update_screen().await {
        cam.set_as_current_cam();
        draw2d::clear(Color::BLACK);


        slider.draw(10);
        bar.draw(10);
        text_field.draw(10);
        button.draw(10);
        switch.draw(10);
        tick_box.draw(10);
        entrance_text_field.draw(100);

        if bar.fill_percent <= 0.1 || bar.fill_percent >= 0.9 {
            left = !left;
        }

        if left {
            bar.fill_percent -= 0.001;
        }
        else {
            bar.fill_percent += 0.001;
        }

    }
}


#[rongor::main]
async fn main() {
    start(run, "test".into(), 1920, 1080, false, true, true, AntialiasingType::Lanczos3).await
}