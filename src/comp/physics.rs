use bevy::prelude::*;
//use bevy::sprite::collide_aabb::{collide, Collision};
use std::ops::{Deref, DerefMut};

/// This component represents entity's velocity.
#[derive(Clone, Debug, Default, Properties)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity(Vec2::new(x, y))
    }
}

impl Deref for Velocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// This component represents how much "drag" affects this entity.
#[derive(Debug, Default, Properties)]
pub struct Drag(pub f32);

/// This component represents primitive box collider around an entity.
#[derive(Debug, Default, Properties)]
pub struct ColliderBox {
    pub w: f32,
    pub h: f32,
}

impl ColliderBox {
    pub fn get_size(&self) -> Vec2 {
        Vec2::new(self.w, self.h)
    }
}

pub struct Gravity(pub f32);

#[derive(Debug, Properties)]
pub struct GravitationalAttraction {
    pub is_active: bool,
}

impl Default for GravitationalAttraction {
    fn default() -> Self {
        Self { is_active: true, }
    }
}

#[derive(Debug, Properties)]
pub struct CollisionData {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub below: bool,
}

impl CollisionData {
    pub fn either_side(&self) -> bool {
        self.left || self.right
    }

    pub fn reset(&mut self) {
        self.left = false;
        self.right = false;
        self.top = false;
        self.below = false;
    }
}

impl Default for CollisionData {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
            top: false,
            below: false,
        }
    }
}

#[derive(Debug, Properties)]
pub struct Raycast {
    pub origin: Vec2,
    pub direction: Vec2,
    pub t_min: f32,
    pub t_max: f32,
}

impl Raycast {
    pub fn down() -> Self {
        Self {
            origin: Vec2::zero(),
            direction: Vec2::new(0., -1.),
            t_min: 8.,
            t_max: 12.,
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up, 
    Down, 
}

impl Direction {
    pub fn into_direction(dir: Vec2) -> Direction {
        if dir == Vec2::new(-1., 0.) {
            Direction::Left
        } else if dir == Vec2::new(1., 0.) {
            Direction::Right
        } else if dir == Vec2::new(0., 1.) {
            Direction::Up
        } else {
            Direction::Down
        }
    }
}