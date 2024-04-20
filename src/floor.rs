use bevy::{prelude::*, render::mesh::Mesh, sprite::MaterialMesh2dBundle};

use crate::vertice::Vertice;
use crate::CustomMaterial;
use crate::Player;
use crate::SceneAssets;

#[derive(Component, Clone)]
pub struct Floor {
    pub a: Vertice,
    pub b: Vertice,
    pub c: Vertice,
    pub complement: bool,
}

impl Floor {
    pub fn new(a: Vertice, b: Vertice, c: Vertice, complement: bool) -> Self {
        Self {
            a,
            b,
            c,
            complement,
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        custom_materials: &mut ResMut<Assets<CustomMaterial>>,
        asset_server: &mut Res<SceneAssets>,
        a: Vertice,
        b: Vertice,
        c: Vertice,
    ) {
        commands.spawn((
            Floor::new(a, b, c, false),
            MaterialMesh2dBundle {
                mesh: meshes.add(Triangle2d::default()).into(),
                material: custom_materials.add(CustomMaterial {
                    texture: asset_server.textures[0].clone(),
                    a: Vec3::ZERO,
                    b: Vec3::ZERO,
                    c: Vec3::ZERO,
                    a_uv: Vec2::ZERO,
                    b_uv: Vec2::ZERO,
                    c_uv: Vec2::ZERO,
                    uv_scalar: Vec2::new(1., 1.),
                    uv_offset: Vec2::new(0., 0.),
                    uv_rotation: 0.,
                }),
                ..Default::default()
            },
        ));

        commands.spawn((
            Floor::new(a, b, c, true),
            MaterialMesh2dBundle {
                mesh: meshes.add(Triangle2d::default()).into(),
                material: custom_materials.add(CustomMaterial {
                    texture: asset_server.textures[0].clone(),
                    a: Vec3::ZERO,
                    b: Vec3::ZERO,
                    c: Vec3::ZERO,
                    a_uv: Vec2::ZERO,
                    b_uv: Vec2::ZERO,
                    c_uv: Vec2::ZERO,
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
        let mut a = self.a.transform_vertice(player);
        let mut b = self.b.transform_vertice(player);
        let mut c = self.c.transform_vertice(player);

        // Copies of original vertices, non mutual
        let (org_a, org_b, org_c) = (a, b, c);

        // Zero vertice
        let zero = Vertice::zero();

        // All vertices are behind player
        if a.position.z > 0. && b.position.z > 0. && c.position.z > 0. {
            return (zero, zero, zero, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO);
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

            return (a, b, c, a.screen(), b.screen(), c.screen());
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

            return (a, b, c, a.screen(), b.screen(), c.screen());
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

            return (a, b, c, a.screen(), b.screen(), c.screen());
        }

        // Edge case. A is behind player. Yields complementary triangle
        if a.position.z > 0. {
            a.clip(b);

            let a_per = org_a.position.z / (org_a.position.z - org_b.position.z);
            a.uv = ((self.b.original_uv - self.a.original_uv) * a_per) + self.a.original_uv;

            if self.complement == true {
                let mut comp_b = org_a;
                comp_b.clip(c);

                let a_per = org_a.position.z / (org_a.position.z - org_c.position.z);
                comp_b.uv =
                    ((self.c.original_uv - self.a.original_uv) * a_per) + self.a.original_uv;

                return (a, comp_b, c, a.screen(), comp_b.screen(), c.screen());
            } else {
                return (b, a, c, b.screen(), a.screen(), c.screen());
            }
        }

        // Edge case. B is behind player. Yields complementary triangle
        if b.position.z > 0. {
            b.clip(c);

            let b_per = org_b.position.z / (org_b.position.z - org_c.position.z);
            b.uv = ((self.c.original_uv - self.b.original_uv) * b_per) + self.b.original_uv;

            if self.complement == true {
                let mut comp_c = org_b;
                comp_c.clip(a);

                let b_per = org_b.position.z / (org_b.position.z - org_a.position.z);
                comp_c.uv =
                    ((self.a.original_uv - self.b.original_uv) * b_per) + self.b.original_uv;

                return (a, b, comp_c, a.screen(), b.screen(), comp_c.screen());
            } else {
                return (a, b, c, a.screen(), b.screen(), c.screen());
            }
        }

        // Edge case. C is behind player. Yields complementary triangle
        if c.position.z > 0. {
            c.clip(a);

            let c_per = org_c.position.z / (org_c.position.z - org_a.position.z);
            c.uv = ((self.a.original_uv - self.c.original_uv) * c_per) + self.c.original_uv;

            if self.complement == true {
                let mut comp_a = org_c;
                comp_a.clip(b);

                let c_per = org_c.position.z / (org_c.position.z - org_b.position.z);
                comp_a.uv =
                    ((self.b.original_uv - self.c.original_uv) * c_per) + self.c.original_uv;

                return (comp_a, b, c, comp_a.screen(), b.screen(), c.screen());
            } else {
                return (a, b, c, a.screen(), b.screen(), c.screen());
            }
        }

        // No vertices are behind player
        return (a, b, c, a.screen(), b.screen(), c.screen());
    }
}
