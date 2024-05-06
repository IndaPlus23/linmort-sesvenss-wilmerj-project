use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::Mesh,
        render_resource::{AsBindGroup, ShaderRef},
        texture::Image,
    },
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};
use core::f32::consts::PI;
use nalgebra::{Rotation3, Unit, Vector3};

use crate::{floor::Floor, wall::Wall, EditorState, GameState, Player, SceneAssets, vertex::Vertex, enemy::Enemy};
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};

pub const MAX_STRUCTURES: usize = 1000;

#[derive(Component, Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub id: f32,
    #[uniform(3)]
    pub mask: [Vec3; MAX_STRUCTURES],
    #[uniform(4)]
    pub mask_len: i32,
    #[uniform(5)]
    pub a_screen: Vec3,
    #[uniform(6)]
    pub b_screen: Vec3,
    #[uniform(7)]
    pub c_screen: Vec3,
    #[uniform(8)]
    pub a_uv: Vec2,
    #[uniform(9)]
    pub b_uv: Vec2,
    #[uniform(10)]
    pub c_uv: Vec2,
    #[uniform(11)]
    pub uv_scalar: Vec2,
    #[uniform(12)]
    pub uv_offset: Vec2,
    #[uniform(13)]
    pub uv_rotation: f32,
    #[uniform(14)]
    pub a_position: Vec3,
    #[uniform(15)]
    pub b_position: Vec3,
    #[uniform(16)]
    pub c_position: Vec3,
    #[uniform(17)]
    pub pitch: f32,
}

#[derive(Component)]
struct EnemyComponent;

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

