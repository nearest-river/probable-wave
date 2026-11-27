
use wgpu::*;
use glam::Mat4;
use super::bind_group::*;

use core::{
  cmp,
  mem,
};




pub struct UniformBuf {
  pub buf: Buffer,
  pub bind_groups: Vec<BindGroup>,
  align: u64,
}

impl UniformBuf {
  #[inline]
  pub fn new(dev: &Device,capacity: usize,layout: BindGroupLayout)-> Self {
    let lims=dev.limits();
    let align=cmp::max(lims.min_uniform_buffer_offset_alignment as usize,mem::size_of::<Mat4>()) as u64;
    let size=capacity as u64 * align;

    let desc=BufferDescriptor {
      size,
      label: Some("uniform buffer object"),
      usage: BufferUsages::UNIFORM|BufferUsages::COPY_DST,
      mapped_at_creation: false,
    };
    let buf=dev.create_buffer(&desc);

    let mut bind_groups=Vec::<BindGroup>::with_capacity(capacity);
    for i in 0..capacity as u64 {
      let bind_group=BindGroupBuilder::new()
      .with_label("uniform buf bind group")
      .with_layout(&layout)
      .with_buffer(&buf,align*i)
      .build(dev);

      bind_groups.push(bind_group);
    }

    Self {
      buf,
      align,
      bind_groups,
    }
  }

  pub fn upload(&mut self,idx: u64,transformation: Mat4,queue: &Queue) {
    let offset=idx*self.align;
    let data=bytemuck::bytes_of(&transformation);

    queue.write_buffer(&self.buf,offset,data);
  }


}


























