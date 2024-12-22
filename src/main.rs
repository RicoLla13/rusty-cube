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

    let mut angle = Vector3D::new(0.0, 0.0, 0.0);
    let mut offset = 0;
    let mut state = State::YAxisRot;
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
        draw_canvas1(&mut canvas1, &mut angle, offset, &state);
        draw_canvas2(&mut canvas2);

        match state {
            State::YAxisRot => {
                angle.y += ROTATION;
                if angle.angle_overshoot() {
                    state = State::FromYToX;
                    angle.zero();
                }
            }
            State::FromYToX => {
                offset += 1;
                if offset >= 100 {
                    offset = 0;
                    state = State::XAxisRot;
                }
            }
            State::XAxisRot => {
                angle.x += ROTATION;
                if angle.angle_overshoot() {
                    state = State::FromXToZ;
                    angle.zero();
                }
            }
            State::FromXToZ => {
                offset += 1;
                if offset >= 200 {
                    offset = 0;
                    state = State::ZAxisRot;
                }
            }
            State::ZAxisRot => {
                angle.z += ROTATION;
                if angle.angle_overshoot() {
                    state = State::FromZToY;
                    angle.zero();
                }
            }
            State::FromZToY => {
                offset += 1;
                if offset >= 100 {
                    offset = 0;
                    state = State::YAxisRot;
                }
            }
        }

        canvas1.present();
        canvas2.present();
        std::thread::sleep(Duration::new(0, FRAMERATE_CALC));
    }
}

fn draw_canvas1(canvas: &mut Canvas, angle: &mut Vector3D, offset: i32, state: &State) {
    canvas.clear();

    match state {
        State::YAxisRot => {
            let center_c = Point3D::new(100, 0, -100);
            let center_p = Point3D::new(-100, 0, 100);

            // Y axis
            canvas.draw_line_3d(
                &Point3D::new(0, -100, 0),
                &Point3D::new(0, 100, 0),
                Color::RED,
            );
            canvas.draw_cube(&center_c, &angle);
            canvas.draw_pyramid(&center_p, &angle);
        }
        State::FromYToX => {
            let center_c = Point3D::new(100 - offset, 0 + offset, -100 + offset);
            let center_p = Point3D::new(-100 + offset, 0 - offset, 100 - offset);

            canvas.draw_cube(&center_c, &angle);
            canvas.draw_pyramid(&center_p, &angle);
        }
        State::XAxisRot => {
            let center_c = Point3D::new(0, 100, 0);
            let center_p = Point3D::new(0, -100, 0);

            // X axis
            canvas.draw_line_3d(
                &Point3D::new(-100, 0, 0),
                &Point3D::new(100, 0, 0),
                Color::BLUE,
            );
            canvas.draw_cube(&center_c, &angle);
            canvas.draw_pyramid(&center_p, &angle);
        }
        State::FromXToZ => {
            let center_c = Point3D::new(0, 100 - offset, 0);
            let center_p = Point3D::new(0, -100 + offset, 0);

            canvas.draw_cube(&center_c, &angle);
            canvas.draw_pyramid(&center_p, &angle);
        }
        State::ZAxisRot => {
            let center_c = Point3D::new(0, -100, 0);
            let center_p = Point3D::new(0, 100, 0);

            // Z axis
            canvas.draw_line_3d(
                &Point3D::new(0, 0, 100),
                &Point3D::new(0, 0, -100),
                Color::GREEN,
            );
            canvas.draw_cube(&center_c, &angle);
            canvas.draw_pyramid(&center_p, &angle);
        }
        State::FromZToY => {
            let center_c = Point3D::new(0 + offset, -100 + offset, 0 - offset);
            let center_p = Point3D::new(0 - offset, 100 - offset, 0 + offset);

            canvas.draw_cube(&center_c, &angle);
            canvas.draw_pyramid(&center_p, &angle);
        }
    }
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
