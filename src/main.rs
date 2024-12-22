extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod canvas;
mod points;
mod utils;

use canvas::*;
use points::*;
use utils::*;

fn main() {
    let mut canvas = Canvas::new();

    'running: loop {
        for event in canvas.event_pump.poll_iter() {
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
        canvas.draw_pixel(Point2D::from_3d(&pos), Color::WHITE);

        // Test for equal x
        canvas.draw_line(Point2D::new(50, 10), Point2D::new(50, 70), Color::WHITE);
        canvas.draw_line(Point2D::new(100, 70), Point2D::new(100, 10), Color::WHITE);

        // Test for equal y
        canvas.draw_line(Point2D::new(50, 100), Point2D::new(150, 100), Color::WHITE);
        canvas.draw_line(Point2D::new(150, 110), Point2D::new(50, 110), Color::WHITE);

        // Test shallow line
        canvas.draw_line(Point2D::new(20, 30), Point2D::new(100, 50), Color::WHITE);
        canvas.draw_line(Point2D::new(30, 20), Point2D::new(50, 100), Color::WHITE);

        canvas.present();
        std::thread::sleep(Duration::new(0, FRAMERATE));
    }
}
