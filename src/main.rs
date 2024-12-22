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
    let mut context = SdlContext::init();
    let mut canvas1 = context.new_canvas(&"3D Shapes");
    let mut canvas2 = context.new_canvas(&"2D Shapes");

    let mut frame_count = 0;

    let rotation = 0.02;
    let mut angle = Vector3D::new(0.0, 0.0, 0.0);
    'running: loop {
        for event in context.event_pump.poll_iter() {
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
        canvas1.clear();
        canvas2.clear();

        // 3D Shapes Window
        let center_c = Point3D::new(70, 0, -70);
        let center_p = Point3D::new(-70, 0, 70);

        canvas1.draw_line(
            &Point3D::new(0, 100, 0),
            &Point3D::new(0, -100, 0),
            Color::RED,
        );
        canvas1.draw_cube(&center_c, &angle);
        canvas1.draw_pyramid(&center_p, &angle);

        // angle.x += rotation;
        angle.y += rotation;
        // angle.z += rotation;
        angle.angle_overshoot();

        // 2D Shapes Window
        canvas2.draw_pixel(Point2D::new(0, 0), Color::BLUE);

        frame_count += 1;
        if frame_count > FRAMERATE {
            frame_count = 0;
        }

        canvas1.present();
        canvas2.present();
        std::thread::sleep(Duration::new(0, FRAMERATE_CALC));
    }
}
