use bevy::{prelude::*, render::mesh::Mesh, sprite::MaterialMesh2dBundle};
use std::f32::consts::PI;

use crate::Vertice;
use crate::CustomMaterial;
use crate::Player;

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
        asset_server: &mut Res<AssetServer>,
        a: Vertice,
        b: Vertice,
        c: Vertice,
    ) {
        commands.spawn((
            Floor::new(a, b, c,  false),
            MaterialMesh2dBundle {
                mesh: meshes.add(Triangle2d::default()).into(),
                material: custom_materials.add(CustomMaterial {
                    texture: asset_server.load("grass_front.png"),
                    a: Vec3::ZERO,
                    b: Vec3::ZERO,
                    c: Vec3::ZERO,
                    a_uv: Vec2::ZERO,
                    b_uv: Vec2::ZERO,
                    c_uv: Vec2::ZERO,
                }),
                ..Default::default()
            },
        ));

        commands.spawn((
            Floor::new(a, b, c, true),
            MaterialMesh2dBundle {
                mesh: meshes.add(Triangle2d::default()).into(),
                material: custom_materials.add(CustomMaterial {
                    texture: asset_server.load("grass_front.png"),
                    a: Vec3::ZERO,
                    b: Vec3::ZERO,
                    c: Vec3::ZERO,
                    a_uv: Vec2::ZERO,
                    b_uv: Vec2::ZERO,
                    c_uv: Vec2::ZERO,
                }),
                ..Default::default()
            },
        ));
    }

    // Returns clipped vertices and screen coordinates
    pub fn clipping(&mut self, player: &Player) -> (Vertice, Vertice, Vertice, Vec2, Vec2, Vec2) {
        let mut a = transform_vertice(player, self.a);
        let mut b = transform_vertice(player, self.b);
        let mut c = transform_vertice(player, self.c);

        // Copies of original vertices, non mutual
        let (org_a, org_b, org_c) = (a, b, c);

        // Zero vertice
        let zero = Vertice::zero();

        // All vertices are behind player
        if a.position.z > 0. && b.position.z > 0. && c.position.z > 0. {
            return (zero, zero, zero, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO)
        }

        // Both A and B are behind player
        if a.position.z > 0. && b.position.z > 0. {
            (a, c) = clip(a, c);
            (b, c) = clip(b, c);

            // Calculate correct uv coordinates for the clipped vertices
            let a_per = org_a.position.z / (org_a.position.z -org_c.position.z);
            a.uv = ((self.c.original_uv - self.a.original_uv) * a_per) + self.a.original_uv;
            let b_per = org_b.position.z / (org_b.position.z -org_c.position.z);
            b.uv = ((self.c.original_uv - self.b.original_uv) * b_per) + self.b.original_uv;

            return (a, b, c, screen(a), screen(b), screen(c))
        }

        // Both A and C are behind player
        if a.position.z > 0. && c.position.z > 0. {
            (a, b) = clip(a, b);
            (c, b) = clip(c, b);

            // Calculate correct uv coordinates for the clipped vertices
            let a_per = org_a.position.z / (org_a.position.z -org_b.position.z);
            a.uv = ((self.b.original_uv - self.a.original_uv) * a_per) + self.a.original_uv;
            let c_per = org_c.position.z / (org_c.position.z -org_b.position.z);
            c.uv = ((self.b.original_uv - self.c.original_uv) * c_per) + self.c.original_uv;

            return (a, b, c, screen(a), screen(b), screen(c))
        }

        // Both B and C are behind player
        if b.position.z > 0. && c.position.z > 0. {
            (b, a) = clip(b, a);
            (c, a) = clip(c, a);

            // Calculate correct uv coordinates for the clipped vertices
            let b_per = org_b.position.z / (org_b.position.z -org_a.position.z);
            b.uv = ((self.a.original_uv - self.b.original_uv) * b_per) + self.b.original_uv;
            let c_per = org_c.position.z / (org_c.position.z -org_a.position.z);
            c.uv = ((self.a.original_uv - self.c.original_uv) * c_per) + self.c.original_uv;

            return (a, b, c, screen(a), screen(b), screen(c))
        }

        // Edge case. A is behind player. Yields complementary triangle
        if a.position.z > 0. {
            (a, b) = clip(a, b);

            let a_per = org_a.position.z / (org_a.position.z -org_b.position.z);
            a.uv = ((self.b.original_uv - self.a.original_uv) * a_per) + self.a.original_uv;

            if self.complement == true {
                let (mut comp_b, _) = clip(org_a, c);

                let a_per = org_a.position.z / (org_a.position.z -org_c.position.z);
                comp_b.uv = ((self.c.original_uv - self.a.original_uv) * a_per) + self.a.original_uv;

                return (a, comp_b, c, screen(a), screen(comp_b), screen(c))
            } else {
                return (b, a, c, screen(b), screen(a), screen(c))
            }
        }

        // Edge case. B is behind player. Yields complementary triangle
        if b.position.z > 0. {
            (b, c) = clip(b, c);

            let b_per = org_b.position.z / (org_b.position.z -org_c.position.z);
            b.uv = ((self.c.original_uv - self.b.original_uv) * b_per) + self.b.original_uv;

            if self.complement == true {
                let (mut comp_c, _) = clip(org_b, a);

                let b_per = org_b.position.z / (org_b.position.z -org_a.position.z);
                comp_c.uv = ((self.a.original_uv - self.b.original_uv) * b_per) + self.b.original_uv;

                return (a, b, comp_c, screen(a), screen(b), screen(comp_c))
            } else {
                return (a, b, c, screen(a), screen(b), screen(c))
            }
        }

        // Edge case. C is behind player. Yields complementary triangle
        if c.position.z > 0. {
            (c, a) = clip(c, a);

            let c_per = org_c.position.z / (org_c.position.z -org_a.position.z);
            c.uv = ((self.a.original_uv - self.c.original_uv) * c_per) + self.c.original_uv;

            if self.complement == true {
                let (mut comp_a, _) = clip(org_c, b);

                let c_per = org_c.position.z / (org_c.position.z -org_b.position.z);
                comp_a.uv = ((self.b.original_uv - self.c.original_uv) * c_per) + self.c.original_uv;

                return (comp_a, b, c, screen(comp_a), screen(b), screen(c))
            } else {
                return (a, b, c, screen(a), screen(b), screen(c))
            }
        }

        // No vertices are behind player
        return (a, b, c, screen(a), screen(b), screen(c))
    }
}

