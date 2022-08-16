#![allow(dead_code)]

use bevy::{ 
    prelude::*,
    render::{texture::ImageSettings},
    // reflect::TypeUuid,
    input::{keyboard::KeyCode, Input},
};
    //render::pass::ClearColor,
    //diagnostic::FrameTimeDiagnosticsPlugin,

// mod setup;
mod animation;
// mod util;
// mod res;
mod comp;
mod sys;

use comp::physics::{Body, Velocity, Drag, GravitationalAttraction};
use comp::actor::{Player, Controller};

use crate::{comp::stats::{Facing, MovementSpeed}, 
animation::{Animation, AnimationState, AnimationData, AnimationTimer}};

fn main() {
    App::new()
    .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
    //.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(ClearColor(Color::rgb(0.24, 0.24, 0.24)))
    .insert_resource(WindowDescriptor {
        title: "Oh no, lava!".to_string(),
        width: 800.0,
        height: 600.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(sys::GameLogicPlugin)
    .add_plugin(animation::AnimationPlugin)
    // .add_plugin(AnimationPlugin{})
    .add_startup_system(setup)
    // .add_system(handle_input)
    // .add_system(animate_sprite)
    .run();

    
        // .add_resource(ClearColor(Color::rgb(67. / 255., 75. / 255., 77. / 255.)))
        // .add_resource(WindowDescriptor {
        //     title: "Oh no, lava!".to_string(),
        //     width: util::SCR_WIDTH as u32,
        //     height: util::SCR_HEIGHT as u32,
        //     resizable: false,
        //     ..Default::default()
        // })
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(animation::AnimationPlugin)
        // .add_plugin(sys::GameLogicPlugin)
        // .add_plugin(setup::GameSetupPlugin)
        // .add_plugins(DefaultPlugins)
        // .add_resource(comp::physics::Gravity(9.82 * 40.))
        //.run();
}



// struct AnimationPlugin {}

// impl Plugin for AnimationPlugin {
//     fn build(&self, app: &mut App) { 
        
//         let animation = Animation::new(
//             vec!{
//                 AnimationData::new(0, 4),
//                 AnimationData::new(10, 5),
//                 AnimationData::new(20, 10),
//             }
//         );
//         app.insert_resource(animation)
//         .add_system(animate_system);
//     }

    
// }

// fn animate_system(
//     time: Res<Time>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     animation: Res<Assets<Animation>>,
//     mut query: Query<(
//         &mut AnimationTimer,
//         &mut TextureAtlasSprite,
//         &Handle<TextureAtlas>,
//     )>
// ) {
    
//     // if let Some(animation_data) = animation.get_current_data() {
//     //     for (timer, mut sprite) in query.iter_mut() {
//     //         if timer.finished {
//     //             sprite.index = animation_data.get_index();
//     //         }
//     //     }
//     // }
// }

// #[derive(TypeUuid)
// #[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
// struct Animation {
//     data: Vec<AnimationData>,
// }

// impl Animation {
//     pub fn new(data: Vec<AnimationData>) -> Self {
//         Self { data }
//     }
// }

// struct AnimationData {
//     start_index: u32,
//     frames_count: u32,
//     current_index: u32,
// }

// impl AnimationData {
//     pub fn new(start_index: u32, frames_count: u32) -> Self {
//         Self { 
//             start_index,
//             frames_count,
//             current_index: 0,
//         }
//     }

//     fn get_index(&mut self) -> u32 {
//         self.current_index += 1;
//         if self.current_index >= self.frames_count {
//             self.current_index = 0;
//         }
        
//         let i = self.frames_count - self.current_index;
//         (i % self.frames_count) + self.start_index
//     }
// }

// #[derive(Component, Deref, DerefMut)]
// struct AnimationTimer(Timer);

// fn animate_sprite(
//     time: Res<Time>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<(
//         &mut AnimationTimer,
//         &mut TextureAtlasSprite,
//         &Handle<TextureAtlas>,
//     )>,
// ) {
//     for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
//             sprite.index = (20 + sprite.index + 1) % texture_atlas.textures.len();
//         }
//     }
// }

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("inframe_idlerun.png");
    
//let texture_handle = asset_server.load("player_animation.png");
    let tile_size = Vec2::new(32.0, 32.0);
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, 8, 3);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, 8, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(Camera2dBundle::default());


    const SCALE: f32 = 2.;
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(SCALE)),
            ..default()
        })
        .insert(Player)
        .insert(Controller::default())
        .insert(Velocity::default())
        .insert(MovementSpeed{accel: 100., max: 200.})
        .insert(Drag(1.85))
        .insert(GravitationalAttraction::default())
        .insert(Body((16. * SCALE, 32. * SCALE)))
        .insert(Facing(1.))
        .insert( Animation::new( 
            vec![
                // AnimationData::new(AnimationState::Idle, 0, 4, 0),
                // AnimationData::new(AnimationState::Run, 10, 5, 0),
                AnimationData::new(AnimationState::Idle, 0, 5, 0),
                AnimationData::new(AnimationState::Run, 16, 8, 0),
            ], 
        AnimationState::Run))
        
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
        // .insert(Body((32. * SCALE, 32. * SCALE)));
        // .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

// fn setup_ground(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     commands
//         .spawn_bundle(SpriteBundle {
//         texture: asset_server.load("lava_floor.png"),
//         transform: Transform::from_translation(Vec3::new(0., -600. / 2. + 64. / 2., 1.)),
//         ..default()
//     })
//     .insert(Ground)
//     .insert(Body((800., 64.)));
// }



// fn handle_input(
//     time: Res<Time>,
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<(With<Player>, &mut Transform)>
// ) {
//     for (_, mut transform) in &mut query {
//         if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
//             transform.translation.y += 1.0; // * time.delta_seconds();
//         }

//         if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
//             transform.translation.y -= 1.0;
//         }
        
//         if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
//             transform.translation.x -= 1.0;
//         }

//         if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
//             transform.translation.x += 1.0;
//         }
//     } 
// }