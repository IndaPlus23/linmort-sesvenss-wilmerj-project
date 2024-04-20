use bevy::{prelude::*, render::mesh::Mesh, sprite::MaterialMesh2dBundle};

use crate::CustomMaterial;
use crate::Player;
use crate::SceneAssets;
use crate::Vertice;

#[derive(Component, Clone)]
pub struct Wall {
    pub start: Vertice,
    pub end: Vertice,
    pub height: f32,
    pub upper_triangle: bool,
    pub uv_scalar: Vec2,
    pub uv_offset: Vec2,
    pub uv_rotation: f32,
    pub texture: Handle<Image>,
}

impl Wall {
    pub fn new(
        start: Vertice,
        end: Vertice,
        height: f32,
        upper_triangle: bool,
        uv_scalar: Vec2,
        uv_offset: Vec2,
        uv_rotation: f32,
        texture: Handle<Image>,
    ) -> Self {
        Self {
            start,
            end,
            height,
            upper_triangle,
            uv_scalar,
            uv_offset,
            uv_rotation,
            texture,
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        custom_materials: &mut ResMut<Assets<CustomMaterial>>,
        asset_server: &mut Res<SceneAssets>,
        start: Vertice,
        end: Vertice,
        height: f32,
    ) {
        let texture = asset_server.textures[0].clone();

        commands.spawn((
            Wall::new(
                start,
                end,
                height,
                true,
                Vec2::new(1., 1.),
                Vec2::new(0., 0.),
                0.,
                texture.clone(),
            ),
            MaterialMesh2dBundle {
                mesh: meshes.add(Triangle2d::default()).into(),
                material: custom_materials.add(CustomMaterial {
                    texture: texture.clone(),
                    a: Vec3::new(0., 0., 0.),
                    b: Vec3::new(0., 0., 0.),
                    c: Vec3::new(0., 0., 0.),
                    a_uv: Vec2::new(0., 0.),
                    b_uv: Vec2::new(0., 0.),
                    c_uv: Vec2::new(0., 0.),
                    uv_scalar: Vec2::new(1., 1.),
                    uv_offset: Vec2::new(0., 0.),
                    uv_rotation: 0.,
                }),
                ..Default::default()
            },
        ));

        commands.spawn((
            Wall::new(
                start,
                end,
                height,
                false,
                Vec2::new(1., 1.),
                Vec2::new(0., 0.),
                0.,
                texture.clone(),
            ),
            MaterialMesh2dBundle {
                mesh: meshes.add(Triangle2d::default()).into(),
                material: custom_materials.add(CustomMaterial {
                    texture: texture.clone(),
                    a: Vec3::new(0., 0., 0.),
                    b: Vec3::new(0., 0., 0.),
                    c: Vec3::new(0., 0., 0.),
                    a_uv: Vec2::new(0., 0.),
                    b_uv: Vec2::new(0., 0.),
                    c_uv: Vec2::new(0., 0.),
                    uv_scalar: Vec2::new(1., 1.),
                    uv_offset: Vec2::new(0., 0.),
                    uv_rotation: 0.,
                }),
                ..Default::default()
            },
        ));
    }

    // Returns clipped vertices and screen coordinates
    pub fn transform(&mut self, player: &Player) -> (Vertice, Vertice, Vertice, Vec2, Vec2, Vec2) {
        let mut start = self.start.transform_vertice(player);
        let mut end = self.end.transform_vertice(player);

        // Both wall's starting and end points are behind the player
        // The wall does not have to be rendered
        if start.position.z > 0. && end.position.z > 0. {
            return (
                Vertice::zero(),
                Vertice::zero(),
                Vertice::zero(),
                Vec2::ZERO,
                Vec2::ZERO,
                Vec2::ZERO,
            );
        }

        // Initialize variables
        let (mut a, mut b, mut c);
        let (original_start, original_end) = (start, end);
        let mut percentage = 0.;

        // Wall starting point is behind the player
        if start.position.z > 0. {
            percentage = start.position.z / (start.position.z - end.position.z);
            start.clip(end);
        }

        // Wall end point is behind the player
        if end.position.z > 0. {
            percentage = end.position.z / (end.position.z - start.position.z);
            end.clip(start);
        }

        if self.upper_triangle == true {
            a = Vertice::new(
                Vec3::new(start.position.x, start.position.y, start.position.z),
                Vec2::new(0., 1.),
            );
            b = Vertice::new(
                Vec3::new(
                    start.position.x,
                    start.position.y + self.height,
                    start.position.z,
                ),
                Vec2::new(0., 0.),
            );
            c = Vertice::new(
                Vec3::new(end.position.x, end.position.y + self.height, end.position.z),
                Vec2::new(1., 0.),
            );
        } else {
            a = Vertice::new(
                Vec3::new(end.position.x, end.position.y + self.height, end.position.z),
                Vec2::new(1., 0.),
            );
            b = Vertice::new(
                Vec3::new(start.position.x, start.position.y, start.position.z),
                Vec2::new(0., 1.),
            );
            c = Vertice::new(
                Vec3::new(end.position.x, end.position.y, end.position.z),
                Vec2::new(1., 1.),
            );
        }

        // Calculate correct uv coordinates for the clipped vertices
        if original_start.position.z > 0. {
            if self.upper_triangle == true {
                b.uv = ((c.uv - b.uv) * percentage) + b.uv;
                let c_uv = Vec2::new(1., 1.);
                a.uv = ((c_uv - a.uv) * percentage) + a.uv;
            } else {
                let c_uv = Vec2::new(1., 1.);
                b.uv = ((c_uv - b.uv) * percentage) + b.uv;
            }
        }

        // Calculate correct uv coordinates for the clipped vertices
        if original_end.position.z > 0. {
            if self.upper_triangle == true {
                c.uv = ((b.uv - c.uv) * percentage) + c.uv;
            } else {
                c.uv = ((b.uv - c.uv) * percentage) + c.uv;
                let b_uv = Vec2::new(0., 0.);
                a.uv = ((b_uv - a.uv) * percentage) + a.uv;
            }
        }

        return (a, b, c, a.screen(), b.screen(), c.screen());
    }
}
