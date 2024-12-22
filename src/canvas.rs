use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::Sdl;

use crate::points::*;
use crate::utils::*;

pub struct Canvas {
    sdl_context: Sdl,
    pub canvas: WindowCanvas,
    pub event_pump: EventPump,
}

impl Canvas {
    pub fn new() -> Self {
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
        let event_pump = sdl_context.event_pump().unwrap();

        Canvas {
            sdl_context,
            canvas,
            event_pump,
        }
    }

    pub fn draw_pixel(&mut self, pos: Point2D, color: Color) {
        let rect = Rect::new(pos.x, pos.y, PIXEL_SIZE, PIXEL_SIZE);
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn draw_line(&mut self, start: Point2D, end: Point2D, color: Color) {
        let slope = if start.x - end.x == 0 {
            0.0
        } else {
            (start.x - end.x) as f32
        };
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
