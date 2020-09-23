use bevy::prelude::*;

#[derive(Debug, Default, Properties)]
pub struct MovementSpeed {
    pub accel: f32,
    pub max: f32,
}

#[derive(Debug, Default, Properties)]
pub struct JumpForce(pub f32);

#[derive(Debug, Default, Properties)]
pub struct Grounded(pub bool);

