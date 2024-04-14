#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var color_texture: texture_2d<f32>;
@group(2) @binding(1) var color_sampler: sampler;
@group(2) @binding(2) var<uniform> a: vec3<f32>;
@group(2) @binding(3) var<uniform> b: vec3<f32>;
@group(2) @binding(4) var<uniform> c: vec3<f32>;
@group(2) @binding(5) var<uniform> a_uv: vec2<f32>;
@group(2) @binding(6) var<uniform> b_uv: vec2<f32>;
@group(2) @binding(7) var<uniform> c_uv: vec2<f32>;

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

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let p: vec2<f32> = vec2<f32>(mesh.world_position[0], mesh.world_position[1]);
    let bary: vec3<f32> = barycentric(p, vec2<f32>(a[0], a[1]), vec2<f32>(b[0], b[1]), vec2<f32>(c[0], c[1]));

    let z = bary[0] / a[2] + bary[1] / b[2] + bary[2] / c[2];
    let uv: vec2<f32> = (bary[0] * a_uv) / a[2] + (bary[1] * b_uv) / b[2] + (bary[2] * c_uv) / c[2];
    let modified_uv = vec2<f32>(uv[0] / z, uv[1] / z);

    //https://computergraphics.stackexchange.com/questions/1866/how-to-map-square-texture-to-triangle
    //https://stackoverflow.com/questions/12360023/barycentric-coordinates-texture-mapping

    return textureSample(color_texture, color_sampler, modified_uv);
}
