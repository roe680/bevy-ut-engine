#import bevy_sprite::mesh2d_vertex_output::FullscreenVertexOutput
@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<storage, read> division_slicers: array<Slice>;
@group(0) @binding(3) var<uniform> len: u32;

const pi = radians(180.0); // 3.14159... がコンパイル時に代入される

struct Slice {
    translate: vec2<f32>,
    rotation: f32,
    left: f32,
    right: f32,
}
@fragment
fn fragment(point: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var new_point: vec2<f32> = point.uv;

    for (var i: u32 = 0u; i < len; i = i + 1u) {
        let s = division_slicers[i];

        if s.rotation % pi == 0.0 {
            let pattern = (s.rotation % (2 * pi)) == 0.0;
            if new_point.x < s.translate.x {
                new_point.y = new_point.y + select(s.left, s.right, pattern);
            } else {
                new_point.y = new_point.y + select(s.right, s.left, pattern);
            }
        } else {
            let pattern = s.rotation > pi;
            let a = tan(s.rotation);
            let b = s.translate.y - a * s.translate.x;
            let sin_r = sin(s.rotation);
            let cos_r = cos(s.rotation);
            if new_point.y > new_point.x * a + b {
                let add = select(s.left, s.right, pattern);
                new_point = new_point + vec2<f32>(cos_r * add, sin_r * add);
            } else {
                let add = select(s.right, s.left, pattern);
                new_point = new_point + vec2<f32>(cos_r * add, sin_r * add);
            }
        }
    }
    return textureSample(texture, texture_sampler, new_point);
}
