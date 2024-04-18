use bevy::{prelude::*, render::mesh::Mesh, sprite::MaterialMesh2dBundle};
use std::f32::consts::PI;

use crate::CustomMaterial;
use crate::Player;

#[derive(Clone, PartialEq)]
pub enum Triangle {
    Upper,
    Lower,
}

#[derive(Clone, PartialEq, Copy)]
pub enum Kind {
    Wall,
    Floor,
}

#[derive(Component, Clone)]
pub struct Wall {
    pub start: Vec3,
    pub end: Vec3,
    pub height: f32,
    pub kind: Kind,
    pub triangle: Triangle,
}

impl Wall {
    pub fn new(start: Vec3, end: Vec3, height: f32, kind: Kind, triangle: Triangle) -> Self {
        Self {
            start,
            end,
            height,
            kind,
            triangle,
        }
    }

    pub fn spawn_wall(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        custom_materials: &mut ResMut<Assets<CustomMaterial>>,
        asset_server: &mut Res<AssetServer>,
        start: Vec3,
        end: Vec3,
        height: f32,
        kind: Kind,
    ) {
        commands.spawn((
            Wall::new(start, end, height, kind, Triangle::Upper),
            MaterialMesh2dBundle {
                mesh: meshes.add(Triangle2d::default()).into(),
                material: custom_materials.add(CustomMaterial {
                    texture: asset_server.load("grass_front.png"),
                    a: Vec3::new(0., 0., 0.),
                    b: Vec3::new(0., 0., 0.),
                    c: Vec3::new(0., 0., 0.),
                    a_uv: Vec2::new(0., 0.),
                    b_uv: Vec2::new(0., 0.),
                    c_uv: Vec2::new(0., 0.),
                }),
                ..Default::default()
            },
        ));

        commands.spawn((
            Wall::new(start, end, height, kind.clone(), Triangle::Lower),
            MaterialMesh2dBundle {
                mesh: meshes.add(Triangle2d::default()).into(),
                material: custom_materials.add(CustomMaterial {
                    texture: asset_server.load("grass_front.png"),
                    a: Vec3::new(0., 0., 0.),
                    b: Vec3::new(0., 0., 0.),
                    c: Vec3::new(0., 0., 0.),
                    a_uv: Vec2::new(0., 0.),
                    b_uv: Vec2::new(0., 0.),
                    c_uv: Vec2::new(0., 0.),
                }),
                ..Default::default()
            },
        ));
    }

    pub fn clipping(&self, player: &Player) -> (Vec3, Vec3) {
        let world_start =
            vertices_to_world_coordinates(player, self.start.x, self.start.y, self.start.z);

        let world_end = vertices_to_world_coordinates(player, self.end.x, self.end.y, self.end.z);

        if world_start.z > 0. && world_end.z > 0. {
            // Both wall's starting and end points are behind the player
            // The wall does not have to be rendered
            return (Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
        } else if world_start.z > 0. {
            // Wall starting point is behind the player
            // Clip the starting point so it never is behind the player
            clip_line_segment(world_start, world_end)
        } else if world_end.z > 0. {
            // Wall end point is behind the player
            // Clip the end point so it never is behind the player
            clip_line_segment(world_end, world_start)
        } else {
            // No point is behind the player
            return (world_start, world_end);
        }
    }
}

// Converts vertices to world coordinates based on player rotation and position.
fn vertices_to_world_coordinates(player: &Player, mut x: f32, mut y: f32, mut z: f32) -> Vec3 {
    let cos = player.yaw.cos();
    let sin = player.yaw.sin();

    x -= player.x;
    y -= player.y;
    z -= player.z;

    let world_x = x * cos + z * sin;
    let world_z = z * cos - x * sin;
    let world_y = y + (player.pitch * world_z);

    Vec3::new(world_x, world_y, world_z)
}

pub fn clipping_vertice(player: &Player, x: f32, y: f32, z: f32, reference_z: f32) -> Vec3 {
    let world = vertices_to_world_coordinates(player, x, y, z);

    if world.z > 0. && reference_z > 0. {
        return Vec3::new(0., 0., 0.);
    } else if world.z > 0. {
        return Vec3::new(world.x, world.y, -1.);
    } else {
        return world;
    }
}

fn clip_line_segment(start: Vec3, end: Vec3) -> (Vec3, Vec3) {
    let delta_z = end.z - start.z;
    let delta_x = end.x - start.x;
    let delta_y = end.y - start.y;

    let k = delta_z / delta_x;
    let m = start.z - (k * start.x);
    let new_start_x = -m / k;

    let k = delta_z / delta_y;
    let m = start.z - (k * start.y);
    let new_start_y = -m / k;

    let new_start = Vec3::new(new_start_x, new_start_y, -0.01);
    return (new_start, end);
}