
struct Camera {
  view_projection: mat4x4<f32>,
  position: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

struct Vertex {
  @location(0) position: vec4<f32>,
  @location(1) color: vec4<f32>,
  @location(2) normal: vec4<f32>,
};

struct VertexPayload {
  @builtin(position) position: vec4<f32>,
  @location(0) color: vec4<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) world_position: vec3<f32>,
};

@vertex
fn vs_main(vertex: Vertex)-> VertexPayload {
  var out: VertexPayload;

  out.position=camera.view_projection*vertex.position;
  out.color=vertex.color;
  out.normal=vertex.normal.xyz;
  out.world_position=vertex.position.xyz;

  return out;
}

@fragment
fn fs_main(in: VertexPayload)-> @location(0) vec4<f32> {
  let normal=normalize(in.normal);

  let light_dir=normalize(vec3<f32>(0.45,0.70,0.55));
  let view_dir=normalize(camera.position.xyz-in.world_position);
  let half_dir=normalize(light_dir+view_dir);

  let diffuse=max(dot(normal,light_dir), 0.0);

  let specular=0.22*pow(
    max(dot(normal,half_dir),0.0),
    40.0,
  );

  let ambient=0.16;
  let illumination=ambient + (1.0-ambient)*diffuse;

  let rgb=in.color.rgb*illumination + vec3<f32>(specular);

  return vec4<f32>(rgb, in.color.a);
}



