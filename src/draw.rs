use crate::canvas::*;
use crate::points::*;
use crate::utils::*;
use sdl2::pixels::Color;

pub fn draw_canvas1(canvas: &mut Canvas, angle: &mut Vector3D, offset: i32, state: &State3D) {
    canvas.clear();

    match state {
        State3D::YAxisRot => {
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
        State3D::FromYToX => {
            let center_c = Point3D::new(100 - offset, 0 + offset, -100 + offset);
            let center_p = Point3D::new(-100 + offset, 0 - offset, 100 - offset);

            canvas.draw_cube(&center_c, &angle);
            canvas.draw_pyramid(&center_p, &angle);
        }
        State3D::XAxisRot => {
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
        State3D::FromXToZ => {
            let center_c = Point3D::new(0, 100 - offset, 0);
            let center_p = Point3D::new(0, -100 + offset, 0);

            canvas.draw_cube(&center_c, &angle);
            canvas.draw_pyramid(&center_p, &angle);
        }
        State3D::ZAxisRot => {
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
        State3D::FromZToY => {
            let center_c = Point3D::new(0 + offset, -100 + offset, 0 - offset);
            let center_p = Point3D::new(0 - offset, 100 - offset, 0 + offset);

            canvas.draw_cube(&center_c, &angle);
            canvas.draw_pyramid(&center_p, &angle);
        }
    }
}

pub fn draw_canvas2(canvas: &mut Canvas, offset: i32, state: &State2D) {
    canvas.clear();

    let center_scr = Point2D::new(SCREEN_WIDTH as i32 / 2, SCREEN_HEIGTH as i32 / 2);

    let rect_w = 350;
    let rect_h = 250;

    // House Base
    let rect_x = center_scr.x - rect_w / 2;
    let rect_y = center_scr.y - rect_h / 2 + 50;
    let rect_p = Point2D::new(rect_x, rect_y);

    // Triangle Roof
    let tri_p1 = Point2D::new(rect_p.x - 30, rect_p.y);
    let tri_p2 = Point2D::new(rect_p.x + rect_w + 30, rect_p.y);
    let tri_p3 = Point2D::new(center_scr.x, rect_p.y - 150);

    // Windows Circle
    let radius = 40;
    let mut radius1 = radius / 2;
    let mut radius2 = radius / 2;
    let c1_x = center_scr.x - rect_w / 4;
    let c2_x = center_scr.x + rect_w / 4;
    let c_y = center_scr.y + 40 - rect_h / 8;
    let center_c1 = Point2D::new(c1_x, c_y);
    let center_c2 = Point2D::new(c2_x, c_y);

    // Door
    let door_w = 50;
    let door_h = 100;
    let door_x = center_scr.x - door_w / 2;
    let door_y = center_scr.y - door_h + 50 + rect_h / 2;
    let door_p = Point2D::new(door_x, door_y);

    match state {
        State2D::HouseInit => {}
        State2D::WindowStrLeft | State2D::WindowStrRight | State2D::WindowSettle => {
            radius1 -= offset;
            radius2 += offset;
        }
    }

    // Draw Order
    // Door
    canvas.draw_rect(&door_p, door_w, door_h, Color::MAGENTA);
    // House Base
    canvas.draw_rect(&rect_p, rect_w, rect_h, Color::WHITE);
    // Roof
    canvas.draw_line_2d(&tri_p1, &tri_p2, Color::RED);
    canvas.draw_line_2d(&tri_p2, &tri_p3, Color::RED);
    canvas.draw_line_2d(&tri_p1, &tri_p3, Color::RED);
    // Windows
    canvas.draw_circle(&center_c1, radius1, Color::CYAN);
    canvas.draw_circle(&center_c2, radius2, Color::CYAN);
}
