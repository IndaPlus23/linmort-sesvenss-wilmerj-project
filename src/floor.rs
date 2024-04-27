use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::{prelude::*, render::mesh::Mesh};

use crate::vertex::Vertex;
use crate::Player;

#[derive(Component, Clone)]
pub struct Floor {
    pub id: usize,
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
    pub uv_scalar: Vec2,
    pub uv_offset: Vec2,
    pub uv_rotation: f32,
    pub world_aligned_uv: bool,
    pub texture_id: usize,
}

impl Floor {
    pub fn new(
        id: usize,
        a: Vertex,
        b: Vertex,
        c: Vertex,
        uv_scalar: Vec2,
        uv_offset: Vec2,
        uv_rotation: f32,
        world_aligned_uv: bool,
        texture_id: usize,
    ) -> Self {
        Self {
            id,
            a,
            b,
            c,
            uv_scalar,
            uv_offset,
            uv_rotation,
            world_aligned_uv,
            texture_id,
        }
    }

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

    // Returns clipped vertices and screen coordinates
    pub fn transform(
        &self,
        player: &Player,
    ) -> (Vertex, Vertex, Vertex, Vertex, Vec2, Vec2, Vec2, Vec2) {
        let mut a = self.a.transform_vertice(player);
        let mut b = self.b.transform_vertice(player);
        let mut c = self.c.transform_vertice(player);

        // Copies of original vertices, non mutual
        let (org_a, org_b, org_c) = (a, b, c);

        // Zero vertex
        let zero = Vertex::zero();

        // All vertices are behind player
        if a.position.z > 0. && b.position.z > 0. && c.position.z > 0. {
            return (
                zero,
                zero,
                zero,
                zero,
                Vec2::ZERO,
                Vec2::ZERO,
                Vec2::ZERO,
                Vec2::ZERO,
            );
        }

        // Both A and B are behind player
        if a.position.z > 0. && b.position.z > 0. {
            a.clip(c);
            b.clip(c);

            // Calculate correct uv coordinates for the clipped vertices
            let a_per = org_a.position.z / (org_a.position.z - org_c.position.z);
            a.uv = ((self.c.original_uv - self.a.original_uv) * a_per) + self.a.original_uv;
            let b_per = org_b.position.z / (org_b.position.z - org_c.position.z);
            b.uv = ((self.c.original_uv - self.b.original_uv) * b_per) + self.b.original_uv;

            return (c, a, b, b, c.screen(), a.screen(), b.screen(), b.screen());
        }

        // Both A and C are behind player
        if a.position.z > 0. && c.position.z > 0. {
            a.clip(b);
            c.clip(b);

            // Calculate correct uv coordinates for the clipped vertices
            let a_per = org_a.position.z / (org_a.position.z - org_b.position.z);
            a.uv = ((self.b.original_uv - self.a.original_uv) * a_per) + self.a.original_uv;
            let c_per = org_c.position.z / (org_c.position.z - org_b.position.z);
            c.uv = ((self.b.original_uv - self.c.original_uv) * c_per) + self.c.original_uv;

            return (b, c, a, a, b.screen(), c.screen(), a.screen(), a.screen());
        }

        // Both B and C are behind player
        if b.position.z > 0. && c.position.z > 0. {
            b.clip(a);
            c.clip(a);

            // Calculate correct uv coordinates for the clipped vertices
            let b_per = org_b.position.z / (org_b.position.z - org_a.position.z);
            b.uv = ((self.a.original_uv - self.b.original_uv) * b_per) + self.b.original_uv;
            let c_per = org_c.position.z / (org_c.position.z - org_a.position.z);
            c.uv = ((self.a.original_uv - self.c.original_uv) * c_per) + self.c.original_uv;

            return (a, b, c, c, a.screen(), b.screen(), c.screen(), c.screen());
        }

        // Edge case. A is behind player. Yields complementary vertice
        if a.position.z > 0. {
            a.clip(b);

            let a_per = org_a.position.z / (org_a.position.z - org_b.position.z);
            a.uv = ((self.b.original_uv - self.a.original_uv) * a_per) + self.a.original_uv;

            let mut d = org_a;
            d.clip(c);

            let d_per = org_a.position.z / (org_a.position.z - org_c.position.z);
            d.uv = ((self.c.original_uv - self.a.original_uv) * d_per) + self.a.original_uv;

            return (c, d, a, b, c.screen(), d.screen(), a.screen(), b.screen());
        }

        // Edge case. B is behind player. Yields complementary vertice
        if b.position.z > 0. {
            b.clip(c);

            let b_per = org_b.position.z / (org_b.position.z - org_c.position.z);
            b.uv = ((self.c.original_uv - self.b.original_uv) * b_per) + self.b.original_uv;

            let mut d = org_b;
            d.clip(a);

            let d_per = org_b.position.z / (org_b.position.z - org_a.position.z);
            d.uv = ((self.a.original_uv - self.b.original_uv) * d_per) + self.b.original_uv;

            return (a, d, b, c, a.screen(), d.screen(), b.screen(), c.screen());
        }

        // Edge case. C is behind player. Yields complementary vertice
        if c.position.z > 0. {
            c.clip(a);

            let c_per = org_c.position.z / (org_c.position.z - org_a.position.z);
            c.uv = ((self.a.original_uv - self.c.original_uv) * c_per) + self.c.original_uv;

            let mut d = org_c;
            d.clip(b);

            let d_per = org_c.position.z / (org_c.position.z - org_b.position.z);
            d.uv = ((self.b.original_uv - self.c.original_uv) * d_per) + self.c.original_uv;

            return (b, d, c, a, b.screen(), d.screen(), c.screen(), a.screen());
        }

        // No vertices are behind player
        return (a, b, c, c, a.screen(), b.screen(), c.screen(), c.screen());
    }

