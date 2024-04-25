#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var color_texture: texture_2d<f32>;
@group(2) @binding(1) var color_sampler: sampler;
@group(2) @binding(2) var mask_texture: texture_2d<f32>;
@group(2) @binding(3) var mask_sampler: sampler;
@group(2) @binding(4) var<uniform> window_size: vec2<f32>;
@group(2) @binding(5) var<uniform> a: vec3<f32>;
@group(2) @binding(6) var<uniform> b: vec3<f32>;
@group(2) @binding(7) var<uniform> c: vec3<f32>;
@group(2) @binding(8) var<uniform> a_uv: vec2<f32>;
@group(2) @binding(9) var<uniform> b_uv: vec2<f32>;
@group(2) @binding(10) var<uniform> c_uv: vec2<f32>;
@group(2) @binding(11) var<uniform> uv_scalar: vec2<f32>;
@group(2) @binding(12) var<uniform> uv_offset: vec2<f32>;
@group(2) @binding(13) var<uniform> uv_rotation: f32;

// Compute barycentric coordinates (b1, b2, b3) for point p with respect to triangle (a, b, c)
fn barycentric(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>, c: vec2<f32>) -> vec3<f32>{
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;

    let d00 = dot(v0, v0);
    let d01 = dot(v0, v1);
    let d11 = dot(v1, v1);
    let d20 = dot(v2, v0);
    let d21 = dot(v2, v1);

    let denom = d00 * d11 - d01 * d01;
    let b2 = (d11 * d20 - d01 * d21) / denom;
    let b3 = (d00 * d21 - d01 * d20) / denom;
    let b1 = 1.0 - b2 - b3;

    return vec3<f32>(b1, b2, b3);
}

fn normalize_float(x: f32) -> f32 {
    var fractional_part: f32 = x - floor(x);

    if fractional_part < 0.0 {
        fractional_part = fractional_part + 1.0;
    }

    return fractional_part;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let p: vec2<f32> = vec2<f32>(mesh.world_position[0], mesh.world_position[1]);
    let bary: vec3<f32> = barycentric(p, vec2<f32>(a[0], a[1]), vec2<f32>(b[0], b[1]), vec2<f32>(c[0], c[1]));

    let z = bary[0] / a[2] + bary[1] / b[2] + bary[2] / c[2];
    let uv: vec2<f32> = (bary[0] * a_uv) / a[2] + (bary[1] * b_uv) / b[2] + (bary[2] * c_uv) / c[2];
    let corrected_uv = vec2<f32>(uv[0] / z, uv[1] / z);

    let transformed_uv = vec2<f32>(
        ((corrected_uv[0] + uv_offset[0]) * uv_scalar[0]), 
        ((corrected_uv[1] + uv_offset[1]) * uv_scalar[1]));

    // Calculate sine and cosine of the rotation angle
    let sin_rot = sin(uv_rotation * 3.14159265359 / 180.0);
    let cos_rot = cos(uv_rotation * 3.14159265359 / 180.0);

    let rotated_uv = vec2<f32>(
        transformed_uv[0] * cos_rot - transformed_uv[1] * sin_rot, 
        transformed_uv[0] * sin_rot + transformed_uv[1] * cos_rot);

    let normalized_uv = vec2<f32>(
        normalize_float(rotated_uv[0]),
        normalize_float(rotated_uv[1])
    );

    //https://computergraphics.stackexchange.com/questions/1866/how-to-map-square-texture-to-triangle
    //https://stackoverflow.com/questions/12360023/barycentric-coordinates-texture-mapping

    // Sample the color texture
    var sampled_color: vec4<f32> = textureSample(color_texture, color_sampler, normalized_uv);

    let world_coord: vec3<f32> = (bary[0] * a) + (bary[1] * b) + (bary[2] * c);
    let distance = length(world_coord);

    // Modify the alpha channel (opacity) here (e.g., set it to 0.5 for semi-transparency)
    // sampled_color[3] = 0.5; // Set alpha to 0.5 (50% opacity)

    return sampled_color;
}