extern crate sdl2;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
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

    let mut angle = Vector3D::new(0.0, 0.0, 0.0);
    'running: loop {
        for event in context.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window {
                    win_event: WindowEvent::Close,
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        draw_canvas1(&mut canvas1, &mut angle);
        draw_canvas2(&mut canvas2);

        frame_count += 1;
        if frame_count > FRAMERATE {
            frame_count = 0;
        }

        canvas1.present();
        canvas2.present();
        std::thread::sleep(Duration::new(0, FRAMERATE_CALC));
    }
}

fn draw_canvas1(canvas: &mut Canvas, angle: &mut Vector3D) {
    canvas.clear();

    let center_c = Point3D::new(70, 0, -70);
    let center_p = Point3D::new(-70, 0, 70);

    canvas.draw_line_3d(
        &Point3D::new(0, 100, 0),
        &Point3D::new(0, -100, 0),
        Color::RED,
    );
    canvas.draw_cube(&center_c, &angle);
    canvas.draw_pyramid(&center_p, &angle);

    angle.y += ROTATION;
    angle.angle_overshoot();
}

fn draw_canvas2(canvas: &mut Canvas) {
    canvas.clear();

    let center_circ = Point2D::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGTH as i32 / 2);
    let radius = 50;
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x, center_circ.y - 2 * radius),
        &Point2D::new(center_circ.x, center_circ.y + 2 * radius),
        Color::RGB(252, 128, 5),
    );
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x - 2 * radius, center_circ.y),
        &Point2D::new(center_circ.x + 2 * radius, center_circ.y),
        Color::RGB(252, 128, 5),
    );
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x + 2 * radius, center_circ.y - 2 * radius),
        &Point2D::new(center_circ.x - 2 * radius, center_circ.y + 2 * radius),
        Color::RGB(252, 128, 5),
    );
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x + 2 * radius, center_circ.y + 2 * radius),
        &Point2D::new(center_circ.x - 2 * radius, center_circ.y - 2 * radius),
        Color::RGB(252, 128, 5),
    );
    canvas.draw_pixel(&center_circ, Color::BLUE);
    canvas.draw_circle(&center_circ, radius, Color::WHITE);
    canvas.draw_rect(
        &Point2D::new(center_circ.x - 2 * radius, center_circ.y - 2 * radius),
        4 * radius,
        4 * radius,
        Color::WHITE,
    );
    // Draw Upper Triangle
    let peak_up = Point2D::new(center_circ.x, center_circ.y - 2 * radius - 173);
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x - 2 * radius, center_circ.y - 2 * radius),
        &peak_up,
        Color::WHITE,
    );
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x + 2 * radius, center_circ.y - 2 * radius),
        &peak_up,
        Color::WHITE,
    );
    // Draw Lower Triangle
    let peak_down = Point2D::new(center_circ.x, center_circ.y + 2 * radius + 173);
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x - 2 * radius, center_circ.y + 2 * radius),
        &peak_down,
        Color::WHITE,
    );
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x + 2 * radius, center_circ.y + 2 * radius),
        &peak_down,
        Color::WHITE,
    );
    // Draw Left Triangle
    let peak_left = Point2D::new(center_circ.x - 2 * radius - 173, center_circ.y);
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x - 2 * radius, center_circ.y - 2 * radius),
        &peak_left,
        Color::WHITE,
    );
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x - 2 * radius, center_circ.y + 2 * radius),
        &peak_left,
        Color::WHITE,
    );
    // Draw Right Triangle
    let peak_left = Point2D::new(center_circ.x + 2 * radius + 173, center_circ.y);
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x + 2 * radius, center_circ.y - 2 * radius),
        &peak_left,
        Color::WHITE,
    );
    canvas.draw_line_2d(
        &Point2D::new(center_circ.x + 2 * radius, center_circ.y + 2 * radius),
        &peak_left,
        Color::WHITE,
    );
    // Draw Outer Circle
    canvas.draw_circle(
        &center_circ,
        2 * radius + 173 + PIXEL_SIZE as i32,
        Color::BLUE,
    );
}
