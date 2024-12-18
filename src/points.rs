use crate::utils::*;

pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point2D {
    pub fn from_3d(point_3d: &Point3D) -> Self {
        let x_new = point_3d.x - point_3d.z + X_OFFSET;
        let y_new = (point_3d.x + 2 * point_3d.y + point_3d.z) / 2 + Y_OFFSET;

        Point2D { x: x_new, y: y_new }
    }
}

impl Point3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Point3D { x, y, z }
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let x_new = (self.x as f32 * angle.cos() + self.z as f32 * angle.sin());
        let z_new = (-self.x as f32 * angle.sin() + self.z as f32 * angle.cos());

        self.x = x_new as i32;
        self.z = z_new as i32;
    }

    pub fn cube_vertices(center: &Point3D) -> [Point3D; 8] {
        let half_side = CUBE_LINE_LEN / 2;
        [
            // Bottom-front-left
            Point3D {
                x: center.x - half_side,
                y: center.y - half_side,
                z: center.z - half_side,
            },
            // Bottom-front-right
            Point3D {
                x: center.x + half_side,
                y: center.y - half_side,
                z: center.z - half_side,
            },
            // Bottom-back-left
            Point3D {
                x: center.x - half_side,
                y: center.y - half_side,
                z: center.z + half_side,
            },
            // Bottom-back-right
            Point3D {
                x: center.x + half_side,
                y: center.y - half_side,
                z: center.z + half_side,
            },
            // Top-front-left
            Point3D {
                x: center.x - half_side,
                y: center.y + half_side,
                z: center.z - half_side,
            },
            // Top-front-right
            Point3D {
                x: center.x + half_side,
                y: center.y + half_side,
                z: center.z - half_side,
            },
            // Top-back-left
            Point3D {
                x: center.x - half_side,
                y: center.y + half_side,
                z: center.z + half_side,
            },
            // Top-back-right
            Point3D {
                x: center.x + half_side,
                y: center.y + half_side,
                z: center.z + half_side,
            },
        ]
    }
}
