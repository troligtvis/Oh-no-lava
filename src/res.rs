use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;
// use std::collections::HashMap;

pub struct CollisionEvent {
    pub hit_collision: Collision, 
    pub hit_transform: Transform,
    pub hit_size: Vec2,
}

#[derive(Default)]
pub struct ContactListenerState {
    pub event_reader: EventReader<CollisionEvent>,
}

pub struct ShootEvent;

#[derive(Default)]
pub struct WeaponShootCommandListenerState {
    pub event_reader: EventReader<ShootEvent>,
}

pub struct JumpEvent;

#[derive(Default)]
pub struct JumpListenerState {
    pub event_reader: EventReader<JumpEvent>,
}