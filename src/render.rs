use bevy::{
    prelude::*,
    reflect::TypePath,
    render::mesh::Mesh,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Mesh2dHandle},
};

use bevy::render::texture::Image;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDimension;
use bevy::render::render_resource::TextureFormat;
use bevy::render::render_asset::RenderAssetUsages;

use crate::floor::Floor;
use crate::wall::Wall;
use crate::Player;
use crate::SceneAssets;

#[derive(Component, Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub mask: Handle<Image>,
    #[uniform(4)]
    pub window_size: Vec2,
    #[uniform(5)]
    pub a: Vec3,
    #[uniform(6)]
    pub b: Vec3,
    #[uniform(7)]
    pub c: Vec3,
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
}

pub fn new_mask(width: f32, height: f32) -> Image {
    Image::new_fill(
        Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1
            }, 
            TextureDimension::D2, 
            &[255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],             
            TextureFormat::Rgba32Float,
            RenderAssetUsages::RENDER_WORLD,
        )
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

pub fn render(
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
        for (mut wall, _, mesh2dhandle, material_handle) in wall_query.iter_mut() {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();

            let material_handle = material_handle.clone();
            let material = custom_materials.get_mut(material_handle).unwrap();

            let (a, b, c, _, a_screen, b_screen, c_screen, d_screen) = wall.transform(player);

            // Gets sent to shader for correct uv mapping
            material.a = Vec3::new(a_screen.x, a_screen.y, -a.position.z);
            material.b = Vec3::new(b_screen.x, b_screen.y, -b.position.z);
            material.c = Vec3::new(c_screen.x, c_screen.y, -c.position.z);
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
                        [a_screen.x, a_screen.y, 0.0],
                        [b_screen.x, b_screen.y, 0.0],
                        [c_screen.x, c_screen.y, 0.0],
                        [d_screen.x, d_screen.y, 0.0],
                    ],
                );
            }
        }

        for (mut floor, _, mesh2dhandle, material_handle) in floor_query.iter_mut() {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();

            let material_handle = material_handle.clone();
            let material = custom_materials.get_mut(material_handle).unwrap();

            let (mut a, mut b, mut c, _, a_screen, b_screen, c_screen, d_screen) = floor.transform(player);

            // Gets sent to shader for correct uv mapping
            material.a = Vec3::new(a_screen.x, a_screen.y, -a.position.z);
            material.b = Vec3::new(b_screen.x, b_screen.y, -b.position.z);
            material.c = Vec3::new(c_screen.x, c_screen.y, -c.position.z);
 
            // World aligned uv
            if floor.world_aligned_uv {
                a.uv = Vec2::new(a.reverse_transform_vertice(player).position.x / 10., a.reverse_transform_vertice(player).position.z / 10.);
                b.uv = Vec2::new(b.reverse_transform_vertice(player).position.x / 10., b.reverse_transform_vertice(player).position.z/ 10.);
                c.uv = Vec2::new(c.reverse_transform_vertice(player).position.x / 10., c.reverse_transform_vertice(player).position.z/ 10.);
            }

            material.a_uv = a.uv;
            material.b_uv = b.uv;
            material.c_uv = c.uv;
            material.uv_scalar = floor.uv_scalar;
            material.uv_offset = floor.uv_offset;
            material.uv_rotation = floor.uv_rotation;
            material.texture = asset_server.textures[floor.texture_id].clone();

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
    }
}