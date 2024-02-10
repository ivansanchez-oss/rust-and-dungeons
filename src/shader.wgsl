struct Output {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32) -> Output {

    var pos = array<vec2<f32>,6>(
        vec2<f32>(-0.5,-0.5),
        vec2<f32>(0.5,-0.5),
        vec2<f32>(-0.5,0.5),
        vec2<f32>(-0.5,0.5),
        vec2<f32>(0.5,-0.5),
        vec2<f32>(0.5,0.5),
    );

    var out: Output;
    out.position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
 
 