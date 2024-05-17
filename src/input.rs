use crate::collision_detection::{floor_collision, wall_collision};
use crate::sound::play_audio;
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::f32::consts::PI;

use crate::{
    floor::Floor, play_background_audio, sound::BackgroundSong, EditorState, GameState,
    MainMenuText, Player, Wall,
};
use crate::asset_loader::SceneAssets;
use crate::enemy::{ActionState, create_projectile, EnemyState};
use crate::map::ShotgunTag;
use crate::player::{PLAYER_HIT_RADIUS, PLAYER_PROJECTILE_SPEED};
use crate::utility::normalize;


#[derive(Default)]
pub struct MouseState {
    pub press_coords: Vec<Vec2>,
}

impl Resource for MouseState {}

pub fn main_menu_input(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut text_query: Query<(&mut MainMenuText, &mut Text, &mut Transform)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    background_song: Query<Entity, With<BackgroundSong>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if keyboard_input.just_pressed(KeyCode::Tab) {
        lock_cursor(&mut window);
    }

    let mut text_count = 0;
    let mut selected_id = 0;
    for (text, _, _) in text_query.iter_mut() {
        if !text.shadow {
            text_count += 1;
        }
        selected_id = text.selected_id;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        if selected_id != 0 {
            selected_id -= 1;
        }
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        if selected_id != text_count - 1 {
            selected_id += 1;
        }
    }

    for (mut text, _, _) in text_query.iter_mut() {
        text.selected_id = selected_id;
    }

    if keyboard_input.just_pressed(KeyCode::Enter) {
        if selected_id == 0 {
            next_game_state.set(GameState::InGame);

            for (_, _, mut transform) in text_query.iter_mut() {
                transform.scale = Vec3::ZERO;
            }

            for entity in background_song.iter() {
                commands.entity(entity).despawn();
            }

            play_background_audio(
                &mut asset_server,
                &mut commands,
                "sounds\\at_dooms_gate.ogg".to_string(),
            );
        }

        if selected_id == 2 {
            std::process::exit(0);
        }
    }
}

pub fn keyboard_input(
    mut wall_query: Query<&mut Wall>,
    mut floor_query: Query<&mut Floor>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
    time: Res<'_, Time<Real>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    editor_state: Res<State<EditorState>>,
    mut next_editor_state: ResMut<NextState<EditorState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if keyboard_input.just_pressed(KeyCode::AltLeft) {
        match game_state.get() {
            GameState::InGame => {
                next_game_state.set(GameState::InEditor);
            }
            GameState::InEditor => {
                next_game_state.set(GameState::InGame);
                let mut primary_window = window.single_mut();

                if primary_window.cursor.grab_mode != CursorGrabMode::Locked {
                    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
                    primary_window.cursor.visible = false;
                }
            }
            GameState::MainMenu => {}
        }
    }

    if keyboard_input.just_pressed(KeyCode::Tab) {
        lock_cursor(&mut window);
    }

    if keyboard_input.just_pressed(KeyCode::KeyM) {
        match game_state.get() {
            GameState::InEditor => match editor_state.get() {
                EditorState::World => {
                    next_editor_state.set(EditorState::Map);
                }
                EditorState::Map => {
                    next_editor_state.set(EditorState::World);
                }
            },
            GameState::InGame => {}
            GameState::MainMenu => {}
        }
    }

    for mut player in query.iter_mut() {
        let mut speed = 50.;

        let mut movement = Vec3::ZERO;

        if keyboard_input.just_pressed(KeyCode::F1) {
            match player.noclip {
                true => player.noclip = false,
                false => player.noclip = true,
            }
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            let yaw_offset = player.yaw - PI / 2.0;
            movement += Vec3::new(yaw_offset.cos(), 0., yaw_offset.sin());
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement -= Vec3::new(player.yaw.cos(), 0., player.yaw.sin());
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let yaw_offset = player.yaw + PI / 2.0;
            movement += Vec3::new(yaw_offset.cos(), 0., yaw_offset.sin());
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement += Vec3::new(player.yaw.cos(), 0., player.yaw.sin());
        }
        if keyboard_input.pressed(KeyCode::KeyS) && keyboard_input.pressed(KeyCode::KeyW) {
            movement = Vec3::new(0., movement.y, 0.);
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            speed = speed * 2.;
        }

        if player.noclip {
            if keyboard_input.pressed(KeyCode::KeyE) {
                movement += Vec3::new(0., 1., 0.);
            }
            if keyboard_input.pressed(KeyCode::KeyQ) {
                movement -= Vec3::new(0., 1., 0.);
            }
        }

        if !player.noclip {
            // GRAVITY + JUMPING
            if keyboard_input.pressed(KeyCode::Space) {
                // jump

                if player.gravity < 30. {
                    movement += Vec3::new(0., 2., 0.); // add y velocity
                }
                player.gravity += 1.0; // sort of a "timer" that counts how long the player jumped
            }

            // first part checks if player hit "rock bottom" so that they dont fall forever, then checks if the player is currently falling
            if player.y + movement.y - player.height < -3. && movement.y <= 0. {
                movement.y = 0.
            } else {
                movement.y -= 1. // If the player is in the air and not fallint -> start falling
            }

            // CHECKS EVERY FLOOR FOR COLLISION
            for floor in floor_query.iter_mut() {
                floor_collision(&floor, &mut movement, &mut player);
            }

            // CHECKS EVERY WALL FOR COLLISION
            for wall in wall_query.iter_mut() {
                wall_collision(&wall, &mut movement, &mut player)
            }

            // when player has landed on the ground reset the "timer"
            if player.gravity > 0. && movement.y == 0. {
                player.gravity = 0.0;
            }
        }

        movement = movement.normalize_or_zero() * speed * time.delta_seconds();

        player.x += movement.x;
        player.y += movement.y;
        player.z += movement.z;
    }
}

