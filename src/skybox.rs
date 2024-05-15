use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::Mesh,
        prelude::Image,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{Material2d, Mesh2dHandle},
    window::PrimaryWindow,
};
use core::f32::consts::PI;
use nalgebra::{Rotation3, Unit, Vector3};

use crate::{render::MAX_STRUCTURES, Player};

#[derive(Component, Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CubeMapMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub px: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub nx: Handle<Image>,
    #[texture(4)]
    #[sampler(5)]
    pub py: Handle<Image>,
    #[texture(6)]
    #[sampler(7)]
    pub ny: Handle<Image>,
    #[texture(8)]
    #[sampler(9)]
    pub pz: Handle<Image>,
    #[texture(10)]
    #[sampler(11)]
    pub nz: Handle<Image>,
    #[uniform(12)]
    pub window_width: f32,
    #[uniform(13)]
    pub window_height: f32,
    #[uniform(14)]
    pub direction: Vec3,
    #[uniform(15)]
    pub horizontal_vector: Vec3,
    #[uniform(16)]
    pub vertical_vector: Vec3,
    #[uniform(17)]
    pub mask: [Vec3; MAX_STRUCTURES],
    #[uniform(18)]
    pub mask_len: i32,
}

impl Material2d for CubeMapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cubemap.wgsl".into()
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

pub fn render_skybox(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut player_query: Query<&Player>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cubemap_materials: ResMut<Assets<CubeMapMaterial>>,
    mut cubemap: Query<(&mut Mesh2dHandle, &mut Handle<CubeMapMaterial>)>,
) {
    let primary_window = window.single_mut();

    for player in player_query.iter_mut() {
        for (mesh2dhandle, material_handle) in cubemap.iter_mut() {
            let mesh_handle = &mesh2dhandle.0;
            let mesh = meshes.get_mut(mesh_handle).unwrap();

            let material = cubemap_materials.get_mut(material_handle.clone()).unwrap();

            material.window_width = primary_window.width();
            material.window_height = primary_window.height();
            material.direction = forward_vector(player.yaw, player.pitch);

            let left_direction = forward_vector(
                (player.yaw - (90. * PI / 180.)).rem_euclid(2.0 * PI),
                player.pitch,
            );
            let right_direction = forward_vector(
                (player.yaw + (90. * PI / 180.)).rem_euclid(2.0 * PI),
                player.pitch,
            );
            material.horizontal_vector = right_direction - left_direction;

            let up_direction = forward_vector(
                player.yaw,
                (player.pitch + (90. * PI / 180.)).clamp(-PI / 2.0, PI / 2.0),
            );
            let down_direction = forward_vector(
                player.yaw,
                (player.pitch - (90. * PI / 180.)).clamp(-PI / 2.0, PI / 2.0),
            );
            material.vertical_vector = up_direction - down_direction;

            //println!("{:?}", forward_vector(player.yaw, player.pitch));

            if let Some(_) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        [
                            -primary_window.width() / 2.,
                            primary_window.height() / 2.,
                            0.0,
                        ],
                        [
                            primary_window.width() / 2.,
                            primary_window.height() / 2.,
                            0.0,
                        ],
                        [
                            primary_window.width() / 2.,
                            -primary_window.height() / 2.,
                            0.0,
                        ],
                        [
                            -primary_window.width() / 2.,
                            -primary_window.height() / 2.,
                            0.0,
                        ],
                    ],
                );
            }
        }
    }
}
