use std::f32::consts::PI;

use ndarray::AssignElem;
use rand::Rng;
use rongor::{draw, hitbox, math::math::{lerp1d, lerp2d, lerp3d}, prelude::*};

static mut T: f32 = 0.;
const num_of_triangles: i32 = 100;
static mut TRANSITION_TIME: f32 = 0.;
const TRANSITION_MAX_TIME: f32 = 15.;
static mut TRANSITION_TIME2: f32 = TRANSITION2_MAX_TIME;
const TRANSITION2_MAX_TIME: f32 = 35.;
static mut num_of_presses_on_page: i32 = 0;
static mut inMinecraft: bool = false;

pub fn scale_function_add(x: f32) -> f32 {
    (0 as f32).max(1. - (x - 0.).powf(2.)) * 10.
}

async fn end(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>) {
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Danke Für eure", Vec2::new(1920. / 2., 350. + TRANSITION_TIME * 111.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        draw2d::text::draw_text_center("Aufmerksamekit", Vec2::new(1920. / 2., 460. + TRANSITION_TIME * 111.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        draw2d::text::draw_text_center("das war's", Vec2::new(1920. / 2., 790. + TRANSITION_TIME * 111.), 60, Color::BLACK, 0., Some(fonts[0].clone()));
    };
}

fn wrap(x: f32, offset: f32, speed: f32) -> f32 {
    let x = x * speed + offset;

    x.sin() / 2. + 0.5
}

async unsafe fn draw_background(window: &Window, rngs: Vec<f32>) {
    draw2d::clear(Color::new(120, 120, 120, 255));

    //draw2d::triangle::triangle(
    //    math::lerp2d(Vec2::new(0., 0.), Vec2::new(43., 78.), wrap(T, 0.2523, 0.34)), 
    //    math::lerp2d(Vec2::new(0., 1080.), Vec2::new(200., 900.), wrap(T, 0.5357, 0.67)), 
    //    math::lerp2d(Vec2::new(523., 623.), Vec2::new(708., 201.), wrap(T, 0.7823, 0.92)), 
    //    Color::new(123, 123, 123, 104)
    //);
    
    let mut rng_index = 0;
    
    let w = (window.width / 2) as f32;
    let h = (window.height / 2) as f32;

    let w2 = (window.width / 2) as f32;
    let h2 = (window.height / 2) as f32;

    for _ in 0..num_of_triangles {
        draw2d::triangle::triangle(
            math::lerp2d(Vec2::new(rngs[0  + rng_index] / 2. * w + w2, rngs[1  + rng_index] / 2. * h + h2), Vec2::new(rngs[2  + rng_index] / 2. * w + w2, rngs[3  + rng_index] / 2. * h + h2), wrap(T, rngs[4  + rng_index], rngs[5  + rng_index] / 10.)), 
            math::lerp2d(Vec2::new(rngs[6  + rng_index] / 2. * w + w2, rngs[7  + rng_index] / 2. * h + h2), Vec2::new(rngs[8  + rng_index] / 2. * w + w2, rngs[9  + rng_index] / 2. * h + h2), wrap(T, rngs[10  + rng_index], rngs[11 + rng_index] / 10.)), 
            math::lerp2d(Vec2::new(rngs[12 + rng_index] / 2. * w + w2, rngs[13 + rng_index] / 2. * h + h2), Vec2::new(rngs[14 + rng_index] / 2. * w + w2, rngs[15 + rng_index] / 2. * h + h2), wrap(T, rngs[14 + rng_index], rngs[16 + rng_index] / 10.)), 
            Color::from_0to1(1. - rngs[17 + rng_index] / 2., 1. - rngs[18 + rng_index] / 2., 1. - rngs[19 + rng_index] / 2., 0.04)
        );

        rng_index += 20;
    }

    draw2d::square::square_top_left(Vec2::new(0., 0.), 1920., 1080., Color::new(0, 0, 0, 100));
    
    draw2d::triangle::triangle(Vec2::newi(0, 0), Vec2::newi(0, 450), Vec2::newi(234, 0), Color::from_0to1(0., 0., 0., 0.6));
    draw2d::triangle::triangle(Vec2::newi(0, 1080), Vec2::newi(395, 1080), Vec2::newi(0, 900), Color::from_0to1(0., 0., 0., 0.6));
    draw2d::triangle::triangle(Vec2::newi(1920, 0), Vec2::newi(1920, 450), Vec2::newi(1790, 0), Color::from_0to1(0., 0., 0., 0.6));
    draw2d::triangle::triangle(Vec2::newi(1920, 1080), Vec2::newi(1920, 780), Vec2::newi(1620, 1080), Color::from_0to1(0., 0., 0., 0.6));
}

async fn page0(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>) {
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Meine eigene", Vec2::new(1920. / 2., 350. + TRANSITION_TIME * 111.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        draw2d::text::draw_text_center("Game Engine", Vec2::new(1920. / 2., 460. + TRANSITION_TIME * 111.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        draw2d::text::draw_text_center("von jesko", Vec2::new(1920. / 2., 790. + TRANSITION_TIME * 111.), 60, Color::BLACK, 0., Some(fonts[0].clone()));
    };
}

async fn page1(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>) {
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Inhaltsverzeichnis", Vec2::new(1920. / 2., 220. + TRANSITION_TIME * 111.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        let width = draw2d::text::measure_text("Inhaltsverzeichnis", 110, Some(fonts[1].clone())).x;
        draw2d::line::line(Vec2::new(1920. / 2. - width / 2., 275. + TRANSITION_TIME * 111.), Vec2::new(1920. / 2. + width / 2., 275. + TRANSITION_TIME * 111.), 5., Color::BLACK);

        
        if num_of_presses_on_page >= 1 {
            if num_of_presses_on_page == 1 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                draw2d::text::draw_text_center("1: Was ist eine Game Engine", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111.), 90, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            else {
                draw2d::text::draw_text_center("1: Was ist eine Game Engine", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111.), 90, Color::BLACK, 0., Some(fonts[0].clone()));
            } 
        }

        if num_of_presses_on_page >= 2 {
            if num_of_presses_on_page == 2 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                draw2d::text::draw_text_center("2: Generelle Informationen", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111. + 110.), 90, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            else {
                draw2d::text::draw_text_center("2: Generelle Informationen", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111. + 110.), 90, Color::BLACK, 0., Some(fonts[0].clone()));
            } 
        }

        if num_of_presses_on_page >= 3 {
            if num_of_presses_on_page == 3 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                draw2d::text::draw_text_center("3: Elemente der Game Engine", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111.  + 220.), 90, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            else {
                draw2d::text::draw_text_center("3: Elemente der Game Engine", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111. + 220.), 90, Color::BLACK, 0., Some(fonts[0].clone()));
            } 
        }

        if num_of_presses_on_page >= 4 {
            if num_of_presses_on_page == 4 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                draw2d::text::draw_text_center("4: Beispiel Spiele mit der Game Engine", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111. + 330.), 90, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            else {
                draw2d::text::draw_text_center("4: Beispiel Spiele mit der Game Engine", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111. + 330.), 90, Color::BLACK, 0., Some(fonts[0].clone()));
            } 
        }
    };
}


