use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;
// use std::collections::HashMap;

pub struct GroundCollisionEvent {
    pub hit_collision: Collision, 
    pub hit_transform: Transform,
    pub hit_size: Vec2,
}

#[derive(Default)]
pub struct GroundContactListenerState {
    pub event_reader: EventReader<GroundCollisionEvent>,
}

pub struct ShootEvent;

#[derive(Default)]
pub struct ShootListenerState {
    pub event_reader: EventReader<ShootEvent>,
}

pub struct JumpEvent;

#[derive(Default)]
pub struct JumpListenerState {
    pub event_reader: EventReader<JumpEvent>,
}