use bevy::{prelude::*, render::mesh::Mesh, sprite::Mesh2dHandle};

use crate::floor::Floor;
use crate::wall::clipping_vertice;
use crate::wall::Wall;
use crate::CustomMaterial;
use crate::Player;
use crate::Triangle;

pub fn render(
    mut query: Query<&Player>,
    mut gizmos: Gizmos,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
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
) {
    for player in query.iter_mut() {
        let mut z_ordering = 0.;

        for (wall, mut transform, mesh2dhandle, material_handle) in wall_query.iter_mut() {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();

            let material_handle = material_handle.clone();
            let material = custom_materials.get_mut(material_handle).unwrap();

            let (start, end) = wall.clipping(player);

            // Debugging circles to see wall positions and player position in 2d (world coordinates)
            //gizmos.circle_2d(Vec2::new(start.x, -start.z), 1., Color::BLUE);
            //gizmos.circle_2d(Vec2::new(end.x, -end.z), 1., Color::GREEN);
            //gizmos.circle_2d(Vec2::new(0., 0.), 1., Color::WHITE);

            if let Some(_positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                let (mut indice1, mut indice2, mut indice3) = (Vec2::ZERO, Vec2::ZERO, Vec2::ZERO);

                if wall.triangle == Triangle::Upper {
                    indice1 = world_to_screen_coordinates(start.x, start.y, start.z);
                    indice2 =
                        world_to_screen_coordinates(start.x, start.y + wall.height, start.z);
                    indice3 = world_to_screen_coordinates(end.x, end.y + wall.height, end.z);

                    material.a = Vec3::new(-indice1.x, -indice1.y, -start.z);
                    material.b = Vec3::new(-indice2.x, -indice2.y, -start.z);
                    material.c = Vec3::new(-indice3.x, -indice3.y, -end.z);
                } else if wall.triangle == Triangle::Lower {
                    indice1 = world_to_screen_coordinates(start.x, start.y, start.z);
                    indice2 = world_to_screen_coordinates(end.x, end.y + wall.height, end.z);
                    indice3 = world_to_screen_coordinates(end.x, end.y, end.z);

                    material.a = Vec3::new(-indice1.x, -indice1.y, -start.z);
                    material.b = Vec3::new(-indice2.x, -indice2.y, -end.z);
                    material.c = Vec3::new(-indice3.x, -indice3.y, -end.z);
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
                        vec![[0.0, 1.0], [0.0, 0.0], [1.0, 0.0]], //0 1, 0 0, 1 0
                    );

                    material.a_uv = Vec2::new(0.0, 1.0);
                    material.b_uv = Vec2::new(0.0, 0.0);
                    material.c_uv = Vec2::new(1.0, 0.0);
                } else if wall.triangle == Triangle::Lower {
                    mesh.insert_attribute(
                        Mesh::ATTRIBUTE_UV_0,
                        vec![[0.0, 1.0], [1.0, 0.0], [1.0, 1.0]], //0 1, 1 0, 1 1
                    );

                    material.a_uv = Vec2::new(0.0, 1.0);
                    material.b_uv = Vec2::new(1.0, 0.0);
                    material.c_uv = Vec2::new(1.0, 1.0);
                }
            }

            transform.translation.z = z_ordering;
            z_ordering += 1.;
        }

        for (mut floor, _transform, mesh2dhandle, material_handle) in floor_query.iter_mut() {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();

            let material_handle = material_handle.clone();
            let material = custom_materials.get_mut(material_handle).unwrap();

            let (a, b, c, screen_a, screen_b, screen_c) = floor.clipping(player);

            if let Some(_positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        [screen_a.x, screen_a.y, 0.0],
                        [screen_b.x, screen_b.y, 0.0],
                        [screen_c.x, screen_c.y, 0.0],
                    ],
                );

                // Gets sent to shader for correct uv mapping
                material.a = Vec3::new(screen_a.x, screen_a.y, -a.position.z);
                material.b = Vec3::new(screen_b.x, screen_b.y, -b.position.z);
                material.c = Vec3::new(screen_c.x, screen_c.y, -c.position.z);
                material.a_uv = a.uv;
                material.b_uv = b.uv;
                material.c_uv = c.uv;
            }
        }
    }
}

// Converts world 3d coordinates to 2d screen coordinates
fn world_to_screen_coordinates(world_x: f32, world_y: f32, world_z: f32) -> Vec2 {
    let screen_x = world_x * 1500. / world_z;
    let screen_y = world_y * 1500. / world_z;

    Vec2::new(screen_x, screen_y)
}
