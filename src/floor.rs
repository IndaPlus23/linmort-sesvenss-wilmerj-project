use bevy::{prelude::*, render::mesh::Mesh, sprite::MaterialMesh2dBundle};
use std::f32::consts::PI;

use crate::CustomMaterial;
use crate::Player;

#[derive(Component, Clone)]
pub struct Floor {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub a_uv: Vec2,
    pub b_uv: Vec2,
    pub c_uv: Vec2,
    pub complement: bool,
}

impl Floor {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, a_uv: Vec2, b_uv: Vec2, c_uv: Vec2, complement: bool) -> Self {
        Self {
            a,
            b,
            c,
            a_uv,
            b_uv,
            c_uv,
            complement,
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        custom_materials: &mut ResMut<Assets<CustomMaterial>>,
        asset_server: &mut Res<AssetServer>,
        a: Vec3,
        b: Vec3,
        c: Vec3,
        a_uv: Vec2,
        b_uv: Vec2,
        c_uv: Vec2,
    ) {
        commands.spawn((
            Floor::new(a, b, c, a_uv, b_uv, c_uv, false),
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
            Floor::new(a, b, c, a_uv, b_uv, c_uv, true),
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

    pub fn clipping(&mut self, player: &Player) -> (Vec3, Vec3, Vec3, Vec3, Vec3, Vec3, Vec2, Vec2, Vec2, Vec2, Vec2, Vec2) {
        let mut a = vertices_to_world_coordinates(player, self.a.x, self.a.y, self.a.z);
        let mut b = vertices_to_world_coordinates(player, self.b.x, self.b.y, self.b.z);
        let mut c = vertices_to_world_coordinates(player, self.c.x, self.c.y, self.c.z);

        // Copies of original vertices, non mutual
        let (org_a, org_b, org_c) = (a, b, c);

        // Zero vectors for readability
        let z3 = Vec3::ZERO;
        let z2 = Vec2::ZERO;

        // All vertices are behind player
        if a.z > 0. && b.z > 0. && c.z > 0. {
            return (z3, z3, z3, z3, z3, z3, z2, z2, z2, z2, z2, z2);
        }

        // Both A and B are behind player
        if a.z > 0. && b.z > 0. {
            (a, c) = clip_line_segment(a, c);
            (b, c) = clip_line_segment(b, c);

            // Calculate correct uv coordinates for the clipped vertices
            let a_per = org_a.z / (org_a.z -org_c.z);
            let a_uv = ((self.c_uv - self.a_uv) * a_per) + self.a_uv;
            let b_per = org_b.z / (org_b.z -org_c.z);
            let b_uv = ((self.c_uv - self.b_uv) * b_per) + self.b_uv;

            return (a, b, c, z3, z3, z3, a_uv, b_uv, self.c_uv, z2, z2, z2);
        }

        // Both A and C are behind player
        if a.z > 0. && c.z > 0. {
            (a, b) = clip_line_segment(a, b);
            (c, b) = clip_line_segment(c, b);

            // Calculate correct uv coordinates for the clipped vertices
            let a_per = org_a.z / (org_a.z -org_b.z);
            let a_uv = ((self.b_uv - self.a_uv) * a_per) + self.a_uv;
            let c_per = org_c.z / (org_c.z -org_b.z);
            let c_uv = ((self.b_uv - self.c_uv) * c_per) + self.c_uv;

            return (a, b, c, z3, z3, z3, a_uv, self.b_uv, c_uv, z2, z2, z2);
        }

        // Both B and C are behind player
        if b.z > 0. && c.z > 0. {
            (b, a) = clip_line_segment(b, a);
            (c, a) = clip_line_segment(c, a);

            // Calculate correct uv coordinates for the clipped vertices
            let b_per = org_b.z / (org_b.z -org_a.z);
            let b_uv = ((self.a_uv - self.b_uv) * b_per) + self.b_uv;
            let c_per = org_c.z / (org_c.z -org_a.z);
            let c_uv = ((self.a_uv - self.c_uv) * c_per) + self.c_uv;

            return (a, b, c, z3, z3, z3, self.a_uv, b_uv, c_uv, z2, z2, z2);
        }

        // Edge case. A is behind player. Yields complementary triangle
        if a.z > 0. {
            (a, b) = clip_line_segment(a, b);
            let (comp_b, _) = clip_line_segment(org_a, c);

            let a_per = org_a.z / (org_a.z -org_b.z);
            let a_uv = ((self.b_uv - self.a_uv) * a_per) + self.a_uv;

            let comp_b_per = org_a.z / (org_a.z -org_c.z);
            let comp_b_uv = ((self.c_uv - self.a_uv) * comp_b_per) + self.a_uv;

            return (a, b, c, a, comp_b, c, self.a_uv, self.b_uv, self.c_uv, z2, z2, z2);
        }

        // Edge case. B is behind player. Yields complementary triangle
        if b.z > 0. {
            (b, c) = clip_line_segment(b, c);
            let (comp_c, _) = clip_line_segment(org_b, a);

            let b_per = org_b.z / (org_b.z -org_c.z);
            let b_uv = ((self.c_uv - self.b_uv) * b_per) + self.b_uv;

            let comp_c_per = org_b.z / (org_b.z -org_a.z);
            let comp_c_uv = ((self.a_uv - self.b_uv) * comp_c_per) + self.b_uv;

            let x = Vec2::new(0., 1.);
            let y = Vec2::new(0., 0.,);
            let z = Vec2::new(1., 1.);

            return (a, b, c, a, comp_c, b, self.a_uv, b_uv, self.c_uv, x, y, z);
        }

        // Edge case. C is behind player. Yields complementary triangle
        if c.z > 0. {
            (c, a) = clip_line_segment(c, a);
            let (comp_a, _) = clip_line_segment(org_c, b);
            return (a, b, c, comp_a, b, c, self.a_uv, self.b_uv, self.c_uv, z2, z2, z2);
        }

        // No vertices are behind player
        return (a, b, c, Vec3::ZERO, Vec3::ZERO, Vec3::ZERO, self.a_uv, self.b_uv, self.c_uv, z2, z2, z2);
    }
}

// Converts vertices to world coordinates based on player rotation and position.
fn vertices_to_world_coordinates(player: &Player, mut x: f32, mut y: f32, mut z: f32) -> Vec3 {
    let cos = player.yaw.cos();
    let sin = player.yaw.sin();

    x -= player.x;
    y -= player.y;
    z -= player.z;

    let world_x = x * cos + z * sin;
    let world_z = z * cos - x * sin;
    let world_y = y + (player.pitch * world_z);

    Vec3::new(world_x, world_y, world_z)
}

// Starting point is behind the player
// Clip the starting point so it never is behind the player
fn clip_line_segment(start: Vec3, end: Vec3) -> (Vec3, Vec3) {
    let delta_z = end.z - start.z;
    let delta_x = end.x - start.x;
    let delta_y = end.y - start.y;

    let k = delta_z / delta_x;
    let m = start.z - (k * start.x);
    let new_start_x = -m / k;

    let k = delta_z / delta_y;
    let m = start.z - (k * start.y);
    let new_start_y = -m / k;

    let new_start = Vec3::new(new_start_x, new_start_y, -0.01);
    return (new_start, end);
}

// Converts world 3d coordinates to 2d screen coordinates
fn world_to_screen_coordinates(world_x: f32, world_y: f32, world_z: f32) -> Vec2 {
    let screen_x = world_x * 1500. / world_z;
    let screen_y = world_y * 1500. / world_z;

    Vec2::new(screen_x, screen_y)
}