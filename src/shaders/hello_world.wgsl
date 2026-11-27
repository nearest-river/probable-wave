

struct Vertex {
  @location(0) pos: vec4<f32>,
  @location(1) color: vec4<f32>,
}


struct VertexPayload {
  @builtin(position) pos: vec4<f32>,
  @location(0) color: vec4<f32>,
}


@vertex
fn vs_main(vertex: Vertex)-> VertexPayload {
  var out: VertexPayload;
  out.pos=vertex.pos;
  out.color=vertex.color;

  return out;
}

@fragment
fn fs_main(in: VertexPayload)-> @location(0) vec4<f32> {
  let rotate=mat2x2<f32>(
    vec2(0,-1),
    vec2(1,0),
  );
  let mirror=mat2x2<f32>(
    vec2(-1,0),
    vec2(0,1),
  );

  return in.color;
}