async fn page2(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>) {
    unsafe {
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Was ist eine Game Engine?", Vec2::new(1920. / 2., 220. + TRANSITION_TIME * 111.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        let width = draw2d::text::measure_text("Was ist eine Game Engine?", 110, Some(fonts[1].clone())).x;
        draw2d::line::line(Vec2::new(1920. / 2. - width / 2., 285. + TRANSITION_TIME * 111.), Vec2::new(1920. / 2. + width / 2., 285. + TRANSITION_TIME * 111.), 5., Color::BLACK);
    
        if num_of_presses_on_page >= 1 {
            if num_of_presses_on_page == 1 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                draw2d::text::draw_text_center("• eine Game Engine ist hauptsächlich", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111. + 0.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("  dazu da um Spiele zu entwickeln", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111. + 110.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            else {
                draw2d::text::draw_text_center("• eine Game Engine ist hauptsächlich", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111. + 0.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("  dazu da um Spiele zu entwickeln", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111. + 110.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            } 
        }

        if num_of_presses_on_page >= 2 {
            if num_of_presses_on_page == 2 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                draw2d::text::draw_text_center("• die Game Engine gibt einem Elemente ", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111. + 220.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("  für die Entwicklung von Spielen", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111. + 330.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            else {
                draw2d::text::draw_text_center("• die Game Engine gibt einem Elemente ", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111. + 220.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("  für die Entwicklung von Spielen", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111. + 330.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            } 
        }

        if num_of_presses_on_page >= 3 {
            if num_of_presses_on_page == 3 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                draw2d::text::draw_text_center("• die Game Engine nimmt einem auch viel Arbeit", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111.  + 440.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("   bei dem programmieren von Videospielen ab", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111.  + 550.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            else {
                draw2d::text::draw_text_center("• die Game Engine nimmt einem auch viel Arbeit", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111.  + 440.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("   bei dem Programmieren von Videospielen ab", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111.  + 550.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            } 
        }
    };
}

async fn page3(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>) {
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Generelle Informationen", Vec2::new(1920. / 2., 220. + TRANSITION_TIME * 111.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        let width = draw2d::text::measure_text("Generelle Informationen", 110, Some(fonts[1].clone())).x;
        draw2d::line::line(Vec2::new(1920. / 2. - width / 2., 275. + TRANSITION_TIME * 111.), Vec2::new(1920. / 2. + width / 2., 285. + TRANSITION_TIME * 111.), 5., Color::BLACK);

        if num_of_presses_on_page >= 1 {
            if num_of_presses_on_page == 1 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                draw2d::text::draw_text_center("• die Game Engine ist in einer Programmier-", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111.  + 0.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("  sprache namens Rust programmiert", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111.  + 110.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            else {
                draw2d::text::draw_text_center("• die Game Engine ist in einer Programmier-", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111.  + 0.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("  sprache namens Rust programmiert", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111.  + 110.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            } 
        }

        if num_of_presses_on_page >= 2 {
            if num_of_presses_on_page == 2 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                draw2d::text::draw_text_center("• diese Präsentation ist mit der Game Engine", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111.  + 220.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("  programmiert und nicht mit PowerPoint", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111.  + 330.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            else {
                draw2d::text::draw_text_center("• diese Präsentation ist mit der Game Engine", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111.  + 220.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("  programmiert und nicht mit PowerPoint", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111.  + 330.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            } 
        }

        if num_of_presses_on_page >= 3 {
            if num_of_presses_on_page == 3 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                draw2d::text::draw_text_center("• der code der Game Engine ist", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111.  + 440.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("  c.a. 4700 Zeilen lang", Vec2::new(1920. / 2. - TRANSITION_TIME2 * 60., 410. + TRANSITION_TIME * 111.  + 550.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            else {
                draw2d::text::draw_text_center("• der code der Game Engine ist", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111.  + 440.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("  c.a. 4700 Zeilen lang", Vec2::new(1920. / 2., 410. + TRANSITION_TIME * 111.  + 550.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            } 
        }
    }
}

async fn page4(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>){
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Elemente der Game Engine", Vec2::new(1920. / 2., 220. + (TRANSITION_TIME * 111.).min(0.)), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        let width = draw2d::text::measure_text("Elemente der Game Engine", 110, Some(fonts[1].clone())).x;
        draw2d::line::line(Vec2::new(1920. / 2. - width / 2., 285. + (TRANSITION_TIME * 111.).min(0.)), Vec2::new(1920. / 2. + width / 2., 285. + (TRANSITION_TIME * 111.).min(0.)), 5., Color::BLACK);
        
        {
            if num_of_presses_on_page == 1 {
                if num_of_presses_on_page == 1 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    draw2d::square::square_middle(Vec2::new(400. - TRANSITION_TIME2 * 60., 600. + TRANSITION_TIME * 111.), 100., 100., Color::WHITE);
                }
                else if num_of_presses_on_page == 1 {
                    draw2d::square::square_middle(Vec2::new(400., 600. + TRANSITION_TIME * 111.), 100., 100., Color::WHITE); 
                }
            }
            if num_of_presses_on_page == 2 {
                draw2d::square::square_middle(Vec2::new(400. + (T * 10.).cos() * 60., 600. + (T * 10.).sin() * 60. + TRANSITION_TIME * 111.), 100., 100., Color::WHITE);
            }

            if num_of_presses_on_page == 3 {
                draw2d::square::square_middle(Vec2::new(400. + (T * 10.).cos() * 60., 600. + (T * 10.).sin() * 60. + TRANSITION_TIME * 111.), lerp1d(100., 160., wrap(T, 2.523, 2.7)), lerp1d(100., 160., wrap(T, 1.9312, 2.4)), Color::WHITE);
            }
            
            if num_of_presses_on_page == 4 {
                draw2d::square::square_middle(Vec2::new(400. + (T * 10.).cos() * 60., 600. + (T * 10.).sin() * 60. + TRANSITION_TIME * 111.), lerp1d(100., 160., wrap(T, 2.523, 2.7)), lerp1d(100., 160., wrap(T, 1.9312, 2.4)), 
                Color::from_0to1(wrap(T, 0.234975213, 0.8034586), wrap(T, 0.92346262, 1.324962), wrap(T, 0.640340976, 1.622356), 1.3 - wrap(T, 0.352, 1.72346)));
            }

            if num_of_presses_on_page >= 5 {
                draw2d::square::square_middle_lines(Vec2::new(400. + (T * 10.).cos() * 60., 600. + (T * 10.).sin() * 60. + TRANSITION_TIME * 111.), lerp1d(100., 160., wrap(T, 2.523, 2.7)), lerp1d(100., 160., wrap(T, 1.9312, 2.4)), lerp1d(10., 50., wrap(T, 1.9312, 2.4)),
                Color::from_0to1(wrap(T, 0.234975213, 0.8034586), wrap(T, 0.92346262, 1.324962), wrap(T, 0.640340976, 1.622356), 1.3 - wrap(T, 0.352, 1.72346)));
            }
        }

        {
            if num_of_presses_on_page == 6 {
                if num_of_presses_on_page == 6 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    draw2d::circle::draw_smooth_circle(Vec2::new(800. - TRANSITION_TIME2 * 60., 600. + TRANSITION_TIME * 111.), 100. / 2., 100, Color::WHITE);
                }
                else if num_of_presses_on_page == 6 {
                    draw2d::circle::draw_smooth_circle(Vec2::new(800., 600. + TRANSITION_TIME * 111.), 100. / 2., 100, Color::WHITE);
                }
            }
            if num_of_presses_on_page == 7 {
                draw2d::circle::draw_smooth_circle(Vec2::new(800. + (T * 10. + 0.9347).cos() * 60., 600. + (T * 10. + 0.9347).sin() * 60. + TRANSITION_TIME * 111.), 100. / 2., 100, Color::WHITE);
            }

            if num_of_presses_on_page == 8 {
                draw2d::circle::draw_smooth_circle(Vec2::new(800. + (T * 10. + 0.9347).cos() * 60., 600. + (T * 10. + 0.9347).sin() * 60. + TRANSITION_TIME * 111.), lerp1d(100., 160., wrap(T, 2.1637, 2.7)) / 2., 100, Color::WHITE);
            }
            
            if num_of_presses_on_page >= 9 {
                draw2d::circle::draw_smooth_circle(Vec2::new(800. + (T * 10. + 0.9347).cos() * 60., 600. + (T * 10. + 0.9347).sin() * 60. + TRANSITION_TIME * 111.), lerp1d(100., 160., wrap(T, 2.92323, 2.7)) / 2., 100, 
                Color::from_0to1(wrap(T, 0.234975213, 0.234767), wrap(T, 0.2384096, 1.9272437), wrap(T, 0.703450987, 1.7298067), 1.3 - wrap(T, 0.352, 1.72346)));
            }
        }

        {
            if num_of_presses_on_page == 10 {
                if num_of_presses_on_page == 10 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    draw2d::triangle::triangle( 
                        Vec2::new(1200. - TRANSITION_TIME2 * 60., 600. + TRANSITION_TIME * 111.), 
                    Vec2::new(1200. - TRANSITION_TIME2 * 60., 750. + TRANSITION_TIME * 111.), 
                    Vec2::new(1350. - TRANSITION_TIME2 * 60., 600. + TRANSITION_TIME * 111.), 
                    Color::WHITE
                    );
                }
                else if num_of_presses_on_page == 10 {
                    draw2d::triangle::triangle( 
                        Vec2::new(1200., 600. + TRANSITION_TIME * 111.), 
                        Vec2::new(1200., 750. + TRANSITION_TIME * 111.),
                         Vec2::new(1350., 600. + TRANSITION_TIME * 111.), 
                         Color::WHITE
                    );
                }
            }
            if num_of_presses_on_page == 11 {
                draw2d::triangle::triangle(
                    lerp2d(Vec2::new(1200. - 12., 600. - 24.), Vec2::new(1200. + 43., 600. + 33.), wrap(T, 0.62355, 2.623)) + Vec2::new(0., TRANSITION_TIME * 111.),
                    lerp2d(Vec2::new(1200. - 32., 750. - 35.), Vec2::new(1200. + 12., 700. + 22.), wrap(T, 0.2346623, 2.95623)) + Vec2::new(0., TRANSITION_TIME * 111.),
                    lerp2d(Vec2::new(1350. - 23., 600. - 10.), Vec2::new(1300. + 25., 600. + 63.), wrap(T, 0.83467, 2.2366)) + Vec2::new(0., TRANSITION_TIME * 111.),
                Color::WHITE);
            }
            
            if num_of_presses_on_page >= 12 {
                draw2d::triangle::triangle(
                    lerp2d(Vec2::new(1200. - 12., 600. - 24.), Vec2::new(1200. + 43., 600. + 33.), wrap(T, 0.62355, 2.623)) + Vec2::new(0., TRANSITION_TIME * 111.),
                    lerp2d(Vec2::new(1200. - 32., 750. - 35.), Vec2::new(1200. + 12., 700. + 22.), wrap(T, 0.2346623, 2.95623)) + Vec2::new(0., TRANSITION_TIME * 111.),
                    lerp2d(Vec2::new(1350. - 23., 600. - 10.), Vec2::new(1300. + 25., 600. + 63.), wrap(T, 0.83467, 2.2366)) + Vec2::new(0., TRANSITION_TIME * 111.),
                    Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.3 - wrap(T, 0.62345, 1.72346))
                );
            }
        }

        {
            if num_of_presses_on_page == 13 {
                if num_of_presses_on_page == 13 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    draw2d::line::line( Vec2::new(600. - TRANSITION_TIME2 * 60., 900. + TRANSITION_TIME * 60.), Vec2::new(1200. - TRANSITION_TIME2 * 60., 900. + TRANSITION_TIME * 111.), 10., Color::WHITE);
                }
                else if num_of_presses_on_page == 13 {
                    draw2d::line::line( Vec2::new(600., 900. + TRANSITION_TIME * 111.), Vec2::new(1200., 900. + TRANSITION_TIME * 111.), 10., Color::WHITE);
                }
            }
            if num_of_presses_on_page == 14 {
                draw2d::line::line(
                    Vec2::new(600. + (T * 10. + 0.823532).cos() * 60., 900. + (T * 10. + 0.823532).sin() * 60. + TRANSITION_TIME * 111.), 
                    Vec2::new(1200. + (T * 4.5 + 0.15245).cos() * 60., 900. + (T * 4.5 + 0.15245).sin() * 60. + TRANSITION_TIME * 111.), 
                    10., 
                Color::WHITE);
            }
            
            if num_of_presses_on_page == 15 {
                draw2d::line::line(
                    Vec2::new(600. + (T * 10. + 0.823532).cos() * 60., 900. + (T * 10. + 0.823532).sin() * 60. + TRANSITION_TIME * 111.), 
                    Vec2::new(1200. + (T * 4.5 + 0.15245).cos() * 60., 900. + (T * 4.5 + 0.15245).sin() * 60. + TRANSITION_TIME * 111.), 
                    10., 
                    Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.3 - wrap(T, 0.62345, 1.72346))
                );
            }

            if num_of_presses_on_page >= 16 {
                draw2d::line::line(
                    Vec2::new(600. + (T * 10. + 0.823532).cos() * 60., 900. + (T * 10. + 0.823532).sin() * 60. + TRANSITION_TIME * 111.), 
                    Vec2::new(1200. + (T * 4.5 + 0.15245).cos() * 60., 900. + (T * 4.5 + 0.15245).sin() * 60. + TRANSITION_TIME * 111.),  
                    lerp1d(10., 50., wrap(T, 1.9312, 2.4)),
                    Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.3 - wrap(T, 0.62345, 1.72346))
                );
            }
        }
   };
}

async fn page5(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>) {
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Elemente der Game Engine", Vec2::new(1920. / 2., 220.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        let width = draw2d::text::measure_text("Elemente der Game Engine", 110, Some(fonts[1].clone())).x;
        draw2d::line::line(Vec2::new(1920. / 2. - width / 2., 285.), Vec2::new(1920. / 2. + width / 2., 285.), 5., Color::BLACK);
        
        cam3d.set_as_current_cam();

        {
            if num_of_presses_on_page == 1 {
                if num_of_presses_on_page == 1 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    draw3d::cube::cube_center(
                        Vec3::new(2. + TRANSITION_TIME2 / 2., -1., 4.),
                        Vec3::new(1., 1., 1.),
                        None, 
                        Color::WHITE
                    );
                }
                else if num_of_presses_on_page == 1 {
                    draw3d::cube::cube_center(
                        Vec3::new(2., -1., 4.),
                        Vec3::new(1., 1., 1.),
                        None, 
                        Color::WHITE
                    );
                }
            }
            if num_of_presses_on_page == 2 {
                draw3d::cube::cube_center(
                    Vec3::new(2. + wrap(T, 0.623423, 3.5234), -1. + wrap(T, 1.2366234, 3.762345), 4. + wrap(T, 2.873546, 3.111)),
                    Vec3::new(1., 1., 1.),
                    None, 
                    Color::WHITE
                );
            }

            if num_of_presses_on_page == 3 {
                draw3d::cube::cube_center(
                    Vec3::new(2. + wrap(T, 0.623423, 3.5234), -1. + wrap(T, 1.2366234, 3.762345), 4. + wrap(T, 2.873546, 3.111)),
                    Vec3::new(1. + wrap(T, 0.623423, 2.5234) / 3., 1. + wrap(T, 1.2366234, 2.762345) / 3., 1. + wrap(T, 2.873546, 2.111) / 3.),
                    None, 
                    Color::WHITE
                );
            }
            
            if num_of_presses_on_page == 4 {
                draw3d::cube::cube_center(
                    Vec3::new(2. + wrap(T, 0.623423, 3.5234), -1. + wrap(T, 1.2366234, 3.762345), 4. + wrap(T, 2.873546, 3.111)),
                    Vec3::new(1. + wrap(T, 0.623423, 2.5234) / 3., 1. + wrap(T, 1.2366234, 2.762345) / 3., 1. + wrap(T, 2.873546, 2.111) / 3.),
                    None, 
                    Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.)
                );
            }

            if num_of_presses_on_page == 5 {
                draw3d::cube::cube_center(
                    Vec3::new(2. + wrap(T, 0.623423, 3.5234), -1. + wrap(T, 1.2366234, 3.762345), 4. + wrap(T, 2.873546, 3.111)),
                    Vec3::new(1. + wrap(T, 0.623423, 2.5234) / 3., 1. + wrap(T, 1.2366234, 2.762345) / 3., 1. + wrap(T, 2.873546, 2.111) / 3.),
                    Some(&textures[0]), 
                    Color::WHITE
                );
            }
            if num_of_presses_on_page == 6 {
                draw3d::cube::cube_center(
                    Vec3::new(2. + wrap(T, 0.623423, 3.5234), -1. + wrap(T, 1.2366234, 3.762345), 4. + wrap(T, 2.873546, 3.111)),
                    Vec3::new(1. + wrap(T, 0.623423, 2.5234) / 3., 1. + wrap(T, 1.2366234, 2.762345) / 3., 1. + wrap(T, 2.873546, 2.111) / 3.),
                    Some(&textures[1]), 
                    Color::WHITE
                );
            }
            if num_of_presses_on_page == 7 {
                draw3d::cube::cube_center(
                    Vec3::new(2. + wrap(T, 0.623423, 3.5234), -1. + wrap(T, 1.2366234, 3.762345), 4. + wrap(T, 2.873546, 3.111)),
                    Vec3::new(1. + wrap(T, 0.623423, 2.5234) / 3., 1. + wrap(T, 1.2366234, 2.762345) / 3., 1. + wrap(T, 2.873546, 2.111) / 3.),
                    Some(&textures[0]), 
                    Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.)
                );
            }
            if num_of_presses_on_page == 8 {
                draw3d::cube::cube_center(
                    Vec3::new(2. + wrap(T, 0.623423, 3.5234), -1. + wrap(T, 1.2366234, 3.762345), 4. + wrap(T, 2.873546, 3.111)),
                    Vec3::new(1. + wrap(T, 0.623423, 2.5234) / 3., 1. + wrap(T, 1.2366234, 2.762345) / 3., 1. + wrap(T, 2.873546, 2.111) / 3.),
                    Some(&textures[1]), 
                    Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.)
                );
            }
            if num_of_presses_on_page >= 9 {
                draw3d::cube::cube_center(
                    Vec3::new(2. + wrap(T, 0.623423, 3.5234), -1. + wrap(T, 1.2366234, 3.762345) - TRANSITION_TIME / 1.5, 4. + wrap(T, 2.873546, 3.111)),
                    Vec3::new(1. + wrap(T, 0.623423, 2.5234) / 3., 1. + wrap(T, 1.2366234, 2.762345) / 3., 1. + wrap(T, 2.873546, 2.111) / 3.),
                    Some(&textures[1]), 
                    Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.)
                );
            }
        }

        {
            if num_of_presses_on_page == 9 {
                if num_of_presses_on_page == 9 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    draw3d::sphere::sphere_center(
                        Vec3::new(-2. + TRANSITION_TIME2 / 2., -1., 4.),
                        0.5,
                        None, 
                        Color::WHITE
                    );
                }
                else if num_of_presses_on_page == 9 {
                    draw3d::sphere::sphere_center(
                        Vec3::new(-2., -1., 4.),
                        0.5,
                        None, 
                        Color::WHITE
                    );
                }
            }
            if num_of_presses_on_page == 10 {
                draw3d::sphere::sphere_center(
                    Vec3::new(-2. + wrap(T, 0.2362, 3.5234), -1. + wrap(T, 1.23623, 3.762345), 4. + wrap(T, 2.22, 3.111)),
                    0.5,
                    None, 
                    Color::WHITE
                );
            }

            if num_of_presses_on_page == 11 {
                draw3d::sphere::sphere_center(
                    Vec3::new(-2. + wrap(T, 0.2362, 3.5234), -1. + wrap(T, 1.23623, 3.762345), 4. + wrap(T, 2.22, 3.111)),
                    0.5 + wrap(T, 0.253623, 3.62345) / 3.,
                    None, 
                    Color::WHITE
                );
            }
            
            if num_of_presses_on_page == 12 {
                draw3d::sphere::sphere_center(
                    Vec3::new(-2. + wrap(T, 0.2362, 3.5234), -1. + wrap(T, 1.23623, 3.762345), 4. + wrap(T, 2.22, 3.111)),
                    0.5 + wrap(T, 0.253623, 3.62345) / 3.,
                    None, 
                    Color::from_0to1(wrap(T, 0.2436235, 0.234767), wrap(T, 0.73452, 1.9272437), wrap(T, 0.72345, 1.7298067), 1.)
                );
            }

            if num_of_presses_on_page == 13 {
                draw3d::sphere::sphere_center(
                    Vec3::new(-2. + wrap(T, 0.2362, 3.5234), -1. + wrap(T, 1.23623, 3.762345), 4. + wrap(T, 2.22, 3.111)),
                    0.5 + wrap(T, 0.253623, 3.62345) / 3.,
                    Some(&textures[0]), 
                    Color::WHITE
                );
            }
            if num_of_presses_on_page == 14 {
                draw3d::sphere::sphere_center(
                    Vec3::new(-2. + wrap(T, 0.2362, 3.5234), -1. + wrap(T, 1.23623, 3.762345), 4. + wrap(T, 2.22, 3.111)),
                    0.5 + wrap(T, 0.253623, 3.62345) / 3.,
                    Some(&textures[1]),  
                    Color::WHITE
                );
            }
            if num_of_presses_on_page == 15 {
                draw3d::sphere::sphere_center(
                    Vec3::new(-2. + wrap(T, 0.2362, 3.5234), -1. + wrap(T, 1.23623, 3.762345), 4. + wrap(T, 2.22, 3.111)),
                    0.5 + wrap(T, 0.253623, 3.62345) / 3.,
                    Some(&textures[0]),  
                    Color::from_0to1(wrap(T, 0.2436235, 0.234767), wrap(T, 0.73452, 1.9272437), wrap(T, 0.72345, 1.7298067), 1.)
                );
            }
            if num_of_presses_on_page == 16 {
                draw3d::sphere::sphere_center(
                    Vec3::new(-2. + wrap(T, 0.2362, 3.5234), -1. + wrap(T, 1.23623, 3.762345), 4. + wrap(T, 2.22, 3.111)),
                    0.5 + wrap(T, 0.253623, 3.62345) / 3.,
                    Some(&textures[1]),  
                    Color::from_0to1(wrap(T, 0.2436235, 0.234767), wrap(T, 0.73452, 1.9272437), wrap(T, 0.72345, 1.7298067), 1.)
                );
            }
            if num_of_presses_on_page == 17 {
                draw3d::sphere::wire_sphere_center(
                    Vec3::new(-2. + wrap(T, 0.2362, 3.5234), -1. + wrap(T, 1.23623, 3.762345), 4. + wrap(T, 2.22, 3.111)),
                    0.5 + wrap(T, 0.253623, 3.62345) / 3.,
                    Color::from_0to1(wrap(T, 0.2436235, 0.234767), wrap(T, 0.73452, 1.9272437), wrap(T, 0.72345, 1.7298067), 1.)
                );
            }
            if num_of_presses_on_page >= 18 {
                draw3d::sphere::wire_sphere_center(
                    Vec3::new(-2. + wrap(T, 0.2362, 3.5234), -1. + wrap(T, 1.23623, 3.762345) - TRANSITION_TIME / 1.5, 4. + wrap(T, 2.22, 3.111)),
                    0.5 + wrap(T, 0.253623, 3.62345) / 3.,
                    Color::from_0to1(wrap(T, 0.2436235, 0.234767), wrap(T, 0.73452, 1.9272437), wrap(T, 0.72345, 1.7298067), 1.)
                );
            }
        }
   };
}

async fn page6(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>) {
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Elemente der Game Engine", Vec2::new(1920. / 2., 220.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        let width = draw2d::text::measure_text("Elemente der Game Engine", 110, Some(fonts[1].clone())).x;
        draw2d::line::line(Vec2::new(1920. / 2. - width / 2., 285.), Vec2::new(1920. / 2. + width / 2., 285.), 5., Color::BLACK);

        {
            if num_of_presses_on_page == 1 {
                if num_of_presses_on_page == 1 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    draw2d::image::image_center(Vec2::new(400. - TRANSITION_TIME2 * 66., 600.), &textures[3])
                }
                else if num_of_presses_on_page == 1 {
                    draw2d::image::image_center(Vec2::new(400., 600.), &textures[3])
                }
            }
            
            if num_of_presses_on_page >= 2 {
                draw2d::image::image_center(Vec2::new(400. + (T * 10.).cos() * 60., 600. + (T * 10.).sin() * 60. + TRANSITION_TIME * 110.), &textures[3]);
            }
            if num_of_presses_on_page >= 3 {
                textures[3].size = Vec2::new(lerp1d(200., 260., wrap(T, 2.523, 3.7)), lerp1d(200., 260., wrap(T, 1.9312, 3.4)));
            }
            if num_of_presses_on_page >= 4 {
                textures[3].flip_x = true;
            }
            if num_of_presses_on_page >= 5 {
                textures[3].flip_y = true;
            }
            if num_of_presses_on_page >= 6 {
                textures[3].pivot = Some(Vec2::new(400. + (T * 10.).cos() * 60., 600. + (T * 10.).sin() * 60.));
                textures[3].rotation = T;
            }
        }
        println!("{}", num_of_presses_on_page);
        {
            if num_of_presses_on_page == 7 {
                if num_of_presses_on_page == 7 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200. - TRANSITION_TIME2 * 66., 500.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                }
                else if num_of_presses_on_page == 7 {
                    draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 500.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                }
            }
                // + (T * 10. + 0.9347).sin()  + (T * 10. + 0.9347).cos() Vec2::new(1200. + (T * 10. + 0.9347).cos() * 60., 500. + (T * 10. + 0.9347).sin() * 60.)
            println!("{}", num_of_presses_on_page);
            if num_of_presses_on_page == 8 {
                println!("sdfsd");
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200. + (T * 10. + 0.9347).cos() * 60., 500. + (T * 10. + 0.9347).sin() * 60.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            if num_of_presses_on_page == 9 {
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200. + (T * 10. + 0.9347).cos() * 60., 500. + (T * 10. + 0.9347).sin() * 60.), 70 + (wrap(T, 0.74235, 3.325) * 40.) as u16, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            if num_of_presses_on_page == 10 {
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200. + (T * 10. + 0.9347).cos() * 60., 500. + (T * 10. + 0.9347).sin() * 60.), 70 + (wrap(T, 0.74235, 0.325) * 10.) as u16, Color::BLACK, T, Some(fonts[0].clone()));
            }
            if num_of_presses_on_page == 11 {
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200. + (T * 10. + 0.9347).cos() * 60., 500. + (T * 10. + 0.9347).sin() * 60.), 70 + (wrap(T, 0.74235, 0.325) * 10.) as u16, Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.), T, Some(fonts[0].clone()));
            }
            if num_of_presses_on_page == 12 {
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 500.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
            }
            if num_of_presses_on_page == 13 {
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 500.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 580.), 70, Color::BLACK, 0., Some(fonts[1].clone()));
            }
            if num_of_presses_on_page == 14 {
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 500.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 580.), 70, Color::BLACK, 0., Some(fonts[1].clone()));
                draw2d::text::draw_text_center("Drehturmodell", Vec2::new(1200., 660.), 70, Color::BLACK, 0., Some(fonts[2].clone()));
            }
            if num_of_presses_on_page == 15 {
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 500.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 580.), 70, Color::BLACK, 0., Some(fonts[1].clone()));
                draw2d::text::draw_text_center("Drehturmodell", Vec2::new(1200., 660.), 70, Color::BLACK, 0., Some(fonts[2].clone()));
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 740.), 50, Color::BLACK, 0., Some(fonts[3].clone()));
            }
            if num_of_presses_on_page >= 16 {
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 500. + TRANSITION_TIME * 110.), 70, Color::BLACK, 0., Some(fonts[0].clone()));
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 580. + TRANSITION_TIME * 110.), 70, Color::BLACK, 0., Some(fonts[1].clone()));
                draw2d::text::draw_text_center("Drehturmodell", Vec2::new(1200., 660. + TRANSITION_TIME * 110.), 70, Color::BLACK, 0., Some(fonts[2].clone()));
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 740. + TRANSITION_TIME * 110.), 50, Color::BLACK, 0., Some(fonts[3].clone()));
                draw2d::text::draw_text_center("Drehtürmodell", Vec2::new(1200., 860. + TRANSITION_TIME * 110.), 70, Color::BLACK, 0., Some(fonts[4].clone()));
            }
        }
   };
}

