use crate::Player;
use bevy::math::{Vec2, Vec3};

#[derive(Clone, Copy)]
pub struct Vertice {
    pub original_position: Vec3,
    pub position: Vec3,
    pub original_uv: Vec2,
    pub uv: Vec2,
    pub transformation: Vec3,
}

impl Vertice {
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

    pub fn new_position(&mut self, position: Vec3) {
        let position = position;
        self.position = position;
    }

    // Apply transformation based on player rotation and position.
    pub fn transform_vertice(&self, player: &Player) -> Vertice {
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

        Vertice::new(Vec3::new(new_x, new_y, new_z), self.uv)
    }

    // Starting point is behind the player
    // Clip the starting point so it never is behind the player
    pub fn clip(&mut self, with: Vertice) {
        let delta_z = with.position.z - self.position.z;
        let delta_x = with.position.x - self.position.x;
        let delta_y = with.position.y - self.position.y;

        let k = delta_z / delta_x;
        let m = self.position.z - (k * self.position.x);
        let new_start_x = -m / k;

        let k = delta_z / delta_y;
        let m = self.position.z - (k * self.position.y);
        let new_start_y = -m / k;

        self.new_position(Vec3::new(new_start_x, new_start_y, -0.01));
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

    pub fn uv_scale(&mut self, scalar: Vec2) {
        fn normalize_float(x: f32) -> f32 {
            let fractional_part = x % 1.0;
            if fractional_part < 0.0 {
                fractional_part + 1.0
            } else {
                fractional_part
            }
        }

        self.uv.x *= scalar.x;
        self.uv.y *= scalar.y;

        self.uv.x = normalize_float(self.uv.x);
        self.uv.y = normalize_float(self.uv.y);
    }
}
