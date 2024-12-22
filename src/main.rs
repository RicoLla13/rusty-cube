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

    let mut frame_count = 0;
    let mut state: u16 = 0;

    let rotation = 0.05;
    let mut angle = Vector3D::new(0.0, 0.0, 0.0);
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
        canvas.clear();

        let center = Point3D::new(0, 0, 0);
        canvas.draw_pixel(Point2D::from_3d(&center), Color::WHITE);

        canvas.draw_cube(&center, &angle);

        match state {
            0 => {
                angle.x += rotation;
                angle.y = 0.0;
                angle.z = 0.0;
            }
            1 => {
                angle.x = 0.0;
                angle.y += rotation;
                angle.z = 0.0;
            }
            _ => {
                angle.x = 0.0;
                angle.y = 0.0;
                angle.z += rotation;
            }
        }

        frame_count += 1;
        println!("Frame: {}", frame_count);
        if frame_count > FRAMERATE {
            frame_count = 0;
            state += 1;
            if state > 2 {
                state = 0;
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, FRAMERATE_CALC));
    }
}
