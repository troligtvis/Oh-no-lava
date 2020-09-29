pub mod actor;
pub mod physics;
pub mod player;
pub mod stats;
pub mod furnitures;
pub mod particles;

use bevy::prelude::*;
use crate::res;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<res::GroundContactListenerState>()
            .add_plugin(actor::GameActorPlugin)
            .add_plugin(physics::GamePhysicsPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(furnitures::FurniturePlugin)
            .add_plugin(particles::ParticlePlugin)
            .add_system(stats::collider_contact_system.system()); // TODO - add to plugin 
    }
}