async fn page7(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>) {
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Elemente der Game Engine", Vec2::new(1920. / 2., 220.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        let width = draw2d::text::measure_text("Elemente der Game Engine", 110, Some(fonts[1].clone())).x;
        draw2d::line::line(Vec2::new(1920. / 2. - width / 2., 285.), Vec2::new(1920. / 2. + width / 2., 285.), 5., Color::BLACK);

        {   
            let hitbox1 = hitbox::SquareHitbox::new(mouse::get_position(), Vec2::new(100., 160.));
            let hitbox2 = hitbox::SquareHitbox::new(Vec2::new(600., 800.), Vec2::new(100., 160.));
            
            if num_of_presses_on_page >= 1  && num_of_presses_on_page < 4 {
                if hitbox1.collision_square(hitbox2.clone()) || hitbox1.collision_point(Vec2::new(700., 600.)) {
                    draw2d::square::square_middle(mouse::get_position(), 100., 160., Color::new(0, 255, 0, 160));
                }
                else {
                    draw2d::square::square_middle(mouse::get_position(), 100., 160., Color::new(255, 0, 0, 255));
                }
            }
            if num_of_presses_on_page >= 2 && num_of_presses_on_page < 4 {
                if hitbox1.collision_square(hitbox2.clone()) {
                    draw2d::square::square_middle(Vec2::new(600., 800.), 100., 160., Color::new(0, 255, 0, 160));
                }
                else {
                    draw2d::square::square_middle(Vec2::new(600., 800.), 100., 160., Color::new(255, 0, 0, 255));
                }
            }

            if num_of_presses_on_page == 3 {
                if hitbox1.collision_point(Vec2::new(700., 600.)) {
                    draw2d::circle::draw_smooth_circle(Vec2::new(700., 600.), 6., 100, Color::new(0, 255, 0, 160));
                }
                else {
                    draw2d::circle::draw_smooth_circle(Vec2::new(700., 600.), 6., 100, Color::new(255, 0, 0, 255));
                }
            }
        }

        {   
            cam3d.set_as_current_cam();

            let hitbox1 = hitbox::CubeHitbox::new(Vec3::new(1.5, 0.2436, 5.), Vec3::new(1., 1., 1.));
            let mut hitbox2 = hitbox::CubeHitbox::new(Vec3::new(-1.5, -0.634, 3.5), Vec3::new(1., 1., 1.));
            
            if num_of_presses_on_page == 6 {
                hitbox2.position = lerp3d(Vec3::new(-1.5, -0.634, 3.5), Vec3::new(1.5, 0.2436, 5.), wrap(T, PI / 2., 1.63));
            }

            if num_of_presses_on_page >= 4 {
                if hitbox1.collision_cube(hitbox2.clone()) {
                    draw3d::cube::cube_center(hitbox1.position, hitbox1.size, Some(&textures[2]), Color::GREEN);
                }
                else {
                    draw3d::cube::cube_center(hitbox1.position, hitbox1.size, Some(&textures[2]), Color::RED);
                }
            }
            if num_of_presses_on_page >= 5 {
                if hitbox1.collision_cube(hitbox2.clone()) {
                    draw3d::cube::cube_center(hitbox2.position, hitbox2.size, Some(&textures[2]), Color::GREEN);
                }
                else {
                    draw3d::cube::cube_center(hitbox2.position, hitbox2.size, Some(&textures[2]), Color::RED);
                }
            }
        }
   };
}


