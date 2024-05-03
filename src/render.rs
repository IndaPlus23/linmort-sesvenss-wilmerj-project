use bevy::{
    prelude::*,
    reflect::TypePath,
    render::mesh::Mesh,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Mesh2dHandle},
};
use crate::enemy::Enemy;

use crate::floor::Floor;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::wall::Wall;
use crate::{enemy, Player};
use crate::SceneAssets;

#[derive(Component, Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub a: Vec3,
    #[uniform(3)]
    pub b: Vec3,
    #[uniform(4)]
    pub c: Vec3,
    #[uniform(5)]
    pub a_uv: Vec2,
    #[uniform(6)]
    pub b_uv: Vec2,
    #[uniform(7)]
    pub c_uv: Vec2,
    #[uniform(8)]
    pub uv_scalar: Vec2,
    #[uniform(9)]
    pub uv_offset: Vec2,
    #[uniform(10)]
    pub uv_rotation: f32,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

pub fn render(
    mut commands: Commands,
    mut query: Query<&Player>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<SceneAssets>,
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
    mut enemy_query: Query<
        (
            &mut Enemy,
            &mut Transform,
            &mut Handle<Scene>,
        )
    >
) {
    for player in query.iter_mut() {
        let mut z_ordering = 0.;

        for (mut wall, mut transform, mesh2dhandle, material_handle) in wall_query.iter_mut() {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();

            let material_handle = material_handle.clone();
            let material = custom_materials.get_mut(material_handle).unwrap();

            let (a, b, c, _, screen_a, screen_b, screen_c, screen_d) = wall.transform(player);

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
            material.texture = asset_server.textures[wall.texture_id].clone();

            if let Some(_positions) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        [screen_a.x, screen_a.y, 0.0],
                        [screen_b.x, screen_b.y, 0.0],
                        [screen_c.x, screen_c.y, 0.0],
                        [screen_d.x, screen_d.y, 0.0],
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

        for (mut enemy, transform, sprite) in enemy_query.iter() {
            commands.spawn((
                MovingObjectBundle {
                    velocity: Velocity::new(Vec3::ZERO),
                    acceleration: Acceleration::new(Vec3::ZERO),
                    model: SceneBundle {
                        scene: enemy.texture.clone(),
                        transform: Transform::from_translation(enemy.transform(player)),
                        ..default()
                    },
                }, Enemy,
            ));
        }
    }
}
