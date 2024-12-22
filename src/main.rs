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
