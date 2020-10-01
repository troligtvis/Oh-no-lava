#![allow(dead_code)]

use bevy::{
    self, 
    prelude::*, 
    render::pass::ClearColor,
    diagnostic::FrameTimeDiagnosticsPlugin,
};

mod setup;
mod animation;
mod util;
mod res;
mod comp;
mod sys;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(67. / 255., 75. / 255., 77. / 255.)))
        .add_resource(WindowDescriptor {
            title: "Oh no, lava!".to_string(),
            width: util::SCR_WIDTH as u32,
            height: util::SCR_HEIGHT as u32,
            resizable: false,
            ..Default::default()
        })
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(animation::AnimationPlugin)
        .add_plugin(sys::GameLogicPlugin)
        .add_plugin(setup::GameSetupPlugin)
        .add_default_plugins()
        
        .add_resource(comp::physics::Gravity(9.82 * 40.))
        
        .run();
}