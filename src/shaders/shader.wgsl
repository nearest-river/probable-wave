struct Camera {
    view_projection: mat4x4<f32>,
    position: vec4<f32>,
};

struct Animation {
    time: f32,
    angular_velocity: f32,
    _padding: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(0) @binding(1)
var<uniform> animation: Animation;

struct Vertex {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) normal: vec4<f32>,
};

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

fn rotation_y(angle: f32) -> mat4x4<f32> {
    let c = cos(angle);
    let s = sin(angle);

    return mat4x4<f32>(
        vec4<f32>( c, 0.0, -s, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>( s, 0.0,  c, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
}

@vertex
fn vs_main(vertex: Vertex) -> VertexOut {
    var out: VertexOut;

    let angle = animation.time * animation.angular_velocity;
    let rotation = rotation_y(angle);

    out.position = camera.view_projection * rotation * vertex.position;
    out.color = vertex.color;

    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return in.color;
}



