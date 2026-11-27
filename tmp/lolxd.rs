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
            Buffer, Device, BufferUsages, VertexFormat, VertexAttribute,
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
            pub normal: Vec4,
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
                == (::core::mem::size_of::<Vec4>() + ::core::mem::size_of::<Color>()
                    + ::core::mem::size_of::<Vec4>()))
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
        const _: fn() = || {
            #[allow(clippy::missing_const_for_fn)]
            #[doc(hidden)]
            fn check() {
                fn assert_impl<T: ::bytemuck::Pod>() {}
                assert_impl::<Vec4>();
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
        const _: fn() = || {
            #[allow(clippy::missing_const_for_fn)]
            #[doc(hidden)]
            fn check() {
                fn assert_impl<T: ::bytemuck::Zeroable>() {}
                assert_impl::<Vec4>();
            }
        };
        unsafe impl ::bytemuck::Zeroable for Vertex {}
        #[automatically_derived]
        impl ::core::fmt::Debug for Vertex {
            #[inline]
            #[rustc_trivial_field_reads]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Vertex position color normal",
                    &self.position,
                    &self.color,
                    &&self.normal,
                )
            }
        }
        impl Vertex {
            #[inline(always)]
            pub const fn new(position: Vec4, color: Color) -> Self {
                Self {
                    position,
                    color,
                    normal: vec4(0.0, 0.0, 1.0, 0.0),
                }
            }
            #[inline(always)]
            pub const fn with_normal(
                position: Vec4,
                color: Color,
                normal: Vec3,
            ) -> Self {
                Self {
                    position,
                    color,
                    normal: vec4(normal.x, normal.y, normal.z, 0.0),
                }
            }
            #[inline(always)]
            pub const fn layout() -> VertexBufferLayout<'static> {
                static ATTRS: &[VertexAttribute] = {
                    let fields = [
                        VertexFormat::Float32x4,
                        VertexFormat::Float32x4,
                        VertexFormat::Float32x4,
                    ];
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
        use wgpu::*;
        use std::borrow::Cow;
        pub struct PipelineBuilder<'a> {
            shader_source: Option<&'a str>,
            vertex_entry: Option<&'a str>,
            fragment_entry: Option<&'a str>,
            px_fmt: TextureFormat,
            depth_stencil: Option<DepthStencilState>,
            bind_group_layouts: Vec<Option<&'a BindGroupLayout>>,
            vertex_buffer_layouts: Vec<Option<VertexBufferLayout<'a>>>,
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for PipelineBuilder<'a> {
            #[inline]
            #[rustc_trivial_field_reads]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = "PipelineBuilder shader_source vertex_entry fragment_entry px_fmt depth_stencil bind_group_layouts vertex_buffer_layouts";
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.shader_source,
                    &self.vertex_entry,
                    &self.fragment_entry,
                    &self.px_fmt,
                    &self.depth_stencil,
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
                    depth_stencil: None,
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
            #[inline(always)]
            pub const fn with_depth_stencil(
                &mut self,
                depth_stencil: DepthStencilState,
            ) -> &mut Self {
                self.depth_stencil = Some(depth_stencil);
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
                    multiview_mask: None,
                    depth_stencil: self.depth_stencil.clone(),
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
    use crate::style::Color;
    pub use pipeline::PipelineBuilder;
    use bytemuck::{Pod, Zeroable};
    use winit::{window::Window, dpi::PhysicalSize};
    pub use bind_group::{BindGroupBuilder, BindGroupLayoutBuilder};
    pub use mesh::{Mesh, Vertex, MeshBuilder};
    use std::{mem, iter, sync::Arc, f32::consts as f32};
    use wgpu::{*, util::{DeviceExt, BufferInitDescriptor}};
    use glam::{prelude::*, camera::rh::{view as view_rh, proj::opengl as proj_rh}};
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
    struct OrbitCamera {
        target: Vec3,
        yaw: f32,
        pitch: f32,
        distance: f32,
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for OrbitCamera {}
    #[automatically_derived]
    impl ::core::clone::Clone for OrbitCamera {
        #[inline]
        #[rustc_trivial_field_reads]
        fn clone(&self) -> Self {
            let _: ::core::clone::AssertParamIsClone<Vec3>;
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for OrbitCamera {}
    #[automatically_derived]
    impl ::core::fmt::Debug for OrbitCamera {
        #[inline]
        #[rustc_trivial_field_reads]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "OrbitCamera target yaw pitch distance",
                &self.target,
                &self.yaw,
                &self.pitch,
                &&self.distance,
            )
        }
    }
    #[repr(C, align(16))]
    struct CameraUniform {
        view_projection: Mat4,
        position: Vec4,
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for CameraUniform {}
    #[automatically_derived]
    impl ::core::clone::Clone for CameraUniform {
        #[inline]
        #[rustc_trivial_field_reads]
        fn clone(&self) -> Self {
            let _: ::core::clone::AssertParamIsClone<Mat4>;
            let _: ::core::clone::AssertParamIsClone<Vec4>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CameraUniform {}
    const _: () = {
        if !(::core::mem::size_of::<CameraUniform>()
            == (::core::mem::size_of::<Mat4>() + ::core::mem::size_of::<Vec4>()))
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
            assert_impl::<Mat4>();
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
    unsafe impl ::bytemuck::Pod for CameraUniform {}
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Zeroable>() {}
            assert_impl::<Mat4>();
        }
    };
    const _: fn() = || {
        #[allow(clippy::missing_const_for_fn)]
        #[doc(hidden)]
        fn check() {
            fn assert_impl<T: ::bytemuck::Zeroable>() {}
            assert_impl::<Vec4>();
        }
    };
    unsafe impl ::bytemuck::Zeroable for CameraUniform {}
    const CAMERA_FOV: f32 = f32::FRAC_PI_2;
    const CAMERA_NEAR: f32 = 0.01;
    const CAMERA_FAR: f32 = 100.0;
    static SHADER_SOURCE: &str = "\nstruct Camera {\n  view_projection: mat4x4<f32>,\n  position: vec4<f32>,\n};\n\n@group(0) @binding(0)\nvar<uniform> camera: Camera;\n\nstruct Vertex {\n  @location(0) position: vec4<f32>,\n  @location(1) color: vec4<f32>,\n  @location(2) normal: vec4<f32>,\n};\n\nstruct VertexPayload {\n  @builtin(position) position: vec4<f32>,\n  @location(0) color: vec4<f32>,\n  @location(1) normal: vec3<f32>,\n  @location(2) world_position: vec3<f32>,\n};\n\n@vertex\nfn vs_main(vertex: Vertex)-> VertexPayload {\n  var out: VertexPayload;\n\n  out.position=camera.view_projection*vertex.position;\n  out.color=vertex.color;\n  out.normal=vertex.normal.xyz;\n  out.world_position=vertex.position.xyz;\n\n  return out;\n}\n\n@fragment\nfn fs_main(in: VertexPayload)-> @location(0) vec4<f32> {\n  let normal=normalize(in.normal);\n\n  let light_dir=normalize(vec3<f32>(0.45,0.70,0.55));\n  let view_dir=normalize(camera.position.xyz-in.world_position);\n  let half_dir=normalize(light_dir+view_dir);\n\n  let diffuse=max(dot(normal,light_dir), 0.0);\n\n  let specular=0.22*pow(\n    max(dot(normal,half_dir),0.0),\n    40.0,\n  );\n\n  let ambient=0.16;\n  let illumination=ambient + (1.0-ambient)*diffuse;\n\n  let rgb=in.color.rgb*illumination + vec3<f32>(specular);\n\n  return vec4<f32>(rgb, in.color.a);\n}\n\n\n\n";
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
            let camera = OrbitCamera::default();
            let camera_uniform = camera.uniform();
            let camera_buf = dev
                .create_buffer_init(
                    &BufferInitDescriptor {
                        label: Some("camera uniform buffer"),
                        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                        contents: bytemuck::bytes_of(&camera_uniform),
                    },
                );
            let camera_bind_group_layout = dev
                .create_bind_group_layout(
                    &BindGroupLayoutDescriptor {
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
                    },
                );
            let camera_bind_group = dev
                .create_bind_group(
                    &BindGroupDescriptor {
                        label: Some("camera bind group"),
                        layout: &camera_bind_group_layout,
                        entries: &[
                            BindGroupEntry {
                                binding: 0,
                                resource: camera_buf.as_entire_binding(),
                            },
                        ],
                    },
                );
            let surface_config = surface
                .get_default_config(&adapter, inner_size.width, inner_size.height)
                .expect("surface is incompatible with adapter");
            surface.configure(&dev, &surface_config);
            let (_depth_texture, depth_view) = Self::create_depth_target(
                &dev,
                surface_config.width,
                surface_config.height,
            );
            let render_pipeline = PipelineBuilder::new_with_shader(
                    SHADER_SOURCE,
                    "vs_main",
                    "fs_main",
                )
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
        fn create_depth_target(
            dev: &Device,
            width: u32,
            height: u32,
        ) -> (Texture, TextureView) {
            let texture = dev
                .create_texture(
                    &TextureDescriptor {
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
                    },
                );
            let view = texture.create_view(&TextureViewDescriptor::default());
            (texture, view)
        }
        #[inline(always)]
        pub const fn surface_size(&self) -> UVec2 {
            UVec2::new(self.surface_config.width, self.surface_config.height)
        }
        #[inline(always)]
        pub const fn clear_color(&self) -> Color {
            self.clear_color
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
                let depth_stencil_attachment = RenderPassDepthStencilAttachment {
                    stencil_ops: None,
                    view: &self.depth_view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                };
                let render_pass_desc = RenderPassDescriptor {
                    label: Some("render pass"),
                    color_attachments: &[Some(color_attachment)],
                    depth_stencil_attachment: Some(depth_stencil_attachment),
                    ..Default::default()
                };
                let mut render_pass = command_encoder
                    .begin_render_pass(&render_pass_desc);
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                let width = self.surface_config.width as f32;
                let height = self.surface_config.height as f32;
                let size = width.min(height);
                render_pass
                    .set_viewport(
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
            let (depth_texture, depth_view) = Self::create_depth_target(
                &self.dev,
                self.surface_config.width,
                self.surface_config.height,
            );
            self._depth_texture = depth_texture;
            self.depth_view = depth_view;
        }
        #[inline]
        fn update_camera_buf(&self) {
            let uniform = self.camera.uniform();
            self.queue.write_buffer(&self.camera_buf, 0, bytemuck::bytes_of(&uniform));
        }
        #[inline]
        pub(crate) fn orbit_camera(&mut self, dx: f32, dy: f32) {
            const SENSITIVITY: f32 = 0.005;
            const MAX_PITCH: f32 = 1.553;
            self.camera.yaw -= dx * SENSITIVITY;
            self.camera.pitch = (self.camera.pitch - dy * SENSITIVITY)
                .clamp(-MAX_PITCH, MAX_PITCH);
            self.update_camera_buf();
        }
        #[inline]
        pub(crate) fn zoom_camera(&mut self, amount: f32) {
            self.camera.distance *= f32::exp(-amount * 0.12);
            self.camera.distance = self.camera.distance.clamp(0.1, 100.0);
            self.update_camera_buf();
        }
    }
    impl OrbitCamera {
        #[inline]
        fn position(&self) -> Vec3 {
            let (sin_yaw, cos_yaw) = f32::sin_cos(self.yaw);
            let (sin_pitch, cos_pitch) = f32::sin_cos(self.pitch);
            self.target
                + self.distance
                    * vec3(sin_yaw * cos_pitch, sin_pitch, cos_yaw * cos_pitch)
        }
        #[inline]
        fn view_projection(&self) -> Mat4 {
            let position = self.position();
            let view = view_rh::look_at_mat4(position, self.target, Vec3::Y);
            let projection = proj_rh::perspective(
                CAMERA_FOV,
                1.0,
                CAMERA_NEAR,
                CAMERA_FAR,
            );
            projection * view
        }
        #[inline(always)]
        fn uniform(&self) -> CameraUniform {
            CameraUniform {
                view_projection: self.view_projection(),
                position: self.position().extend(1.0),
            }
        }
    }
    impl Default for OrbitCamera {
        #[inline(always)]
        fn default() -> Self {
            Self {
                yaw: 0.0,
                pitch: 0.0,
                distance: 2.0,
                target: Vec3::ZERO,
            }
        }
    }
}
pub mod model {
    mod particles {
        use wgpu::Device;
        use glam::prelude::*;
        use std::f32::consts as f32;
        use crate::{prelude::*, renderer::{Mesh, Vertex, MeshBuilder}};
        pub struct Particles {
            pub radii: Vec<f32>,
            pub charges: Vec<i32>,
            pub positions: Vec<Vec3A>,
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
        const ELECTRON_RADIUS: f32 = 0.05;
        const PROTON_RADIUS: f32 = 2.0 * ELECTRON_RADIUS;
        const NEUTRON_RADIUS: f32 = PROTON_RADIUS;
        impl Particle {
            #[inline(always)]
            pub const fn electron() -> Self {
                Self {
                    charge: -1,
                    position: Vec3::ZERO,
                    radius: ELECTRON_RADIUS,
                }
            }
            #[inline(always)]
            pub const fn proton() -> Self {
                Self {
                    charge: 1,
                    position: Vec3::ZERO,
                    radius: PROTON_RADIUS,
                }
            }
            #[inline(always)]
            pub const fn neutron() -> Self {
                Self {
                    charge: 0,
                    position: Vec3::ZERO,
                    radius: NEUTRON_RADIUS,
                }
            }
            #[inline(always)]
            pub const fn electron_at(position: Vec3) -> Self {
                Self {
                    position,
                    charge: -1,
                    radius: ELECTRON_RADIUS,
                }
            }
            #[inline(always)]
            pub const fn proton_at(position: Vec3) -> Self {
                Self {
                    position,
                    charge: 1,
                    radius: PROTON_RADIUS,
                }
            }
            #[inline(always)]
            pub const fn neutron_at(position: Vec3) -> Self {
                Self {
                    position,
                    charge: 0,
                    radius: NEUTRON_RADIUS,
                }
            }
            fn draw(&self, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
                const LATITUDE_SEGMENTS: u32 = 16;
                const LONGITUDE_SEGMENTS: u32 = 32;
                let color = self.color();
                vertices
                    .reserve(
                        (2 + (LATITUDE_SEGMENTS - 1) * LONGITUDE_SEGMENTS) as usize,
                    );
                indices
                    .reserve(
                        (6 * LONGITUDE_SEGMENTS * (LATITUDE_SEGMENTS - 1)) as usize,
                    );
                let mut push_vertex = |normal: Vec3| {
                    let world_position = self.position + self.radius * normal;
                    vertices
                        .push(
                            Vertex::with_normal(
                                world_position.extend(1.0),
                                color,
                                normal,
                            ),
                        );
                };
                push_vertex(Vec3::Y);
                for lat in 1..LATITUDE_SEGMENTS {
                    let phi = f32::PI * lat as f32 / LATITUDE_SEGMENTS as f32;
                    let (sin_phi, cos_phi) = f32::sin_cos(phi);
                    for lon in 0..LONGITUDE_SEGMENTS {
                        let theta = f32::TAU * lon as f32 / LONGITUDE_SEGMENTS as f32;
                        let (sin_theta, cos_theta) = f32::sin_cos(theta);
                        let normal = Vec3::new(
                            sin_phi * cos_theta,
                            cos_phi,
                            sin_phi * sin_theta,
                        );
                        push_vertex(normal);
                    }
                }
                let bottom = 1 + (LATITUDE_SEGMENTS - 1) * LONGITUDE_SEGMENTS;
                push_vertex(-Vec3::Y);
                for lon in 0..LONGITUDE_SEGMENTS {
                    let current = 1 + lon;
                    let next = 1 + (lon + 1) % LONGITUDE_SEGMENTS;
                    indices.extend([0, next, current]);
                }
                for lat in 0..(LATITUDE_SEGMENTS - 2) {
                    let ring_a = 1 + lat * LONGITUDE_SEGMENTS;
                    let ring_b = ring_a + LONGITUDE_SEGMENTS;
                    for lon in 0..LONGITUDE_SEGMENTS {
                        let next_lon = (lon + 1) % LONGITUDE_SEGMENTS;
                        let a = ring_a + lon;
                        let b = ring_b + lon;
                        let next_a = ring_a + next_lon;
                        let next_b = ring_b + next_lon;
                        indices.extend([a, next_a, b, next_a, next_b, b]);
                    }
                }
                let last_ring = 1 + (LATITUDE_SEGMENTS - 2) * LONGITUDE_SEGMENTS;
                for lon in 0..LONGITUDE_SEGMENTS {
                    let current = last_ring + lon;
                    let next = last_ring + (lon + 1) % LONGITUDE_SEGMENTS;
                    indices.extend([bottom, current, next]);
                }
            }
            #[inline(always)]
            const fn color(&self) -> Color {
                if self.charge.is_positive() {
                    Color::RED
                } else if self.charge.is_negative() {
                    Color::BLUE
                } else {
                    Color::WHITE
                }
            }
        }
        impl Particles {
            pub const fn new() -> Self {
                Self {
                    radii: Vec::new(),
                    charges: Vec::new(),
                    positions: Vec::new(),
                }
            }
            pub fn from_vec(particles: Vec<Particle>) -> Self {
                let n = particles.len();
                let mut radii = Vec::<f32>::with_capacity(n);
                let mut charges = Vec::<i32>::with_capacity(n);
                let mut positions = Vec::<Vec3A>::with_capacity(n);
                for Particle { radius, charge, position } in particles {
                    radii.push(radius);
                    charges.push(charge);
                    positions.push(position.to_vec3a());
                }
                Self { radii, charges, positions }
            }
            #[inline(always)]
            pub const fn count(&self) -> usize {
                self.charges.len()
            }
            #[inline(always)]
            pub fn insert(&mut self, Particle { position, charge, radius }: Particle) {
                self.radii.push(radius);
                self.charges.push(charge);
                self.positions.push(position.to_vec3a());
            }
            pub(crate) fn gen_mesh(&mut self, dev: &Device) -> Mesh {
                let mut builder = MeshBuilder::new();
                for i in 0..self.count() {
                    builder
                        .insert_indexed_buf_with(|indices, vertices| unsafe {
                            self.get_unchecked(i).draw(vertices, indices)
                        });
                }
                builder.build(dev)
            }
            pub unsafe fn get_unchecked(&self, idx: usize) -> Particle {
                unsafe {
                    Particle {
                        radius: *self.radii.get_unchecked(idx),
                        charge: *self.charges.get_unchecked(idx),
                        position: self.positions.get_unchecked(idx).to_vec3(),
                    }
                }
            }
        }
        impl From<Vec<Particle>> for Particles {
            #[inline(always)]
            fn from(particles: Vec<Particle>) -> Self {
                Self::from_vec(particles)
            }
        }
        impl Default for Particles {
            #[inline(always)]
            fn default() -> Self {
                Particles::new()
            }
        }
    }
    use std::time::Instant;
    use crate::renderer::{Mesh, Renderer};
    pub use particles::{Particle, Particles};
    pub struct Scene {
        mesh: Option<Mesh>,
        particles: Particles,
        time: Option<Instant>,
        time_0: Option<Instant>,
        on_tick: Option<Box<dyn Fn(Query<'_>)>>,
    }
    pub struct Query<'a> {
        pub time: f32,
        pub delta_time: f32,
        pub particles: &'a mut Particles,
    }
    impl Scene {
        pub fn new() -> Self {
            let particles = Particles::default();
            Self {
                particles,
                mesh: None,
                time: None,
                time_0: None,
                on_tick: None,
            }
        }
        #[inline(always)]
        pub fn from_particles(particles: impl Into<Particles>) -> Self {
            let particles = particles.into();
            Self {
                particles,
                mesh: None,
                time: None,
                time_0: None,
                on_tick: None,
            }
        }
        #[inline(always)]
        pub fn on_tick(&mut self, f: impl Fn(Query<'_>) + 'static) {
            self.on_tick = Some(Box::from(f));
        }
        pub(crate) fn render(&mut self, renderer: &mut Renderer) {
            let now = Instant::now();
            let dt = self
                .time
                .replace(now)
                .map_or(0.0, |t_0| now.duration_since(t_0).as_secs_f32());
            let time_0 = match self.time_0 {
                Some(time_0) => time_0,
                None => {
                    self.time_0 = Some(now);
                    now
                }
            };
            if let Some(on_tick) = self.on_tick.as_deref() {
                on_tick(Query {
                    delta_time: dt,
                    particles: &mut self.particles,
                    time: now.duration_since(time_0).as_secs_f32(),
                });
                self.update(renderer);
            }
            if self.mesh.is_none() {
                self.mesh = Some(self.particles.gen_mesh(&renderer.dev));
            }
            let mesh = unsafe { self.mesh.as_ref().unwrap_unchecked() };
            renderer.render(mesh);
        }
        #[inline]
        fn update(&mut self, renderer: &Renderer) {
            self.mesh = Some(self.particles.gen_mesh(&renderer.dev));
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
        dpi::PhysicalSize, application::ApplicationHandler, window::{Window, WindowId},
        event_loop::{EventLoop, ActiveEventLoop},
        event::{
            DeviceId, DeviceEvent, WindowEvent, MouseButton, ElementState,
            MouseScrollDelta,
        },
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
        pub fn with_scene(mut self, scene: Scene) -> Self {
            self.state.scene = scene;
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
        dragging: bool,
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
                WindowEvent::MouseInput { state, button: MouseButton::Left, .. } => {
                    self.dragging = state == ElementState::Pressed;
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let amount = match delta {
                        MouseScrollDelta::LineDelta(_, y) => y,
                        MouseScrollDelta::PixelDelta(position) => {
                            position.y as f32 / 120.0
                        }
                    };
                    if let Some(renderer) = &mut self.renderer {
                        renderer.zoom_camera(amount);
                    }
                }
                WindowEvent::Focused(false) => self.dragging = false,
                _ => {}
            }
        }
        fn device_event(
            &mut self,
            _event_loop: &ActiveEventLoop,
            _device_id: DeviceId,
            event: DeviceEvent,
        ) {
            if !self.dragging {
                return;
            }
            let DeviceEvent::MouseMotion { delta } = event else {
                return;
            };
            if let Some(renderer) = &mut self.renderer {
                renderer.orbit_camera(delta.0 as f32, delta.1 as f32);
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
                dragging: false,
                scene: Scene::new(),
                title: "brush window".to_owned(),
                clear_color: Color::default(),
                inner_size: PhysicalSize::new(1280, 720),
            }
        }
    }
}
pub mod physics {
    use std::f32::consts as f32;
    pub const PLANKS_CONSTANT: f32 = 6.62607015e-34;
    pub const REDUCED_PLANKS_CONSTANT: f32 = PLANKS_CONSTANT / f32::TAU;
    pub const E_0: f32 = -13.6;
    pub const W_0: f32 = E_0 / REDUCED_PLANKS_CONSTANT;
}
pub mod prelude {
    pub use crate::{
        physics, style::*, engine::Engine, renderer::{Mesh, Vertex},
        model::{Query, Scene, Particle, Particles},
    };
}
