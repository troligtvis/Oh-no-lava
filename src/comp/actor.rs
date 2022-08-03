use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Component)]
pub struct Player {}

pub struct Furniture;

// // Different types of spawning furnitures
// enum FurnitureShape {
//     Chair,
//     Table,
//     Sofa,
//     Refrigerator,
//     TV,
//     Lamp,
// }

#[derive(Debug, Default)]
pub struct Controller {
    pub cursor_position: Vec2,
    pub movement: Vec2,
    pub action: VecDeque<ControllerAction>,
}

impl Controller {
    pub fn reset_movement(&mut self) {
        self.movement = Vec2::zero();
    }
}

#[derive(Debug)]
pub enum ControllerAction {
    Shoot,
    Jump,
    WallJump,
}

pub struct Crosshair {
    pub distance: f32,
}

pub struct CrosshairController {
    pub aim: Vec2,
    pub distance: f32,
}

pub struct Projectile {
    pub direction: Vec2,
}