// Apply transformation based on player rotation and position.
fn transform_vertice(player: &Player, vertice: Vertice) -> Vertice {
    let mut x = vertice.position.x;
    let mut y = vertice.position.y;
    let mut z = vertice.position.z;
    
    let cos = player.yaw.cos();
    let sin = player.yaw.sin();

    x -= player.x;
    y -= player.y;
    z -= player.z;

    let new_x = x * cos + z * sin;
    let new_z = z * cos - x * sin;
    let new_y = y + (player.pitch * new_z);

    Vertice::new(Vec3::new(new_x, new_y, new_z), vertice.uv)
}

// Starting point is behind the player
// Clip the starting point so it never is behind the player
fn clip(mut start: Vertice, end: Vertice) -> (Vertice, Vertice) {
    let delta_z = end.position.z - start.position.z;
    let delta_x = end.position.x - start.position.x;
    let delta_y = end.position.y - start.position.y;

    let k = delta_z / delta_x;
    let m = start.position.z - (k * start.position.x);
    let new_start_x = -m / k;

    let k = delta_z / delta_y;
    let m = start.position.z - (k * start.position.y);
    let new_start_y = -m / k;

    start.new_position(Vec3::new(new_start_x, new_start_y, -0.01));
    return (start, end)
}

// Converts vertice coordinates to 2d screen coordinates
fn screen(vertice: Vertice) -> Vec2 {
    let world_x = vertice.position.x;
    let world_y = vertice.position.y;
    let world_z = vertice.position.z;

    let screen_x = world_x * 1500. / world_z;
    let screen_y = world_y * 1500. / world_z;

    Vec2::new(-screen_x, -screen_y)
}