use crate::{
    bevy::{
        input::mouse::MouseButtonInput,
        prelude::*,
        sprite::collide_aabb::{collide, Collision},
        window::CursorMoved,
    },
    bevy::transform::components::{Transform},
    projectile::*,
    util::*,
    animation::*,
    particles::*,
    Collider, Force, GravitationalAttraction, Gravity, SpawnTimer, Speed, Velocity,
    Wall, Ground, 
};

const TOTAL_NUMBER_OF_JUMPS: usize = 2;
const FALL_MULTIPLIER: f32 = 2.5;
const LOW_JUMP_MULTIPLIER: f32 = 2.;

#[derive(Default)]
struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

pub struct Player {
    speed: Speed,
    air_speed: Speed,
    jump_force: Force,
    num_of_jumps: usize,
    controls: KeyboardControls,
    collision_data: CollisionData,
    is_wall_jumping: bool,
}

struct KeyboardControls {
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
    jump: KeyCode,
}

fn spawn_system(
    mut commands: Commands, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {

    let texture_handle = asset_server
        .load_sync(
            &mut textures,
            "resources/player_animation.png",
        )
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 10, 3);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let translation = Translation(Vec3::new(0., -SCR_HEIGHT / 2. + 80., 0.));

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(1.0),
            translation,
            draw: Draw {
                is_transparent: true,
                is_visible: true,
                render_commands: Vec::new(),
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true)) // Anim timer
        .with(Sprite {
            size: Vec2::new(32., 32.),
            ..Default::default()
        })
        .with(Player {
            speed: Speed(200.),
            air_speed: Speed(180.),
            jump_force: Force(10.),
            num_of_jumps: TOTAL_NUMBER_OF_JUMPS,
            controls: KeyboardControls {
                left: KeyCode::A,
                right: KeyCode::D,
                up: KeyCode::W,
                down: KeyCode::S,
                jump: KeyCode::Space,
            },
            collision_data: CollisionData::default(),
            is_wall_jumping: false,
        })
        .with(Collider::Solid)
        .with(GravitationalAttraction::default())
        .with(Velocity(Vec2::zero()))
        .with(Raycast {
            origin: Vec2::zero(), 
            direction: Direction::Left,
            size: Vec2::new(4., 1.),
        });

        commands.spawn(
            SpriteComponents {
                material: materials.add(Color::rgba(1., 0.2, 0., 1.).into()),
                translation: Translation(Vec3::zero()),
                sprite: Sprite {
                    size: Vec2::zero(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(DebugRaycast);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            translation: Translation(Vec3::zero()),
            sprite: Sprite {
                size: Vec2::new(5., 5.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Crosshair {
            aim: Vec2::zero(),
            distance: 40.,
        });
}

fn flip_sprite_system(
    mut rotation: Mut<Rotation>,
    player: Mut<Player>,
) {
    let pi = std::f32::consts::PI;
    rotation.0 = if player.collision_data.facing_direction > 0 {
        if player.collision_data.touching_wall {
            Quat::from_rotation_y(-pi)
        } else {
            Quat::identity()
        }
    } else {
        if player.collision_data.touching_wall {
            Quat::identity()
        } else {
            Quat::from_rotation_y(-pi)
        }
    };
}

fn crosshair_system(
    windows: Res<Windows>,
    mut state: ResMut<MouseState>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut query: Query<(&Player, &Translation)>,
    mut c_query: Query<(&mut Crosshair, &mut Translation, &mut Sprite)>,
) {
    let window_size = get_window_size(windows);

    for (_player, translation) in &mut query.iter() {
        for (mut crosshair, mut c_translation, _) in &mut c_query.iter() {
            let mut b_receive_event = false;
            for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
                b_receive_event = true;

                let cursor_pos =
                    event.position - Vec2::new(window_size.width / 2., window_size.height / 2.);

                set_aim(
                    &translation.0.truncate(),
                    &cursor_pos,
                    crosshair.distance,
                    &mut c_translation,
                );
                crosshair.aim = cursor_pos;
            }

            if !b_receive_event {
                set_aim(
                    &translation.0.truncate(),
                    &crosshair.aim,
                    crosshair.distance,
                    &mut c_translation,
                );
            }
        }
    }

    for (_, _, mut sprite) in &mut c_query.iter() {
        sprite.size = if mouse_button_input.pressed(MouseButton::Left) {
            Vec2::new(10., 10.)
        } else {
            Vec2::new(5., 5.)
        };
    }
}

fn set_aim(a: &Vec2, b: &Vec2, distance: f32, translation: &mut Translation) {
    let direction = get_direction(a, b);
    let norm = direction.normalize() * distance;
    let aim = Vec2::new(a.x() + norm.x(), a.y() + norm.y());

    *translation.x_mut() = aim.x();
    *translation.y_mut() = aim.y();
}

fn jump(velocity: &mut Velocity, player: &mut Player) {
    if player.num_of_jumps == 0 { return }

    player.collision_data.below = false;

    // Adjust jump force depending on how many jumps player has already made
    let adjuster: f32 = if TOTAL_NUMBER_OF_JUMPS == player.num_of_jumps {
        1.
    } else {
        0.7
    };

    velocity.0.set_y(player.jump_force.0 * 20. * adjuster);

    player.num_of_jumps -= 1;
    // println!("Normal jump");
}

fn wall_jump(velocity: &mut Velocity, player: &mut Player, direction: Vec2) {
    let force = player.jump_force.0 * 10.;
    velocity.0.set_x(force * direction.x());
    velocity.0.set_y(force * direction.y());

    // Reset jumps when wall jumping
    player.num_of_jumps = TOTAL_NUMBER_OF_JUMPS;
    // println!("Wall jump");
}

fn player_input_system(
    _time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut animation: ResMut<Animation>,
    mut query: Query<(
        &mut Player,
        &mut Translation,
        &mut Velocity,
        &mut GravitationalAttraction,
        &mut Transform,
    )>,
) {
    for (mut player, mut translation, mut velocity, mut attraction, mut transform) in &mut query.iter() {
        if keyboard_input.just_pressed(player.controls.jump) {
            attraction.is_grounded = false;

            if player.collision_data.below {
                player.collision_data.prev_below = false;

                // Move tha position of the player up a bit to avoid colliding with object before jumping
                *translation.0.y_mut() = translation.0.y() + 0.2;
                jump(&mut velocity, &mut player);
            } else if player.collision_data.either_side_collision() && !player.collision_data.below
            {
                let multiplier = if player.collision_data.right { -1. } else { 1. };

                *translation.0.x_mut() += 4. * multiplier;

                player.is_wall_jumping = true;
                player.num_of_jumps = TOTAL_NUMBER_OF_JUMPS;

                player.collision_data.facing_direction = if player.collision_data.left {
                    1
                } else {
                    -1
                };
                
                //transform.set_scale(Vec3::new(2.1, 1., 0.));

                wall_jump(&mut velocity, &mut player, Vec2::new(1. * multiplier, 3.));
            } else {
                jump(&mut velocity, &mut player);
            }
        }

        let mut direction = Vec2::zero();
        if keyboard_input.pressed(player.controls.left) {
            let x = if attraction.is_grounded {
                player.speed.0
            } else {
                player.air_speed.0
            };

            direction.set_x(-x);
        }

        if keyboard_input.pressed(player.controls.right) {
            let x = if attraction.is_grounded {
                player.speed.0
            } else {
                player.air_speed.0
            };

            direction.set_x(x);
        }

        if direction.x() > 0. {
            player.collision_data.facing_direction = 1;
        } else if direction.x() < 0. {
            player.collision_data.facing_direction = -1;
        }

        if player.is_wall_jumping {
            let direction: Vec3 = direction.extend(0.);
            if velocity.0.x().abs() > player.air_speed.0 {
                velocity.0.set_x(direction.x());
            } else {
                *velocity.0.x_mut() += direction.x();
            }
        } else {
            let direction: Vec3 = direction.extend(0.);
            velocity.0.set_x(direction.x());
        }

        animation.current_index = if velocity.0.x().abs() > 0.0 {
            1
        } else {
            0
        };
    }
}

fn adjust_jump_system(
    time: Res<Time>,
    gravity: Res<Gravity>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Velocity, &GravitationalAttraction)>,
) {
    let dt = time.delta_seconds;

    for (player, mut velocity, affected) in &mut query.iter() {
        if affected.is_grounded || player.collision_data.touching_wall {
            break;
        }

        // Better jumping
        if velocity.0.y() < 0.0 {
            let vel = Vec2::unit_y() * -gravity.0 * (FALL_MULTIPLIER - 1.) * dt;
            velocity.0 += vel;
        } else if velocity.0.y() > 0.0 && !keyboard_input.pressed(player.controls.jump) {
            let vel = Vec2::unit_y() * -gravity.0 * (LOW_JUMP_MULTIPLIER - 1.) * dt;
            velocity.0 += vel;
        }
    }
}

fn player_collision_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(
        &mut Player,
        &mut Translation,
        &mut Velocity,
        &mut GravitationalAttraction,
        &Sprite,
    )>,
    mut collider_query: Query<(&Collider, Without<Player, &Velocity>, &Translation, &Sprite)>,
) {
    for (mut player, mut player_translation, mut velocity, mut attraction, sprite) in
        &mut player_query.iter()
    {
        let player_size = sprite.size;
        let check_translation = Translation::new(
            player_translation.x(), 
            player_translation.0.y() - 0.2, 
            0.
        );

        attraction.is_grounded = false;

        player.collision_data.reset();

        for (_collider, c_velocity, translation, sprite) in &mut collider_query.iter() {
            let collision = collide(translation.0, sprite.size, check_translation.0, player_size);
            if let Some(collision) = collision {
                match collision {
                    Collision::Bottom => {
                        attraction.is_grounded = true;
                        player.collision_data.below = true;
                        player.is_wall_jumping = false;

                        if !player.collision_data.prev_below {
                            player.collision_data.prev_below = true;
                            let mut position = player_translation.0.truncate();
                            *position.y_mut() -= player_size.y() / 2.; 
                            spawn_dust_particle(&mut commands, &mut materials, position);
                        }

                        // Adjust player to be on top of platform
                        player_translation.0.set_y(
                            translation.y() + sprite.size.y() / 2. + player_size.y() / 2. + 0.1,
                        );
                        player.num_of_jumps = TOTAL_NUMBER_OF_JUMPS;
                        // Set players velocity the same as the platform
                        *velocity.0.x_mut() = velocity.0.x() + c_velocity.0.x();
                    }
                    Collision::Right => {
                        player_translation.0.set_x(
                            translation.0.x() - sprite.size.x() / 2. - player_size.x() / 2. + 0.1,
                        );

                        player.collision_data.right = true
                    }
                    Collision::Left => {
                        player_translation.0.set_x(
                            translation.0.x() + sprite.size.x() / 2. + player_size.x() / 2. - 0.1,
                        );

                        player.collision_data.left = true;
                    }
                    _ => {}
                };
            }
        }
    }
}

struct CollisionData {
    left: bool,
    right: bool,
    above: bool,
    below: bool,
    facing_direction: i8, // 1 or -1
    prev_below: bool,
    touching_wall: bool,
}

impl Default for CollisionData {
    fn default() -> Self {
        CollisionData {
            left: false,
            right: false,
            above: false,
            below: false,
            facing_direction: 1,
            prev_below: false,
            touching_wall: false,
        }
    }
}

impl CollisionData {
    pub fn reset(&mut self) {
        self.left = false;
        self.right = false;
        self.above = false;
        self.below = false;
    }

    pub fn either_side_collision(&self) -> bool {
        self.left || self.right
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let animation = Animation {
            data: vec!{
                AnimationData {
                    start_index: 0,
                    frames_count: 4,
                },
                AnimationData {
                    start_index: 10,
                    frames_count: 5,
                },
            },
            current_index: 0,
        };

        app.add_startup_system(spawn_system.system())
            .init_resource::<MouseState>()
            .add_resource(SpawnTimer {
                timer: Timer::from_seconds(2.0, true),
            })
            .add_resource(animation)
            .add_plugin(AnimationPlugin)
            .add_system(player_collision_system.system())
            .add_system(player_input_system.system())
            .add_system(update_raycast_system.system())
            .add_system(draw_raycast_gizmo_system.system())
            .add_system(raycast_hit_system.system())
            .add_system(adjust_jump_system.system())
            .add_system(crosshair_system.system())
            .add_system(dust_particle_cleanup_system.system())
            .add_system(flip_sprite_system.system())
            .add_stage_before(stage::UPDATE, "spawn_projectile")
            .add_stage_after(stage::UPDATE, "shoot_projectile")
            .add_system_to_stage("spawn_projectile", spawn_projectile_system.system())
            .add_system_to_stage("shoot_projectile", shoot_projectile_system.system());
    }
}

fn update_raycast_system(
    mut r_query: Query<&mut Raycast>,
    mut query: Query<(&Player, &Translation, &Sprite)>
) {
    for (player, p_translation, p_sprite) in &mut query.iter() {
        for mut raycast in &mut r_query.iter() {

            let direction = player.collision_data.facing_direction;
            let x = if direction > 0 {
                p_translation.x() + p_sprite.size.x() / 2.
            } else {
                p_translation.x() - p_sprite.size.x() / 2.
            };
            
            raycast.origin = Vec2::new(x, p_translation.y());
        }
    }
}

fn draw_raycast_gizmo_system(
    mut d_query: Query<(&DebugRaycast, &mut Translation, &mut Sprite)>,
    mut p_query: Query<(&Player, &Raycast)>,
) {
    for (_player, raycast) in &mut p_query.iter() {
        for (_, mut translation, mut sprite) in &mut d_query.iter() {
            translation.set_x(raycast.origin.x());
            translation.set_y(raycast.origin.y());
            sprite.size = raycast.size;
        }
    }
}

fn raycast_hit_system(
    mut r_query: Query<(&mut Player, &Raycast, &mut GravitationalAttraction)>,
    mut q: Query<(&Wall, &Translation, &Sprite)>,
) {
    for (mut player, raycast, mut attraction) in &mut r_query.iter() {
        player.collision_data.touching_wall = false;

        for (_wall, w_translation, w_sprite) in &mut q.iter() {
            let collide = collide(raycast.origin.extend(0.), raycast.size, w_translation.0, w_sprite.size);
            if let Some(collision) = collide {
                match collision {
                    Collision::Right => {
                        player.collision_data.touching_wall = true;
                    }, 
                    Collision::Left => {
                        player.collision_data.touching_wall = true;
                    },
                    _ => {},
                };   
            }
        }

        attraction.is_touching_wall = player.collision_data.touching_wall;
    }
}

fn flip_sprite(sprite: &Sprite) {

}

struct DebugRaycast;
struct DebugRaycastPlugin;

impl Plugin for DebugRaycastPlugin {
    fn build(&self, app: &mut AppBuilder) {
        
    }
}

fn debug_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(
        SpriteComponents {
            material: materials.add(Color::rgba(1., 0.2, 0., 1.).into()),
            translation: Translation(Vec3::zero()),
            sprite: Sprite {
                size: Vec2::zero(),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(DebugRaycast);
}

fn update_raycast_gizmos_system(
    mut d_query: Query<&Raycast>,
    mut query: Query<&Raycast>,
) {
    for raycast in &mut query.iter() {

    }
}

struct Raycast {
    origin: Vec2,
    direction: Direction,
    //t: f32,
    size: Vec2,
    // t_min: f32,
    // t_max: f32,
}

struct RayHit {
    kind: RayHitKind,
    t: f32,
}

#[derive(Clone, Copy, PartialEq)]
enum RayHitKind {
    Wall,
    Ground,
    Enemy,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn get_vector(&self) -> Vec2 {
        match self {
            Direction::Left => Vec2::new(-1., 0.),
            Direction::Right => Vec2::new(1., 0.),
            Direction::Up => Vec2::new(0., -1.),
            Direction::Down => Vec2::new(0., 1.),
        }
    }
}