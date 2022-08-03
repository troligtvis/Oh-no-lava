#![allow(dead_code)]

use bevy::{ 
    prelude::*,
    render::{texture::ImageSettings, camera::ScalingMode},
    // reflect::TypeUuid,
    input::{keyboard::KeyCode, Input}, ecs::system::Insert, transform, sprite::collide_aabb::{self, collide, Collision},
};
    //render::pass::ClearColor,
    //diagnostic::FrameTimeDiagnosticsPlugin,

// mod setup;
// mod animation;
// mod util;
// mod res;
// mod comp;
// mod sys;

fn main() {
    App::new()
    .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor {
        title: "Oh no, lava!".to_string(),
        width: 800.0,
        height: 600.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(PhysicsPlugin)
    // .add_plugin(AnimationPlugin{})
    .add_startup_system(setup)
    .add_startup_system(setup_ground)
    .add_system(handle_input)
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
    let texture_handle = asset_server.load("player_animation.png");
    let tile_size = Vec2::new(32.0, 32.0);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, 10, 3);
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
        .insert(Velocity::default())
        .insert(Drag(1.85))
        .insert(GravitationalAttraction::default())
        .insert(Body((16. * SCALE, 32. * SCALE)));
        // .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}

fn setup_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

#[derive(Component)]
struct Player;

fn handle_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(With<Player>, &mut Transform)>
) {
    for (_, mut transform) in &mut query {
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 1.0; // * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 1.0;
        }
        
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 1.0;
        }
    } 
}

struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Gravity::default())
        .add_system(apply_velocity)
        .add_system(apply_drag)
        .add_system(apply_gravity)
        .add_system(player_ground_collision);
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>
) {
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.0.extend(0.) * time.delta_seconds();
    }
}

fn apply_drag(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Drag)>
) {
    for (mut velocity, drag) in &mut query {
        *velocity = Velocity(velocity.0.lerp(Vec2::ZERO, time.delta_seconds() * drag.0));
    }
}

fn apply_gravity(
    gravity: Res<Gravity>,
    time: Res<Time>,
    mut query: Query<(&GravitationalAttraction, &mut Velocity)>
) {
    for (attraction, mut velocity) in &mut query {
        if attraction.is_active {
            velocity.0.y -= gravity.0 * time.delta_seconds(); 
        } else {
            velocity.0.y = 0.;
        }
    }
}

#[derive(Component, Default)]
struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

#[derive(Component)]
struct Drag(pub f32);

#[derive(Component)]
struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
        Self(9.82 * 40.)
    }
}

#[derive(Component)]
struct GravitationalAttraction {
    pub is_active: bool,
}

impl Default for GravitationalAttraction {
    fn default() -> Self {
        Self { is_active: true, }
    }
}

#[derive(Component)]
struct Body(pub (f32, f32));

impl Body {
    pub fn get_size(&self) -> Vec2 {
        Vec2::new(self.0.0, self.0.1)
    }
}

#[derive(Component)]
struct Ground;

fn player_ground_collision(
    mut query1: Query<(With<Player>, &Body, &Transform, &mut GravitationalAttraction)>,
    mut query2: Query<(With<Ground>, &Body, &Transform)>
) {
    for (_, player_body, player_transform, mut attraction) in query1.iter_mut() {
        for (_, other_body, other_transform) in query2.iter_mut() {
            let mut translation = player_transform.translation.clone();
            translation.y -= 1.;

            let collision = collide(
                other_transform.translation,
                other_body.get_size(),
                translation,
                player_body.get_size()
            );

            if let Some(hit) = collision {
                attraction.is_active = !matches!(hit, Collision::Bottom);    
            } else {
                attraction.is_active = true;
            }
        }
    }
}