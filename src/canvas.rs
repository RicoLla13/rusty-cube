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
        let mut x1 = start.x;
        let mut y1 = start.y;
        let mut x2 = end.x;
        let mut y2 = end.y;

        if start == end {
            self.draw_pixel(start, color);
        } else if x1 == x2 {
            if y1 > y2 {
                (y1, y2) = (y2, y1);
            }
            for y in y1..y2 {
                self.draw_pixel(Point2D::new(x1, y), color);
            }
        } else if y1 == y2 {
            if x1 > x2 {
                (x1, x2) = (x2, x1);
            }
            for x in x1..x2 {
                self.draw_pixel(Point2D::new(x, y1), color);
            }
        } else {
            let slope = (y1 - y2) as f32 / (x1 - x2) as f32;
            if slope <= 1.0 {
                if x1 > x2 {
                    (x1, x2) = (x2, x1);
                    (y1, y2) = (y2, y1);
                }
                for x in x1..x2 {
                    let y = (slope * ((x - x1) as f32) + y1 as f32) as i32;
                    self.draw_pixel(Point2D::new(x, y), color);
                }
            } else {
                if y1 > y2 {
                    (x1, x2) = (x2, x1);
                    (y1, y2) = (y2, y1);
                }
                for y in y1..y2 {
                    let x = (((y - y1) as f32 / slope) + x1 as f32) as i32;
                    self.draw_pixel(Point2D::new(x, y), color);
                }
            }
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
