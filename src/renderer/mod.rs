
mod mesh;
mod pipeline;
mod bind_group;

use glam::UVec2;
use mesh::BufHeader;
use crate::prelude::*;
pub use pipeline::PipelineBuilder;

pub use bind_group::{
  BindGroupBuilder,
  BindGroupLayoutBuilder,
};

use std::{
  mem,
  iter,
  sync::Arc,
};

pub use mesh::{
  Mesh,
  Vertex,
  MeshBuilder,
};

use winit::{
  window::Window,
  dpi::PhysicalSize,
};

use wgpu::{
  Queue,
  Device,
  LoadOp,
  StoreOp,
  Surface,
  Instance,
  Operations,
  IndexFormat,
  RenderPipeline,
  PowerPreference,
  DeviceDescriptor,
  RenderPassDescriptor,
  SurfaceConfiguration,
  CurrentSurfaceTexture,
  TextureViewDescriptor,
  CommandEncoderDescriptor,
  RenderPassColorAttachment,
  RequestAdapterOptionsBase,
};


pub struct Renderer {
  pub(crate) dev: Device,
  pub(crate) queue: Queue,
  clear_color: Color,
  window: Arc<Window>,
  surface: Surface<'static>,
  render_pipeline: RenderPipeline,
  surface_config: SurfaceConfiguration,
}

static SHADER_SOURCE: &str=include_str!("../shaders/hello_world.wgsl");


impl Renderer {
  pub async fn new(window: Arc<Window>,clear_color: Color)-> Self {
    let inner_size=window.inner_size();
    let instance=Instance::default();

    let surface=instance.create_surface(window.clone())
    .expect("failed to create surface");

    let adapter_descriptor=RequestAdapterOptionsBase {
      power_preference: PowerPreference::HighPerformance,
      compatible_surface: Some(&surface),
      ..Default::default()
    };
    let adapter=instance.request_adapter(&adapter_descriptor)
    .await
    .expect("no compatible GPU adapter");

    let dev_descriptor=DeviceDescriptor {
      label: Some("device"),
      ..Default::default()
    };
    let (dev,queue)=adapter.request_device(&dev_descriptor)
    .await
    .expect("failed to request a device");

    let surface_config=surface.get_default_config(&adapter, inner_size.width, inner_size.height)
    .expect("surface is incompatible with adapter");
    surface.configure(&dev,&surface_config);

    let render_pipeline=PipelineBuilder::new_with_shader(SHADER_SOURCE,"vs_main","fs_main")
    .with_pixel_format(surface_config.format)
    .add_vertex_buffer_layout(Vertex::layout())
    .build(&dev);

    Self {
      dev,
      queue,
      window,
      surface,
      clear_color,
      surface_config,
      render_pipeline,
    }
  }

  #[inline(always)]
  pub const fn surface_size(&self)-> UVec2 {
    UVec2::new(self.surface_config.width,self.surface_config.height)
  }

  #[inline(always)]
  pub const fn clear_color(&self)-> Color {
    self.clear_color
  }

  pub(crate) fn render(&self,mesh: &Mesh) {
    let surface_texture=match self.surface.get_current_texture() {
      CurrentSurfaceTexture::Success(frame)=> frame,
      CurrentSurfaceTexture::Suboptimal(frame)=> {
        self.surface.configure(&self.dev,&self.surface_config);
        frame
      }
      CurrentSurfaceTexture::Lost|CurrentSurfaceTexture::Outdated=> {
        self.surface.configure(&self.dev,&self.surface_config);
        return;
      }
      CurrentSurfaceTexture::Timeout|CurrentSurfaceTexture::Occluded|CurrentSurfaceTexture::Validation=> return,
    };

    let texture_view_desc=TextureViewDescriptor::default();
    let texture_view=surface_texture.texture.create_view(&texture_view_desc);

    let command_encoder_desc=CommandEncoderDescriptor {
      label: Some("render encoder"),
    };

    let mut command_encoder=self.dev.create_command_encoder(&command_encoder_desc);

    let color_attachment=RenderPassColorAttachment {
      view: &texture_view,
      ops: Operations {
        store: StoreOp::Store,
        load: LoadOp::Clear(self.clear_color.into_wgpu_color()),
      },
      resolve_target: None,
      depth_slice: None,
    };

    {
      let render_pass_desc=RenderPassDescriptor {
        label: Some("render pass"),
        color_attachments: &[Some(color_attachment)],
        ..Default::default()
      };

      let mut render_pass=command_encoder.begin_render_pass(&render_pass_desc);
      render_pass.set_pipeline(&self.render_pipeline);

      let width=self.surface_config.width as f32;
      let height=self.surface_config.height as f32;
      let size=width.min(height);
      render_pass.set_viewport(
        (width-size) * 0.5,
        (height-size) * 0.5,
        size,
        size,
        0.0,
        1.0
      );

      for header in &mesh.headers {
        match header {
          &BufHeader::VertexBuf { offset, size }=> {
            let vertex_bounds=offset..offset+size as u64;
            let vertices=0..size/mem::size_of::<Vertex>() as u32;

            render_pass.set_vertex_buffer(0,mesh.vertex_buf.slice(vertex_bounds));
            render_pass.draw(vertices,0..1);
          },
          &BufHeader::IndexedBuf { ix_offset, vx_offset, ix_size, vx_size }=> {
            let idx_bounds=ix_offset..ix_offset+ix_size as u64;
            let vertex_bounds=vx_offset..vx_offset+vx_size as u64;
            let indices=0..ix_size/mem::size_of::<u32>() as u32;

            render_pass.set_index_buffer(mesh.index_buf.slice(idx_bounds),IndexFormat::Uint32);
            render_pass.set_vertex_buffer(0,mesh.vertex_buf.slice(vertex_bounds));
            render_pass.draw_indexed(indices,0,0..1);
          },
        }
      }
    }


    self.queue.submit(iter::once(command_encoder.finish()));

    self.window.pre_present_notify();
    self.queue.present(surface_texture);
  }

  pub(crate) fn resize(&mut self,size: PhysicalSize<u32>) {
    if size.width==0 || size.height==0 {
      return;
    }

    self.surface_config.width=size.width;
    self.surface_config.height=size.height;

    self.surface.configure(&self.dev,&self.surface_config);
  }
}













