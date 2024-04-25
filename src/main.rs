mod asset_loader;
mod egui;
mod floor;
mod input;
mod map;
mod player;
mod render;
mod vertex;
mod wall;

use bevy::{
    prelude::*,
    core::FrameCount,
    render::mesh::Mesh,
    sprite::Material2dPlugin,
    window::{PresentMode, WindowTheme},
};
use bevy_egui::EguiPlugin;
use std::f32::consts::PI;

use crate::{
    asset_loader::{load_assets, AssetLoaderPlugin, SceneAssets},
    egui::ui_example_system,
    input::{keyboard_input, mouse_input, MouseState},
    map::load_from_file,
    player::Player,
    render::render,
    render::CustomMaterial,
    wall::Wall,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(MouseState {
            press_coords: Vec::new(),
        })
        .add_plugins(AssetLoaderPlugin)
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Raycaster".into(),
                    name: Some("Raycaster".into()),
                    resolution: (1280., 720.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
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
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup)
        .add_systems(Update, (change_title, make_visible))
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
    mut asset_server: Res<SceneAssets>,
    window_query: Query<&Window>,
) {
    let map = load_from_file("map.txt").expect("Error: could not open map");

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
        &mut asset_server,
        window_query,
    );

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

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    // The delay may be different for your app or system.
    if frames.0 == 3 {
        // At this point the gpu is ready to show the app so we can make the window visible.
        // Alternatively, you could toggle the visibility in Startup.
        // It will work, but it will have one white frame before it starts rendering
        window.single_mut().visible = true;
    }
}