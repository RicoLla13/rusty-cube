use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::VideoSubsystem;

use crate::points::*;
use crate::utils::*;

pub struct SdlContext {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    pub event_pump: EventPump,
}

pub struct Canvas {
    pub canvas: WindowCanvas,
}

impl SdlContext {
    pub fn init() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        SdlContext {
            sdl_context,
            video_subsystem,
            event_pump,
        }
    }

    pub fn new_canvas(&self, title: &str) -> Canvas {
        let window = self
            .video_subsystem
            .window(title, SCREEN_WIDTH, SCREEN_HEIGTH)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        Canvas { canvas }
    }
}

impl Canvas {
    pub fn draw_pixel(&mut self, pos: Point2D, color: Color) {
        let rect = Rect::new(pos.x, pos.y, PIXEL_SIZE, PIXEL_SIZE);
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn draw_line(&mut self, start: &Point3D, end: &Point3D, color: Color) {
        let start_2d = Point2D::from_3d(start);
        let end_2d = Point2D::from_3d(end);

        let mut x1 = start_2d.x;
        let mut y1 = start_2d.y;
        let x2 = end_2d.x;
        let y2 = end_2d.y;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            self.draw_pixel(Point2D::new(x1, y1), color);

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x1 += sx;
            }
            if e2 < dx {
                err += dx;
                y1 += sy;
            }
        }
    }

    pub fn draw_cube(&mut self, center: &Point3D, angle: &Vector3D) {
        let mut points = Point3D::cube_vertices(center);

        for point in points.iter_mut() {
            point.rotate_x(angle.x);
            point.rotate_y(angle.y);
            point.rotate_z(angle.z);
        }

        let edges = [
            // Frame Top
            (0, 1),
            (0, 2),
            (2, 3),
            (3, 1),
            // Frame Bottom
            (4, 5),
            (4, 6),
            (7, 5),
            (7, 6),
            // Frame Sides
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ];

        for &(start_ind, end_ind) in edges.iter() {
            self.draw_line(&points[start_ind], &points[end_ind], Color::WHITE);
        }
    }

    pub fn draw_pyramid(&mut self, center: &Point3D, angle: &Vector3D) {
        let mut points = Point3D::pyramid_vertices(center);

        for point in points.iter_mut() {
            point.rotate_x(angle.x);
            point.rotate_y(angle.y);
            point.rotate_z(angle.z);
        }

        let edges = [
            // Base
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            // Apex links
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
        ];

        for &(start_ind, end_ind) in edges.iter() {
            self.draw_line(&points[start_ind], &points[end_ind], Color::WHITE);
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}