async fn page8(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>) {
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Elemente der Game Engine", Vec2::new(1920. / 2., 220.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        let width = draw2d::text::measure_text("Elemente der Game Engine", 110, Some(fonts[1].clone())).x;
        draw2d::line::line(Vec2::new(1920. / 2. - width / 2., 285.), Vec2::new(1920. / 2. + width / 2., 285.), 5., Color::BLACK);

        {
            if num_of_presses_on_page == 1 {
                if num_of_presses_on_page == 1 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    draw2d::square::square_middle(Vec2::new(1920. / 2. - TRANSITION_TIME2 * 66., 1080. / 2.), 400., 400., Color::RED);
                }
                else if num_of_presses_on_page == 1 {
                    draw2d::square::square_middle(Vec2::new(1920. / 2., 1080. / 2.), 400., 400., Color::RED);
                }
            }
            
            if num_of_presses_on_page == 2 {
                draw2d::square::square_middle(mouse::get_position(), 400., 400., Color::RED);
            }
            if num_of_presses_on_page == 3 {

                draw2d::square::square_middle(mouse::get_position(), 400., 400., 
            
            if mouse::is_pressed(mouse::LEFT) {
                        Color::GREEN
                    }
                    else {
                        Color::RED
                    }   
                );
            }


            if num_of_presses_on_page >= 4 {

                draw2d::square::square_middle(Vec2::new(1920. / 2., 1080. / 2. + TRANSITION_TIME * 111.), 400., 400., 
            
            if key::is_pressed("k") {
                        Color::GREEN
                    }
                    else {
                        Color::RED
                    }   
                );
            }
        }
   };
}

