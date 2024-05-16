#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var color_texture: texture_2d<f32>;
@group(2) @binding(1) var color_sampler: sampler;
@group(2) @binding(2) var<uniform> id: f32;
@group(2) @binding(3) var<uniform> mask: array<vec3<f32>, 1000>;
@group(2) @binding(4) var<uniform> mask_len: i32;
@group(2) @binding(5) var<uniform> a: vec3<f32>;
@group(2) @binding(6) var<uniform> b: vec3<f32>;
@group(2) @binding(7) var<uniform> c: vec3<f32>;
@group(2) @binding(8) var<uniform> a_uv: vec2<f32>;
@group(2) @binding(9) var<uniform> b_uv: vec2<f32>;
@group(2) @binding(10) var<uniform> c_uv: vec2<f32>;
@group(2) @binding(11) var<uniform> uv_scalar: vec2<f32>;
@group(2) @binding(12) var<uniform> uv_offset: vec2<f32>;
@group(2) @binding(13) var<uniform> uv_rotation: f32;
@group(2) @binding(14) var<uniform> a_position: vec3<f32>;
@group(2) @binding(15) var<uniform> b_position: vec3<f32>;
@group(2) @binding(16) var<uniform> c_position: vec3<f32>;
@group(2) @binding(17) var<uniform> pitch: f32;
@group(2) @binding(18) var<uniform> selected: u32;

// Compute barycentric coordinates (b1, b2, b3) for pixel p with respect to triangle (a, b, c)
fn barycentric(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>, c: vec2<f32>) -> vec3<f32>{
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;

    let d00 = dot(v0, v0);
    let d01 = dot(v0, v1);
    let d11 = dot(v1, v1);
    let d20 = dot(v2, v0);
    let d21 = dot(v2, v1);

    let epsilon = 100.0;
    let denom = d00 * d11 - d01 * d01 + epsilon;
    let b2 = (d11 * d20 - d01 * d21) / denom;
    let b3 = (d00 * d21 - d01 * d20) / denom;
    let b1 = 1.0 - b2 - b3;

    return vec3<f32>(b1, b2, b3);
}

// Calculate the shortest distance yet rendered at pixel p
fn shortest_distance(p: vec2<f32>) -> vec2<f32> {
    var shortest_distance = 0x1.fffffep+127;
    var id = -1.0;

    for (var i = 0; i < mask_len + 7; i += 7) {
        let current_id = mask[i];
        var a1_position = mask[i + 1];
        let a1 = mask[i + 2];
        var b1_position = mask[i + 3];
        let b1 = mask[i + 4];
        var c1_position = mask[i + 5];
        let c1 = mask[i + 6];

        let min_x = min(a1[0], min(b1[0], c1[0]));
        let max_x = max(a1[0], max(b1[0], c1[0]));
        let min_y = min(a1[1], min(b1[1], c1[1]));
        let max_y = max(a1[1], max(b1[1], c1[1]));

        if (p[0] >= min_x && p[0] <= max_x && p[1] >= min_y && p[1] <= max_y) {
            let bary: vec3<f32> = barycentric(p, vec2<f32>(a1[0], a1[1]), vec2<f32>(b1[0], b1[1]), vec2<f32>(c1[0], c1[1]));

            if 0.0 <= bary[1] && bary[1] <= 1.0 && 0.0 <= bary[2] && bary[2] <= 1.0 && (bary[1] + bary[2]) <= 1.0 {
                let z = bary[0] / a1[2] + bary[1] / b1[2] + bary[2] / c1[2];

                a1_position[1] = a1_position[1] - (pitch * a1_position[2]);
                b1_position[1] = b1_position[1] - (pitch * b1_position[2]);
                c1_position[1] = c1_position[1] - (pitch * c1_position[2]);

                let position: vec3<f32> = (bary[0] * a1_position) / a1[2] + (bary[1] * b1_position) / b1[2] + (bary[2] * c1_position) / c1[2];
                let corrected_position = vec3<f32>(position[0] / z, position[1] / z, position[2] / z);
                let distance = length(corrected_position);

                if distance < shortest_distance {
                    shortest_distance = distance;
                    id = current_id[0];
                }
            }
        }
    }

    let values: vec2<f32> = vec2<f32>(shortest_distance, id);
    return values;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate barycentric coordinates for p and apply UV correction
    let p: vec2<f32> = vec2<f32>(mesh.world_position[0], mesh.world_position[1]);
    let bary: vec3<f32> = barycentric(p, vec2<f32>(a[0], a[1]), vec2<f32>(b[0], b[1]), vec2<f32>(c[0], c[1]));
    let z = bary[0] / a[2] + bary[1] / b[2] + bary[2] / c[2];
    let uv: vec2<f32> = (bary[0] * a_uv) / a[2] + (bary[1] * b_uv) / b[2] + (bary[2] * c_uv) / c[2];
    let corrected_uv = vec2<f32>(uv[0] / z, uv[1] / z);

    // Calculate sine and cosine of the rotation angle
    let sin_rot = sin(uv_rotation * 3.14159265359 / 180.0);
    let cos_rot = cos(uv_rotation * 3.14159265359 / 180.0);

    // Rotate and normalize UV
    let rotated_uv = vec2<f32>(
        corrected_uv[0] * cos_rot - corrected_uv[1] * sin_rot, 
        corrected_uv[0] * sin_rot + corrected_uv[1] * cos_rot);

    // Transform UV
    let transformed_uv = vec2<f32>(
        ((rotated_uv[0] + uv_offset[0]) * uv_scalar[0]), 
        ((rotated_uv[1] + uv_offset[1]) * uv_scalar[1]));

    let normalized_uv = fract(transformed_uv);

    // Sample color from texture at calculated UV
    var sampled_color: vec4<f32> = textureSample(color_texture, color_sampler, normalized_uv);

    var a_pos = a_position;
    var b_pos = b_position;
    var c_pos = c_position;

    a_pos[1] = a_position[1] - (pitch * a_position[2]);
    b_pos[1] = b_position[1] - (pitch * b_position[2]);
    c_pos[1] = c_position[1] - (pitch * c_position[2]);

    let position: vec3<f32> = (bary[0] * a_pos) / a[2] + (bary[1] * b_pos) / b[2] + (bary[2] * c_pos) / c[2];
    let corrected_position = vec3<f32>(position[0] / z, position[1] / z, position[2] / z);

    let shortest_distance: vec2<f32> = shortest_distance(p);

    // if current distance > shortest distance and current id is not shortest distance's id
    if length(corrected_position) >= shortest_distance[0] && id != shortest_distance[1] {
        sampled_color[3] = 0.0;
    } else {
        let shadow = -0.0003 * length(corrected_position);
        sampled_color[0] += shadow;
        sampled_color[1] += shadow;
        sampled_color[2] += shadow;
    }

    if selected == 1 {
        sampled_color[0] += 0.3;
        sampled_color[1] += 0.3;
        sampled_color[2] += 0.3;
    }

    return sampled_color;
}

//https://computergraphics.stackexchange.com/questions/1866/how-to-map-square-texture-to-triangle
//https://stackoverflow.com/questions/12360023/barycentric-coordinates-texture-mapping