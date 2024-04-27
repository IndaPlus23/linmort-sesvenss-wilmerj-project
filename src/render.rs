use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::Mesh,
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDimension, TextureFormat},
        texture::Image,
    },
    sprite::{Material2d, Mesh2dHandle},
};

use crate::{floor::Floor, wall::Wall, Player, SceneAssets};

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

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

pub fn render(
    mut gizmos: Gizmos,
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
) {
    for player in query.iter_mut() {
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
            material.texture = asset_server.textures[wall.texture_id].clone();
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

            let (mut a, mut b, mut c, d, a_screen, b_screen, c_screen, d_screen) =
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
            material.texture = asset_server.textures[floor.texture_id].clone();
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

            gizmos.circle_2d(Vec2::new(0., 0.), 1., Color::WHITE);
            gizmos.line_2d(
                Vec2::new(a.position.x, -a.position.z),
                Vec2::new(b.position.x, -b.position.z),
                Color::BLUE,
            );
            gizmos.line_2d(
                Vec2::new(b.position.x, -b.position.z),
                Vec2::new(c.position.x, -c.position.z),
                Color::BLUE,
            );
            gizmos.line_2d(
                Vec2::new(a.position.x, -a.position.z),
                Vec2::new(c.position.x, -c.position.z),
                Color::BLUE,
            );
            gizmos.line_2d(
                Vec2::new(a.position.x, -a.position.z),
                Vec2::new(d.position.x, -d.position.z),
                Color::BLUE,
            );
        }
    }
}
