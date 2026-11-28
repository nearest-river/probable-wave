
use glam::*;
use core::mem;
use crate::prelude::*;

use bytemuck::{
  Pod,
  Zeroable,
};


use wgpu::{
  Buffer,
  Device,
  BufferUsages,
  VertexFormat,
  VertexAttribute,
  VertexBufferLayout,
  util::{
    DeviceExt,
    BufferInitDescriptor,
  },
};

pub struct Mesh {
  pub(crate) index_buf: Buffer,
  pub(crate) vertex_buf: Buffer,
  pub(crate) index_buf_size: u64,
  pub(crate) vertex_buf_size: u64,
  pub(crate) headers: Vec<BufHeader>,
}

#[derive(Debug)]
pub struct MeshBuilder {
  indices: Vec<u32>,
  vertices: Vec<Vertex>,
  headers: Vec<BufHeader>,
}

#[derive(Debug,Clone)]
pub(crate) enum BufHeader {
  VertexBuf {
    offset: u64,
    size: u32,
  },
  IndexedBuf {
    ix_offset: u64,
    vx_offset: u64,
    ix_size: u32,
    vx_size: u32,
  },
}



#[repr(C)]
#[derive(Clone,Copy,Pod,Zeroable,Debug)]
pub struct Vertex {
  pub position: Vec4,
  pub color: Color,
  pub normal: Vec4,
}


impl Vertex {
  #[inline(always)]
  pub const fn new(position: Vec4,color: Color)-> Self {
    Self {
      position,
      color,
      normal: vec4(0.0,0.0,1.0,0.0),
    }
  }

  #[inline(always)]
  pub const fn with_normal(position: Vec4,color: Color,normal: Vec3)-> Self {
    Self {
      position,
      color,
      normal: vec4(normal.x,normal.y,normal.z,0.0),
    }
  }

  #[inline(always)]
  pub const fn layout()-> VertexBufferLayout<'static> {
    static ATTRS: &[VertexAttribute]={
      let fields=[VertexFormat::Float32x4,VertexFormat::Float32x4,VertexFormat::Float32x4];
      assert!(vertex_attr_array_size(&fields)==mem::size_of::<Vertex>());

      &vertex_attr_array(&fields)
    };

    VertexBufferLayout {
      array_stride: mem::size_of::<Vertex>() as _,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: ATTRS
    }
  }
}

impl Mesh {
  #[inline(always)]
  pub const fn index_buf_size(&self)-> u64 {
    self.index_buf_size
  }

  #[inline(always)]
  pub const fn vertex_buf_size(&self)-> u64 {
    self.vertex_buf_size
  }
}


impl MeshBuilder {
  #[inline(always)]
  pub const fn new()-> Self {
    Self {
      headers: Vec::new(),
      indices: Vec::new(),
      vertices: Vec::new(),
    }
  }

  #[inline(always)]
  fn vertex_offset(&self)-> u64 {
    bytemuck::cast_slice::<_,u8>(&self.vertices).len() as u64
  }

  #[inline(always)]
  fn index_offset(&self)-> u64 {
    bytemuck::cast_slice::<_,u8>(&self.indices).len() as u64
  }

  #[inline]
  pub fn insert_vertex_buf_with(&mut self,f: impl Fn(&mut Vec<Vertex>))-> &mut Self {
    let offset=self.vertex_offset();

    f(&mut self.vertices);

    let size=(self.vertex_offset() - offset) as u32;
    let header=BufHeader::VertexBuf {
      offset,
      size,
    };

    self.headers.push(header);

    self
  }

  pub fn insert_indexed_buf_with(&mut self,f: impl Fn(&mut Vec<u32>,&mut Vec<Vertex>))-> &mut Self {
    let ix_offset=self.index_offset();
    let vx_offset=self.vertex_offset();

    f(&mut self.indices,&mut self.vertices);

    let ix_size=(self.index_offset() - ix_offset) as u32;
    let vx_size=(self.vertex_offset() - vx_offset) as u32;

    let header=BufHeader::IndexedBuf {
      ix_offset,
      vx_offset,
      ix_size,
      vx_size,
    };
    self.headers.push(header);

    self
  }

  pub fn build(&self,dev: &Device)-> Mesh {
    let indices=bytemuck::cast_slice(&self.indices);
    let vertices=bytemuck::cast_slice(&self.vertices);

    let index_buf=dev.create_buffer_init(&BufferInitDescriptor {
      contents: indices,
      label: Some("index buf"),
      usage: BufferUsages::INDEX,
    });

    let vertex_buf=dev.create_buffer_init(&BufferInitDescriptor {
      contents: vertices,
      label: Some("vertex buf"),
      usage: BufferUsages::VERTEX,
    });

    Mesh {
      index_buf,
      vertex_buf,
      index_buf_size: indices.len() as u64,
      vertex_buf_size: vertices.len() as u64,
      headers: self.headers.clone(),
    }
  }
}



#[inline(always)]
const fn vertex_attr_array<const N: usize>(field_array: &[VertexFormat;N])-> [VertexAttribute;N] {
  const DUMMY: VertexAttribute=VertexAttribute { offset: 0, shader_location: 0, format: VertexFormat::Uint8 };
  let mut attrs=[DUMMY;N];

  let mut i=0usize;
  let mut offset=0u64;
  while i<N {
    attrs[i]=VertexAttribute {
      offset,
      format: field_array[i],
      shader_location: i as u32,
    };

    offset+=field_array[i].size();
    i+=1;
  }

  attrs
}

#[inline(always)]
const fn vertex_attr_array_size<const N: usize>(field_array: &[VertexFormat;N])-> usize {
  let mut sum=0usize;
  let mut i=0usize;
  while i<field_array.len() {
    sum+=field_array[i].size() as usize;
    i+=1;
  }

  sum
}