pub fn render(
    mut commands: Commands,
    mut player_query: Query<&Player>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    scene_assets: Res<SceneAssets>,
    mut wall_query: Query<(
        &mut Wall,
        &mut Transform,
        &mut Mesh2dHandle,
        &mut Handle<CustomMaterial>,
    )>,
    mut floor_query: Query<
        (
            &mut Floor,
            &mut Transform,
            &mut Mesh2dHandle,
            &mut Handle<CustomMaterial>,
        ),
        Without<Wall>,
    >,
    mut enemy_query: Query<(&Enemy, &mut Transform), (Without<Wall>, Without<Floor>)>,
) {
    for player in player_query.iter_mut() {
        // Calculate mask for z-buffering
        let mut mask: [Vec3; 1000] = [Vec3::new(0., 0., 0.); 1000];
        let mut i = 0;

        // Very ugly but gets the job done for now
        for (wall, _, _, _) in wall_query.iter_mut() {
            let (a, b, c, d) = wall.mask(player);
            mask[i] = Vec3::new(wall.id as f32, wall.id as f32, wall.id as f32);
            mask[i + 1] = a.position;
            mask[i + 2] = Vec3::new(a.screen()[0], a.screen()[1], -a.position.z);
            mask[i + 3] = b.position;
            mask[i + 4] = Vec3::new(b.screen()[0], b.screen()[1], -b.position.z);
            mask[i + 5] = c.position;
            mask[i + 6] = Vec3::new(c.screen()[0], c.screen()[1], -c.position.z);
            i += 7;
            mask[i] = Vec3::new(wall.id as f32, wall.id as f32, wall.id as f32);
            mask[i + 1] = a.position;
            mask[i + 2] = Vec3::new(a.screen()[0], a.screen()[1], -a.position.z);
            mask[i + 3] = c.position;
            mask[i + 4] = Vec3::new(c.screen()[0], c.screen()[1], -c.position.z);
            mask[i + 5] = d.position;
            mask[i + 6] = Vec3::new(d.screen()[0], d.screen()[1], -d.position.z);
            i += 7;
        }

        for (floor, _, _, _) in floor_query.iter_mut() {
            let (a, b, c, d, e, f) = floor.mask(player);
            mask[i] = Vec3::new(floor.id as f32, floor.id as f32, floor.id as f32);
            mask[i + 1] = a.position;
            mask[i + 2] = Vec3::new(a.screen()[0], a.screen()[1], -a.position.z);
            mask[i + 3] = b.position;
            mask[i + 4] = Vec3::new(b.screen()[0], b.screen()[1], -b.position.z);
            mask[i + 5] = c.position;
            mask[i + 6] = Vec3::new(c.screen()[0], c.screen()[1], -c.position.z);
            i += 7;
            mask[i] = Vec3::new(floor.id as f32, floor.id as f32, floor.id as f32);
            mask[i + 1] = d.position;
            mask[i + 2] = Vec3::new(d.screen()[0], d.screen()[1], -d.position.z);
            mask[i + 3] = e.position;
            mask[i + 4] = Vec3::new(e.screen()[0], e.screen()[1], -e.position.z);
            mask[i + 5] = f.position;
            mask[i + 6] = Vec3::new(f.screen()[0], f.screen()[1], -f.position.z);
            i += 7;
        }

        // Render walls with calculated mask
        for (wall, _, mesh2dhandle, material_handle) in wall_query.iter_mut() {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();

            let material_handle = material_handle.clone();
            let material = custom_materials.get_mut(material_handle).unwrap();

            let (a, b, c, _, a_screen, b_screen, c_screen, d_screen) = wall.transform(player);

            // Gets sent to shader for correct uv mapping
            material.a_screen = Vec3::new(a_screen.x, a_screen.y, -a.position.z);
            material.b_screen = Vec3::new(b_screen.x, b_screen.y, -b.position.z);
            material.c_screen = Vec3::new(c_screen.x, c_screen.y, -c.position.z);
            material.a_uv = a.uv;
            material.b_uv = b.uv;
            material.c_uv = c.uv;
            material.uv_scalar = wall.uv_scalar;
            material.uv_offset = wall.uv_offset;
            material.uv_rotation = wall.uv_rotation;
            material.texture = scene_assets.textures[wall.texture_id].clone();
            material.a_position = a.position;
            material.b_position = b.position;
            material.c_position = c.position;
            material.pitch = player.pitch;

            material.id = wall.id as f32;
            material.mask = mask;
            material.mask_len = i as i32;

            if let Some(_positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        [a_screen.x, a_screen.y, 0.0],
                        [b_screen.x, b_screen.y, 0.0],
                        [c_screen.x, c_screen.y, 0.0],
                        [d_screen.x, d_screen.y, 0.0],
                    ],
                );
            }
        }

        // Render floors with calculated mask
        for (floor, _, mesh2dhandle, material_handle) in floor_query.iter_mut() {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();

            let material_handle = material_handle.clone();
            let material = custom_materials.get_mut(material_handle).unwrap();

            let (mut a, mut b, mut c, _, a_screen, b_screen, c_screen, d_screen) =
                floor.transform(player);

            // Gets sent to shader for correct uv mapping
            material.a_screen = Vec3::new(a_screen.x, a_screen.y, -a.position.z);
            material.b_screen = Vec3::new(b_screen.x, b_screen.y, -b.position.z);
            material.c_screen = Vec3::new(c_screen.x, c_screen.y, -c.position.z);

            // World aligned uv
            if floor.world_aligned_uv {
                a.uv = Vec2::new(
                    a.reverse_transform_vertice(player).position.x / 10.,
                    a.reverse_transform_vertice(player).position.z / 10.,
                );
                b.uv = Vec2::new(
                    b.reverse_transform_vertice(player).position.x / 10.,
                    b.reverse_transform_vertice(player).position.z / 10.,
                );
                c.uv = Vec2::new(
                    c.reverse_transform_vertice(player).position.x / 10.,
                    c.reverse_transform_vertice(player).position.z / 10.,
                );
            }

            // Gets sent to shader
            material.a_uv = a.uv;
            material.b_uv = b.uv;
            material.c_uv = c.uv;
            material.uv_scalar = floor.uv_scalar;
            material.uv_offset = floor.uv_offset;
            material.uv_rotation = floor.uv_rotation;
            material.texture = scene_assets.textures[floor.texture_id].clone();
            material.a_position = a.position;
            material.b_position = b.position;
            material.c_position = c.position;
            material.pitch = player.pitch;

            material.id = floor.id as f32;
            material.mask = mask;
            material.mask_len = i as i32;

            if let Some(_positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        [a_screen.x, a_screen.y, 0.0],
                        [b_screen.x, b_screen.y, 0.0],
                        [c_screen.x, c_screen.y, 0.0],
                        [d_screen.x, d_screen.y, 0.0],
                    ],
                );
            }
        }

        // Enemies
        for (enemy, mut transform) in enemy_query.iter_mut() {
            commands.spawn((
                MovingObjectBundle {
                    velocity: Velocity::new(Vec3::ZERO),
                    acceleration: Acceleration::new(Vec3::ZERO),
                    sprite: SpriteBundle {
                        texture: scene_assets.enemy.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            1.0,
                            1.0,
                            0.)),
                        ..default()
                    },
                }, EnemyComponent,
            ));
        }
    }
}