pub fn mouse_input(
    mut commands: Commands,
    mut scene_assets: Res<SceneAssets>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_state: ResMut<MouseState>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<&mut Player>,
    game_state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut shotgun_query: Query<&mut EnemyState, With<ShotgunTag>>
) {
    for event in mouse_motion_events.read() {
        let primary_window = window_query.single_mut();

        if primary_window.cursor.grab_mode == CursorGrabMode::Locked {
            let delta = event.delta;
            for mut player in query.iter_mut() {
                let sensitivity = 0.005;

                player.yaw += delta.x * sensitivity;
                player.yaw = player.yaw.rem_euclid(2.0 * PI);
                player.pitch -= delta.y * sensitivity;
                player.pitch = player.pitch.clamp(-30.0 * PI / 180., 30.0 * PI / 180.);
            }
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        mouse_state.press_coords.clear();

        // Animation for realod, do not fire
        let mut state = shotgun_query.single_mut();

        match state.state {
            ActionState::Dormant => {
                state.state = ActionState::Attacking;

                // Shoot projectile
                for player in query.iter_mut() {
                    let position = Vec3::new(player.x, player.y, player.z);
                    let direction = normalize(player.forward_vector());

                    create_projectile(
                        &mut commands,
                        scene_assets.projectile.clone(),
                        position,
                        direction,
                        PLAYER_HIT_RADIUS + 10.,
                        PLAYER_PROJECTILE_SPEED,
                    )
                }

                let window = window_query.single_mut();
                let _window_pos = window.cursor_position().unwrap();

                if game_state.get() != &GameState::InEditor {
                    // plays shotgun sound
                    play_audio(asset_server, commands, "shotgun.ogg");
                }
            }
            _ => ()
        }


    }

    if mouse_button_input.just_released(MouseButton::Left) {
        let window = window_query.single_mut();

        if window.cursor_position() == None {
            return;
        }

        let _window_pos = window.cursor_position().unwrap();
    }
}

pub fn lock_cursor(window: &mut Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = window.single_mut();

    if primary_window.cursor.grab_mode == CursorGrabMode::Locked {
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    } else {
        // for a game that doesn't use the cursor (like a shooter):
        // use `Locked` mode to keep the cursor in one place
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;

        // also hide the cursor
        primary_window.cursor.visible = false;
    }
}
