#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<storage, read> triangles: array<mat3x2<f32>>;
@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;
@group(2) @binding(3) var<uniform> indexs_len: u32; // CPU側から送る（三角形インデックス個数 = u16 要素数）
@group(2) @binding(4) var<storage, read> indexs: array<u32>;
fn cross2d(a: vec2<f32>, b: vec2<f32>) -> f32 {
    return a.x * b.y - a.y * b.x;
}

fn is_inside(v0: vec2<f32>, v1: vec2<f32>, v2: vec2<f32>, p: vec2<f32>) -> bool {
// バウンディングボックスチェック（早期リターン）
let min_x = min(v0.x, min(v1.x, v2.x));
let max_x = max(v0.x, max(v1.x, v2.x));
let min_y = min(v0.y, min(v1.y, v2.y));
let max_y = max(v0.y, max(v1.y, v2.y));
if p.x < min_x || p.x > max_x || p.y < min_y || p.y > max_y {
    return false;
}


    // 2Dクロス積で内外判定（determinant代替）
    let e01 = v1 - v0;
    let e12 = v2 - v1;
    let e20 = v0 - v2;
    let p0 = p - v0;
    let p1 = p - v1;
    let p2 = p - v2;

    let c0 = cross2d(e01, p0);
    let c1 = cross2d(e12, p1);
    let c2 = cross2d(e20, p2);

    return (c0 >= 0.0 && c1 >= 0.0 && c2 >= 0.0) || (c0 <= 0.0 && c1 <= 0.0 && c2 <= 0.0);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let frag_coord = mesh.world_position.xy;

    let texture_color = textureSample(texture, texture_sampler, mesh.uv);

    if texture_color.a == 0.0 {
        discard;
    }


    var inside = false;
    // indexs_len は “indexs 配列中の要素数(u16)” を表す
    for (var i = 0u; i < indexs_len; i = i + 1u) {
        let tri_index = indexs[i];
        let tri = triangles[tri_index];
        if is_inside(tri[0], tri[1], tri[2], frag_coord) {
            inside = true;
            break;
        }
    }

    if !inside {
        discard;
    }
    return texture_color;
}
