pub mod actor;
pub mod physics;
pub mod stats;

use bevy::prelude::*;

#[derive(Debug, Default, Properties)]
pub struct Despawnable;