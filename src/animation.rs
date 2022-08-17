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
    start_index: u32,
    frames_count: u32,
    current_index: u32,
    offset: u32,
}

impl AnimationData {
    pub fn new(state: AnimationState, start_index: u32, frames_count: u32, offset: u32) -> Self {
        Self { state, start_index, frames_count, current_index: start_index, offset }
    }

    fn get_state(&self) -> u32 {
        self.state as u32
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