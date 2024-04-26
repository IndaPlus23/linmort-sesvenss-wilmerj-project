use crate::Player;
use bevy::math::{Vec2, Vec3};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub original_position: Vec3,
    pub position: Vec3,
    pub original_uv: Vec2,
    pub uv: Vec2,
    pub transformation: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, uv: Vec2) -> Self {
        let original_position = position;
        let original_uv = uv;
        let transformation = Vec3::ZERO;
        Self {
            original_position,
            position,
            original_uv,
            uv,
            transformation,
        }
    }

    pub fn zero() -> Self {
        let original_position = Vec3::ZERO;
        let position = Vec3::ZERO;
        let original_uv = Vec2::ZERO;
        let uv = Vec2::ZERO;
        let transformation = Vec3::ZERO;
        Self {
            original_position,
            position,
            original_uv,
            uv,
            transformation,
        }
    }

    // Apply transformation based on player rotation and position.
    pub fn transform_vertice(&self, player: &Player) -> Vertex {
        let mut x = self.position.x;
        let mut y = self.position.y;
        let mut z = self.position.z;

        let cos = player.yaw.cos();
        let sin = player.yaw.sin();

        x -= player.x;
        y -= player.y;
        z -= player.z;

        let new_x = x * cos + z * sin;
        let new_z = z * cos - x * sin;
        let new_y = y + (player.pitch * new_z);

        Vertex::new(Vec3::new(new_x, new_y, new_z), self.uv)
    }

    pub fn reverse_transform_vertice(&self, player: &Player) -> Vertex {
        let new_x = self.position.x;
        let mut new_y = self.position.y;
        let new_z = self.position.z;

        // Invert the pitch rotation
        new_y -= player.pitch * new_z;

        // Invert the yaw rotation
        let cos = player.yaw.cos();
        let sin = player.yaw.sin();
        let mut old_x = new_x * cos - new_z * sin;
        let mut old_z = new_z * cos + new_x * sin;

        // Invert the translation
        old_x += player.x;
        old_z += player.z;
        new_y += player.y;

        Vertex::new(Vec3::new(old_x, new_y, old_z), self.uv)
    }

    // Starting point is behind the player
    // Clip the starting point so it never is behind the player
    pub fn clip(&mut self, with: Vertex) {
        let delta_z = with.position.z - self.position.z;
        let delta_x = with.position.x - self.position.x;
        let delta_y = with.position.y - self.position.y;

        let k = delta_z / delta_x;
        let m = self.position.z - (k * self.position.x);
        let new_start_x = -m / k;

        let k = delta_z / delta_y;
        let m = self.position.z - (k * self.position.y);
        let new_start_y = -m / k;

        self.position = Vec3::new(new_start_x, new_start_y, -0.01);
    }

    // Converts vertice coordinates to 2d screen coordinates
    pub fn screen(&self) -> Vec2 {
        let world_x = self.position.x;
        let world_y = self.position.y;
        let world_z = self.position.z;

        let screen_x = world_x * 1500. / world_z;
        let screen_y = world_y * 1500. / world_z;

        Vec2::new(-screen_x, -screen_y)
    }
}