async fn page9(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>) {
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Elemente der Game Engine", Vec2::new(1920. / 2., 220.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        let width = draw2d::text::measure_text("Elemente der Game Engine", 110, Some(fonts[1].clone())).x;
        draw2d::line::line(Vec2::new(1920. / 2. - width / 2., 285.), Vec2::new(1920. / 2. + width / 2., 285.), 5., Color::BLACK);

        {
            if num_of_presses_on_page == 1 {
                if num_of_presses_on_page == 1 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    let curve = QuadraticBeziercurve::new(Vec2::new(1920. / 3. - TRANSITION_TIME2 * 66., 1080. / 2.), Vec2::new(1920. - 1920. / 3. - TRANSITION_TIME2 * 66., 1080. / 2.), 
                    Vec2::new(1920. / 2. - TRANSITION_TIME2 * 66., 1080. / 1.5),
                    0., 10., Color::BLACK);

                    curve.draw(100)
                }
                else if num_of_presses_on_page == 1 {
                    let curve = QuadraticBeziercurve::new(Vec2::new(1920. / 3., 1080. / 2.), Vec2::new(1920. - 1920. / 3., 1080. / 2.), 
                    Vec2::new(1920. / 2., 1080. / 1.5),
                    0., 10., Color::BLACK);

                    curve.draw(100)
                }
            }   
            if num_of_presses_on_page == 2 {
                let curve = QuadraticBeziercurve::new(Vec2::new(1920. / 3., 1080. / 2.), Vec2::new(1920. - 1920. / 3., 1080. / 2.), 
                Vec2::new(1920. / 2., 1080. / 1.5),
                0., 10., Color::BLACK);

                draw2d::circle::circle(Vec2::new(1920. / 3., 1080. / 2.), 10., Color::WHITE);
                draw2d::circle::circle(Vec2::new(1920. - 1920. / 3., 1080. / 2.), 10., Color::WHITE);
                draw2d::circle::circle(Vec2::new(1920. / 2., 1080. / 1.5), 10., Color::WHITE);

                curve.draw(100)
            }
            if num_of_presses_on_page == 3 {
                let curve = QuadraticBeziercurve::new(
                    if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2.)
                    },
                    if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                    },
                
                    if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2., 1080. / 1.5)
                    },
                
                
    
                0., 10., Color::BLACK);

                draw2d::circle::circle(if key::is_pressed("key1") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 3., 1080. / 2.)
                }, 10., Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key2") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                }, 10., Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key3") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 2., 1080. / 1.5)
                }, 10., Color::WHITE);

                curve.draw(100)
            }
            if num_of_presses_on_page == 4 {
                let dicke = 10. + wrap(T, 0.6234, 3.2345) * 20.;

                let curve = QuadraticBeziercurve::new(
                    if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2.)
                    },
                    if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                    },
                
                    if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2., 1080. / 1.5)
                    },
                
                
    
                0., dicke, Color::BLACK);
                
                draw2d::circle::circle(if key::is_pressed("key1") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 3., 1080. / 2.)
                }, dicke, Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key2") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                }, dicke, Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key3") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 2., 1080. / 1.5)
                }, dicke, Color::WHITE);

                curve.draw(100);
            }
            if num_of_presses_on_page == 5 {
                let curve = QuadraticBeziercurve::new(
                    if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                    if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                
                    if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2., 1080. / 1.5 + TRANSITION_TIME * 111.)
                    },
                
                
    
                    wrap(T, 0.235234, 2.0234), 10., Color::BLACK);
                
                    draw2d::circle::circle(if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2.)
                    }, 10., Color::WHITE);
                    draw2d::circle::circle( if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                    }, 10., Color::WHITE);
                    draw2d::circle::circle( if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2., 1080. / 1.5)
                    }, 10., Color::WHITE);

                curve.draw(100);
            }

            if num_of_presses_on_page == 6 {
                let curve = QuadraticBeziercurve::new(
                    if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                    if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                
                    if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2., 1080. / 1.5 + TRANSITION_TIME * 111.)
                    },
                
                    
    
            wrap(T, 0.235234, 2.0234), 10., Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.));


                draw2d::circle::circle(if key::is_pressed("key1") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 3., 1080. / 2.)
                }, 10., Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key2") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                }, 10., Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key3") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 2., 1080. / 1.5)
                }, 10., Color::WHITE);

                curve.draw(100);
            }

            if num_of_presses_on_page == 7 {
                let curve = QuadraticBeziercurve::new(
                    if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                    if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                
                    if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2., 1080. / 1.5 + TRANSITION_TIME * 111.)
                    },
                
                    
    
            wrap(T, 0.235234, 2.0234), 10., Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.));

                curve.draw(100);
            }

            if num_of_presses_on_page >= 8 {
                let curve = DottedQuadraticBeziercurve::new(
                    if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                    if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                
                    if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2., 1080. / 1.5 + TRANSITION_TIME * 111.)
                    },
                
                    
    
            wrap(T, 0.235234, 2.0234), 10., Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.));

                curve.draw(30 + (wrap(T, 0.5234253, 3.2346345) * 30.) as i32)
            }
        }
   };
}


