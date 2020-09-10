use crate::bevy::prelude::*;

// due to not being able to access windows from a `startup_system`
// fixed values will be needed for screen size during startup
// https://github.com/bevyengine/bevy/issues/175
pub const SCR_WIDTH: f32 = 800.0;
pub const SCR_HEIGHT: f32 = 600.0;

pub fn get_distance(a: &Vec2, b: &Vec2) -> f32 {
    (a.x().powi(2) - b.x().powi(2) + a.y().powi(2) - b.y().powi(2)).sqrt()
}

pub fn get_direction(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2::new(b.x() - a.x(), b.y() - a.y())
}

pub fn get_window_size(windows: Res<Windows>) -> Size {
    if let Some(window) = windows.get_primary() {
        Size::new(window.height as f32, window.height as f32)
    } else {
        Size::new(SCR_WIDTH, SCR_HEIGHT)
    }
}
