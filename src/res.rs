use bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;
use std::collections::HashMap;
use crate::comp;

pub struct Colors;
impl Colors {
    pub const WATER: Color = Color::rgb_linear(212. / 255., 241. / 255., 249. / 255.);
    pub const LAVA: Color = Color::rgb_linear(207. / 255., 16. / 255., 32. / 255.);
    pub const INTENSE_LAVA: Color = Color::rgb_linear(238. / 255., 18. / 255., 66. / 255.);
    pub const LINEN: Color = Color::rgba_linear(246. / 255., 242. / 255., 237. / 255., 0.6);
}

pub struct GroundCollisionEvent {
    pub hit_collision: Collision, 
    pub hit_transform: Transform,
    pub hit_size: Vec2,
    pub hit_velocity: comp::physics::Velocity,
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

pub struct WallJumpEvent;

#[derive(Default)]
pub struct WallJumpListenerState(pub EventReader<WallJumpEvent>);

#[derive(Debug, Default)]
pub struct ColorMaterialStorage {
    pub storage: HashMap<String, Handle<ColorMaterial>>,
}