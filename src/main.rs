use bevy::{
    //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::mesh::Mesh,
    window::{PresentMode, WindowTheme},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Mesh2dHandle, Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use std::f32::consts::PI;

mod input;
use crate::input::{MouseState, mouse_input, keyboard_input};
mod player;
use crate::player::Player;
mod render;
use crate::render::render;
mod structures;
use crate::structures::Wall;
use crate::structures::Triangle;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
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
            DefaultPlugins.set(WindowPlugin {
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
            }),
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
    mut materials: ResMut<Assets<ColorMaterial>>,
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
        &mut materials,
        &mut asset_server,
        Vec3::new(-5., -5., -40.),
        Vec3::new(100., -5., -40.),
        10.,
    );

    let mut mesh: Mesh = Rectangle::default().into();
    if let Some(_positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        let positions = vec![[0.5, 0.3, 0.0], [-0.5, 1.5, 0.0], [-0.5, -1.5, 0.0], [0.5, -0.3, 0.0]];
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    }

    let mesh_handle = meshes.add(mesh);
    let mesh_2d_handle = Mesh2dHandle::from(mesh_handle);

    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_2d_handle,
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: custom_materials.add(CustomMaterial {
            texture: asset_server.load("grass_front.png"),
        }),
        ..default()
    });
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