async fn page10(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>) {
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Elemente der Game Engine", Vec2::new(1920. / 2., 220.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        let width = draw2d::text::measure_text("Elemente der Game Engine", 110, Some(fonts[1].clone())).x;
        draw2d::line::line(Vec2::new(1920. / 2. - width / 2., 285.), Vec2::new(1920. / 2. + width / 2., 285.), 5., Color::BLACK);

        {
            if num_of_presses_on_page == 1 {
                if num_of_presses_on_page == 1 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    let curve: CubicBeziercurve = CubicBeziercurve::new(Vec2::new(1920. / 3. - TRANSITION_TIME2 * 66., 1080. / 2.), Vec2::new(1920. - 1920. / 3. - TRANSITION_TIME2 * 66., 1080. / 2.), 
                    Vec2::new(1920. / 2.5 - TRANSITION_TIME2 * 66., 1080. / 1.5),
                    Vec2::new(1920. - 1920. / 2.5 - TRANSITION_TIME2 * 66., 1080. / 1.5), 10., Color::BLACK);

                    curve.draw(100);
                }
                else if num_of_presses_on_page == 1 {
                    let curve: CubicBeziercurve = CubicBeziercurve::new(Vec2::new(1920. / 3., 1080. / 2.), Vec2::new(1920. - 1920. / 3., 1080. / 2.), 
                    Vec2::new(1920. / 2.5, 1080. / 1.5),
                    Vec2::new(1920. - 1920. / 2.5, 1080. / 1.5), 10., Color::BLACK);

                    curve.draw(100);
                }
            }   
            if num_of_presses_on_page == 2 {
                let curve: CubicBeziercurve = CubicBeziercurve::new(
                    Vec2::new(1920. / 3., 1080. / 2.),
                     Vec2::new(1920. - 1920. / 3., 1080. / 2.), 
                    Vec2::new(1920. / 2.5, 1080. / 1.5),
                    Vec2::new(1920. - 1920. / 2.5, 1080. / 1.5), 10., Color::BLACK);

                    curve.draw(100);

                draw2d::circle::circle(Vec2::new(1920. / 3., 1080. / 2.), 10., Color::WHITE);
                draw2d::circle::circle(Vec2::new(1920. - 1920. / 3., 1080. / 2.), 10., Color::WHITE);
                draw2d::circle::circle(Vec2::new(1920. / 2.5, 1080. / 1.5), 10., Color::WHITE);
                draw2d::circle::circle(Vec2::new(1920. - 1920. / 2.5, 1080. / 1.5), 10., Color::WHITE);

                curve.draw(100)
            }
            if num_of_presses_on_page == 3 {
                let curve = CubicBeziercurve::new(
                    if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2.)
                    },
                    if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                    },
                
                    if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2.5, 1080. / 1.5)
                    },
                    if key::is_pressed("key4") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 2.5, 1080. / 1.5)
                    },
                
                
    
                 10., Color::BLACK);

                draw2d::circle::circle(if key::is_pressed("key1") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 3., 1080. / 2.)
                }, 10., Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key2") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                }, 10., Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key3") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 2.5, 1080. / 1.5)
                }, 10., Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key4") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. - 1920. / 2.5, 1080. / 1.5)
                }, 10., Color::WHITE);
                

                curve.draw(100)
            }
            if num_of_presses_on_page == 4 {
                let dicke = 10. + wrap(T, 0.6234, 3.2345) * 20.;

                let curve = CubicBeziercurve::new(
                    if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2.)
                    },
                    if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                    },
                
                    if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2.5, 1080. / 1.5)
                    },
                    if key::is_pressed("key4") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 2.5, 1080. / 1.5)
                    },
                
                
    
                 dicke, Color::BLACK);

                draw2d::circle::circle(if key::is_pressed("key1") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 3., 1080. / 2.)
                }, dicke, Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key2") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                }, dicke, Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key3") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 2.5, 1080. / 1.5)
                }, dicke, Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key4") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. - 1920. / 2.5, 1080. / 1.5)
                }, dicke, Color::WHITE);
                

                curve.draw(100)
            }
            if num_of_presses_on_page == 5 {
                let curve = CubicBeziercurve::new(
                    if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2.)
                    },
                    if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                    },
                
                    if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2.5, 1080. / 1.5)
                    },
                    if key::is_pressed("key4") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 2.5, 1080. / 1.5)
                    },
                
                
    
                 10., Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.));

                draw2d::circle::circle(if key::is_pressed("key1") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 3., 1080. / 2.)
                }, 10., Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key2") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. - 1920. / 3., 1080. / 2.)
                }, 10., Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key3") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. / 2.5, 1080. / 1.5)
                }, 10., Color::WHITE);
                draw2d::circle::circle( if key::is_pressed("key4") {
                    mouse::get_position()
                }
                else {
                    Vec2::new(1920. - 1920. / 2.5, 1080. / 1.5)
                }, 10., Color::WHITE);
                

                curve.draw(100)
            }

            if num_of_presses_on_page == 6 {
                let curve = CubicBeziercurve::new(
                    if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                    if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                
                    if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2.5, 1080. / 1.5 + TRANSITION_TIME * 111.)
                    },
                    if key::is_pressed("key4") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 2.5, 1080. / 1.5 + TRANSITION_TIME * 111.)
                    },
                
                
    
                 10., Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.));
             

                curve.draw(100)
            }


            if num_of_presses_on_page >= 7 {
                let curve = DottedCubicBeziercurve::new(
                    if key::is_pressed("key1") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                    if key::is_pressed("key2") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 3., 1080. / 2. + TRANSITION_TIME * 111.)
                    },
                
                    if key::is_pressed("key3") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. / 2.5, 1080. / 1.5 + TRANSITION_TIME * 111.)
                    },
                    if key::is_pressed("key4") {
                        mouse::get_position()
                    }
                    else {
                        Vec2::new(1920. - 1920. / 2.5, 1080. / 1.5 + TRANSITION_TIME * 111.)
                    },
                
                
    
                 10., Color::from_0to1(wrap(T, 0.236235, 0.234767), wrap(T, 0.723623, 1.9272437), wrap(T, 0.2367326, 1.7298067), 1.));
             

                curve.draw(30 + (wrap(T, 0.5234253, 3.2346345) * 30.) as i32)
            }
        }
   };
}

