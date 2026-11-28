
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
  animation_buf: Buffer,
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

#[repr(C,align(16))]
#[derive(Clone,Copy,Pod,Zeroable)]
struct AnimationUniform {
  time: f32,
  angular_velocity: f32,
  _padding: [f32; 2],
}

const CAMERA_FOV: f32=120f32.to_radians(); // 90deg
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

    let animation_uniform=AnimationUniform {
      time: 0.0,
      angular_velocity: 0.5,
      _padding: [0.0; 2]
    };

    let animation_buf=dev.create_buffer_init(&BufferInitDescriptor {
      label: Some("animation uniform buffer"),
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
      contents: bytemuck::bytes_of(&animation_uniform),
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
        BindGroupLayoutEntry {
          binding: 1,
          visibility: ShaderStages::VERTEX,
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
        BindGroupEntry {
          binding: 1,
          resource: animation_buf.as_entire_binding(),
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
      animation_buf,
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
  pub const fn surface_format(&self)-> TextureFormat {
    self.surface_config.format
  }

  #[inline(always)]
  pub const fn surface_size(&self)-> UVec2 {
    UVec2::new(self.surface_config.width,self.surface_config.height)
  }

  #[inline(always)]
  pub const fn clear_color(&self)-> Color {
    self.clear_color
  }

  pub(crate) fn render(
    &self,
    mesh: &Mesh,
    time: f32,
    egui_renderer: &mut egui_wgpu::Renderer,
    paint_jobs: &[egui::ClippedPrimitive],
    textures_delta: &egui::TexturesDelta,
) {
    let animation = AnimationUniform {
      time,
      angular_velocity: 0.5,
      _padding: [0.0; 2]
    };

    self.queue.write_buffer(
      &self.animation_buf,
      0,
      bytemuck::bytes_of(&animation),
    );

    let surface_texture = match self.surface.get_current_texture() {
        CurrentSurfaceTexture::Success(frame) => frame,
        CurrentSurfaceTexture::Suboptimal(frame) => {
            self.surface.configure(&self.dev, &self.surface_config);
            frame
        }
        CurrentSurfaceTexture::Lost | CurrentSurfaceTexture::Outdated => {
            self.surface.configure(&self.dev, &self.surface_config);
            return;
        }
        CurrentSurfaceTexture::Timeout
        | CurrentSurfaceTexture::Occluded
        | CurrentSurfaceTexture::Validation => return,
    };

    let texture_view = surface_texture
        .texture
        .create_view(&TextureViewDescriptor::default());

    let mut encoder = self.dev.create_command_encoder(
        &CommandEncoderDescriptor {
            label: Some("render encoder"),
        },
    );

    let screen = egui_wgpu::ScreenDescriptor {
        size_in_pixels: [
            self.surface_config.width,
            self.surface_config.height,
        ],
        pixels_per_point: self.window.scale_factor() as f32,
    };
for (id, deltas) in &textures_delta.set {
    for delta in deltas {
        egui_renderer.update_texture(
            &self.dev,
            &self.queue,
            *id,
            delta,
        );
    }
}

    let callback_buffers = egui_renderer.update_buffers(
        &self.dev,
        &self.queue,
        &mut encoder,
        paint_jobs,
        &screen,
    );

    // 3D scene
    {
        let color_attachment = RenderPassColorAttachment {
            view: &texture_view,
            ops: Operations {
                store: StoreOp::Store,
                load: LoadOp::Clear(self.clear_color.into_wgpu_color()),
            },
            resolve_target: None,
            depth_slice: None,
        };

        let depth_attachment = RenderPassDepthStencilAttachment {
            view: &self.depth_view,
            depth_ops: Some(Operations {
                load: LoadOp::Clear(1.0),
                store: StoreOp::Store,
            }),
            stencil_ops: None,
        };

        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("3D render pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: Some(depth_attachment),
            ..Default::default()
        });

        pass.set_pipeline(&self.render_pipeline);
        pass.set_bind_group(0, &self.camera_bind_group, &[]);

        let width = self.surface_config.width as f32;
        let height = self.surface_config.height as f32;
        let size = width.min(height);

        pass.set_viewport(
            (width - size) * 0.5,
            (height - size) * 0.5,
            size,
            size,
            0.0,
            1.0,
        );

        for header in &mesh.headers {
            match header {
                &BufHeader::VertexBuf { offset, size } => {
                    pass.set_vertex_buffer(
                        0,
                        mesh.vertex_buf.slice(offset..offset + size as u64),
                    );
                    pass.draw(
                        0..size / mem::size_of::<Vertex>() as u32,
                        0..1,
                    );
                }
                &BufHeader::IndexedBuf {
                    ix_offset,
                    vx_offset,
                    ix_size,
                    vx_size,
                } => {
                    pass.set_index_buffer(
                        mesh.index_buf.slice(ix_offset..ix_offset + ix_size as u64),
                        IndexFormat::Uint32,
                    );
                    pass.set_vertex_buffer(
                        0,
                        mesh.vertex_buf.slice(vx_offset..vx_offset + vx_size as u64),
                    );
                    pass.draw_indexed(
                        0..ix_size / mem::size_of::<u32>() as u32,
                        0,
                        0..1,
                    );
                }
            }
        }
    }

    // egui overlay
    {
        let attachment = RenderPassColorAttachment {
            view: &texture_view,
            ops: Operations {
                load: LoadOp::Load,
                store: StoreOp::Store,
            },
            resolve_target: None,
            depth_slice: None,
        };

        let pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("egui render pass"),
            color_attachments: &[Some(attachment)],
            depth_stencil_attachment: None,
            ..Default::default()
        });

        egui_renderer.render(
            &mut pass.forget_lifetime(),
            paint_jobs,
            &screen,
        );
    }

    self.queue.submit(
        callback_buffers
            .into_iter()
            .chain(std::iter::once(encoder.finish())),
    );

    for id in &textures_delta.free {
        egui_renderer.free_texture(id);
    }

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
      distance: 5.0,
      target: Vec3::ZERO,
    }
  }
}









