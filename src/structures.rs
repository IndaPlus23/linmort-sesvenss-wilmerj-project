use bevy::{
    prelude::*,
    render::mesh::Mesh,
    sprite::MaterialMesh2dBundle,
};

use crate::Player;

#[derive(Clone, PartialEq)]
pub enum Triangle {
    Upper,
    Lower,
}

#[derive(Component, Clone)]
pub struct Wall {
    pub start: Vec3,
    pub end: Vec3,
    pub height: f32,
    pub triangle: Triangle,
}

impl Wall {
    pub fn new(start: Vec3, end: Vec3, height: f32, triangle: Triangle) -> Self {
        Self {
            start,
            end,
            height,
            triangle,
        }
    }

    pub fn spawn_wall(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        start: Vec3,
        end: Vec3,
        height: f32,
    ) {
        commands.spawn((
            Wall::new(
                Vec3::new(start.x, start.y, start.z),
                Vec3::new(end.x, end.y, end.z),
                height,
                Triangle::Upper,
            ),
            MaterialMesh2dBundle {
                mesh: meshes.add(Triangle2d::default()).into(),
                material: materials.add(Color::YELLOW),
                ..Default::default()
            },
        ));

        commands.spawn((
            Wall::new(
                Vec3::new(start.x, start.y, start.z),
                Vec3::new(end.x, end.y, end.z),
                height,
                Triangle::Lower,
            ),
            MaterialMesh2dBundle {
                mesh: meshes.add(Triangle2d::default()).into(),
                material: materials.add(Color::RED),
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
            let delta_z = world_end.z - world_start.z;
            let delta_x = world_end.x - world_start.x;
            let k = delta_z / delta_x;
            let m = world_start.z - (k * world_start.x);
            let new_start_x = -m / k;
            let new_world_start = Vec3::new(new_start_x, world_start.y, -1.);
            return (new_world_start, world_end);
        } else if world_end.z > 0. {
            // Wall end point is behind the player
            // Clip the end point so it never is behind the player
            let delta_z = world_start.z - world_end.z;
            let delta_x = world_start.x - world_end.x;
            let k = delta_z / delta_x;
            let m = world_end.z - (k * world_end.x);
            let new_end_x = -m / k;
            let new_world_end = Vec3::new(new_end_x, world_end.y, -1.);
            return (world_start, new_world_end);
        } else {
            // No point is behind the player
            return (world_start, world_end);
        }
    }
}

// Converts vertices to world coordinates based on player rotation and position.
// Following the exact method showed in the doom engine video
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