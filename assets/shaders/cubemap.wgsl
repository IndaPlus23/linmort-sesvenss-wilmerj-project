#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var px_texture: texture_2d<f32>;
@group(2) @binding(1) var px_sampler: sampler;
@group(2) @binding(2) var nx_texture: texture_2d<f32>;
@group(2) @binding(3) var nx_sampler: sampler;
@group(2) @binding(4) var py_texture: texture_2d<f32>;
@group(2) @binding(5) var py_sampler: sampler;
@group(2) @binding(6) var ny_texture: texture_2d<f32>;
@group(2) @binding(7) var ny_sampler: sampler;
@group(2) @binding(8) var pz_texture: texture_2d<f32>;
@group(2) @binding(9) var pz_sampler: sampler;
@group(2) @binding(10) var nz_texture: texture_2d<f32>;
@group(2) @binding(11) var nz_sampler: sampler;
@group(2) @binding(12) var<uniform> window_width: f32;
@group(2) @binding(13) var<uniform> window_height: f32;
@group(2) @binding(14) var<uniform> direction: vec3<f32>;
@group(2) @binding(15) var<uniform> horizontal_vector: vec3<f32>;
@group(2) @binding(16) var<uniform> vertical_vector: vec3<f32>;
@group(2) @binding(17) var<uniform> mask: array<vec3<f32>, 1000>;
@group(2) @binding(18) var<uniform> mask_len: i32;

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

fn is_behind_structure(p: vec2<f32>) -> bool {
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
                return true;
            }
        }
    }

    return false;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let p: vec2<f32> = vec2<f32>(mesh.world_position[0], mesh.world_position[1]);
    let is_behind = is_behind_structure(p);

    if is_behind == true {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    let abs_horizontal: vec3<f32> = (mesh.world_position[0] * 0.9 / (window_width) * horizontal_vector);
    let abs_vertical: vec3<f32> = (mesh.world_position[1] * 0.9 / (window_height) * vertical_vector);
    let abs_direction: vec3<f32> = normalize(direction + abs_horizontal + abs_vertical);

    let x = abs_direction[0];
    let y = abs_direction[1];
    let z = abs_direction[2];

    var isXPositive = 0;
    if (x > 0) {
        isXPositive = 1;
    }

    var isYPositive = 0;
    if (y > 0) {
        isYPositive = 1;
    }

    var isZPositive = 0;
    if (z > 0) {
        isZPositive = 1;
    }

    var absX = abs(x);
    var absY = abs(y);
    var absZ = abs(z);

    var index = 0;
    var maxAxis = 0.0;
    var uc = 0.0;
    var vc = 0.0;

    // POSITIVE X
    if (isXPositive == 1 && absX >= absY && absX >= absZ) {
        // u (0 to 1) goes from +z to -z
        // v (0 to 1) goes from -y to +y
        maxAxis = absX;
        uc = -z;
        vc = -y;
        index = 0;
    }
    // NEGATIVE X
    if (isXPositive == 0 && absX >= absY && absX >= absZ) {
        // u (0 to 1) goes from -z to +z
        // v (0 to 1) goes from -y to +y
        maxAxis = absX;
        uc = z;
        vc = -y;
        index = 1;
    }
    // POSITIVE Y
    if (isYPositive == 1 && absY >= absX && absY >= absZ) {
        // u (0 to 1) goes from -x to +x
        // v (0 to 1) goes from +z to -z
        maxAxis = absY;
        uc = x;
        vc = z;
        index = 2;
    }
    // NEGATIVE Y
    if (isYPositive == 0 && absY >= absX && absY >= absZ) {
        // u (0 to 1) goes from -x to +x
        // v (0 to 1) goes from -z to +z
        maxAxis = absY;
        uc = x;
        vc = -z;
        index = 3;
    }
    // POSITIVE Z
    if (isZPositive == 1 && absZ >= absX && absZ >= absY) {
        // u (0 to 1) goes from -x to +x
        // v (0 to 1) goes from -y to +y
        maxAxis = absZ;
        uc = x;
        vc = -y;
        index = 4;
    }
    // NEGATIVE Z
    if (isZPositive == 0 && absZ >= absX && absZ >= absY) {
        // u (0 to 1) goes from +x to -x
        // v (0 to 1) goes from -y to +y
        maxAxis = absZ;
        uc = -x;
        vc = -y;
        index = 5;
    }

    // Convert range from -1 to 1 to 0 to 1
    let u = 0.5 * (uc / maxAxis + 1.0);
    let v = 0.5 * (vc / maxAxis + 1.0);
    let uv = vec2<f32>(u, v);

    if index == 0 {
        return textureSample(px_texture, px_sampler, uv);
    }
    if index == 1 {
        return textureSample(nx_texture, nx_sampler, uv);
    }
    if index == 2 {
        return textureSample(py_texture, py_sampler, uv);
    }
    if index == 3 {
        return textureSample(ny_texture, ny_sampler, uv);
    }
    if index == 4 {
        return textureSample(pz_texture, pz_sampler, uv);
    }
    else {
        return textureSample(nz_texture, nz_sampler, uv);
    }
}