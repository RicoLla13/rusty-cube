extern crate sdl2;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod canvas;
mod draw;
mod points;
mod utils;

use canvas::*;
use draw::*;
use points::*;
use utils::*;

fn main() {
    let mut context = SdlContext::init();
    let mut canvas1 = context.new_canvas(&"3D Shapes");
    let mut canvas2 = context.new_canvas(&"2D Shapes");

    let mut angle = Vector3D::new(0.0, 0.0, 0.0);
    let mut offset = 0;
    let mut offset2d = 0;
    let mut state3d = State3D::YAxisRot;
    let mut state2d = State2D::HouseInit;
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
        draw_canvas1(&mut canvas1, &mut angle, offset, &state3d);
        draw_canvas2(&mut canvas2, offset2d, &state2d);

        match state3d {
            State3D::YAxisRot => {
                angle.y += ROTATION;
                if angle.angle_overshoot() {
                    state3d = State3D::FromYToX;
                    angle.zero();
                }
            }
            State3D::FromYToX => {
                offset += 1;
                if offset >= 100 {
                    offset = 0;
                    state3d = State3D::XAxisRot;
                }
            }
            State3D::XAxisRot => {
                angle.x += ROTATION;
                if angle.angle_overshoot() {
                    state3d = State3D::FromXToZ;
                    angle.zero();
                }
            }
            State3D::FromXToZ => {
                offset += 1;
                if offset >= 200 {
                    offset = 0;
                    state3d = State3D::ZAxisRot;
                }
            }
            State3D::ZAxisRot => {
                angle.z += ROTATION;
                if angle.angle_overshoot() {
                    state3d = State3D::FromZToY;
                    angle.zero();
                }
            }
            State3D::FromZToY => {
                offset += 1;
                if offset >= 100 {
                    offset = 0;
                    state3d = State3D::YAxisRot;
                }
            }
        }

        match state2d {
            State2D::HouseInit => {
                state2d = State2D::WindowStrLeft;
                offset2d = 0;
            }
            State2D::WindowStrLeft => {
                offset2d += 1;
                if offset2d >= 20 {
                    state2d = State2D::WindowStrRight;
                }
            }
            State2D::WindowStrRight => {
                offset2d -= 1;
                if offset2d <= -20 {
                    state2d = State2D::WindowSettle;
                }
            }
            State2D::WindowSettle => {
                offset2d += 1;
                if offset2d >= 0 {
                    state2d = State2D::HouseInit;
                }
            }
        }

        canvas1.present();
        canvas2.present();
        std::thread::sleep(Duration::new(0, FRAMERATE_CALC));
    }
}
