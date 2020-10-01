use crate::bevy::prelude::*;
use crate::util::{SCR_WIDTH, SCR_HEIGHT};
use crate::{res, comp, animation::{self, Lava, LavaAnimData}};

use rand::{thread_rng, Rng};

pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<res::ColorMaterialStorage>()
            .add_plugin(animation::AnimationPlugin)
            .add_startup_system(setup_resource.system())
            .add_startup_system(setup_player_system.system())
            .add_startup_system(setup_game_system.system())
            .add_startup_system(setup_lava_bubbles_system.system());
    }
}

fn setup_resource(
    mut material_storage: ResMut<res::ColorMaterialStorage>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    material_storage.storage.insert(
        "Projectile".to_string(), 
        materials.add(res::Colors::WATER.into())
    );

    material_storage.storage.insert(
        "Default_Furniture".to_string(), 
        materials.add(Color::rgb(0.1, 0.1, 0.1).into())
    );

    material_storage.storage.insert(
        "Dust".to_string(), 
        materials.add(res::Colors::LINEN.into())
    );
}

fn setup_game_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dComponents::default());

    let texture_handle = asset_server
    .load_sync(
        &mut textures,
        "resources/lava_floor.png",
    )
    .unwrap();
    let texture = textures.get(&texture_handle).unwrap();

    // Ground
    commands
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0., -SCR_HEIGHT / 2. + texture.size.y() / 2., 1.)),
            
            ..Default::default()
        })
        .with(comp::physics::Velocity::default())
        .with(comp::physics::ColliderBox {
            w: SCR_WIDTH,
            h: texture.size.y(),
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
        .with(comp::stats::Ground)
        .with(comp::physics::ColliderBox {
            w: 40.,
            h: SCR_HEIGHT,
        })
        .with(comp::stats::Wall);
}

fn setup_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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
        .with(comp::stats::Facing(1.))
        .with(comp::stats::StretchTimer(Timer::from_seconds(0.6, false)));

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(105. / 255., 105. / 255., 105. / 255.).into()),
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

fn setup_lava_bubbles_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(
            &mut textures,
            "resources/lava_bubbles.png",
        )
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 25, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    let width_8 = SCR_WIDTH / 8.;
    let mut rng = thread_rng();

    for i in 0..8 {
        let start_index = rng.gen_range(0, 24) as u32;
        let x = -SCR_WIDTH / 2. + 32. + i as f32 * width_8;
        
        commands.spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(x, -SCR_HEIGHT / 2. + texture.size.y() + 64., 1.)).with_scale(2.),
            draw: Draw {
                is_transparent: true,
                is_visible: true,
                render_commands: Vec::new(),
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Lava {
            data: LavaAnimData {
                index: start_index,
                frames_count: 25,
            }
        });
    }
}
