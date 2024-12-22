extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

mod points;
mod utils;

use points::*;
use utils::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGTH)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let pos = Point3D::new(0, 0, 0);
        draw_pixel(
            &mut canvas,
            Point2D::from_3d(&pos),
            Color::RGB(255, 255, 255),
        );

        canvas.present();
        std::thread::sleep(Duration::new(0, FRAMERATE));
    }
}

fn draw_pixel(canvas: &mut WindowCanvas, pos: Point2D, color: Color) {
    let rect = Rect::new(pos.x, pos.y, PIXEL_SIZE, PIXEL_SIZE);
    canvas.set_draw_color(color);
    canvas.fill_rect(rect).unwrap();
}
