use bevy::{prelude::*, render::mesh::Mesh, sprite::Mesh2dHandle};

use crate::floor::Floor;
use crate::wall::Wall;
use crate::CustomMaterial;
use crate::Player;

pub fn render(
    mut query: Query<&Player>,
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

            let (a, b, c, screen_a, screen_b, screen_c) = wall.transform(player);

            // Gets sent to shader for correct uv mapping
            material.a = Vec3::new(screen_a.x, screen_a.y, -a.position.z);
            material.b = Vec3::new(screen_b.x, screen_b.y, -b.position.z);
            material.c = Vec3::new(screen_c.x, screen_c.y, -c.position.z);
            material.a_uv = a.uv;
            material.b_uv = b.uv;
            material.c_uv = c.uv;
            material.uv_scalar = wall.uv_scalar;
            material.uv_offset = wall.uv_offset;
            material.uv_rotation = wall.uv_rotation;
            material.texture = wall.texture.clone();

            if let Some(_positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        [screen_a.x, screen_a.y, 0.0],
                        [screen_b.x, screen_b.y, 0.0],
                        [screen_c.x, screen_c.y, 0.0],
                    ],
                );
            }

            transform.translation.z = z_ordering;
            z_ordering += 1.;
        }

        for (mut floor, _transform, mesh2dhandle, material_handle) in floor_query.iter_mut() {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();

            let material_handle = material_handle.clone();
            let material = custom_materials.get_mut(material_handle).unwrap();

            let (a, b, c, screen_a, screen_b, screen_c) = floor.transform(player);

            // Gets sent to shader for correct uv mapping
            material.a = Vec3::new(screen_a.x, screen_a.y, -a.position.z);
            material.b = Vec3::new(screen_b.x, screen_b.y, -b.position.z);
            material.c = Vec3::new(screen_c.x, screen_c.y, -c.position.z);
            material.a_uv = a.uv;
            material.b_uv = b.uv;
            material.c_uv = c.uv;

            if let Some(_positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        [screen_a.x, screen_a.y, 0.0],
                        [screen_b.x, screen_b.y, 0.0],
                        [screen_c.x, screen_c.y, 0.0],
                    ],
                );
            }
        }
    }
}
