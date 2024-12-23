pub const FRAMERATE: u32 = 60;
pub const FRAMERATE_CALC: u32 = 1_000_000_000u32 / FRAMERATE;
pub const PIXEL_SIZE: u32 = 3;
pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGTH: u32 = 800;
pub const X_OFFSET: i32 = (SCREEN_WIDTH / 2) as i32;
pub const Y_OFFSET: i32 = (SCREEN_HEIGTH / 2) as i32;
pub const CUBE_LINE_LEN: i32 = 100;
pub const PYRA_LINE_LEN: i32 = 100;
pub const ROTATION: f32 = 0.02;

pub enum State3D {
    YAxisRot,
    FromYToX,
    XAxisRot,
    FromXToZ,
    ZAxisRot,
    FromZToY,
}

pub enum State2D {
    HouseInit,
    WindowStrLeft,
    WindowStrRight,
    WindowSettle,
}
