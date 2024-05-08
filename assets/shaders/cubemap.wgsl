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

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let horizontal_step = mesh.world_position[0] / window_width;
    let abs_horizontal: vec3<f32> = normalize(direction + horizontal_step * horizontal_vector);

    let vertical_step = mesh.world_position[1] / window_height;
    let abs_vertical: vec3<f32> = normalize(direction + vertical_step * vertical_vector);

    let abs_direction: vec3<f32> = normalize(abs_horizontal + abs_vertical);

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