#[derive(Component, Clone)]
pub struct MapFloor {
    pub id: usize,
    pub scale: f32,
    pub x_offset: f32,
    pub y_offset: f32,
}

impl MapFloor {
    fn new_with_id(id: usize) -> Self {
        let scale = 1.0;
        let x_offset = 0.0;
        let y_offset = 0.0;
        Self {
            id,
            scale,
            x_offset,
            y_offset,
        }
    }
}

pub fn render_grid(
    mut gizmos: Gizmos,
    mut player_query: Query<&Player>,
) {
    for player in player_query.iter_mut() {
        let mut previous = 0.0;
        for x in -1000..1000 {
            let nearest = round_to_nearest(x as f32, 10.0);
            if nearest != previous {
                let position = Vertex::new(Vec3::new(nearest, 0.0, 0.0), Vec2::ZERO).transform_vertice(player);
                let color = Color::Rgba {
                    red: 0.7,
                    green: 0.7,
                    blue: 0.7,
                    alpha: scale_alpha(nearest.abs(), 1000.0),
                };
                if position.position.z < 0.0 {
                    gizmos.circle_2d(position.screen(), scale_alpha(nearest.abs(), 1000.0), color);
                }
                previous = nearest;
            }
        }

        for y in -1000..1000 {
            let nearest = round_to_nearest(y as f32, 10.0);
            if nearest != previous {
                let position = Vertex::new(Vec3::new(0.0, nearest, 0.0), Vec2::ZERO).transform_vertice(player);
                let color = Color::Rgba {
                    red: 0.7,
                    green: 0.7,
                    blue: 0.7,
                    alpha: scale_alpha(nearest.abs(), 1000.0),
                };
                if position.position.z < 0.0 {
                    gizmos.circle_2d(position.screen(), scale_alpha(nearest.abs(), 1000.0), color);
                }
                previous = nearest;
            }
        }

        for z in -1000..1000 {
            let nearest = round_to_nearest(z as f32, 10.0);
            if nearest != previous {
                let position = Vertex::new(Vec3::new(0.0, 0.0, nearest), Vec2::ZERO).transform_vertice(player);
                let color = Color::Rgba {
                    red: 0.7,
                    green: 0.7,
                    blue: 0.7,
                    alpha: scale_alpha(nearest.abs(), 1000.0),
                };
                if position.position.z < 0.0 {
                    gizmos.circle_2d(position.screen(), scale_alpha(nearest.abs(), 1000.0), color);
                }
                previous = nearest;
            }
        }
    }
}

fn round_to_nearest(num: f32, factor: f32) -> f32 {
    (num / factor).round() * factor
}

fn scale_alpha(num: f32, factor: f32) -> f32 {
    let k = -2.5 / factor;
    return k * num + 1.0
}

