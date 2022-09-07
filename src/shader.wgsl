struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vert_idx: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(vert_idx)) * 0.5;
    let y = f32(i32(vert_idx & 1u) * 2 - 1) * 0.5;
    out.pos = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}