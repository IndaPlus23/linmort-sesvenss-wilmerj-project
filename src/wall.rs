use bevy::{prelude::*, render::mesh::Mesh, sprite::MaterialMesh2dBundle};
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::mesh::Indices;

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
    pub texture_id: usize,
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
        texture_id: usize,
    ) -> Self {
        Self {
            start,
            end,
            height,
            upper_triangle,
            uv_scalar,
            uv_offset,
            uv_rotation,
            texture_id,
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
        texture_id: usize,
    ) {
        let texture = asset_server.textures[texture_id].clone();

        commands.spawn((
            Wall::new(
                start,
                end,
                height,
                true,
                Vec2::new(1., 1.),
                Vec2::new(0., 0.),
                0.,
                0,
            ),
            MaterialMesh2dBundle {
                mesh: meshes.add(Self::new_wall_mesh()).into(),
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

    fn new_wall_mesh() -> Mesh {
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.], [0., 0., 0.]]
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]]
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]]
        )
        .with_inserted_indices(Indices::U32(vec![
            0, 3, 1,
            1, 3, 2
        ]))
    }

    // Returns clipped vertices and screen coordinates
    pub fn transform(&mut self, player: &Player) -> (Vertice, Vertice, Vertice, Vertice, Vec2, Vec2, Vec2, Vec2) {
        let mut start = self.start.transform_vertice(player);
        let mut end = self.end.transform_vertice(player);

        // Both wall's starting and end points are behind the player
        // The wall does not have to be rendered
        if start.position.z > 0. && end.position.z > 0. {
            return (
                Vertice::zero(),
                Vertice::zero(),
                Vertice::zero(),
                Vertice::zero(),
                Vec2::ZERO,
                Vec2::ZERO,
                Vec2::ZERO,
                Vec2::ZERO,
            );
        }

        // Initialize variables
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

        // Define four corner vertices A, B, C and D
        let mut a = Vertice::new(
            Vec3::new(start.position.x, start.position.y, start.position.z),
            Vec2::new(0., 1.),
        );
        let mut b = Vertice::new(
            Vec3::new(
                start.position.x,
                start.position.y + self.height,
                start.position.z,
            ),
            Vec2::new(0., 0.),
        );
        let mut c = Vertice::new(
            Vec3::new(end.position.x, end.position.y + self.height, end.position.z),
            Vec2::new(1., 0.),
        );
        let mut d = Vertice::new(
            Vec3::new(end.position.x, end.position.y, end.position.z),
            Vec2::new(1., 1.),
        );
        
        // Calculate correct uv coordinates for the clipped vertices
        if original_start.position.z > 0. {
            b.uv = ((c.uv - b.uv) * percentage) + b.uv;
            a.uv = ((d.uv - a.uv) * percentage) + a.uv;
        }

        // Calculate correct uv coordinates for the clipped vertices
        if original_end.position.z > 0. {
            c.uv = ((b.uv - c.uv) * percentage) + c.uv;
            d.uv = ((a.uv - d.uv) * percentage) + d.uv;
        }

        return (a, b, c, d, a.screen(), b.screen(), c.screen(), d.screen());
    }
}
