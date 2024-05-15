mod asset_loader;
mod egui;
mod floor;
mod input;
mod map;
mod player;
mod render;
mod vertex;
mod wall;
mod collision_detection;
mod enemy;
mod movement;
mod sprites;
mod sound;
mod timer;
mod utility;


use bevy::{
    core::FrameCount,
    prelude::*,
    render::mesh::Mesh,
    sprite::Material2dPlugin,
    window::{PresentMode, PrimaryWindow, WindowTheme},
};
use bevy_egui::EguiPlugin;
use render::render_grid;
use std::f32::consts::PI;

use crate::{
    asset_loader::{load_assets, AssetLoaderPlugin, SceneAssets},
    egui::editor_ui,
    input::{keyboard_input, lock_cursor, mouse_input, MouseState},
    map::load_from_file,
    player::Player,
    render::CustomMaterial,
    render::{render, render_map},
    wall::Wall,
};
use crate::collision_detection::CollisionDetectionPlugin;
use crate::enemy::EnemyPlugin;
use crate::movement::MovementPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    InGame,
    InEditor,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum EditorState {
    World,
    Map,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct EditorSet;

fn main() {
    App::new()
        .insert_state(GameState::InGame)
        .insert_state(EditorState::Map)
        .configure_sets(Update, EditorSet.run_if(in_state(GameState::InEditor)))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(MouseState {
            press_coords: Vec::new(),
        })
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Raycaster".into(),
                    name: Some("Raycaster".into()),
                    resolution: (1280., 720.).into(),
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    visible: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),))
        .add_plugins(Material2dPlugin::<CustomMaterial>::default())
        .add_plugins(EguiPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                make_visible,
                change_title,
                keyboard_input,
                mouse_input,
                render,
                render_map,
            ),
        )
        .add_systems(Update, (editor_ui, render_grid).in_set(EditorSet))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut scene_assets: Res<SceneAssets>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let map = load_from_file("map.txt", &scene_assets.enemy_types).expect("Error: could not open map");

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(map.camera[0], map.camera[1], map.camera[2]).looking_at(
            Vec3::new(map.camera[3], map.camera[4], map.camera[5]),
            Vec3::Y,
        ),
        ..Default::default()
    });

    map.populate_scene(
        &mut commands,
        &mut meshes,
        &mut custom_materials,
        &mut scene_assets,
        &mut window_query,
    );

    commands.spawn(map);

    lock_cursor(&mut window_query);
}

fn change_title(mut windows: Query<&mut Window>, time: Res<'_, Time<Real>>, query: Query<&Player>) {
    let mut window = windows.single_mut();
    for player in query.iter() {
        let fps = 1. / time.delta_seconds();
        window.title = format!("Raycaster. FPS: {:.0}. Player position (x: {:.0}, y: {:.0}, z: {:.0}, yaw: {:.0}, pitch: {:.0})", 
            fps,
            player.x,
            player.y,
            player.z,
            player.yaw * (180.0 / PI),
            player.pitch * (180.0 / PI));
    }
}

/// At this point the gpu is ready to show the app and make the window visible.
fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}