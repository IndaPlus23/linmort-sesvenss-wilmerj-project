use bevy::prelude::*;
use bevy::app::{App, Update};
use bevy::time;
use bevy::prelude::{Component, Plugin};
use crate::enemy::{ActionState, EnemyState};
use crate::timer::AnimationTimer;


#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component)]
struct AnimationComponent {
    dormant: AnimationIndices,
    attack: AnimationIndices,
    dead: AnimationIndices,
}

impl AnimationComponent {
    pub fn new(dormant: usize, attack: usize, dead: usize) -> Self {
        Self {
            dormant: AnimationIndices {first: 0, last: dormant},
            attack: AnimationIndices {first: dormant + 1, last: attack},
            dead: AnimationIndices {first: attack + 1, last: dead},
        }
    }
}

struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}

fn animate(
    time: Res<Time>,
    mut query: Query<(
        &AnimationComponent,
        &mut EnemyState,
        &mut AnimationTimer,
        &mut TextureAtlas
    )>,
) {
    for (animation_component, state, mut timer, mut atlas) in &mut query {

        let indices: &AnimationIndices = match state.state {
            ActionState::Dormant => { &animation_component.dormant },
            ActionState::Attacking => { &animation_component.attack },
            ActionState::Dead => { &animation_component.dead }
        };

        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            }
        }
    }
}