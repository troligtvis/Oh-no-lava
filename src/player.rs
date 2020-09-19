use crate::{
    bevy::{
        input::mouse::MouseButtonInput,
        prelude::*,
        sprite::collide_aabb::{collide, Collision},
        window::CursorMoved,
    },
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

struct StretchTimer(Timer);

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
    did_jump: bool,
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


    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., -SCR_HEIGHT / 2. + 80., 0.)),
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
            did_jump: false,
        })
        .with(Collider::Solid)
        .with(GravitationalAttraction::default())
        .with(Velocity(Vec2::zero()))
        .with(Raycast {
            origin: Vec2::zero(), 
            direction: Direction::Left,
            size: Vec2::new(4., 1.),
        })
        .with(GroundRaycast {
            origin: Vec2::zero(),
            direction: Direction::Down,
            size: Vec2::new(28., 16.)
        })
        .with(StretchTimer(Timer::from_seconds(0.6, false)));

        commands.spawn(
            SpriteComponents {
                material: materials.add(Color::rgba(1., 0.2, 0., 1.).into()),
                transform: Transform::from_translation(Vec3::zero()),
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
            transform: Transform::from_translation(Vec3::zero()),
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
    mut transform: Mut<Transform>,
    // mut rotation: Mut<Rotation>,
    player: Mut<Player>,
) {
    let pi = std::f32::consts::PI;
    let rotation = if player.collision_data.facing_direction > 0 {
        if player.collision_data.touching_wall {
            Quat::from_rotation_y(pi)
        } else {
            Quat::identity()
        }
    } else {
        if player.collision_data.touching_wall {
            Quat::identity()
        } else {
            Quat::from_rotation_y(pi)
        }
    };

    transform.set_rotation(rotation);
}

fn crosshair_system(
    windows: Res<Windows>,
    mut state: ResMut<MouseState>,
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut query: Query<(&Player, &Transform)>,
    mut c_query: Query<(&mut Crosshair, &mut Transform, &mut Sprite)>,
) {
    let window_size = get_window_size(windows);

    for (_player, transform) in &mut query.iter() {
        for (mut crosshair, mut c_transform, _) in &mut c_query.iter() {
            let mut b_receive_event = false;
            for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
                b_receive_event = true;

                let cursor_pos =
                    event.position - Vec2::new(window_size.width / 2., window_size.height / 2.);

                set_aim(
                    &transform.translation().truncate(),
                    &cursor_pos,
                    crosshair.distance,
                    &mut c_transform,
                );
                crosshair.aim = cursor_pos;
            }

            if !b_receive_event {
                set_aim(
                    &transform.translation().truncate(),
                    &crosshair.aim,
                    crosshair.distance,
                    &mut c_transform,
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

fn set_aim(a: &Vec2, b: &Vec2, distance: f32, transform: &mut Transform) {
    let direction = get_direction(a, b);
    let norm = direction.normalize() * distance;
    let aim = Vec2::new(a.x() + norm.x(), a.y() + norm.y());

    transform.set_translation(aim.extend(0.));
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

fn stretch_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut StretchTimer, &mut Transform)>
) {
    for (mut player, mut stretch_timer, mut transform) in &mut query.iter() {
        stretch_timer.0.tick(time.delta_seconds);

        if stretch_timer.0.finished {
            transform.set_non_uniform_scale(Vec3::one());
        }
    }
}

fn player_input_system(
    _time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut animation: ResMut<Animation>,
    mut query: Query<(
        &mut Player,
        &mut StretchTimer,
        &mut Velocity,
        &mut GravitationalAttraction,
        &mut Transform,
    )>,
) {
    for (mut player, mut timer, mut velocity, mut attraction, mut transform) in &mut query.iter() {
        if keyboard_input.just_pressed(player.controls.jump) {
            attraction.is_grounded = false;

            if player.collision_data.below {
                player.collision_data.prev_below = false;

                // Move tha position of the player up a bit to avoid colliding with object before jumping
                let mut translation = transform.translation();
                *translation.y_mut() += 0.2;
                transform.set_translation(translation);
 
                timer.0.reset();
                timer.0.duration = 0.5;
                
                transform.set_non_uniform_scale(Vec3::new(0.8, 1.2, 1.)); 

                jump(&mut velocity, &mut player);
                player.did_jump = true;
            } else if player.collision_data.either_side_collision() && !player.collision_data.below
            {
                let multiplier = if player.collision_data.right { -1. } else { 1. };
                
                let mut translation = transform.translation();
                *translation.x_mut() += 4. * multiplier;
                transform.set_translation(translation);

                player.is_wall_jumping = true;
                player.num_of_jumps = TOTAL_NUMBER_OF_JUMPS;

                player.collision_data.facing_direction = if player.collision_data.left {
                    1
                } else {
                    -1
                };
                

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

        if player.collision_data.touching_wall {

            if keyboard_input.pressed(player.controls.up) {
                velocity.0.set_y(1. * 64.);
            }
            
            if keyboard_input.pressed(player.controls.down) {
                velocity.0.set_y(-1. * 84.);
            }
        }

        let anim_index = if velocity.0.x().abs() > 0.0 {
            AnimationState::Run.name()
        } else {
            AnimationState::Idle.name()
        };
        animation.set_anim(anim_index);
    }
}



enum AnimationState {
    Idle,
    Run,
}

impl AnimationStateDescriptor for AnimationState {
    fn name(&self) -> &str {
        match self {
            Self::Idle => "idle",
            Self::Run => "run",
        }
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
        &mut Transform,
        &mut Velocity,
        &mut GravitationalAttraction,
        &Sprite,
    )>,
    mut collider_query: Query<(&Collider, Without<Player, &Velocity>, &Transform, &Sprite)>,
) {
    for (mut player, mut p_transform, mut velocity, mut attraction, p_sprite) in
        &mut player_query.iter()
    {
        let player_size = p_sprite.size;
        let mut check_translation = p_transform.translation();
        //*check_translation.y_mut() -= 0.1;

        attraction.is_grounded = false;

        let prev_side_collision = player.collision_data.either_side_collision();
        let prev_below = player.collision_data.below;
        player.collision_data.reset();

        for (_collider, c_velocity, c_transform, c_sprite) in &mut collider_query.iter() {

            // // Check ground
            // let mut translation = p_transform.translation();

            // let left_side = translation.x() - 0.2 > c_transform.translation().x() - c_sprite.size.x() / 2.;// - p_sprite.size.x() / 2.;
            // let right_side = translation.x() + 0.2 < c_transform.translation().x() + c_sprite.size.x() / 2.;// + p_sprite.size.x() / 2.;
            // let above = translation.y() - p_sprite.size.y() / 2. - 0.1 <= c_transform.translation().y() + c_sprite.size.y() / 2.;

            // if left_side && right_side && above {
            //     if !prev_bottom {
            //         let mut position = p_transform.translation().truncate();
            //         *position.y_mut() -= p_sprite.size.y() / 2.; 
            //         spawn_dust_particle(&mut commands, &mut materials, position);
            //     }

            //     *translation.y_mut() = c_transform.translation().y() + c_sprite.size.y() / 2. + p_sprite.size.y() / 2.;
            //     p_transform.set_translation(translation);

            //     player.num_of_jumps = TOTAL_NUMBER_OF_JUMPS; 
                
            //     // Set players velocity the same as the platform
            //     // *velocity.0.x_mut() = velocity.0.x() + c_velocity.0.x();

            //     attraction.is_grounded = true;
            //     player.collision_data.below = true;
            // }

            let collision = collide(c_transform.translation(), c_sprite.size, check_translation, p_sprite.size);
            if let Some(collision) = collision {
                match collision {
                    Collision::Bottom => {
                        println!("Bottom");
                        if !player.collision_data.prev_below {
                            player.collision_data.prev_below = true;
                            let mut position = p_transform.translation().truncate();
                            *position.y_mut() -= p_sprite.size.y() / 2.; 
                            spawn_dust_particle(&mut commands, &mut materials, position);
                        }

                        // Adjust player to be on top of platform 
                        let mut translation = p_transform.translation();

                        let left_side = translation.x() > c_transform.translation().x() - c_sprite.size.x() / 2.;// - p_sprite.size.x();
                        let right_side = translation.x() < c_transform.translation().x() + c_sprite.size.x() / 2.;// + p_sprite.size.x();
                        let above = translation.y() - p_sprite.size.y() / 2. - 0.1 <= c_transform.translation().y() + c_sprite.size.y() / 2.;

                        if left_side && right_side && above {
                            *translation.y_mut() = c_transform.translation().y() + c_sprite.size.y() / 2. + p_sprite.size.y() / 2. + 0.1;
                            p_transform.set_translation(translation);
                            
                            player.num_of_jumps = TOTAL_NUMBER_OF_JUMPS;

                            // Set players velocity the same as the platform
                            *velocity.0.x_mut() = velocity.0.x() + c_velocity.0.x();
                            
                            attraction.is_grounded = true;
                            player.collision_data.below = true;
                            player.is_wall_jumping = false; 
                        }
                    }
                    Collision::Right => {
                        let mut translation = p_transform.translation();

                        if translation.y() < c_transform.translation().y() + c_sprite.size.y() / 2. {
                            if translation.x() + p_sprite.size.x() / 2. > c_transform.translation().x() - c_sprite.size.x() / 2. + 0.1 {
                                *translation.x_mut() = c_transform.translation().x() - c_sprite.size.x() / 2. - p_sprite.size.x() / 2.;
                                p_transform.set_translation(translation);
                            }
                            
                            *velocity.0.x_mut() = velocity.0.x() + c_velocity.0.x();
                        }

                        player.collision_data.right = true                            
                    }
                    Collision::Left => {
                        let mut translation = p_transform.translation();

                        if translation.y() < c_transform.translation().y() + c_sprite.size.y() / 2. {
                            if translation.x() - p_sprite.size.x() / 2. < c_transform.translation().x() + c_sprite.size.x() / 2. - 0.1 {
                                *translation.x_mut() = c_transform.translation().x() + c_sprite.size.x() / 2. + p_sprite.size.x() / 2.;
                                p_transform.set_translation(translation);
                            }
                            
                            *velocity.0.x_mut() = velocity.0.x() + c_velocity.0.x();
                        }
                        
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
    left_edge: bool,
    right: bool,
    right_edge: bool,
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
            left_edge: false,
            right: false,
            right_edge: false,
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
        self.left_edge = false;
        self.right = false;
        self.right_edge = false;
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
        let animation = Animation::new(
            vec!{
                AnimationData::new(AnimationState::Idle.name(), 0, 4),
                AnimationData::new(AnimationState::Run.name(), 10, 5),
            },
            AnimationState::Idle.name()
        );

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
            // .add_system(ground_check_system.system())
            .add_system(draw_raycast_gizmo_system.system())
            .add_system(raycast_hit_system.system())
            .add_system(adjust_jump_system.system())
            .add_system(crosshair_system.system())
            .add_system(dust_particle_cleanup_system.system())
            .add_system(stretch_sprite_system.system())
            .add_system(flip_sprite_system.system())
            .add_stage_before(stage::UPDATE, "spawn_projectile")
            .add_stage_after(stage::UPDATE, "shoot_projectile")
            .add_system_to_stage("spawn_projectile", spawn_projectile_system.system())
            .add_system_to_stage("shoot_projectile", shoot_projectile_system.system())
            ;
    }
}

fn update_raycast_system(
    mut r_query: Query<(&mut Raycast, &mut GroundRaycast)>,
    mut query: Query<(&Player, &Transform, &Sprite)>
) {
    for (player, p_transform, p_sprite) in &mut query.iter() {
        for (mut raycast, mut g_raycast) in &mut r_query.iter() {
 
            let direction = player.collision_data.facing_direction;
            let x = if direction > 0 {
                p_transform.translation().x() + p_sprite.size.x() / 2.
            } else {
                p_transform.translation().x() - p_sprite.size.x() / 2.
            };
            
            raycast.origin = Vec2::new(x, p_transform.translation().y());
            // g_raycast.origin = Vec2::new(
            //     p_transform.translation().x() - p_sprite.size.x() / 2. + 2.,
            //     p_transform.translation().y() - p_sprite.size.y() / 2.,//+ g_raycast.size.y(),
            // );
        }
    }
}

fn draw_raycast_gizmo_system(
    mut d_query: Query<(&DebugRaycast, &mut Transform, &mut Sprite)>,
    mut p_query: Query<(&Player, &Raycast)>,
) {
    for (_player, raycast) in &mut p_query.iter() {
        for (_, mut transform, mut sprite) in &mut d_query.iter() {
            transform.set_translation(raycast.origin.extend(0.));
            // transform.translate(raycast.origin.extend(0.));
            // translation.set_x(raycast.origin.x());
            // translation.set_y(raycast.origin.y());
            sprite.size = raycast.size;
        }
    }
}

fn raycast_hit_system(
    mut r_query: Query<(&mut Player, &mut Transform, &Sprite, &mut Velocity, &Raycast, &mut GravitationalAttraction)>,
    mut q: Query<(&Wall, &Transform, &Velocity, &Sprite)>,
) {
    for (mut player, mut p_transform, p_sprite, mut p_velocity, raycast, mut attraction) in &mut r_query.iter() {
        player.collision_data.touching_wall = false;
        let player_size = p_sprite.size;
        for (_wall, w_transform, w_velocity, w_sprite) in &mut q.iter() {
            let collide = collide(raycast.origin.extend(0.), raycast.size, w_transform.translation(), w_sprite.size);
            if let Some(collision) = collide {
                match collision {
                    Collision::Left => {
                        player.collision_data.touching_wall = true;
                        let mut translation = p_transform.translation();

                        if translation.y() > w_transform.translation().y() + w_sprite.size.y() / 2. {
                            *translation.x_mut() = w_transform.translation().x() - w_sprite.size.x() / 2. + p_sprite.size.x() / 2.;
                            *translation.y_mut() = w_transform.translation().y() + w_sprite.size.y() / 2. + p_sprite.size.y() / 2.;
                            p_transform.set_translation(translation);
                        } 
                    },
                    Collision::Right => {
                        player.collision_data.touching_wall = true;
                        let mut translation = p_transform.translation();
                      
                        if translation.y() > w_transform.translation().y() + w_sprite.size.y() / 2. {
                            *translation.x_mut() = w_transform.translation().x() + w_sprite.size.x() / 2. - p_sprite.size.x() / 2.;
                            *translation.y_mut() = w_transform.translation().y() + w_sprite.size.y() / 2. + p_sprite.size.y() / 2.;
                            p_transform.set_translation(translation);
                        } 
                    },
                    _ => {},
                };   
            }
        }

        attraction.is_touching_wall = player.collision_data.touching_wall;
    }
}

fn ground_check_system(
    mut r_query: Query<(&mut Player, &mut Transform, &Sprite, &mut Velocity, &GroundRaycast, &mut GravitationalAttraction)>,
    mut q: Query<(&Ground, &Transform, &Velocity, &Sprite)>
) {
    for (mut player, mut p_transform, p_sprite, mut p_velocity, raycast, mut attraction) in &mut r_query.iter() {
        
        for (_ground, w_transform, w_velocity, w_sprite) in &mut q.iter() {
            let collide = collide(raycast.origin.extend(0.), raycast.size, w_transform.translation(), w_sprite.size);
            if let Some(collision) = collide { 
                match collision {
                    Collision::Top => {
                        // Adjust player to be on top of platform 
                        let mut translation = p_transform.translation();

                        *translation.y_mut() = w_transform.translation().y() + w_sprite.size.y() / 2. + p_sprite.size.y() / 2.;
                        p_transform.set_translation(translation);

                        player.num_of_jumps = TOTAL_NUMBER_OF_JUMPS;

                        // Set players velocity the same as the platform
                        *p_velocity.0.x_mut() += w_velocity.0.x();
                        
                        attraction.is_grounded = true;
                        player.collision_data.below = true;
                        // player.is_wall_jumping = false; 
                    },
                    _ => {},
                };
            }
        }
    }
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
            transform: Transform::from_translation(Vec3::zero()),
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

struct GroundRaycast {
    origin: Vec2,
    direction: Direction,
    size: Vec2,
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