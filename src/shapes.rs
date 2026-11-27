
pub fn make_quad(dev: &Device)-> Mesh {
  const VERTICES: [(IVec2,u32);4]=[
    (ivec2(-1,-1),0xff0000),
    (ivec2(1,-1),0x0000ff),
    (ivec2(1,1),0x0000ff),
    (ivec2(-1,1),0xff0000),
  ];


  MeshBuilder::new()
  .insert_indexed_buf_with(|indices,vertices| {
    indices.extend([0,1,2,2,3,0u32]);

    vertices.extend(VERTICES.map(|(pos,color)| {
      let pos=pos.as_vec2().extend(0.0).extend(1.0);
      let color=Rgb::from_u32(color).into_color();
      Vertex::new(pos,color)
    }));
  })
  .build(dev)
}