async fn page11(window: &Window, rngs: Vec<f32>, fonts: Vec<Font>, textures: &mut Vec<Image>, cam3d: &mut Camera3d, objects: &mut Vec<Mesh>) {
    unsafe { 
        draw_background(window, rngs).await;
        draw2d::text::draw_text_center("Elemente der Game Engine", Vec2::new(1920. / 2., 220.), 110, Color::BLACK, 0., Some(fonts[1].clone()));
        let width = draw2d::text::measure_text("Elemente der Game Engine", 110, Some(fonts[1].clone())).x;
        draw2d::line::line(Vec2::new(1920. / 2. - width / 2., 285.), Vec2::new(1920. / 2. + width / 2., 285.), 5., Color::BLACK);

        {
            if num_of_presses_on_page == 1 {
                if num_of_presses_on_page == 1 && TRANSITION_TIME2 != TRANSITION2_MAX_TIME {
                    let rounded_field = RoundedField::new(
                        Vec2::new(450., 250.), 
                        Vec2::new(1920. / 2. - TRANSITION_TIME2 * 66., 1280. / 2.), 
                        10., 
                        0.4, 
                        Color::LIME, 
                        Color::BLACK
                    );

                    rounded_field.draw(100, true)
                }
                else if num_of_presses_on_page == 1 {
                    let rounded_field = RoundedField::new(
                        Vec2::new(450., 250.), 
                        Vec2::new(1920. / 2., 1280. / 2.), 
                        10., 
                        0.4, 
                        Color::LIME, 
                        Color::BLACK
                    );

                    rounded_field.draw(100, true)
                }
            }


            if num_of_presses_on_page == 2 {
                let rounded_field = RoundedField::new(
                    Vec2::new(450., 250.), 
                    Vec2::new(1920. / 2. + (T * 10.).cos() * 60., 1280. / 2. + (T * 10.).sin() * 60.), 
                    10., 
                    0.4, 
                    Color::LIME, 
                    Color::BLACK
                );

                rounded_field.draw(100, true)
            }

            if num_of_presses_on_page == 3 {
                let rounded_field = RoundedField::new(
                    Vec2::new(450. + wrap(T, 0.6234234, 3.62349128347) * 100., 250.  + wrap(T, 0.12565, 4.325234) * 100.), 
                    Vec2::new(1920. / 2. + (T * 10.).cos() * 60., 1280. / 2. + (T * 10.).sin() * 60.), 
                    10., 
                    0.4, 
                    Color::LIME, 
                    Color::BLACK
                );

                rounded_field.draw(100, true)
            }

            if num_of_presses_on_page == 4 {
                let rounded_field = RoundedField::new(
                    Vec2::new(450. + wrap(T, 0.6234234, 3.62349128347) * 100., 250.  + wrap(T, 0.12565, 4.325234) * 100.), 
                    Vec2::new(1920. / 2. + (T * 10.).cos() * 60., 1280. / 2. + (T * 10.).sin() * 60.), 
                    10. + wrap(T, 0.892346, 2.62349128347) * 15., 
                    0.4, 
                    Color::LIME, 
                    Color::BLACK
                );

                rounded_field.draw(100, true)
            }

            if num_of_presses_on_page == 5 {
                let rounded_field = RoundedField::new(
                    Vec2::new(450. + wrap(T, 0.6234234, 3.62349128347) * 100., 250.  + wrap(T, 0.12565, 4.325234) * 100.), 
                    Vec2::new(1920. / 2. + (T * 10.).cos() * 60., 1280. / 2. + (T * 10.).sin() * 60.), 
                    10. + wrap(T, 0.892346, 2.62349128347) * 15., 
                    wrap(T, 0.154512, 2.125562), 
                    Color::LIME, 
                    Color::BLACK
                );

                rounded_field.draw(100, true)
            }

            if num_of_presses_on_page >= 6 {
                let rounded_field = RoundedField::new(
                    Vec2::new(450. + wrap(T, 0.6234234, 3.62349128347) * 100., 250.  + wrap(T, 0.12565, 4.325234) * 100.), 
                    Vec2::new(1920. / 2. + (T * 10.).cos() * 60., 1280. / 2. + (T * 10.).sin() * 60. + TRANSITION_TIME * 111.), 
                    10. + wrap(T, 0.892346, 2.62349128347) * 15., 
                    wrap(T, 0.154512, 2.125562), 
                    Color::from_0to1(wrap(T, 0.2436235, 0.234767), wrap(T, 0.73452, 1.9272437), wrap(T, 0.72345, 1.7298067), 1.), 
                    Color::from_0to1(wrap(T, 0.7345, 0.2366), wrap(T, 0.23467346, 1.7234656), wrap(T, 0.235497, 1.3646352), 1.)
                );

                rounded_field.draw(100, true)
            }
        }
   };
}



