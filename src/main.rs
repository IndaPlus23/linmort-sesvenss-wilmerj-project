use bevy::{
    //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    reflect::TypePath,
    render::mesh::Mesh,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
    window::{PresentMode, WindowTheme},
};
use std::f32::consts::PI;

mod input;
use crate::input::{keyboard_input, mouse_input, MouseState};
mod player;
use crate::player::Player;
mod render;
use crate::render::render;
mod structures;
use crate::structures::Kind;
use crate::structures::Triangle;
use crate::structures::Wall;
mod floor;
use crate::floor::Floor;

#[derive(Component, Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
    #[uniform(2)]
    a: Vec3,
    #[uniform(3)]
    b: Vec3,
    #[uniform(4)]
    c: Vec3,
    #[uniform(5)]
    a_uv: Vec2,
    #[uniform(6)]
    b_uv: Vec2,
    #[uniform(7)]
    c_uv: Vec2,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(MouseState {
            press_coords: Vec::new(),
        })
        .add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
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
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, mouse_input)
        .add_systems(Update, render)
        .add_systems(Update, change_title)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    //mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, -1.0), Vec3::Y),
        ..Default::default()
    });

    commands.spawn((Player::new(0., 0., 0., 0., 0.),));

    Wall::spawn_wall(
        &mut commands,
        &mut meshes,
        &mut custom_materials,
        &mut asset_server,
        Vec3::new(0., -5., -50.),
        Vec3::new(50., -5., -50.),
        10.,
        Kind::Wall,
    );

    Floor::spawn(
        &mut commands,
        &mut meshes,
        &mut custom_materials,
        &mut asset_server,
        Vec3::new(-20., -5., -50.),
        Vec3::new(-20., -5., -100.),
        Vec3::new(50., -5., -50.),
        Vec2::new(0., 1.),
        Vec2::new(0., 0.,),
        Vec2::new(1., 1.),
    );
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
