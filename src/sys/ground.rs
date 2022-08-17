use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::animation::{Animation, AnimationData, AnimationTimer};
use crate::comp::ground::{Ground};
use crate::comp::physics::{Body};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ground)
        .add_startup_system(setup_lava_bubbles);
    }
}

fn setup_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(SpriteBundle {
        texture: asset_server.load("lava_floor.png"),
        transform: Transform::from_translation(Vec3::new(0., -600. / 2. + 64. / 2., 1.)),
        ..default()
    })
    .insert(Ground)
    .insert(Body((800., 64.)));
}

fn test() {
    for i in 0..10 {
        println!("Index: {}", i);
    }
}

fn setup_lava_bubbles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    const _WIDTH: f32 = 800.;
    const _HEIGHT: f32 = 600.;
    let width_8 = _WIDTH / 8.;
    let scale = Vec3::ONE * 2.;
    let padding: f32 = 64.;
    let mut rng = thread_rng();

    let texture_size = Vec2::new(32., 32.);

    for i in 0..8 {
        let texture_handle = asset_server.load("lava_bubbles.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, texture_size, 25, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        
        let start_index = rng.gen_range(0..24) as u32;
        let x = -_WIDTH / 2. + 32. + i as f32 * width_8;

        println!("Start index: {}", start_index);

        let mut transform = Transform::from_scale(scale);
        transform.translation = Vec3::new(x, -_HEIGHT / 2. + texture_size.y + padding, 1.);

        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .insert(Ground)
        .insert(Animation::new(
            vec![
                AnimationData::new(crate::animation::AnimationState::Idle, 0, 25, start_index),
            ],
            crate::animation::AnimationState::Idle,
        ))
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
    }
}