    // Helps with calculating mask for z-buffering, same principle as transform
    pub fn mask(
        &self,
        player: &Player,
    ) -> (Vertex, Vertex, Vertex, Vertex, Vertex, Vertex,) {
        let mut a = self.a.transform_vertice(player);
        let mut b = self.b.transform_vertice(player);
        let mut c = self.c.transform_vertice(player);

        // Copies of original vertices, non mutual
        let (org_a, org_b, org_c) = (a, b, c);

        // Zero vertex
        let zero = Vertex::zero();

        // All vertices are behind player
        if a.position.z > 0. && b.position.z > 0. && c.position.z > 0. {
            return (zero, zero, zero, zero, zero, zero,);
        }

        // Both A and B are behind player
        if a.position.z > 0. && b.position.z > 0. {
            a.clip(c);
            b.clip(c);
            return (c, a, b, b, b, b);
        }

        // Both A and C are behind player
        if a.position.z > 0. && c.position.z > 0. {
            a.clip(b);
            c.clip(b);
            return (b, c, a, a, a, a);
        }

        // Both B and C are behind player
        if b.position.z > 0. && c.position.z > 0. {
            b.clip(a);
            c.clip(a);
            return (a, b, c, c, c, c);
        }

        // Edge case. A is behind player. Yields complementary vertice
        if a.position.z > 0. {
            a.clip(b);
            let mut d = org_a;
            d.clip(c);
            return (c, d, a, c, a, b);
        }

        // Edge case. B is behind player. Yields complementary vertice
        if b.position.z > 0. {
            b.clip(c);
            let mut d = org_b;
            d.clip(a);
            return (a, d, b, a, b, c);
        }

        // Edge case. C is behind player. Yields complementary vertice
        if c.position.z > 0. {
            c.clip(a);
            let mut d = org_c;
            d.clip(b);
            return (b, d, c, b, c, a);
        }

        // No vertices are behind player
        return (a, b, c, zero, zero, zero);
    }
}
