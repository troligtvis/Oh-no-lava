#![allow(dead_code)]

use bevy::{
    self, 
    prelude::*, 
    render::pass::ClearColor,
    diagnostic::FrameTimeDiagnosticsPlugin,
};

// mod furniture;
// mod player;
// mod projectile;
mod animation;
use animation::*;
mod util;
// mod particles;
mod res;

use util::*;
mod comp;
mod sys;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_resource(WindowDescriptor {
            title: "Oh no, lava!".to_string(),
            width: util::SCR_WIDTH as u32,
            height: util::SCR_HEIGHT as u32,
            resizable: false,
            ..Default::default()
        })
        .init_resource::<res::ColorMaterialStorage>()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(AnimationPlugin)
        .add_plugin(sys::GameLogicPlugin)
        .add_default_plugins()
        // .init_resource::<res::ColorMaterialStorage>()
        // .add_plugin(furniture::FurniturePlugin)
        //.add_plugin(player::PlayerPlugin)
        //.add_plugin(PhysicsPlugin)
        // .add_plugin(sys::physics::GamePhysicsPlugin)
        
        .add_startup_system(setup_resource.system())
        .add_startup_system(setup.system())
        .add_startup_system(setup_scene.system())
        .add_resource(comp::physics::Gravity(9.82 * 40.))
        .run();
}

// #[deri`

// fn spawn_player(
//     commands: &mut commands,
//     asset_server: &Res<AssetServer>,
//     materials: &Res<ColorMaterialStorage>
// ) {

// }

fn setup_resource(
    mut material_storage: ResMut<res::ColorMaterialStorage>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = materials.add(Color::rgb(1., 0., 0.).into());
    material_storage.storage.insert("Projectile".to_string(), material);
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,//Res<res::ColorMaterialStorage>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dComponents::default());

    // Player
    let texture_handle = asset_server
    .load_sync(
        &mut textures,
        "resources/player_animation.png",
    )
    .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 10, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(100., -SCR_HEIGHT / 2. + 80., 0.)),
            draw: Draw {
                is_transparent: true,
                is_visible: true,
                render_commands: Vec::new(),
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true)) // Anim timer
        .with(comp::physics::Velocity::default())
        .with(comp::physics::Drag(1.85))
        .with(comp::actor::Player::default())
        .with(comp::actor::Controller::default())
        .with(comp::stats::MovementSpeed {
            accel: 100.0,
            max: 200.0,
        })
        .with(comp::stats::JumpForce(200.))
        .with(comp::physics::ColliderBox {
            w: 16.,
            h: 32.,
        })
        .with(comp::physics::CollisionData::default())
        .with(comp::stats::Grounded(false))
        .with(comp::physics::GravitationalAttraction::default())
        .with(comp::physics::Raycast {
            origin: Vec2::zero(),
            direction: Vec2::new(1., 0.),
            t_min: 8., // Half player size
            t_max: 12.,
        })
        .with(comp::physics::GroundRaycasts(
            vec![
                comp::physics::Raycast::down(),
                comp::physics::Raycast::down(),
                comp::physics::Raycast::down(),
            ],
        ))
        .with(comp::stats::Facing(1.));

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1., 0., 0.).into()),
            transform: Transform::from_translation(Vec3::zero()),
            sprite: Sprite {
                size: Vec2::new(5., 5.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(comp::actor::Crosshair {
            distance: 40.,
        });
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());

    // Ground
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(0., -SCR_HEIGHT / 2., 0.)),
            sprite: Sprite {
                size: Vec2::new(SCR_WIDTH, 20.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(comp::physics::Velocity::default())
        .with(Collider::Solid)
        .with(comp::physics::ColliderBox {
            w: SCR_WIDTH,
            h: 20.,
        })
        .with(comp::stats::Ground);

    // Walls
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.8, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(SCR_WIDTH / 2. - 100., -SCR_HEIGHT / 2., 0.)),
            sprite: Sprite {
                size: Vec2::new(40., SCR_HEIGHT),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(comp::physics::Velocity::default())
        .with(Collider::Solid)
        .with(comp::stats::Wall)
        .with(comp::physics::ColliderBox {
            w: 40.,
            h: SCR_HEIGHT,
        })
        .with(comp::stats::Ground);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.8, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(-SCR_WIDTH / 2. + 100., -SCR_HEIGHT / 2., 0.)),
            sprite: Sprite {

                size: Vec2::new(40., SCR_HEIGHT),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(comp::physics::Velocity::default())
        .with(Collider::Solid)
        .with(comp::stats::Ground)
        .with(comp::physics::ColliderBox {
            w: 40.,
            h: SCR_HEIGHT,
        })
        .with(comp::stats::Wall);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.6, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0., -SCR_HEIGHT + 80., 0.)),
            // x1: -32, x2: 32
            // y1: , y2:
            sprite: Sprite {
                size: Vec2::new(64., SCR_HEIGHT),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(comp::physics::Velocity::default())
        .with(Collider::Solid)
        .with(comp::stats::Ground)
        .with(comp::physics::ColliderBox {
            w: 64.,
            h: SCR_HEIGHT,
        })
        .with(comp::stats::Wall);
}

#[derive(PartialEq)]
enum Collider {
    Solid,
}