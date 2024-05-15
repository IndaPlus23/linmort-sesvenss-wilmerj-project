mod asset_loader;
mod egui;
mod floor;
mod hud;
mod input;
mod map;
mod player;
mod render;
mod vertex;
mod wall;

use bevy::{
    core::FrameCount,
    prelude::*,
    render::mesh::Mesh,
    render::render_asset::RenderAssetUsages,
    render::render_resource::{
        Extent3d, Texture, TextureDescriptor, TextureDimension, TextureFormat,
        TextureViewDescriptor, TextureViewDimension,
    },
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    window::{PresentMode, PrimaryWindow, WindowTheme},
};
use bevy_egui::EguiPlugin;
use render::{render_cubemap, render_grid};
use std::f32::consts::PI;

use crate::{
    asset_loader::{load_assets, AssetLoaderPlugin, SceneAssets},
    egui::editor_ui,
    hud::{main_menu_text, render_hud, render_main_menu, MainMenuText, RenderItem},
    input::{keyboard_input, lock_cursor, main_menu_input, mouse_input, MouseState},
    map::load_from_file,
    player::Player,
    render::{CustomMaterial, MAX_STRUCTURES},
    render::{render, render_map, CubeMapMaterial},
    wall::Wall,
};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
    InEditor,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum EditorState {
    World,
    Map,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MenuSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct EditorSet;

fn main() {
    App::new()
        .insert_state(GameState::MainMenu)
        .insert_state(EditorState::Map)
        .configure_sets(Update, MenuSet.run_if(in_state(GameState::MainMenu)))
        .configure_sets(Update, GameSet.run_if(in_state(GameState::InGame)))
        .configure_sets(Update, EditorSet.run_if(in_state(GameState::InEditor)))
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
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    visible: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),))
        .add_plugins(Material2dPlugin::<CustomMaterial>::default())
        .add_plugins(Material2dPlugin::<CubeMapMaterial>::default())
        .add_plugins(EguiPlugin)
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                make_visible,
                change_title,
                main_menu_input,
                render_main_menu,
                main_menu_text,
            )
                .in_set(MenuSet),
        )
        .add_systems(
            Update,
            (
                change_title,
                keyboard_input,
                mouse_input,
                render,
                render_map,
                render_hud,
                render_cubemap,
            )
                .in_set(GameSet),
        )
        .add_systems(Update, (editor_ui, render_grid).in_set(EditorSet))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut cubemaps: ResMut<Assets<CubeMapMaterial>>,
    mut normal_asset_server: Res<AssetServer>,
    mut asset_server: Res<SceneAssets>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
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
        &mut window_query,
    );

    commands.spawn(map);

    lock_cursor(&mut window_query);

    // Main menu
    commands.spawn((
        RenderItem::new_with_id(0),
        MaterialMesh2dBundle {
            mesh: meshes.add(RenderItem::new_mesh()).into(),
            material: materials.add(asset_server.hud[0].clone()),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..default()
        },
    ));

    // Text
    commands.spawn((
        MainMenuText::new_with_id(0, false),
        Text2dBundle {
            text: Text::from_section(
                "Play game",
                TextStyle {
                    font: normal_asset_server.load("fonts/DooM.ttf").clone(),
                    font_size: 80.0,
                    color: Color::RED,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        MainMenuText::new_with_id(0, true),
        Text2dBundle {
            text: Text::from_section(
                "Play game",
                TextStyle {
                    font: normal_asset_server.load("fonts/DooM.ttf").clone(),
                    font_size: 80.0,
                    color: Color::BLACK,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 100.0, -1.0),
            ..default()
        },
    ));

    commands.spawn((
        MainMenuText::new_with_id(1, false),
        Text2dBundle {
            text: Text::from_section(
                "Settings",
                TextStyle {
                    font: normal_asset_server.load("fonts/DooM.ttf").clone(),
                    font_size: 80.0,
                    color: Color::RED,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        MainMenuText::new_with_id(1, true),
        Text2dBundle {
            text: Text::from_section(
                "Settings",
                TextStyle {
                    font: normal_asset_server.load("fonts/DooM.ttf").clone(),
                    font_size: 80.0,
                    color: Color::BLACK,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
    ));

    commands.spawn((
        MainMenuText::new_with_id(2, false),
        Text2dBundle {
            text: Text::from_section(
                "Exit",
                TextStyle {
                    font: normal_asset_server.load("fonts/DooM.ttf").clone(),
                    font_size: 80.0,
                    color: Color::RED,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        MainMenuText::new_with_id(2, true),
        Text2dBundle {
            text: Text::from_section(
                "Exit",
                TextStyle {
                    font: normal_asset_server.load("fonts/DooM.ttf").clone(),
                    font_size: 80.0,
                    color: Color::BLACK,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -100.0, -1.0),
            ..default()
        },
    ));

    // HUD items
    commands.spawn((
        RenderItem::new_with_id(1),
        MaterialMesh2dBundle {
            mesh: meshes.add(RenderItem::new_mesh()).into(),
            material: materials.add(asset_server.hud[1].clone()),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
    ));

    //Cubemap
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(RenderItem::new_mesh()).into(),
        material: cubemaps.add(CubeMapMaterial {
            px: asset_server.cubemaps[0].clone(),
            nx: asset_server.cubemaps[1].clone(),
            py: asset_server.cubemaps[2].clone(),
            ny: asset_server.cubemaps[3].clone(),
            pz: asset_server.cubemaps[4].clone(),
            nz: asset_server.cubemaps[5].clone(),
            window_width: 0.,
            window_height: 0.,
            direction: Vec3::new(0., 0., -1.),
            horizontal_vector: Vec3::new(0., 0., 0.),
            vertical_vector: Vec3::new(0., 0., 0.),
            mask: [Vec3::new(0., 0., 0.); MAX_STRUCTURES],
            mask_len: 0,
        }),
        transform: Transform::from_xyz(0.0, 0.0, -10.0),
        ..default()
    },));
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
