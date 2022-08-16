use bevy::{prelude::*};

use crate::comp::{actor::Player, ground::Ground};

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(animate_sprite);
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<( &mut Animation, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {

    for (mut animation_data, mut timer, mut sprite) in query.iter_mut() {
        
        timer.0.tick(time.delta());

        if timer.0.finished() {
            if let Some(data) = animation_data.get_current_data() {
                let index = data.get_index();
                sprite.index = index;
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum AnimationState {
    // None,
    Idle,
    Run,
}

#[derive(Copy, Clone)]
pub struct AnimationData {
    state: AnimationState,
    start_index: u8,
    frames_count: u8,
    current_index: u8,
    offset: u8,
}

impl AnimationData {
    pub fn new(state: AnimationState, start_index: u8, frames_count: u8, offset: u8) -> Self {
        Self { state, start_index, frames_count, current_index: start_index, offset }
    }

    fn get_state(&self) -> u8 {
        self.state as u8
    }

    fn get_index(&mut self) -> usize {
        self.current_index += 1;
        if self.current_index >= self.frames_count {
            self.current_index = 0;
        }
                
        let i = self.frames_count - self.current_index;
        (((i + self.offset) % self.frames_count) + self.start_index) as usize
    }
}

#[derive(Component)]
pub struct Animation {
    data: Vec<AnimationData>,
    current_anim: usize,
}

impl Animation {
    pub fn new(data: Vec<AnimationData>, start_anim: AnimationState) -> Self {
        Self {
            data,
            current_anim: start_anim as usize, 
        }
    }
        
    fn get_current_data(&mut self) -> Option<&mut AnimationData> {
        self.data.get_mut(self.current_anim)
    }
    
    pub fn set_animation(&mut self, state: AnimationState) {
        self.current_anim = state as usize;
    }
}

// pub enum AnimCommonState {
//     Idle,
//     Run,
// }

// impl AnimStateDescriptor for AnimCommonState {
//     fn name(&self) -> &str {
//         match self {
//             Self::Idle => "idle",
//             Self::Run => "run",
//         }
//     }
// }

// pub trait AnimStateDescriptor {
//     fn name(&self) -> &str;
// }

// pub struct Animation {
//     data: HashMap<String, AnimData>,
//     current_anim: String,
// }

// impl Animation {
//     pub fn new(data: Vec<AnimData>, start_anim: &str) -> Self {
//         Self {
//             data: data.into_iter().map(|x| (x.get_name(), x)).into_iter().collect(),
//             current_anim: start_anim.to_string(), 
//         }
//     }

//     fn get_current_data(&mut self) -> Option<&mut AnimData> {
//         self.data.get_mut(&self.current_anim)
//     }

//     pub fn set_anim(&mut self, name: &str) {
//         self.current_anim = name.to_string();
//     }
// }

// pub struct AnimData {
//     name: String,
//     start_index: u32,
//     frames_count: u32,
//     current_index: u32,
// }

// impl AnimData {
//     pub fn new(name: &str, start_index: u32, frames_count: u32) -> Self {
//         Self {
//             name: name.to_string(),
//             start_index,
//             frames_count,
//             current_index: 0,
//         }
//     }

//     fn get_name(&self) -> String {
//         self.name.clone()
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

// fn animate_sprite_system(
//     mut animation: ResMut<Animation>,
//     mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
// ) {
//     if let Some(animation_data) = animation.get_current_data() {
//         for (timer, mut sprite) in query.iter_mut() {
//             if timer.finished {
//                 sprite.index = animation_data.get_index();
//             }
//         }
//     }
// }

// pub struct AnimationPlugin;

// impl Plugin for AnimationPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.add_resource(
//             Animation::new(
//                 vec!{
//                     AnimData::new(AnimCommonState::Idle.name(), 0, 4),
//                     AnimData::new(AnimCommonState::Run.name(), 10, 5),
//                 },
//                 AnimCommonState::Idle.name()
//             )
//         )
//         .add_system(animate_sprite_system.system())
//         .add_system(animate_lava_system.system());
//     }
// }

// // Lava bubbles animation
// pub struct Lava {
//     pub data: LavaAnimData,
// }

// pub struct LavaAnimData {
//     pub index: u32,
//     pub frames_count: u32,
// }

// impl LavaAnimData {
//     fn get_next_index(&mut self) -> u32 {
//         self.index = (self.index + 1) % self.frames_count;
//         self.index
//     }
// }

// fn animate_lava_system(
//     mut query: Query<(&mut Lava, &mut Timer, &mut TextureAtlasSprite)>,
// ) {
//     for (mut lava, timer, mut sprite) in query.iter_mut() {
//         if timer.finished {
//             sprite.index = lava.data.get_next_index();
//         }
//     }
// }