use bevy::{
    prelude::*,
    sprite::Mesh2dHandle,
    render::mesh::{Indices, Mesh},
};

use crate::Player;
use crate::structures::Wall;
use crate::Triangle;

pub fn render(
    mut query: Query<&Player>,
    mut gizmos: Gizmos,
    _commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    _standard_materials: ResMut<Assets<StandardMaterial>>,
    mut wall_query: Query<(&mut Wall, &mut Transform, &mut Mesh2dHandle)>,
) {
    for player in query.iter_mut() {
        let mut z_ordering = 0.;

        for (wall, mut transform, mesh2dhandle) in wall_query.iter_mut() {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();

            let (start, end) = wall.clipping(player);

            // Debugging circles to see wall positions and player position in 2d (world coordinates)
            gizmos.circle_2d(Vec2::new(start.x, start.z), 1., Color::BLUE);
            gizmos.circle_2d(Vec2::new(end.x, end.z), 1., Color::GREEN);
            gizmos.circle_2d(Vec2::new(0., 0.), 1., Color::WHITE);

            if let Some(_positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                let (mut indice1, mut indice2, mut indice3) = (Vec2::ZERO, Vec2::ZERO, Vec2::ZERO);

                if wall.triangle == Triangle::Upper {
                    indice1 = world_to_screen_coordinates(start.x, start.y, start.z);
                    indice2 = world_to_screen_coordinates(start.x, start.y + wall.height, start.z);
                    indice3 = world_to_screen_coordinates(end.x, end.y + wall.height, end.z);
                } else if wall.triangle == Triangle::Lower {
                    indice1 = world_to_screen_coordinates(start.x, start.y, start.z);
                    indice2 = world_to_screen_coordinates(end.x, end.y + wall.height, end.z);
                    indice3 = world_to_screen_coordinates(end.x, end.y, end.z);
                }

                let new_positions = vec![
                    [-indice1.x, -indice1.y, 0.0],
                    [-indice2.x, -indice2.y, 0.0],
                    [-indice3.x, -indice3.y, 0.0],
                ];
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);
            }
            if let Some(_uvs) = mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
                if wall.triangle == Triangle::Upper {
                    mesh.insert_attribute(
                        Mesh::ATTRIBUTE_UV_0,
                        vec![[0.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
                    );
                } else if wall.triangle == Triangle::Lower {
                    mesh.insert_attribute(
                        Mesh::ATTRIBUTE_UV_0,
                        vec![[0.0, 1.0], [1.0, 0.0], [1.0, 1.0]],
                    );
                }
            }
            if let Some(_normals) = mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL) {
                let new_normals = vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]];

                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_normals);
            }
            if let Some(_indices) = mesh.indices() {
                let new_indices = Indices::U32(vec![
                    0, 2, 1, // why?
                ]);

                mesh.insert_indices(new_indices);
            }

            transform.translation.z = z_ordering;
            z_ordering += 1.;
        }
    }
}

// Converts world coordinates from above function to actual 3d screen coordinates
fn world_to_screen_coordinates(world_x: f32, world_y: f32, world_z: f32) -> Vec2 {
    let screen_x = world_x * 2000. / world_z;
    let screen_y = world_y * 2000. / world_z;

    Vec2::new(screen_x, screen_y)
}