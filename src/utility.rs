use bevy::math::{Vec2, Vec3};

//TODO: Use library function or something
pub fn norm(vec: Vec3) -> f32 {
    (vec.x.powi(2) + vec.y.powi(2) + vec.z.powi(2)).sqrt()
}