async fn run(window: Window) {
    let mut page = 0;
    let mut in_transition = false;
    let mut next_page_transition = false;
    let mut in_transition2 = false;
    

    let mut visible = false;
    mouse::set_visivle(visible);


    let mut fonts = vec![];
    
    let font = Font::load_new_font("assets/MADEAwelierPERSONALUSE-Regular.otf").await;
    let font2 = Font::load_new_font("assets/MADEAwelierPERSONALUSE-Bold.otf").await;

    let font3 = Font::load_new_font("assets/coffee_normal/Coffee Normal.ttf").await;
    let font4 = Font::load_new_font("assets/creative_vibes/Creative Vibes TTF.ttf").await;
    let font5 = Font::load_new_font("assets/kolak/KOLAK.ttf").await;
    let font6 = Font::load_new_font("assets/super_creamy/Super Creamy Personal Use.ttf").await;
    fonts.push(font);
    fonts.push(font2);
    fonts.push(font3);
    fonts.push(font4);
    fonts.push(font5);
    fonts.push(font6);


    let mut objects = vec![];


    let mut textures = vec![];
    let rust_logo = Image::new("assets/rust.png", 0., false, false, Vec2::new(1000., 1000.), None).await;
    let erde = Image::new("assets/erde.png", 0., false, false, Vec2::new(1000., 1000.), None).await;
    let crossaint = Image::new("assets/crossaint.png", 0., false, false, Vec2::new(1000., 1000.), None).await;
    let donut = Image::new("assets/donut.png", 0., false, false, Vec2::new(200., 200.), None).await;
    textures.push(erde);
    textures.push(crossaint);
    textures.push(rust_logo);
    textures.push(donut);

    
    let mut rngs: Vec<f32> = vec![];
    let mut rng = rand::thread_rng();
    for _ in 0..num_of_triangles * 20 {
        rngs.push((rng.gen::<f32>() - 0.5) * 10.)
    }

    let mut cam3d = Camera3d::default();
    let cam2d = Camera2d::default();

    while window.update_screen().await {
        let dt = window.get_delta_time();

        unsafe {
            T += 0.5 * dt;

        }

        
        if key::just_pressed("down") {
            mouse::set_visivle(visible);
            visible = !visible;
        }

        if key::just_pressed("right") 
        {
            in_transition = true;
        }

        if key::just_pressed("up") {
            in_transition2 = true;
            unsafe {
                num_of_presses_on_page += 1;
            }
        }

        if in_transition2{
            unsafe {
                TRANSITION_TIME2 -= 79. * dt;
            }
        }
        unsafe {
            if TRANSITION_TIME2 <= 0. {
                in_transition2 = false;
                TRANSITION_TIME2 = TRANSITION2_MAX_TIME;
            }
        }
        

        if in_transition {
            unsafe {
                TRANSITION_TIME += 24. * dt;
            }
        }

        unsafe {

            if TRANSITION_TIME >= TRANSITION_MAX_TIME {
                TRANSITION_TIME = -TRANSITION_MAX_TIME;
                page += 1;
                num_of_presses_on_page = 0;
                next_page_transition = true;
            }

            if TRANSITION_TIME >= 0. && next_page_transition {
                TRANSITION_TIME = 0.;
                in_transition = false;
                next_page_transition = false;
            }
        }

        unsafe {
            println!("{}", TRANSITION_TIME2);
        }
        

        match page {
            0 => page0(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            1 => page1(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            2 => page2(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            3 => page3(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            4 => page4(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            5 => page5(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            6 => page6(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            7 => page7(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            8 => page8(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            9 => page9(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            10 => page10(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            11 => page11(&window, rngs.clone(), fonts.clone(), &mut textures, &mut cam3d, &mut objects).await, 
            12 => minecraft_run(&window, rngs.clone()).await,
            13 => end(&window, rngs.clone(), fonts.clone()).await,
            _other => break
        };
        
        cam2d.set_as_current_cam();


        if page == 12 {
            in_transition = true;
            page += 1;
            unsafe {    
                TRANSITION_TIME = -TRANSITION_MAX_TIME;
            }
            
        }
    }
    

    
}

use std::collections::HashMap;

static MOVEMENT_SPEED: f32 = 4.5;
static MOVEMENT_INCREASE: f32 = 1.1;
static MOVEMENT_STRAFE: f32 = 0.3;
static LOOK_SPEED: f32 = 0.8;
static JUMP_HEIGHT: f32 = 4.;
static GRAVITY: f32 = 10.;
static RENDER_DISTANCE: f32 = 4.;


async fn init() -> (Vec<Image>, Camera2d, Camera3d, Player, Vec<Boden>) {
    mouse::set_visivle(false);

    let mut textures = vec![];
    let grass = Image::new("assets/grass.png", 0., false, false, Vec2::new(100., 100.), None).await;
    let sky = Image::new("assets/sky.png", 0., false, false, Vec2::new(100., 100.), None).await;
    textures.push(grass.clone());
    textures.push(sky);


    let default_cam = Camera2d::default();
    let cam: Camera3d = Camera3d::default();
    


    let player = Player {
        position: Vec3::new(0., 0., 0.),
        size: Vec3::new(1., 2., 1.),
        velocity: 0.
    };


    let bigness = 100;

    let mut xz_ys: HashMap<(i32, i32), f32> = HashMap::new();

    let mut böden = vec![];

    for x in -bigness..=bigness {
        for z in -bigness..=bigness {
            let y = math::simplex_noise((x + bigness) as f32, (z + bigness) as f32, 1, 0.05, 0.05, 5., 234563456).ceil();
            
            xz_ys.insert((x, z), y);
        }
    }

    for x in -bigness..=bigness {
        for z in -bigness..=bigness {
            let y = xz_ys.get(&(x, z)).unwrap();

            let top_y = xz_ys.get(&(x, z - 1)).unwrap_or(y);
            let bottom_y = xz_ys.get(&(x, z + 1)).unwrap_or(y);
            let right_y = xz_ys.get(&(x + 1, z)).unwrap_or(y);
            let left_y = xz_ys.get(&(x - 1, z)).unwrap_or(y);

            let diff1 = (y - top_y).clamp(0., f32::MAX);
            let diff2 = (y - bottom_y).clamp(0., f32::MAX);
            let diff3 = (y - right_y).clamp(0., f32::MAX);
            let diff4 = (y - left_y).clamp(0., f32::MAX);

            let mut max_diff = diff1.max(diff2).max(diff3).max(diff4) as i32;

            if max_diff == 1 {
                max_diff = 0;
            }

            for new_y in *y as i32 - max_diff.. *y as i32 + 1 {
                böden.push(Boden::new(Vec3::new(x as f32 - 0.5, new_y as f32 - 10., z as f32 - 0.5), Vec3::new(1., 1., 1.), grass.clone()));
            }
        }
    }



    (textures, default_cam, cam, player, böden)
}   


struct Boden {
    position: Vec3,
    size: Vec3,
    texture: Image
}

impl Boden {
    fn new(position: Vec3, size: Vec3, texture: Image) -> Boden {
        Boden {
            position,
            size,
            texture
        }
    }

    fn update(&self, cam3d: &mut Camera3d, front: Vec3) {
        let distancex = (cam3d.position.x - self.position.x).abs();
        let distancez = (cam3d.position.z - self.position.z).abs();

        if distancex + distancez > RENDER_DISTANCE * 16. {
            return;
        }
        
        let mut dir_to_self: Vec3 = cam3d.position - self.position;
        dir_to_self.normalize();

        let dir = dir_to_self.dot(&front).acos();

        if dir.is_nan() {
            return;
        }
        if dir < PI - cam3d.fovy  {
            return;
        }

        draw3d::cube::cube_center(self.position, self.size, Some(&self.texture), Color::WHITE);
    }

    fn get_hitbox(&self) -> CubeHitbox {
        CubeHitbox::new(self.position, self.size)
    }
}

struct Player {
    position: Vec3,
    size: Vec3,
    velocity: f32
}


impl Player {
    fn update(&mut self, cam3d: &mut Camera3d, delta: f32, böden: &Vec<Boden>) {        
        self.look(cam3d);
        let touching = self.gravity(delta, böden);
        self.movement(delta, cam3d, touching);

        cam3d.position = self.position;
        cam3d.update_all_rotation();
    }

    fn movement(&mut self, delta: f32, cam3d: &Camera3d, touching: bool) {
        let mut offset = Vec3::new(0., 0., 0.);
        if key::is_pressed("w") {
            let mut front = cam3d.get_front();
            front.y = 0.;
            front.normalize();
            offset = offset + front;
        }
        if key::is_pressed("a") {
            let mut left = cam3d.get_left();
            left.y = 0.;
            left.normalize();
            offset = offset + left;
        }
        if key::is_pressed("s") {
            let mut back = cam3d.get_back();
            back.y = 0.;
            back.normalize();
            offset = offset + back;
        }
        if key::is_pressed("d") {
            let mut right = cam3d.get_right();
            right.y = 0.;
            right.normalize();
            offset = offset + right;
        }

        
        offset.normalize();

        if (key::is_pressed("a") ^ key::is_pressed("d")) && key::is_pressed("w") {
            let mut front = cam3d.get_front();
            front.y = 0.;
            front.normalize();
            offset = offset + front.scale(MOVEMENT_STRAFE);
        }

        offset = offset.scale(delta).scale(MOVEMENT_SPEED);

        if !touching {
            offset = offset.scale(MOVEMENT_INCREASE);
        }

        self.position = self.position + offset;
    }

    fn gravity(&mut self, delta: f32, böden: &Vec<Boden>) -> bool{
        let mut touching = false;
        let mut böden_touching = vec![];


        for boden in böden {
            let distancex = (boden.position.x - self.position.x).abs();
            let distancey = (boden.position.y - self.position.y).abs();
            let distancez = (boden.position.z - self.position.z).abs();
            if distancex + distancey + distancez > 5. {
                continue;
            }
            if self.get_hitbox().collision_cube(boden.get_hitbox()) {
                touching = true;
                böden_touching.push(boden);
                break;
            }
        }
        if touching {
            if key::is_pressed("space") {
                self.velocity = -JUMP_HEIGHT;
                touching = false;
            } else {
                self.position.y = böden_touching[0].position.y + böden_touching[0].size.y / 2. + self.size.y / 2.;
            }
        }
        if !touching {
            self.position.y -= self.velocity * delta;
            self.velocity += GRAVITY * delta;
        }
        touching
    }

    fn look(&self, cam3d: &mut Camera3d) {
        cam3d.rotation = cam3d.rotation + mouse::get_delta_position().scale(LOOK_SPEED);
        if cam3d.rotation.y < -PI / 2. + 0.001  {
            cam3d.rotation.y = -PI / 2. + 0.001;
        }
        if cam3d.rotation.y > PI / 2. - 0.001 {
            cam3d.rotation.y = PI / 2. - 0.001;
        }

    }


    fn get_hitbox(&self) -> CubeHitbox {
        CubeHitbox::new(self.position, self.size)
    }
}



async fn minecraft_run(window: &Window, rngs: Vec<f32>){   
    let (
        textures, 
        default_cam, 
        mut cam3d, 
        mut player, 
        böden
    ) = init().await;
    
    mouse::set_visivle(true);

    let mut slider = Slider::new(Vec2::newi(600, 70), Vec2::newi(600, 530), 5., Color::RED, Color::BLUE, Color::GREEN, 0., 40., Color::GRAY, Color::DARKGRAY, 5.);
    let mut bar = Bar::new(Vec2::newi(600, 77), Vec2::newi(600, 300), 5., Color::PURPLE, Color::YELLOW, Color::BROWN, 0.5);
    let text_field = Textfield::new(Vec2::newi(200, 120), Vec2::newi(1450, 530), 10., 0.3, Color::DARKPURPLE, Color::MAGENTA, "hallo".to_string(), None, 80, Color::DARKBROWN, 0.);
    let mut button = Button::new(Vec2::newi(200, 90), Vec2::newi(1450, 760), 10., 0.7, Color::BLUE, Color::MAROON, Some("click".to_string()), None, Some(80), Some(Color::PINK), Some(0.), None, true, Some(scale_function_add));
    let mut switch = Switch::new(Vec2::newi(140, 70), Vec2::newi(670, 760), 5., Color::BROWN, Color::MAROON, Color::BEIGE, false, 40., Color::VIOLET, Color::SKYBLUE, 5.);
    let mut tick_box = TickBox::new(70., Vec2::newi(440, 760), 10., 0.5, Color::BLANK, Color::MAGENTA, Color::GREEN, false);
    let mut entrance_text_field = EntranceTextfield::new(Vec2::newi(200, 120), Vec2::newi(1450, 300), 10., 0.3, Color::DARKPURPLE, Color::MAGENTA, "".to_string(), None, 80, Color::DARKBROWN, 0., true, true, "_".to_string());
    

    let mut left = false;


    while window.update_screen().await {
        let delta = window.get_delta_time();
        
       

        unsafe {
            if inMinecraft {


















                mouse::set_visivle(false);
                mouse::set_grab(true);
                default_cam.set_as_current_cam();
                draw2d::clear(Color::new(0, 0, 0, 255));
                cam3d.set_as_current_cam();
                
        
        
                player.update(&mut cam3d, delta, &böden);
        
                let mut front = cam3d.get_front();
                front.normalize();
        

                for boden in &böden {
                    boden.update(&mut cam3d, front);
                }
        
                draw3d::sphere::sphere_center(Vec3::new(0., 0., 0.), 1000., None, Color::new(0, 168, 243, 255));
        
                if key::is_pressed("right") {
                    break;
                }


































            }
            else {
                default_cam.set_as_current_cam();
                draw2d::clear(Color::BLACK);


                if key::just_pressed("up") {
                    inMinecraft = true;
                    mouse::set_visivle(false);
                    mouse::set_grab(true);
                }
                
                draw_background(window, rngs.clone()).await;

                slider.draw(10);
                bar.draw(50);
                text_field.draw(10);
                button.draw(10);
                switch.draw(10);
                tick_box.draw(10);
                entrance_text_field.draw(50);

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
    }
    
}








#[rongor::main]
async fn main() {
    start(run, "präsentation".into(), 1920, 1080, false, true, true, AntialiasingType::Lanczos3).await;
}