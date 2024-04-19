use bevy::{prelude::*, render::mesh::Mesh, sprite::Mesh2dHandle};

use crate::floor::Floor;
use crate::wall::Wall;
use crate::CustomMaterial;
use crate::Player;

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

        for (mut wall, mut transform, mesh2dhandle, material_handle) in wall_query.iter_mut() {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();

            let material_handle = material_handle.clone();
            let material = custom_materials.get_mut(material_handle).unwrap();

            let (a, b, c, screen_a, screen_b, screen_c) = wall.clipping(player);

            // Debugging circles to see wall positions and player position in 2d (world coordinates)
            gizmos.circle_2d(Vec2::new(a.position.x, -a.position.z), 1., Color::BLUE);
            gizmos.circle_2d(Vec2::new(c.position.x, -c.position.z), 1., Color::GREEN);
            gizmos.circle_2d(Vec2::new(0., 0.), 1., Color::WHITE);

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