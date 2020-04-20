use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::process::exit;
use std::time;
use std::thread;
use sdl2::render::WindowCanvas;

const SCALE_FACTOR: u32 = 20;
const W_HEIGHT: u32 = 32;
const W_WIDTH: u32 = 64;

fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let window = video_subsystem.window(
        "Test application",
        W_WIDTH * SCALE_FACTOR,
        W_HEIGHT * SCALE_FACTOR
    ).position_centered()
        .opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap_or_else(|_e| panic!("ERROR"));

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.present();

    loop {
        for i in 0..40 {
            draw_loop(&mut canvas, i % 2 == 0);
        }
        exit(0)
    }
}

fn draw_loop(canvas: &mut WindowCanvas, mode: bool) {
    let mut color1: Color;
    let mut color2: Color;
    let color_black = Color::RGB(0, 0, 0);
    let color_white = Color::RGB(255, 255, 255);

    for i in 0..W_HEIGHT {
        if i % 2 == 0 {
            color1 = color_white;
            color2 = color_black;
        } else {
            color1 = color_black;
            color2 = color_white;
        }
        for j in 0..W_WIDTH {
            let x = (j as u32) * SCALE_FACTOR;
            let y = (i as u32) * SCALE_FACTOR;
            if !mode {
                // Remove all white dots
                canvas.set_draw_color(color_black);
            } else if j % 2 == 0 {
                canvas.set_draw_color(color1);
            } else {
                canvas.set_draw_color(color2);
            }
            canvas.fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR)).unwrap_or_else(|_e| panic!("ERROR"));
            canvas.present();
            thread::sleep(time::Duration::from_millis(1))
        }
    }
}