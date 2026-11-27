#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
pub mod renderer {
    mod mesh {
        use glam::*;
        use core::mem;
        use crate::prelude::*;
        use bytemuck::{Pod, Zeroable};
        use wgpu::{
            Queue, Buffer, Device, BufferUsages, VertexFormat, VertexAttribute,
            VertexBufferLayout, util::{DeviceExt, BufferInitDescriptor},
        };
        pub struct Mesh {
            pub(crate) index_buf: Buffer,
            pub(crate) vertex_buf: Buffer,
            pub(crate) index_buf_size: u64,
            pub(crate) vertex_buf_size: u64,
            pub(crate) headers: Vec<BufHeader>,
        }
        pub struct MeshBuilder {
            indices: Vec<u32>,
            vertices: Vec<Vertex>,
            headers: Vec<BufHeader>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MeshBuilder {
            #[inline]
            #[rustc_trivial_field_reads]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "MeshBuilder indices vertices headers",
                    &self.indices,
                    &self.vertices,
                    &&self.headers,
                )
            }
        }
        pub(crate) enum BufHeader {
            VertexBuf { offset: u64, size: u32 },
            IndexedBuf { ix_offset: u64, vx_offset: u64, ix_size: u32, vx_size: u32 },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BufHeader {
            #[inline]
            #[rustc_trivial_field_reads]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Self::VertexBuf { offset: __self_0, size: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "VertexBuf offset size",
                            __self_0,
                            &__self_1,
                        )
                    }
                    Self::IndexedBuf {
                        ix_offset: __self_0,
                        vx_offset: __self_1,
                        ix_size: __self_2,
                        vx_size: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "IndexedBuf ix_offset vx_offset ix_size vx_size",
                            __self_0,
                            __self_1,
                            __self_2,
                            &__self_3,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for BufHeader {
            #[inline]
            #[rustc_trivial_field_reads]
            fn clone(&self) -> Self {
                match self {
                    Self::VertexBuf { offset: __self_0, size: __self_1 } => {
                        BufHeader::VertexBuf {
                            offset: ::core::clone::Clone::clone(__self_0),
                            size: ::core::clone::Clone::clone(__self_1),
                        }
                    }
                    Self::IndexedBuf {
                        ix_offset: __self_0,
                        vx_offset: __self_1,
                        ix_size: __self_2,
                        vx_size: __self_3,
                    } => {
                        BufHeader::IndexedBuf {
                            ix_offset: ::core::clone::Clone::clone(__self_0),
                            vx_offset: ::core::clone::Clone::clone(__self_1),
                            ix_size: ::core::clone::Clone::clone(__self_2),
                            vx_size: ::core::clone::Clone::clone(__self_3),
                        }
                    }
                }
            }
        }
        #[repr(C)]
        pub struct Vertex {
            pub position: Vec4,
            pub color: Color,
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for Vertex {}
        #[automatically_derived]
        impl ::core::clone::Clone for Vertex {
            #[inline]
            #[rustc_trivial_field_reads]
            fn clone(&self) -> Self {
                let _: ::core::clone::AssertParamIsClone<Vec4>;
                let _: ::core::clone::AssertParamIsClone<Color>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Vertex {}
        const _: () = {
            if !(::core::mem::size_of::<Vertex>()
                == (::core::mem::size_of::<Vec4>() + ::core::mem::size_of::<Color>()))
            {
                {
                    ::std::rt::begin_panic(
                        "derive(Pod) was applied to a type with padding",
                    );
                }
            }
        };
        const _: fn() = || {
            #[allow(clippy::missing_const_for_fn)]
            #[doc(hidden)]
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Vec4>();
            }
        };
        const _: fn() = || {
            #[allow(clippy::missing_const_for_fn)]
            #[doc(hidden)]
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Color>();
            }
        };
        unsafe impl ::bytemuck::Pod for Vertex {}
        const _: fn() = || {
            #[allow(clippy::missing_const_for_fn)]
            #[doc(hidden)]
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Vec4>();
            }
        };
        const _: fn() = || {
            #[allow(clippy::missing_const_for_fn)]
            #[doc(hidden)]
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Color>();
            }
        };
        unsafe impl ::bytemuck::Zeroable for Vertex {}
        #[automatically_derived]
        impl ::core::fmt::Debug for Vertex {
            #[inline]
            #[rustc_trivial_field_reads]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Vertex position color",
                    &self.position,
                    &&self.color,
                )
            }
        }
        impl Vertex {
            #[inline(always)]
            pub const fn new(position: Vec4, color: Color) -> Self {
                Self { position, color }
            }
            #[inline(always)]
            pub const fn layout() -> VertexBufferLayout<'static> {
                static ATTRS: &[VertexAttribute] = {
                    let fields = [VertexFormat::Float32x4, VertexFormat::Float32x4];
                    if !(vertex_attr_array_size(&fields) == mem::size_of::<Vertex>()) {
                        ::core::panicking::panic(
                            "assertion failed: vertex_attr_array_size(&fields) == mem::size_of::<Vertex>()",
                        )
                    }
                    &vertex_attr_array(&fields)
                };
                VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as _,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: ATTRS,
                }
            }
        }
        impl Mesh {
            #[inline(always)]
            pub const fn index_buf_size(&self) -> u64 {
                self.index_buf_size
            }
            #[inline(always)]
            pub const fn vertex_buf_size(&self) -> u64 {
                self.vertex_buf_size
            }
        }
        impl MeshBuilder {
            #[inline(always)]
            pub const fn new() -> Self {
                Self {
                    headers: Vec::new(),
                    indices: Vec::new(),
                    vertices: Vec::new(),
                }
            }
            #[inline(always)]
            fn vertex_offset(&self) -> u64 {
                bytemuck::cast_slice::<_, u8>(&self.vertices).len() as u64
            }
            #[inline(always)]
            fn index_offset(&self) -> u64 {
                bytemuck::cast_slice::<_, u8>(&self.indices).len() as u64
            }
            #[inline]
            pub fn insert_vertex_buf_with(
                &mut self,
                f: impl Fn(&mut Vec<Vertex>),
            ) -> &mut Self {
                let offset = self.vertex_offset();
                f(&mut self.vertices);
                let size = (self.vertex_offset() - offset) as u32;
                let header = BufHeader::VertexBuf {
                    offset,
                    size,
                };
                self.headers.push(header);
                self
            }
            pub fn insert_indexed_buf_with(
                &mut self,
                f: impl Fn(&mut Vec<u32>, &mut Vec<Vertex>),
            ) -> &mut Self {
                let ix_offset = self.index_offset();
                let vx_offset = self.vertex_offset();
                f(&mut self.indices, &mut self.vertices);
                let ix_size = (self.index_offset() - ix_offset) as u32;
                let vx_size = (self.vertex_offset() - vx_offset) as u32;
                let header = BufHeader::IndexedBuf {
                    ix_offset,
                    vx_offset,
                    ix_size,
                    vx_size,
                };
                self.headers.push(header);
                self
            }
            pub fn build(&self, dev: &Device) -> Mesh {
                let indices = bytemuck::cast_slice(&self.indices);
                let vertices = bytemuck::cast_slice(&self.vertices);
                let index_buf = dev
                    .create_buffer_init(
                        &BufferInitDescriptor {
                            contents: indices,
                            label: Some("index buf"),
                            usage: BufferUsages::INDEX,
                        },
                    );
                let vertex_buf = dev
                    .create_buffer_init(
                        &BufferInitDescriptor {
                            contents: vertices,
                            label: Some("vertex buf"),
                            usage: BufferUsages::VERTEX,
                        },
                    );
                Mesh {
                    index_buf,
                    vertex_buf,
                    index_buf_size: indices.len() as u64,
                    vertex_buf_size: vertices.len() as u64,
                    headers: self.headers.clone(),
                }
            }
        }
        pub fn make_quad(dev: &Device) -> Mesh {
            const VERTICES: [(IVec2, u32); 4] = [
                (ivec2(-1, -1), 0xff0000),
                (ivec2(1, -1), 0x0000ff),
                (ivec2(1, 1), 0x0000ff),
                (ivec2(-1, 1), 0xff0000),
            ];
            MeshBuilder::new()
                .insert_indexed_buf_with(|indices, vertices| {
                    indices.extend([0, 1, 2, 2, 3, 0u32]);
                    vertices
                        .extend(
                            VERTICES
                                .map(|(pos, color)| {
                                    let pos = pos.as_vec2().extend(0.0).extend(1.0);
                                    let color = Rgb::from_u32(color).into_color();
                                    Vertex::new(pos, color)
                                }),
                        );
                })
                .build(dev)
        }
        #[inline(always)]
        const fn vertex_attr_array<const N: usize>(
            field_array: &[VertexFormat; N],
        ) -> [VertexAttribute; N] {
            const DUMMY: VertexAttribute = VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: VertexFormat::Uint8,
            };
            let mut attrs = [DUMMY; N];
            let mut i = 0usize;
            let mut offset = 0u64;
            while i < N {
                attrs[i] = VertexAttribute {
                    offset,
                    format: field_array[i],
                    shader_location: i as u32,
                };
                offset += field_array[i].size();
                i += 1;
            }
            attrs
        }
        #[inline(always)]
        const fn vertex_attr_array_size<const N: usize>(
            field_array: &[VertexFormat; N],
        ) -> usize {
            let mut sum = 0usize;
            let mut i = 0usize;
            while i < field_array.len() {
                sum += field_array[i].size() as usize;
                i += 1;
            }
            sum
        }
    }
    mod pipeline {
        use std::borrow::Cow;
        use wgpu::{
            Face, Device, FrontFace, BlendState, ColorWrites, PolygonMode, VertexState,
            ShaderSource, FragmentState, TextureFormat, PrimitiveState, RenderPipeline,
            BindGroupLayout, ColorTargetState, MultisampleState, PrimitiveTopology,
            VertexBufferLayout, ShaderModuleDescriptor, PipelineLayoutDescriptor,
            RenderPipelineDescriptor, PipelineCompilationOptions,
        };
        pub struct PipelineBuilder<'a> {
            shader_source: Option<&'a str>,
            vertex_entry: Option<&'a str>,
            fragment_entry: Option<&'a str>,
            px_fmt: TextureFormat,
            bind_group_layouts: Vec<Option<&'a BindGroupLayout>>,
            vertex_buffer_layouts: Vec<Option<VertexBufferLayout<'a>>>,
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for PipelineBuilder<'a> {
            #[inline]
            #[rustc_trivial_field_reads]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = "PipelineBuilder shader_source vertex_entry fragment_entry px_fmt bind_group_layouts vertex_buffer_layouts";
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.shader_source,
                    &self.vertex_entry,
                    &self.fragment_entry,
                    &self.px_fmt,
                    &self.bind_group_layouts,
                    &&self.vertex_buffer_layouts,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, names, values)
            }
        }
        impl<'a> PipelineBuilder<'a> {
            #[inline(always)]
            pub const fn new() -> Self {
                Self {
                    vertex_entry: None,
                    shader_source: None,
                    fragment_entry: None,
                    bind_group_layouts: Vec::new(),
                    px_fmt: TextureFormat::Rgba8Unorm,
                    vertex_buffer_layouts: Vec::new(),
                }
            }
            #[inline(always)]
            pub const fn new_with_shader(
                shader_source: &'a str,
                vertex_entry: &'a str,
                fragment_entry: &'a str,
            ) -> Self {
                let mut this = Self::new();
                this.with_shader(shader_source, vertex_entry, fragment_entry);
                this
            }
            #[inline(always)]
            pub fn add_vertex_buffer_layout(
                &mut self,
                layout: VertexBufferLayout<'a>,
            ) -> &mut Self {
                self.vertex_buffer_layouts.push(Some(layout));
                self
            }
            #[allow(unused)]
            #[inline(always)]
            pub fn add_vertex_buffer_layouts(
                &mut self,
                vertex_buffer_layouts: &[VertexBufferLayout<'a>],
            ) -> &mut Self {
                self.vertex_buffer_layouts.reserve(vertex_buffer_layouts.len());
                for layout in vertex_buffer_layouts {
                    self.vertex_buffer_layouts.push(Some(layout.clone()));
                }
                self
            }
            #[inline(always)]
            pub fn add_bind_group_layout(
                &mut self,
                layout: &'a BindGroupLayout,
            ) -> &mut Self {
                self.bind_group_layouts.push(Some(layout));
                self
            }
            #[allow(unused)]
            #[inline(always)]
            pub fn add_bind_group_layouts(
                &mut self,
                bind_group_layouts: &'a [BindGroupLayout],
            ) -> &mut Self {
                self.vertex_buffer_layouts.reserve(bind_group_layouts.len());
                for layout in bind_group_layouts {
                    self.bind_group_layouts.push(Some(layout));
                }
                self
            }
            #[inline(always)]
            pub const fn with_shader(
                &mut self,
                shader_source: &'a str,
                vertex_entry: &'a str,
                fragment_entry: &'a str,
            ) -> &mut Self {
                self.shader_source = Some(shader_source);
                self.vertex_entry = Some(vertex_entry);
                self.fragment_entry = Some(fragment_entry);
                self
            }
            #[inline(always)]
            pub const fn with_pixel_format(
                &mut self,
                px_fmt: TextureFormat,
            ) -> &mut Self {
                self.px_fmt = px_fmt;
                self
            }
            pub fn build(&self, dev: &Device) -> RenderPipeline {
                let shader_source = self
                    .shader_source
                    .expect("no shader source provided");
                let shader_module_desc = ShaderModuleDescriptor {
                    label: Some("shader module"),
                    source: ShaderSource::Wgsl(Cow::Borrowed(shader_source)),
                };
                let shader_module = dev.create_shader_module(shader_module_desc);
                let pipeline_layout_desc = PipelineLayoutDescriptor {
                    label: Some("render pipeline layout"),
                    bind_group_layouts: self.bind_group_layouts.as_slice(),
                    ..Default::default()
                };
                let pipeline_layout = dev.create_pipeline_layout(&pipeline_layout_desc);
                let render_targets = [
                    Some(ColorTargetState {
                        format: self.px_fmt,
                        write_mask: ColorWrites::ALL,
                        blend: Some(BlendState::REPLACE),
                    }),
                ];
                let render_pipeline_desc = RenderPipelineDescriptor {
                    cache: None,
                    depth_stencil: None,
                    multiview_mask: None,
                    label: Some("render pipeline"),
                    layout: Some(&pipeline_layout),
                    multisample: MultisampleState {
                        mask: !0,
                        count: 1,
                        alpha_to_coverage_enabled: false,
                    },
                    vertex: VertexState {
                        module: &shader_module,
                        entry_point: self.vertex_entry,
                        buffers: &self.vertex_buffer_layouts,
                        compilation_options: PipelineCompilationOptions::default(),
                    },
                    fragment: Some(FragmentState {
                        module: &shader_module,
                        targets: &render_targets,
                        entry_point: self.fragment_entry,
                        compilation_options: PipelineCompilationOptions::default(),
                    }),
                    primitive: PrimitiveState {
                        topology: PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: FrontFace::Ccw,
                        cull_mode: Some(Face::Back),
                        polygon_mode: PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                };
                dev.create_render_pipeline(&render_pipeline_desc)
            }
        }
        impl Default for PipelineBuilder<'_> {
            #[inline(always)]
            fn default() -> Self {
                Self::new()
            }
        }
    }
    mod bind_group {
        use wgpu::{
            Buffer, Device, Sampler, BindGroup, BindingType, TextureView, ShaderStages,
            BufferBinding, BindGroupEntry, BindGroupLayout, BindingResource,
            BufferBindingType, TextureSampleType, SamplerBindingType,
            BindGroupDescriptor, TextureViewDimension, BindGroupLayoutEntry,
            BindGroupLayoutDescriptor,
        };
        pub struct BindGroupBuilder<'a> {
            label: Option<&'a str>,
            entries: Vec<BindGroupEntry<'a>>,
            layout: Option<&'a BindGroupLayout>,
        }
        pub struct BindGroupLayoutBuilder<'a> {
            label: Option<&'a str>,
            entries: Vec<BindGroupLayoutEntry>,
        }
        impl<'a> BindGroupBuilder<'a> {
            #[inline(always)]
            pub const fn new() -> Self {
                Self {
                    label: None,
                    layout: None,
                    entries: Vec::new(),
                }
            }
            #[inline(always)]
            pub fn with_label(&mut self, label: &'a str) -> &mut Self {
                self.label = Some(label);
                self
            }
            #[inline(always)]
            pub fn with_layout(&mut self, layout: &'a BindGroupLayout) -> &mut Self {
                self.layout = Some(layout);
                self
            }
            const fn can_insert_material(&self) -> bool {
                self.entries.len() <= u32::MAX as usize - 1
            }
            #[inline]
            pub fn with_material(
                &mut self,
                texture_view: &'a TextureView,
                sampler: &'a Sampler,
            ) -> &mut Self {
                if !self.can_insert_material() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("maximum material count exceeded"),
                        );
                    }
                }
                self.entries
                    .push(BindGroupEntry {
                        binding: self.entries.len() as u32,
                        resource: BindingResource::TextureView(texture_view),
                    });
                self.entries
                    .push(BindGroupEntry {
                        binding: self.entries.len() as u32,
                        resource: BindingResource::Sampler(sampler),
                    });
                self
            }
            #[inline]
            pub fn with_buffer(&mut self, buffer: &'a Buffer, offset: u64) -> &mut Self {
                if !self.can_insert_material() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("maximum material count exceeded"),
                        );
                    }
                }
                self.entries
                    .push(BindGroupEntry {
                        binding: self.entries.len() as u32,
                        resource: BindingResource::Buffer(BufferBinding {
                            buffer,
                            offset,
                            size: None,
                        }),
                    });
                self
            }
            #[inline]
            pub fn build(&self, dev: &Device) -> BindGroup {
                let desc = BindGroupDescriptor {
                    label: self.label,
                    entries: &self.entries,
                    layout: self.layout.expect("a layout must be specified"),
                };
                dev.create_bind_group(&desc)
            }
        }
        impl<'a> BindGroupLayoutBuilder<'a> {
            #[inline(always)]
            pub const fn new() -> Self {
                Self {
                    label: None,
                    entries: Vec::new(),
                }
            }
            #[inline(always)]
            pub fn with_label(&mut self, label: &'a str) -> &mut Self {
                self.label = Some(label);
                self
            }
            #[inline(always)]
            const fn can_insert_item(&self) -> bool {
                self.entries.len() <= u32::MAX as usize - 1
            }
            #[inline]
            pub fn with_uniform_buf(&mut self) -> &mut Self {
                if !self.can_insert_item() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("maximum material count exceeded"),
                        );
                    }
                }
                self.entries
                    .push(BindGroupLayoutEntry {
                        count: None,
                        visibility: ShaderStages::VERTEX,
                        binding: self.entries.len() as u32,
                        ty: BindingType::Buffer {
                            min_binding_size: None,
                            has_dynamic_offset: false,
                            ty: BufferBindingType::Uniform,
                        },
                    });
                self
            }
            #[inline]
            pub fn with_material(&mut self) -> &mut Self {
                if !self.can_insert_item() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("maximum material count exceeded"),
                        );
                    }
                }
                self.entries
                    .push(BindGroupLayoutEntry {
                        count: None,
                        binding: self.entries.len() as u32,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float {
                                filterable: true,
                            },
                        },
                    });
                self.entries
                    .push(BindGroupLayoutEntry {
                        count: None,
                        binding: self.entries.len() as u32,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    });
                self
            }
            pub fn build(&self, dev: &Device) -> BindGroupLayout {
                let desc = BindGroupLayoutDescriptor {
                    label: self.label,
                    entries: &self.entries,
                };
                dev.create_bind_group_layout(&desc)
            }
        }
    }
    use mesh::BufHeader;
    use crate::prelude::*;
    pub use pipeline::PipelineBuilder;
    pub use bind_group::{BindGroupBuilder, BindGroupLayoutBuilder};
    use std::{mem, iter, sync::Arc};
    pub use mesh::{Mesh, Vertex, MeshBuilder};
    use winit::{window::Window, dpi::PhysicalSize};
    use wgpu::{
        Queue, Device, LoadOp, StoreOp, Surface, Instance, Operations, IndexFormat,
        RenderPipeline, PowerPreference, DeviceDescriptor, RenderPassDescriptor,
        SurfaceConfiguration, CurrentSurfaceTexture, TextureViewDescriptor,
        CommandEncoderDescriptor, RenderPassColorAttachment, RequestAdapterOptionsBase,
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
    static SHADER_SOURCE: &str = "\n\nstruct Vertex {\n  @location(0) pos: vec4<f32>,\n  @location(1) color: vec4<f32>,\n}\n\n\nstruct VertexPayload {\n  @builtin(position) pos: vec4<f32>,\n  @location(0) color: vec4<f32>,\n}\n\n\n@vertex\nfn vs_main(vertex: Vertex)-> VertexPayload {\n  var out: VertexPayload;\n  out.pos=vertex.pos;\n  out.color=vertex.color;\n\n  return out;\n}\n\n@fragment\nfn fs_main(in: VertexPayload)-> @location(0) vec4<f32> {\n  let rotate=mat2x2<f32>(\n    vec2(0,-1),\n    vec2(1,0),\n  );\n  let mirror=mat2x2<f32>(\n    vec2(-1,0),\n    vec2(0,1),\n  );\n\n  return in.color;\n}\n\n\n\n\n\n";
    impl Renderer {
        pub async fn new(window: Arc<Window>, clear_color: Color) -> Self {
            let inner_size = window.inner_size();
            let instance = Instance::default();
            let surface = instance
                .create_surface(window.clone())
                .expect("failed to create surface");
            let adapter_descriptor = RequestAdapterOptionsBase {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                ..Default::default()
            };
            let adapter = instance
                .request_adapter(&adapter_descriptor)
                .await
                .expect("no compatible GPU adapter");
            let dev_descriptor = DeviceDescriptor {
                label: Some("device"),
                ..Default::default()
            };
            let (dev, queue) = adapter
                .request_device(&dev_descriptor)
                .await
                .expect("failed to request a device");
            let surface_config = surface
                .get_default_config(&adapter, inner_size.width, inner_size.height)
                .expect("surface is incompatible with adapter");
            surface.configure(&dev, &surface_config);
            let render_pipeline = PipelineBuilder::new_with_shader(
                    SHADER_SOURCE,
                    "vs_main",
                    "fs_main",
                )
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
        pub(crate) fn render(&self, mesh: &Mesh) {
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
            let texture_view_desc = TextureViewDescriptor::default();
            let texture_view = surface_texture.texture.create_view(&texture_view_desc);
            let command_encoder_desc = CommandEncoderDescriptor {
                label: Some("render encoder"),
            };
            let mut command_encoder = self
                .dev
                .create_command_encoder(&command_encoder_desc);
            let color_attachment = RenderPassColorAttachment {
                view: &texture_view,
                ops: Operations {
                    store: StoreOp::Store,
                    load: LoadOp::Clear(self.clear_color.into_wgpu_color()),
                },
                resolve_target: None,
                depth_slice: None,
            };
            {
                let render_pass_desc = RenderPassDescriptor {
                    label: Some("render pass"),
                    color_attachments: &[Some(color_attachment)],
                    ..Default::default()
                };
                let mut render_pass = command_encoder
                    .begin_render_pass(&render_pass_desc);
                render_pass.set_pipeline(&self.render_pipeline);
                for header in &mesh.headers {
                    match header {
                        &BufHeader::VertexBuf { offset, size } => {
                            let vertex_bounds = offset..offset + size as u64;
                            let vertices = 0..size / mem::size_of::<Vertex>() as u32;
                            render_pass
                                .set_vertex_buffer(0, mesh.vertex_buf.slice(vertex_bounds));
                            render_pass.draw(vertices, 0..1);
                        }
                        &BufHeader::IndexedBuf {
                            ix_offset,
                            vx_offset,
                            ix_size,
                            vx_size,
                        } => {
                            let idx_bounds = ix_offset..ix_offset + ix_size as u64;
                            let vertex_bounds = vx_offset..vx_offset + vx_size as u64;
                            let indices = 0..ix_size / mem::size_of::<u32>() as u32;
                            render_pass
                                .set_index_buffer(
                                    mesh.index_buf.slice(idx_bounds),
                                    IndexFormat::Uint32,
                                );
                            render_pass
                                .set_vertex_buffer(0, mesh.vertex_buf.slice(vertex_bounds));
                            render_pass.draw_indexed(indices, 0, 0..1);
                        }
                    }
                }
            }
            self.queue.submit(iter::once(command_encoder.finish()));
            self.window.pre_present_notify();
            self.queue.present(surface_texture);
        }
        pub(crate) fn resize(&mut self, size: PhysicalSize<u32>) {
            if size.width == 0 || size.height == 0 {
                return;
            }
            self.surface_config.width = size.width;
            self.surface_config.height = size.height;
            self.surface.configure(&self.dev, &self.surface_config);
        }
    }
}
pub mod model {
    use wgpu::Device;
    use glam::prelude::*;
    use std::{time::Instant, f32::consts as f32};
    use crate::{prelude::*, renderer::{Mesh, Vertex, Renderer, MeshBuilder}};
    pub struct Scene {
        mesh: Option<Mesh>,
        particles: Particles,
        time: Option<Instant>,
    }
    pub(crate) struct Particles {
        radii: Vec<f32>,
        charges: Vec<i32>,
        positions: Vec<Vec3>,
    }
    pub struct Particle {
        pub position: Vec3,
        pub charge: i32,
        pub radius: f32,
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for Particle {}
    #[automatically_derived]
    impl ::core::clone::Clone for Particle {
        #[inline]
        #[rustc_trivial_field_reads]
        fn clone(&self) -> Self {
            let _: ::core::clone::AssertParamIsClone<Vec3>;
            let _: ::core::clone::AssertParamIsClone<i32>;
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Particle {}
    impl Scene {
        pub fn new() -> Self {
            let particles = Particles::default();
            Self {
                particles,
                mesh: None,
                time: None,
            }
        }
        pub(crate) fn update<F: Fn(&mut Self, f32)>(
            &mut self,
            renderer: &Renderer,
            predicate: F,
        ) {
            let now = Instant::now();
            let dt = self
                .time
                .replace(now)
                .map_or(0.0, |t_0| now.duration_since(t_0).as_secs_f32());
            predicate(self, dt);
            self.mesh = Some(self.particles.gen_mesh(&renderer.dev));
        }
        pub(crate) fn render(&mut self, renderer: &mut Renderer) {
            if self.mesh.is_none() {
                self.mesh = Some(self.particles.gen_mesh(&renderer.dev));
            }
            let mesh = unsafe { self.mesh.as_ref().unwrap_unchecked() };
            renderer.render(mesh);
        }
    }
    impl Particles {
        fn from_vec(particles: Vec<Particle>) -> Self {
            let n = particles.len();
            let mut radii = Vec::<f32>::with_capacity(n);
            let mut charges = Vec::<i32>::with_capacity(n);
            let mut positions = Vec::<Vec3>::with_capacity(n);
            for Particle { radius, charge, position } in particles {
                radii.push(radius);
                charges.push(charge);
                positions.push(position);
            }
            Self { radii, charges, positions }
        }
        #[inline(always)]
        pub const fn count(&self) -> usize {
            self.charges.len()
        }
        fn gen_mesh(&mut self, dev: &Device) -> Mesh {
            let mut builder = MeshBuilder::new();
            for i in 0..self.count() {
                builder
                    .insert_indexed_buf_with(|indices, vertices| unsafe {
                        self.get_unchecked(i).draw(vertices, indices)
                    });
            }
            builder.build(dev)
        }
        unsafe fn get_unchecked(&self, idx: usize) -> Particle {
            unsafe {
                Particle {
                    radius: *self.radii.get_unchecked(idx),
                    charge: *self.charges.get_unchecked(idx),
                    position: *self.positions.get_unchecked(idx),
                }
            }
        }
    }
    impl Default for Particles {
        fn default() -> Self {
            Self::from_vec(
                ::alloc::boxed::box_assume_init_into_vec_unsafe(
                    ::alloc::intrinsics::write_box_via_move(
                        ::alloc::boxed::Box::new_uninit(),
                        [Particle::electron()],
                    ),
                ),
            )
        }
    }
    impl Particle {
        #[inline(always)]
        pub const fn electron() -> Self {
            Self {
                charge: -1,
                radius: 0.1,
                position: Vec3::ZERO,
            }
        }
        #[inline(always)]
        pub const fn proton() -> Self {
            Self {
                charge: 1,
                radius: 0.2,
                position: Vec3::ZERO,
            }
        }
        #[inline(always)]
        pub const fn neutron() -> Self {
            Self {
                charge: 0,
                radius: 0.2,
                position: Vec3::ZERO,
            }
        }
        #[inline(always)]
        pub const fn proton_at(position: Vec3) -> Self {
            Self {
                position,
                charge: 1,
                radius: 0.2,
            }
        }
        #[inline(always)]
        pub const fn electron_at(position: Vec3) -> Self {
            Self {
                position,
                charge: -1,
                radius: 0.1,
            }
        }
        #[inline(always)]
        pub const fn neutron_at(position: Vec3) -> Self {
            Self {
                position,
                charge: 0,
                radius: 0.2,
            }
        }
        fn draw(&self, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
            const SEGMENTS: u32 = 60;
            let centre = self.position;
            let color = self.color();
            vertices.push(Vertex::new(centre.extend(1.0), color));
            vertices.reserve(SEGMENTS as usize);
            for i in 0..SEGMENTS {
                let theta = f32::TAU * i as f32 / SEGMENTS as f32;
                let radius = self.radius * vec2(theta.cos(), theta.sin());
                let position = centre + radius.extend(0.0);
                vertices.push(Vertex::new(position.extend(1.0), color));
            }
            for i in 0..SEGMENTS {
                let current = i + 1;
                let next = (i + 1) % SEGMENTS + 1;
                indices.extend([0, current, next]);
            }
        }
        #[inline(always)]
        const fn color(&self) -> Color {
            match self.charge {
                1 => Color::RED,
                0 => Color::BLUE,
                -1 => Color::WHITE,
                _ => ::core::panicking::unreachable_default_msg(),
            }
        }
    }
}
pub mod style {
    mod srgb {
        use glam::*;
        use super::{Color, IntoColor};
        pub struct Rgba {
            pub r: u8,
            pub g: u8,
            pub b: u8,
            pub a: u8,
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for Rgba {}
        #[automatically_derived]
        impl ::core::clone::Clone for Rgba {
            #[inline]
            #[rustc_trivial_field_reads]
            fn clone(&self) -> Self {
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Rgba {}
        #[automatically_derived]
        impl ::core::fmt::Debug for Rgba {
            #[inline]
            #[rustc_trivial_field_reads]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Rgba r g b a",
                    &self.r,
                    &self.g,
                    &self.b,
                    &&self.a,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Rgba {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Rgba {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.r == other.r && self.g == other.g && self.b == other.b
                    && self.a == other.a
            }
        }
        pub struct Rgb {
            pub r: u8,
            pub g: u8,
            pub b: u8,
        }
        #[automatically_derived]
        #[doc(hidden)]
        unsafe impl ::core::clone::TrivialClone for Rgb {}
        #[automatically_derived]
        impl ::core::clone::Clone for Rgb {
            #[inline]
            #[rustc_trivial_field_reads]
            fn clone(&self) -> Self {
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Rgb {}
        #[automatically_derived]
        impl ::core::fmt::Debug for Rgb {
            #[inline]
            #[rustc_trivial_field_reads]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Rgb r g b",
                    &self.r,
                    &self.g,
                    &&self.b,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Rgb {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Rgb {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.r == other.r && self.g == other.g && self.b == other.b
            }
        }
        impl Rgba {
            #[inline(always)]
            pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
                Self { r, g, b, a }
            }
            #[inline(always)]
            pub const fn from_u32(color: u32) -> Self {
                Self::from_array(color.to_be_bytes())
            }
            #[inline(always)]
            pub const fn from_u8vec4(U8Vec4 { x, y, z, w }: U8Vec4) -> Self {
                Self::new(x, y, z, w)
            }
            #[inline(always)]
            pub const fn from_u8vec3(U8Vec3 { x, y, z }: U8Vec3) -> Self {
                Self::new(x, y, z, 0xff)
            }
            #[inline(always)]
            pub const fn from_array([r, g, b, a]: [u8; 4]) -> Self {
                Self::new(r, g, b, a)
            }
            #[inline(always)]
            pub const fn from_color(Color { r, g, b, a }: Color) -> Self {
                let r = (r * 255.0).round() as u8;
                let g = (g * 255.0).round() as u8;
                let b = (b * 255.0).round() as u8;
                let a = (a * 255.0).round() as u8;
                Self::new(r, g, b, a)
            }
            #[inline(always)]
            pub const fn as_color(self) -> Color {
                let r = self.r as f32 / 255.0;
                let g = self.g as f32 / 255.0;
                let b = self.b as f32 / 255.0;
                let a = self.a as f32 / 255.0;
                unsafe { Color::new_unchecked(r, g, b, a) }
            }
            #[inline(always)]
            pub const fn as_u8vec4(self) -> U8Vec4 {
                let Rgba { r, g, b, a } = self;
                U8Vec4::new(r, g, b, a)
            }
        }
        impl Rgb {
            #[inline(always)]
            pub const fn new(r: u8, g: u8, b: u8) -> Self {
                Self { r, g, b }
            }
            #[inline(always)]
            pub const fn from_u32(color: u32) -> Self {
                if !(color & !0xffffff == 0) {
                    ::core::panicking::panic("assertion failed: color & !0xffffff == 0")
                }
                let bytes = color.to_be_bytes();
                Self::new(bytes[1], bytes[2], bytes[3])
            }
            #[inline(always)]
            pub const fn from_u8vec3(U8Vec3 { x, y, z }: U8Vec3) -> Self {
                Self::new(x, y, z)
            }
            #[inline(always)]
            pub const fn from_array([r, g, b]: [u8; 3]) -> Self {
                Self::new(r, g, b)
            }
            #[inline(always)]
            pub const fn from_color(Color { r, g, b, a: _ }: Color) -> Self {
                let r = (r * 255.0).round() as u8;
                let g = (g * 255.0).round() as u8;
                let b = (b * 255.0).round() as u8;
                Self::new(r, g, b)
            }
            #[inline(always)]
            pub const fn as_color(self) -> Color {
                let r = self.r as f32 / 255.0;
                let g = self.g as f32 / 255.0;
                let b = self.b as f32 / 255.0;
                unsafe { Color::new_unchecked(r, g, b, 1.0) }
            }
            #[inline(always)]
            pub const fn as_u8vec3(self) -> U8Vec3 {
                let Rgb { r, g, b } = self;
                U8Vec3::new(r, g, b)
            }
        }
        impl IntoColor for Rgba {
            #[inline(always)]
            fn into_color(self) -> Color {
                self.as_color()
            }
        }
        impl IntoColor for Rgb {
            #[inline(always)]
            fn into_color(self) -> Color {
                self.as_color()
            }
        }
    }
    pub use srgb::*;
    use glam::prelude::*;
    use bytemuck::{Pod, Zeroable};
    use core::ops::{
        Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
    };
    pub struct Style {
        pub fill: Option<Color>,
        pub stroke: Option<Color>,
        pub stroke_width: f32,
        pub opacity: f32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Style {
        #[inline]
        #[rustc_trivial_field_reads]
        fn clone(&self) -> Self {
            Style {
                fill: ::core::clone::Clone::clone(&self.fill),
                stroke: ::core::clone::Clone::clone(&self.stroke),
                stroke_width: ::core::clone::Clone::clone(&self.stroke_width),
                opacity: ::core::clone::Clone::clone(&self.opacity),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Style {
        #[inline]
        #[rustc_trivial_field_reads]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Style fill stroke stroke_width opacity",
                &self.fill,
                &self.stroke,
                &self.stroke_width,
                &&self.opacity,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Style {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Style {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.stroke_width == other.stroke_width && self.opacity == other.opacity
                && self.fill == other.fill && self.stroke == other.stroke
        }
    }
    #[repr(C)]
    pub struct Color {
        pub r: f32,
        pub g: f32,
        pub b: f32,
        pub a: f32,
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for Color {}
    #[automatically_derived]
    impl ::core::clone::Clone for Color {
        #[inline]
        #[rustc_trivial_field_reads]
        fn clone(&self) -> Self {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Color {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Color {
        #[inline]
        #[rustc_trivial_field_reads]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Color r g b a",
                &self.r,
                &self.g,
                &self.b,
                &&self.a,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Color {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Color {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.r == other.r && self.g == other.g && self.b == other.b
                && self.a == other.a
        }
    }
    const _: () = {
        if !(::core::mem::size_of::<Color>()
            == (::core::mem::size_of::<f32>() + ::core::mem::size_of::<f32>()
                + ::core::mem::size_of::<f32>() + ::core::mem::size_of::<f32>()))
        {
            {
                ::std::rt::begin_panic("derive(Pod) was applied to a type with padding");
            }
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Pod>() {}
            assert_impl::<f32>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Pod>() {}
            assert_impl::<f32>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Pod>() {}
            assert_impl::<f32>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Pod>() {}
            assert_impl::<f32>();
        }
    };
    unsafe impl ::bytemuck::Pod for Color {}
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Zeroable>() {}
            assert_impl::<f32>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Zeroable>() {}
            assert_impl::<f32>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Zeroable>() {}
            assert_impl::<f32>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Zeroable>() {}
            assert_impl::<f32>();
        }
    };
    unsafe impl ::bytemuck::Zeroable for Color {}
    pub trait IntoColor: Sized {
        fn into_color(self) -> Color;
    }
    impl Default for Style {
        fn default() -> Self {
            Self {
                fill: None,
                stroke: Some(Color::BLACK),
                stroke_width: 0.025,
                opacity: 1.0,
            }
        }
    }
    impl Color {
        pub const BLACK: Self = color(0.0, 0.0, 0.0, 1.0);
        pub const WHITE: Self = color(1.0, 1.0, 1.0, 1.0);
        pub const TRANSPARENT: Self = color(0.0, 0.0, 0.0, 0.0);
        pub const RED: Self = color(1.0, 0.0, 0.0, 1.0);
        pub const GREEN: Self = color(0.0, 1.0, 0.0, 1.0);
        pub const BLUE: Self = color(0.0, 0.0, 1.0, 1.0);
        pub const CYAN: Self = color(0.0, 1.0, 1.0, 1.0);
        pub const MAGENTA: Self = color(1.0, 0.0, 1.0, 1.0);
        pub const YELLOW: Self = color(1.0, 1.0, 0.0, 1.0);
        #[inline(always)]
        pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
            if !(is_normalized(r) && is_normalized(g) && is_normalized(b)
                && is_normalized(a))
            {
                ::core::panicking::panic(
                    "assertion failed: is_normalized(r) && is_normalized(g) && is_normalized(b) && is_normalized(a)",
                )
            }
            Self { r, g, b, a }
        }
        #[inline(always)]
        pub const unsafe fn new_unchecked(r: f32, g: f32, b: f32, a: f32) -> Self {
            Self { r, g, b, a }
        }
        #[inline(always)]
        pub const fn from_array([r, g, b, a]: [f32; 4]) -> Self {
            Self::new(r, g, b, a)
        }
        #[inline(always)]
        pub fn from_vec4(v: Vec4) -> Self {
            Self::new(v.x, v.y, v.z, v.w)
        }
        #[inline(always)]
        pub fn from_vec3(v: Vec3) -> Self {
            Self::from_vec4(v.extend(1.0))
        }
        #[inline(always)]
        pub const fn as_vec4(self) -> Vec4 {
            let Self { r, g, b, a } = self;
            vec4(r, g, b, a)
        }
        #[inline(always)]
        pub const fn as_rgba(self) -> Rgba {
            Rgba::from_color(self)
        }
        #[inline(always)]
        pub const fn as_rgb(self) -> Rgb {
            Rgb::from_color(self)
        }
        #[inline(always)]
        pub(crate) const fn into_wgpu_color(self) -> wgpu::Color {
            let Color { r, g, b, a } = self;
            let r = r as f64;
            let g = g as f64;
            let b = b as f64;
            let a = a as f64;
            wgpu::Color { r, g, b, a }
        }
    }
    impl Default for Color {
        #[inline(always)]
        fn default() -> Self {
            Self::BLACK
        }
    }
    #[inline(always)]
    pub const fn color(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color::new(r, g, b, a)
    }
    impl Add for Color {
        type Output = Self;
        #[inline(always)]
        fn add(self, other: Self) -> Self::Output {
            Self::from_vec4(self.as_vec4().add(other.as_vec4()))
        }
    }
    impl Sub for Color {
        type Output = Self;
        #[inline(always)]
        fn sub(self, other: Self) -> Self::Output {
            Self::from_vec4(self.as_vec4().sub(other.as_vec4()))
        }
    }
    impl Mul for Color {
        type Output = Self;
        #[inline(always)]
        fn mul(self, other: Self) -> Self::Output {
            Self::from_vec4(self.as_vec4().mul(other.as_vec4()))
        }
    }
    impl Div for Color {
        type Output = Self;
        #[inline(always)]
        fn div(self, other: Self) -> Self::Output {
            Self::from_vec4(self.as_vec4().div(other.as_vec4()))
        }
    }
    impl Rem for Color {
        type Output = Self;
        #[inline(always)]
        fn rem(self, other: Self) -> Self::Output {
            Self::from_vec4(self.as_vec4().rem(other.as_vec4()))
        }
    }
    impl AddAssign for Color {
        #[inline(always)]
        fn add_assign(&mut self, other: Self) {
            let mut this_vec = self.as_vec4();
            this_vec.add_assign(other.as_vec4());
            *self = Self::from_vec4(this_vec);
        }
    }
    impl SubAssign for Color {
        #[inline(always)]
        fn sub_assign(&mut self, other: Self) {
            let mut this_vec = self.as_vec4();
            this_vec.sub_assign(other.as_vec4());
            *self = Self::from_vec4(this_vec);
        }
    }
    impl MulAssign for Color {
        #[inline(always)]
        fn mul_assign(&mut self, other: Self) {
            let mut this_vec = self.as_vec4();
            this_vec.mul_assign(other.as_vec4());
            *self = Self::from_vec4(this_vec);
        }
    }
    impl DivAssign for Color {
        #[inline(always)]
        fn div_assign(&mut self, other: Self) {
            let mut this_vec = self.as_vec4();
            this_vec.div_assign(other.as_vec4());
            *self = Self::from_vec4(this_vec);
        }
    }
    impl RemAssign for Color {
        #[inline(always)]
        fn rem_assign(&mut self, other: Self) {
            let mut this_vec = self.as_vec4();
            this_vec.rem_assign(other.as_vec4());
            *self = Self::from_vec4(this_vec);
        }
    }
    impl IntoColor for Color {
        #[inline(always)]
        fn into_color(self) -> Color {
            self
        }
    }
    impl Into<wgpu::Color> for Color {
        #[inline(always)]
        fn into(self) -> wgpu::Color {
            self.into_wgpu_color()
        }
    }
    #[inline(always)]
    const fn is_normalized(x: f32) -> bool {
        x.is_finite() && x >= 0.0 && x <= 1.0
    }
}
pub mod engine {
    use std::{mem, sync::Arc};
    use crate::{model::*, prelude::*, renderer::Renderer};
    use winit::{
        dpi::PhysicalSize, event::WindowEvent, application::ApplicationHandler,
        window::{Window, WindowId},
        event_loop::{EventLoop, ActiveEventLoop},
    };
    pub struct Engine {
        state: State,
        event_loop: EventLoop<()>,
    }
    impl Engine {
        #[inline]
        pub fn new() -> Self {
            let event_loop = EventLoop::new().unwrap();
            let state = State::default();
            Self { state, event_loop }
        }
        #[inline(always)]
        pub fn with_title(mut self, title: impl Into<String>) -> Self {
            self.state.title = title.into();
            self
        }
        #[inline(always)]
        pub fn with_clear_color(mut self, color: impl IntoColor) -> Self {
            self.state.clear_color = color.into_color();
            self
        }
        #[inline(always)]
        pub fn with_inner_size(mut self, inner_size: PhysicalSize<u32>) -> Self {
            self.state.inner_size = inner_size;
            self
        }
        #[inline]
        pub fn run(mut self) {
            self.event_loop.run_app(&mut self.state).unwrap();
        }
    }
    struct State {
        title: String,
        clear_color: Color,
        inner_size: PhysicalSize<u32>,
        renderer: Option<Renderer>,
        window: Option<Arc<Window>>,
        scene: Scene,
    }
    impl ApplicationHandler for State {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            if self.window.is_some() {
                return;
            }
            let window = Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title(mem::take(&mut self.title))
                            .with_inner_size(self.inner_size),
                    )
                    .expect("failed to create window"),
            );
            let state = pollster::block_on(
                Renderer::new(window.clone(), self.clear_color),
            );
            self.window = Some(window);
            self.renderer = Some(state);
        }
        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: WindowId,
            event: WindowEvent,
        ) {
            let Some(window) = &self.window else {
                return;
            };
            if window.id() != window_id {
                return;
            }
            match event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::Resized(size) => {
                    if let Some(renderer) = &mut self.renderer {
                        renderer.resize(size);
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Some(renderer) = &mut self.renderer {
                        self.scene.render(renderer);
                    }
                }
                _ => {}
            }
        }
        fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
    impl Default for State {
        #[inline(always)]
        fn default() -> Self {
            Self {
                window: None,
                renderer: None,
                scene: Scene::new(),
                title: "brush window".to_owned(),
                clear_color: Color::default(),
                inner_size: PhysicalSize::new(1280, 720),
            }
        }
    }
}
pub mod prelude {
    pub use crate::{style::*, model::Scene, engine::Engine, renderer::{Mesh, Vertex}};
}
