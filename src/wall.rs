use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, Mesh},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
    sprite::MaterialMesh2dBundle,
};

use crate::{vertex::Vertex, CustomMaterial, Player, SceneAssets, MAX_STRUCTURES};

#[derive(Component, Clone)]
pub struct Wall {
    pub id: usize,
    pub start: Vertex,
    pub end: Vertex,
    pub height: f32,
    pub uv_scalar: Vec2,
    pub uv_offset: Vec2,
    pub uv_rotation: f32,
    pub texture_id: usize,
}

impl Wall {
    pub fn new(
        id: usize,
        start: Vertex,
        end: Vertex,
        height: f32,
        uv_scalar: Vec2,
        uv_offset: Vec2,
        uv_rotation: f32,
        texture_id: usize,
    ) -> Self {
        Self {
            id,
            start,
            end,
            height,
            uv_scalar,
            uv_offset,
            uv_rotation,
            texture_id,
        }
    }

    /// Returns empty TriangleList mesh.
    pub fn mesh() -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[0., 0., 0.], [0., 0., 0.], [0., 0., 0.], [0., 0., 0.]],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
        )
        .with_inserted_indices(Indices::U32(vec![0, 3, 1, 1, 3, 2]))
    }

    /// Returns clipped vertices and screen coordinates.
    pub fn transform(
        &self,
        player: &Player,
    ) -> (Vertex, Vertex, Vertex, Vertex, Vec2, Vec2, Vec2, Vec2) {
        let mut start = self.start.transform_vertice(player);
        let mut end = self.end.transform_vertice(player);

        // Both wall's starting and end points are behind the player
        // The wall does not have to be rendered
        if start.position.z > 0. && end.position.z > 0. {
            return (
                Vertex::zero(),
                Vertex::zero(),
                Vertex::zero(),
                Vertex::zero(),
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
        let mut a = Vertex::new(
            Vec3::new(start.position.x, start.position.y, start.position.z),
            Vec2::new(0., 1.),
        );
        let mut b = Vertex::new(
            Vec3::new(
                start.position.x,
                start.position.y + self.height,
                start.position.z,
            ),
            Vec2::new(0., 0.),
        );
        let mut c = Vertex::new(
            Vec3::new(end.position.x, end.position.y + self.height, end.position.z),
            Vec2::new(1., 0.),
        );
        let mut d = Vertex::new(
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

        if original_start.position.z > 0. {
            return (d, c, b, a, d.screen(), c.screen(), b.screen(), a.screen());
        } else {
            return (a, b, c, d, a.screen(), b.screen(), c.screen(), d.screen());
        }
    }

    /// Helps with calculating mask for z-buffering, same principle as transform.
    pub fn mask(&self, player: &Player) -> (Vertex, Vertex, Vertex, Vertex) {
        let mut start = self.start.transform_vertice(player);
        let mut end = self.end.transform_vertice(player);

        // Initialize variables
        let org_start = start;
        let zero = Vertex::zero();

        // Both wall's starting and end points are behind the player
        // The wall does not have to be rendered
        if start.position.z > 0. && end.position.z > 0. {
            return (zero, zero, zero, zero);
        }

        // Wall starting point is behind the player
        if start.position.z > 0. {
            start.clip(end);
        }

        // Wall end point is behind the player
        if end.position.z > 0. {
            end.clip(start);
        }

        // Define four corner vertices A, B, C and D
        let a = Vertex::new(
            Vec3::new(start.position.x, start.position.y, start.position.z),
            Vec2::new(0., 1.),
        );
        let b = Vertex::new(
            Vec3::new(
                start.position.x,
                start.position.y + self.height,
                start.position.z,
            ),
            Vec2::new(0., 0.),
        );
        let c = Vertex::new(
            Vec3::new(end.position.x, end.position.y + self.height, end.position.z),
            Vec2::new(1., 0.),
        );
        let d = Vertex::new(
            Vec3::new(end.position.x, end.position.y, end.position.z),
            Vec2::new(1., 1.),
        );

        if org_start.position.z > 0. {
            return (d, c, b, a);
        } else {
            return (a, b, c, d);
        }
    }
}

pub fn spawn_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    custom_materials: &mut ResMut<Assets<CustomMaterial>>,
    asset_server: &Res<SceneAssets>,
    wall: Wall,
) {
    commands.spawn((
        wall.clone(),
        MaterialMesh2dBundle {
            mesh: meshes.add(Wall::mesh()).into(),
            material: custom_materials.add(CustomMaterial {
                texture: asset_server.textures[wall.texture_id].clone(),
                id: -1.,
                mask: [Vec3::new(0., 0., 0.); MAX_STRUCTURES],
                mask_len: 0,
                a_screen: Vec3::new(0., 0., 0.),
                b_screen: Vec3::new(0., 0., 0.),
                c_screen: Vec3::new(0., 0., 0.),
                a_uv: Vec2::new(0., 0.),
                b_uv: Vec2::new(0., 0.),
                c_uv: Vec2::new(0., 0.),
                uv_scalar: Vec2::new(1., 1.),
                uv_offset: Vec2::new(0., 0.),
                uv_rotation: 0.,
                a_position: Vec3::new(0., 0., 0.),
                b_position: Vec3::new(0., 0., 0.),
                c_position: Vec3::new(0., 0., 0.),
                pitch: 0.0,
                selected: 0,
            }),
            ..Default::default()
        },
    ));
}