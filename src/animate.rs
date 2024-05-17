use bevy::prelude::*;
use bevy::app::{App, Update};
use bevy::time;
use bevy::prelude::{Component, Plugin};
use crate::enemy::{ActionState, EnemyState};
use crate::timer::AnimationTimer;


#[derive(Component)]
pub struct AnimationIndices {
    pub(crate) first: usize,
    pub(crate) last: usize,
}

#[derive(Component)]
pub struct AnimationComponent {
    pub dormant: AnimationIndices,
    pub attack: AnimationIndices,
    pub dying: AnimationIndices,
    pub dead: AnimationIndices,
}

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
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
    for (animation_component, mut state, mut timer, mut atlas) in &mut query.iter_mut() {

        let indices: &AnimationIndices = match state.state {
            ActionState::Dormant => { &animation_component.dormant },
            ActionState::Attacking => { &animation_component.attack },
            ActionState::Dying => { &animation_component.dying }
            ActionState::Dead => { &animation_component.dead }
        };

        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index >= indices.last {
                // Change back to correct state when animation is done
                match state.state {
                    ActionState::Dormant => {  }, // Don't change
                    ActionState::Attacking => {
                        state.state = ActionState::Dormant;
                        &animation_component.dormant.first;
                    },
                    ActionState::Dying => { state.state = ActionState::Dead }
                    ActionState::Dead => { } // Don't Change
                };

                indices.first
            } else {
                atlas.index + 1
            }
        }
    }
}

