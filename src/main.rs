mod points;
mod utils;

use points::*;
use utils::*;

use raylib::prelude::*;
use std::f64::consts::PI;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGTH)
        .title("Rotating Cube")
        .build();

    rl.set_target_fps(FRAMERATE);
    let mut frame_count = 0;
    let mut state: u16 = 0;

    let rotation = 0.05;
    let mut angle = Vector3D::new(0.0, 0.0, 0.0);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        let center = Point3D::new(0, 0, 0);

        draw_pixel(&mut d, &center);
        draw_cube(&mut d, &center, &angle);

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
        if frame_count > FRAMERATE {
            frame_count = 0;
            state += 1;
            if state > 2 {
                state = 0;
            }
        }
    }
}

fn draw_pixel(d: &mut RaylibDrawHandle, point: &Point3D) {
    let point_t = Point2D::from_3d(point);
    d.draw_circle(point_t.x, point_t.y, PIXEL_SIZE as f32, Color::WHITE);
}

fn draw_line(d: &mut RaylibDrawHandle, begin: &Point3D, end: &Point3D) {
    let begin_t = Point2D::from_3d(begin);
    let end_t = Point2D::from_3d(end);

    d.draw_line(begin_t.x, begin_t.y, end_t.x, end_t.y, Color::WHITE);
}

fn draw_cube(d: &mut RaylibDrawHandle, start_p: &Point3D, angle: &Vector3D) {
    let mut points = Point3D::cube_vertices(start_p);

    for point in points.iter_mut() {
        point.rotate_y(angle.x);
        point.rotate_x(angle.y);
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
        draw_line(d, &points[start_ind], &points[end_ind]);
    }
}
