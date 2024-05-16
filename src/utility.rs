use bevy::math::Vec3;

//TODO: Use library function or something
pub fn norm(vec: Vec3) -> f32 {
    (vec.x.powi(2) + vec.y.powi(2) + vec.z.powi(2)).sqrt()
}

fn vec3_length(vec: Vec3) -> f32 {
    (vec.x * vec.x + vec.y * vec.y + vec.z * vec.z).sqrt()
}

pub fn normalize(mut vec: Vec3) -> Vec3 {
    let len = vec3_length(vec);
    if len != 0.0 {
        vec.x /= len;
        vec.y /= len;
        vec.z /= len;
    }

    vec
}