use crate::{
    bevy::prelude::*,
};

#[derive(Debug, Default, Properties)]
pub struct Particle;

pub struct Shrinkable;

#[derive(Debug, Properties)]
pub struct DustParticle {
    pub size: Vec2,
}

impl Default for DustParticle {
    fn default() -> Self {
        Self {
            size: Vec2::new(2., 2.)
        }
    }
}