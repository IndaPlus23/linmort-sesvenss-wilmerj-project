mod asset_loader;
mod collision_detection;
mod egui;
mod enemy;
mod floor;
mod hud;
mod input;
mod map;
mod movement;
mod player;
mod render;
mod skybox;
mod sound;
mod sprites;
mod timer;
mod utility;
mod animate;

mod vertex;
mod wall;

use bevy::{
    core::FrameCount,
    prelude::*,
    render::mesh::Mesh,
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    window::{PresentMode, PrimaryWindow, WindowTheme},
};
use bevy_egui::EguiPlugin;
use render::render_grid;
use std::f32::consts::PI;

use crate::collision_detection::CollisionDetectionPlugin;
use crate::enemy::EnemyPlugin;
use crate::movement::MovementPlugin;
use crate::{
    asset_loader::{load_assets, AssetLoaderPlugin, SceneAssets},
    egui::editor_ui,
    hud::{
        main_menu_text, render_game_hud, render_main_menu, game_screen_text, GameScreenText, MainMenuText, RenderItem,
    },
    input::{keyboard_input, lock_cursor, main_menu_input, mouse_input, MouseState},
    map::load_from_file,
    player::Player,
    render::{render, render_map},
    render::{CustomMaterial, MAX_STRUCTURES},
    skybox::{render_skybox, CubeMapMaterial},
    sound::play_background_audio,
    wall::Wall,
    enemy::enemy_position,
};
use crate::animate::AnimatePlugin;
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    InGame,
    EndGame,
    Dead,
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
        .add_plugins(MovementPlugin)
        .add_plugins(AnimatePlugin)
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
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, (setup, spawn_enemies))
        .add_systems(Update, change_title)
        .add_systems(
            Update,
            (
                full_health,
                make_visible,
                main_menu_input,
                render_main_menu,
                main_menu_text,
            )
                .in_set(MenuSet),
        )
        .add_systems(
            Update,
            (
                mouse_input,
                keyboard_input,
                render,
                render_game_hud,
                render_skybox,
                render_map,
                game_screen_text,
                enemy_position,
            )
                .in_set(GameSet),
        )
        .add_systems(
            Update,
            (
                mouse_input,
                keyboard_input,
                render,
                render_map,
                render_game_hud,
                render_skybox,
                editor_ui,
                render_grid,
                game_screen_text,
                enemy_position,
            )
                .in_set(EditorSet),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut cubemaps: ResMut<Assets<CubeMapMaterial>>,
    mut asset_server: Res<AssetServer>,
    mut scene_assets: Res<SceneAssets>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let map =
        load_from_file("map.txt", &scene_assets.enemy_types).expect("Error: could not open map");

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

    // Play main menu music
    play_background_audio(
        &mut asset_server,
        &mut commands,
        "sounds\\main_menu.ogg".to_string(),
    );

    // Main menu
    commands.spawn((
        RenderItem::new_main_menu_with_id(0),
        MaterialMesh2dBundle {
            mesh: meshes.add(RenderItem::new_mesh()).into(),
            material: materials.add(scene_assets.hud[0].clone()),
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
                    font: asset_server.load("fonts/DooM.ttf").clone(),
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
                    font: asset_server.load("fonts/DooM.ttf").clone(),
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
                    font: asset_server.load("fonts/DooM.ttf").clone(),
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
                    font: asset_server.load("fonts/DooM.ttf").clone(),
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
                    font: asset_server.load("fonts/DooM.ttf").clone(),
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
                    font: asset_server.load("fonts/DooM.ttf").clone(),
                    font_size: 80.0,
                    color: Color::BLACK,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -100.0, -1.0),
            ..default()
        },
    ));

    // Player health text
    commands.spawn((
        GameScreenText::new_with_id(0, false),
        Text2dBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/DooM.ttf").clone(),
                    font_size: 70.0,
                    color: Color::RED,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.spawn((
        GameScreenText::new_with_id(0, true),
        Text2dBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/DooM.ttf").clone(),
                    font_size: 70.0,
                    color: Color::BLACK,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    // Player ammo text
    commands.spawn((
        GameScreenText::new_with_id(1, false),
        Text2dBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/DooM.ttf").clone(),
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
        GameScreenText::new_with_id(1, true),
        Text2dBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/DooM.ttf").clone(),
                    font_size: 80.0,
                    color: Color::BLACK,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    // HUD items
    commands.spawn((
        RenderItem::new_with_id(1),
        MaterialMesh2dBundle {
            mesh: meshes.add(RenderItem::new_mesh()).into(),
            material: materials.add(scene_assets.hud[1].clone()),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
    ));

    //Cubemap
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(RenderItem::new_mesh()).into(),
        material: cubemaps.add(CubeMapMaterial {
            px: scene_assets.cubemaps[0].clone(),
            nx: scene_assets.cubemaps[1].clone(),
            py: scene_assets.cubemaps[2].clone(),
            ny: scene_assets.cubemaps[3].clone(),
            pz: scene_assets.cubemaps[4].clone(),
            nz: scene_assets.cubemaps[5].clone(),
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

fn full_health(mut player: Query<&mut Player>) {
    for mut player in player.iter_mut() {
        player.health = 100;
    }
}

use crate::collision_detection::Collider;
use crate::enemy::{ActionState, EnemyState};
use crate::movement::MovingObjectSpriteSheetBundle;
use crate::sprites::SpriteComponent;
use crate::timer::{AnimationTimer, ShootingTimer, WalkTimer};
use std::time::Duration;
use crate::movement::Velocity;
use crate::movement::Acceleration;
use crate::animate::{AnimationComponent, AnimationIndices};
use crate::enemy::Enemy;
use rand::Rng;
use std::io;

fn spawn_enemies(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    println!("Enter number of enemies: ");
    let mut input = String::new();

    // Read the input from the standard input (stdin)
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the input to remove any whitespace
    let input = input.trim();

    // Parse the input string to an i32
    let number = match input.parse::<i32>() {
        Ok(number) => number,
        Err(_) => 0,
    };

    for _ in 0..number {
        let mut enemy = Enemy::new(
            5,
            1,
            100,
            5,
            5,
            Some(0),
            2,
        );

        let mut rng = rand::thread_rng();
        // Generate x within the range -80 to 80
        let x: f32 = rng.gen_range(-80.0..80.0);
        // y is always 0
        let y: f32 = 0.0;
        // Generate z within the range 30 to 230
        let z: f32 = rng.gen_range(30.0..230.0);
        // Create the 3D vector
        let random_vector = (x, y, z);

        enemy.position = random_vector.into();

        commands.spawn((
            MovingObjectSpriteSheetBundle {
                velocity: Velocity::new(Vec3::ZERO),
                acceleration: Acceleration::new(Vec3::ZERO),
                sprite: SpriteSheetBundle {
                    texture: scene_assets.enemy_a_spritesheet.clone(),
                    atlas: TextureAtlas {
                        layout: scene_assets.enemy_a_spritelayout.clone(),
                        index: 0,
                    },
                    transform: Transform::from_translation(Vec3::new(100000.,100000.,100000.)),
                    ..default()
                },
            }, SpriteComponent {
                position: enemy.position,
                health: 100.,
            }, ShootingTimer {
                // create the non-repeating fuse timer
                timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
            }, EnemyState {
                state: ActionState::Dormant,
            }, WalkTimer {
                timer: Timer::new(Duration::from_secs(0), TimerMode::Once),
            }, Collider::new(5.),
            AnimationComponent {
                dormant: AnimationIndices{first: 0, last: 4},
                attack: AnimationIndices{first: 5, last: 9},
                dying: AnimationIndices{first: 10, last: 14},
                dead: AnimationIndices{first: 14, last: 14},
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    }
}