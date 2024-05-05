use bevy::{
    prelude::*,
    reflect::TypePath,
    render::mesh::Mesh,
    render::render_resource::AsBindGroup,
    sprite::{Material2d, Material2dPlugin},
    window::{PresentMode, WindowTheme},
};
mod input;
mod player;
mod render;
mod wall;
mod floor;
use crate::floor::Floor;
mod vertice;
mod egui;
mod asset_loader;
mod map;
mod sprites;
mod movement;
mod utility;
mod enemy;
mod collision_detection;
mod sound;
use crate::sprites::SpritePlugin;

use bevy::{
    prelude::*,
};
use bevy_egui::EguiPlugin;
use std::f32::consts::PI;

use crate::{
    input::{keyboard_input, mouse_input, MouseState},
    player::Player,
    render::render,
    wall::Wall,
    vertice::Vertice,
    egui::ui_example_system,
    asset_loader::{AssetLoaderPlugin, load_assets, SceneAssets},
    map::load_from_file,
    render::CustomMaterial,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(MouseState {
            press_coords: Vec::new(),
        })
        .add_plugins(AssetLoaderPlugin)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Raycaster".into(),
                        name: Some("Raycaster".into()),
                        resolution: (1280., 720.).into(),
                        present_mode: PresentMode::AutoVsync,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            //FrameTimeDiagnosticsPlugin,
            //LogDiagnosticsPlugin::default(),
            //bevy::diagnostic::SystemInformationDiagnosticsPlugin::default()
        ))
        .add_plugins(Material2dPlugin::<CustomMaterial>::default())
        .add_plugins(EguiPlugin)
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, mouse_input)
        .add_systems(Update, render)
        .add_systems(Update, change_title)
        .add_systems(Update, ui_example_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    //mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut scene_assets: Res<SceneAssets>,
) {
    let map = load_from_file("map.txt", &scene_assets.enemy_types).expect("Error: could not open map");

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(map.camera[0], map.camera[1], map.camera[2])
            .looking_at(
                Vec3::new(map.camera[3], map.camera[4], map.camera[5]),
                Vec3::Y,
            ),
        ..Default::default()
    });

    map.populate_scene(
        &mut commands,
        &mut meshes,
        &mut custom_materials,
        &mut scene_assets);

    commands.spawn(map);
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
