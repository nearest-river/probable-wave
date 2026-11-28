
mod mesh;
mod pipeline;
mod bind_group;

use mesh::BufHeader;
use crate::style::Color;
pub use pipeline::PipelineBuilder;

use bytemuck::{
  Pod,
  Zeroable,
};

use winit::{
  window::Window,
  dpi::PhysicalSize,
};

pub use bind_group::{
  BindGroupBuilder,
  BindGroupLayoutBuilder,
};

pub use mesh::{
  Mesh,
  Vertex,
  MeshBuilder,
};

use std::{
  mem,
  iter,
  sync::Arc,
  f32::consts as f32,
};

use wgpu::{
  *,
  util::{
    DeviceExt,
    BufferInitDescriptor,
  }
};

use glam::{
  prelude::*,
  camera::rh::{
    view as view_rh,
    proj::opengl as proj_rh,
  },
};



pub struct Renderer {
  pub(crate) dev: Device,
  pub(crate) queue: Queue,
  clear_color: Color,
  window: Arc<Window>,
  surface: Surface<'static>,
  render_pipeline: RenderPipeline,
  surface_config: SurfaceConfiguration,

  _depth_texture: Texture,
  depth_view: TextureView,

  camera: OrbitCamera,
  camera_buf: Buffer,
  camera_bind_group: BindGroup,
}

#[derive(Clone,Copy,Debug)]
struct OrbitCamera {
  target: Vec3,
  yaw: f32,
  pitch: f32,
  distance: f32,
}

#[repr(C,align(16))]
#[derive(Clone,Copy,Pod,Zeroable)]
struct CameraUniform {
  view_projection: Mat4,
  position: Vec4,
}

const CAMERA_FOV: f32=f32::FRAC_PI_2; // 90deg
const CAMERA_NEAR: f32=0.01;
const CAMERA_FAR: f32=100.0;



static SHADER_SOURCE: &str=include_str!("../shaders/shader.wgsl");

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

    let camera=OrbitCamera::default();
    let camera_uniform=camera.uniform();

    let camera_buf=dev.create_buffer_init(&BufferInitDescriptor {
      label: Some("camera uniform buffer"),
      usage: BufferUsages::UNIFORM|BufferUsages::COPY_DST,
      contents: bytemuck::bytes_of(&camera_uniform),
    });

    let camera_bind_group_layout=dev.create_bind_group_layout(&BindGroupLayoutDescriptor {
      label: Some("camera bind group layout"),
      entries: &[
        BindGroupLayoutEntry {
          binding: 0,
          visibility: ShaderStages::VERTEX_FRAGMENT,
          ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        },
      ],
    });

    let camera_bind_group=dev.create_bind_group(&BindGroupDescriptor {
      label: Some("camera bind group"),
      layout: &camera_bind_group_layout,
      entries: &[
        BindGroupEntry {
          binding: 0,
          resource: camera_buf.as_entire_binding(),
        },
      ],
    });


    let surface_config=surface.get_default_config(&adapter, inner_size.width, inner_size.height)
    .expect("surface is incompatible with adapter");
    surface.configure(&dev,&surface_config);

    let (_depth_texture, depth_view)=Self::create_depth_target(&dev,surface_config.width,surface_config.height);

    let render_pipeline=PipelineBuilder::new_with_shader(SHADER_SOURCE,"vs_main","fs_main")
    .with_pixel_format(surface_config.format)
    .with_depth_stencil(DepthStencilState {
      bias: DepthBiasState::default(),
      depth_write_enabled: Some(true),
      stencil: StencilState::default(),
      format: TextureFormat::Depth32Float,
      depth_compare: Some(CompareFunction::Less),
    })
    .add_bind_group_layout(&camera_bind_group_layout)
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
      _depth_texture,
      depth_view,
      camera_buf,
      camera,
      camera_bind_group,
    }
  }

  fn create_depth_target(dev: &Device,width: u32,height: u32)-> (Texture,TextureView) {
    let texture=dev.create_texture(&TextureDescriptor {
      label: Some("depth texture"),
      size: Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: TextureDimension::D2,
      format: TextureFormat::Depth32Float,
      usage: TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
    });

    let view=texture.create_view(
      &TextureViewDescriptor::default()
    );

    (texture,view)
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
      let depth_stencil_attachment=RenderPassDepthStencilAttachment {
        stencil_ops: None,
        view: &self.depth_view,
        depth_ops: Some(Operations {
          load: LoadOp::Clear(1.0),
          store: StoreOp::Store,
        }),
      };

      let render_pass_desc=RenderPassDescriptor {
        label: Some("render pass"),
        color_attachments: &[Some(color_attachment)],
        depth_stencil_attachment: Some(depth_stencil_attachment),
        ..Default::default()
      };

      let mut render_pass=command_encoder.begin_render_pass(&render_pass_desc);
      render_pass.set_pipeline(&self.render_pipeline);
      render_pass.set_bind_group(0,&self.camera_bind_group,&[]);

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
    let (depth_texture,depth_view)=Self::create_depth_target(&self.dev,self.surface_config.width,self.surface_config.height);

    self._depth_texture=depth_texture;
    self.depth_view=depth_view;
  }

  #[inline]
  fn update_camera_buf(&self) {
    let uniform=self.camera.uniform();
    self.queue.write_buffer(&self.camera_buf,0,bytemuck::bytes_of(&uniform));
  }

  #[inline]
  pub(crate) fn orbit_camera(&mut self,dx: f32,dy: f32) {
    const SENSITIVITY: f32=0.005;
    const MAX_PITCH: f32=1.553;

    self.camera.yaw-=dx*SENSITIVITY;
    self.camera.pitch=(self.camera.pitch - dy*SENSITIVITY).clamp(-MAX_PITCH,MAX_PITCH);
    self.update_camera_buf();
  }

  #[inline]
  pub(crate) fn zoom_camera(&mut self,amount: f32) {
    self.camera.distance*=f32::exp(-amount*0.12);
    self.camera.distance=self.camera.distance.clamp(0.1,100.0);

    self.update_camera_buf();
  }
}



impl OrbitCamera {
  #[inline]
  fn position(&self)-> Vec3 {
    let (sin_yaw,cos_yaw)=f32::sin_cos(self.yaw);
    let (sin_pitch,cos_pitch)=f32::sin_cos(self.pitch);

    self.target + self.distance*vec3(sin_yaw*cos_pitch,sin_pitch,cos_yaw*cos_pitch)
  }

  #[inline]
  fn view_projection(&self)-> Mat4 {
    let position=self.position();

    let view=view_rh::look_at_mat4(position,self.target,Vec3::Y);
    let projection=proj_rh::perspective(CAMERA_FOV,1.0,CAMERA_NEAR,CAMERA_FAR);

    projection*view
  }

  #[inline(always)]
  fn uniform(&self)-> CameraUniform {
    CameraUniform {
      view_projection: self.view_projection(),
      position: self.position().extend(1.0),
    }
  }
}



impl Default for OrbitCamera {
  #[inline(always)]
  fn default()-> Self {
    Self {
      yaw: 0.0,
      pitch: 0.0,
      distance: 10.0,
      target: Vec3::ZERO,
    }
  }
}









