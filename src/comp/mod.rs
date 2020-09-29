pub mod actor;
pub mod physics;
pub mod stats;
pub mod particles;

use bevy::prelude::*;

#[derive(Debug, Default, Properties)]
pub struct Despawnable;