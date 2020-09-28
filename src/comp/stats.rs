use bevy::prelude::*;

#[derive(Debug, Default, Properties)]
pub struct MovementSpeed {
    pub accel: f32,
    pub max: f32,
}

#[derive(Debug, Default, Properties)]
pub struct JumpForce(pub f32);

#[derive(Debug, Default, Properties)]
pub struct Grounded(pub bool); // Move into physics.rs ? :shrug:

#[derive(Debug, Default, Properties)]
pub struct Wall;

#[derive(Debug, Default, Properties)]
pub struct Ground;

#[derive(Debug, Default, Properties)]
pub struct Facing(pub f32);

#[derive(Debug, Default, Properties)]
pub struct TimeToLive(pub Timer);
#[derive(Debug, Default, Properties)]
pub struct WallStickTimer(pub Timer);