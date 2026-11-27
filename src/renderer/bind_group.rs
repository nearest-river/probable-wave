
use wgpu::{
  Buffer,
  Device,
  Sampler,
  BindGroup,
  BindingType,
  TextureView,
  ShaderStages,
  BufferBinding,
  BindGroupEntry,
  BindGroupLayout,
  BindingResource,
  BufferBindingType,
  TextureSampleType,
  SamplerBindingType,
  BindGroupDescriptor,
  TextureViewDimension,
  BindGroupLayoutEntry,
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
  pub const fn new()-> Self {
    Self {
      label: None,
      layout: None,
      entries: Vec::new(),
    }
  }

  #[inline(always)]
  pub fn with_label(&mut self,label: &'a str)-> &mut Self {
    self.label=Some(label);
    self
  }

  #[inline(always)]
  pub fn with_layout(&mut self,layout: &'a BindGroupLayout)-> &mut Self {
    self.layout=Some(layout);
    self
  }

  const fn can_insert_material(&self)-> bool {
    self.entries.len() <= u32::MAX as usize - 1
  }

  #[inline]
  pub fn with_material(&mut self,texture_view: &'a TextureView,sampler: &'a Sampler)-> &mut Self {
    assert!(self.can_insert_material(),"maximum material count exceeded");

    self.entries.push(BindGroupEntry {
      binding: self.entries.len() as u32,
      resource: BindingResource::TextureView(texture_view),
    });

    self.entries.push(BindGroupEntry {
      binding: self.entries.len() as u32,
      resource: BindingResource::Sampler(sampler),
    });

    self
  }

  #[inline]
  pub fn with_buffer(&mut self,buffer: &'a Buffer,offset: u64)-> &mut Self {
    assert!(self.can_insert_material(),"maximum material count exceeded");

    self.entries.push(BindGroupEntry {
      binding: self.entries.len() as u32,
      resource: BindingResource::Buffer(BufferBinding {
        buffer,
        offset,
        size: None,
      })
    });

    self
  }

  #[inline]
  pub fn build(&self,dev: &Device)-> BindGroup {
    let desc=BindGroupDescriptor {
      label: self.label,
      entries: &self.entries,
      layout: self.layout.expect("a layout must be specified"),
    };

    dev.create_bind_group(&desc)
  }
}





impl<'a> BindGroupLayoutBuilder<'a> {
  #[inline(always)]
  pub const fn new()-> Self {
    Self {
      label: None,
      entries: Vec::new(),
    }
  }

  #[inline(always)]
  pub fn with_label(&mut self,label: &'a str)-> &mut Self {
    self.label=Some(label);
    self
  }

  #[inline(always)]
  const fn can_insert_item(&self)-> bool {
    self.entries.len() <= u32::MAX as usize - 1
  }

  #[inline]
  pub fn with_uniform_buf(&mut self)-> &mut Self {
    assert!(self.can_insert_item(),"maximum material count exceeded");

    self.entries.push(BindGroupLayoutEntry {
      count: None,
      visibility: ShaderStages::VERTEX,
      binding: self.entries.len() as u32,
      ty: BindingType::Buffer {
        min_binding_size: None,
        has_dynamic_offset: false,
        ty: BufferBindingType::Uniform,
      }
    });

    self
  }

  #[inline]
  pub fn with_material(&mut self)-> &mut Self {
    assert!(self.can_insert_item(),"maximum material count exceeded");

    self.entries.push(BindGroupLayoutEntry {
      count: None,
      binding: self.entries.len() as u32,
      visibility: ShaderStages::FRAGMENT,
      ty: BindingType::Texture {
        multisampled: false,
        view_dimension: TextureViewDimension::D2,
        sample_type: TextureSampleType::Float {
          filterable: true
        },
      },
    });

    self.entries.push(BindGroupLayoutEntry {
      count: None,
      binding: self.entries.len() as u32,
      visibility: ShaderStages::FRAGMENT,
      ty: BindingType::Sampler(SamplerBindingType::Filtering),
    });

    self
  }

  pub fn build(&self,dev: &Device)-> BindGroupLayout {
    let desc=BindGroupLayoutDescriptor {
      label: self.label,
      entries: &self.entries,
    };

    dev.create_bind_group_layout(&desc)
  }
}