pub fn render_map(
    mut commands: Commands,
    mut gizmos: Gizmos,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_floors: Query<(&MapFloor, &mut Mesh2dHandle)>,
    mut player_query: Query<&Player>,
    mut wall_query: Query<&mut Wall>,
    mut floor_query: Query<&mut Floor>,
    editor_state: ResMut<State<EditorState>>,
    game_state: ResMut<State<GameState>>,
) {
    for player in player_query.iter_mut() {
        // If current state does not need map to be rendered, dont render map.
        if *editor_state.get() == EditorState::World || *game_state.get() == GameState::InGame {
            for (_, mesh2dhandle) in map_floors.iter_mut() {
                let mesh_handle = &mesh2dhandle.0;
                let mesh = meshes.get_mut(mesh_handle).unwrap();
                if let Some(_) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                    mesh.insert_attribute(
                        Mesh::ATTRIBUTE_POSITION,
                        vec![[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                    );
                }
            }

            break;
        }

        // Spawn new floors to map_floors query
        for floor in floor_query.iter_mut() {
            let mut found_id = false;

            for (map_floor, _) in map_floors.iter_mut() {
                if map_floor.id == floor.id {
                    found_id = true;
                    break;
                }
            }

            if found_id == true {
                continue;
            } else {
                let mesh = Mesh2dHandle(meshes.add(Triangle2d::new(
                    Vec2::new(0.0, 0.0),
                    Vec2::new(0.0, 0.0),
                    Vec2::new(0.0, 0.0),
                )));
                commands.spawn((
                    MapFloor::new_with_id(floor.id),
                    MaterialMesh2dBundle {
                        mesh: mesh,
                        material: materials.add(Color::BLUE),
                        transform: Transform::from_xyz(0.0, 0.0, 10.0),
                        ..default()
                    },
                ));
            }
        }

        let mut scale = 1.0;
        let mut x_offset = 0.0;
        let mut y_offset = 0.0;

        // Render floors
        for floor in floor_query.iter_mut() {
            for (map_floor, mesh2dhandle) in map_floors.iter_mut() {
                scale = map_floor.scale;
                x_offset = map_floor.x_offset;
                y_offset = map_floor.y_offset;
                if map_floor.id == floor.id {
                    let mesh_handle = &mesh2dhandle.0;
                    let mesh = meshes.get_mut(mesh_handle).unwrap();
                    if let Some(_) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                        mesh.insert_attribute(
                            Mesh::ATTRIBUTE_POSITION,
                            vec![
                                [(floor.a.position.x + x_offset) * scale, -(floor.a.position.z + y_offset) * scale, 0.0],
                                [(floor.b.position.x + x_offset) * scale, -(floor.b.position.z + y_offset) * scale, 0.0],
                                [(floor.c.position.x + x_offset) * scale, -(floor.c.position.z + y_offset) * scale, 0.0],
                            ],
                        );
                    }
                }
            }
        }

        // Render player
        gizmos.circle_2d(Vec2::new((player.x + x_offset)* scale, -(player.z + y_offset)* scale), 2., Color::WHITE);

        for wall in wall_query.iter_mut() {
            gizmos.line_2d(
                Vec2::new((wall.start.position.x + x_offset) * scale, -(wall.start.position.z + y_offset) * scale),
                Vec2::new((wall.end.position.x + x_offset) * scale, -(wall.end.position.z + y_offset) * scale),
                Color::RED,
            );
        }
    }
}

fn forward_vector(yaw: f32, pitch: f32) -> Vec3 {
    let mut vector = Vector3::new(0., 0., -1.);

    //create rotation matrices from yaw and pitch
    let yaw_rotation = Rotation3::from_euler_angles(0., 2. * PI - yaw, 0.);
    vector = yaw_rotation * vector;

    let axis = Unit::new_normalize(vector.cross(&Vector3::y()));
    let pitch_rotation = Rotation3::from_axis_angle(&axis, pitch);

    vector = pitch_rotation * vector;

    Vec3::new(vector.x, vector.y, vector.z)
}
