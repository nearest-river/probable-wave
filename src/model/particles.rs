
use wgpu::Device;
use glam::prelude::*;
use std::f32::consts as f32;

use crate::{
  prelude::*,
  renderer::{
    Mesh,
    Vertex,
    MeshBuilder,
  },
};


pub struct Particles {
  pub radii: Vec<f32>,
  pub charges: Vec<i32>,
  pub positions: Vec<Vec3A>,
}

#[derive(Clone,Copy)]
pub struct Particle {
  pub position: Vec3,
  pub charge: i32,
  pub radius: f32,
}

const ELECTRON_RADIUS: f32=0.01;
const PROTON_RADIUS: f32=4.0 * ELECTRON_RADIUS;
const NEUTRON_RADIUS: f32=PROTON_RADIUS;



impl Particle {
  #[inline(always)]
  pub const fn electron()-> Self {
    Self {
      charge: -1,
      position: Vec3::ZERO,
      radius: ELECTRON_RADIUS,
    }
  }

  #[inline(always)]
  pub const fn proton()-> Self {
    Self {
      charge: 1,
      position: Vec3::ZERO,
      radius: PROTON_RADIUS,
    }
  }

  #[inline(always)]
  pub const fn neutron()-> Self {
    Self {
      charge: 0,
      position: Vec3::ZERO,
      radius: NEUTRON_RADIUS,
    }
  }

  #[inline(always)]
  pub const fn electron_at(position: Vec3)-> Self {
    Self {
      position,
      charge: -1,
      radius: ELECTRON_RADIUS,
    }
  }

  #[inline(always)]
  pub const fn proton_at(position: Vec3)-> Self {
    Self {
      position,
      charge: 1,
      radius: PROTON_RADIUS,
    }
  }

  #[inline(always)]
  pub const fn neutron_at(position: Vec3)-> Self {
    Self {
      position,
      charge: 0,
      radius: NEUTRON_RADIUS,
    }
  }

  fn draw(&self, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>) {
    const SEGMENTS: u32 = 60;

    let centre=self.position;
    let color=self.color();

    vertices.push(Vertex::new(centre.extend(1.0), color));

    vertices.reserve(SEGMENTS as usize);

    for i in 0..SEGMENTS {
      let theta=f32::TAU * i as f32 / SEGMENTS as f32;
      let radius=self.radius*vec2(theta.cos(), theta.sin());

      let position=centre+radius.extend(0.0);
      vertices.push(Vertex::new(position.extend(1.0), color));
    }

    for i in 0..SEGMENTS {
      let current=i+1;
      let next=(i+1) % SEGMENTS + 1;

      indices.extend([0, current, next]);
    }
  }

  #[inline(always)]
  const fn color(&self)-> Color {
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
  pub const fn new()-> Self {
    Self {
      radii: Vec::new(),
      charges: Vec::new(),
      positions: Vec::new(),
    }
  }

  pub fn from_vec(particles: Vec<Particle>)-> Self {
    let n=particles.len();
    let mut radii=Vec::<f32>::with_capacity(n);
    let mut charges=Vec::<i32>::with_capacity(n);
    let mut positions=Vec::<Vec3A>::with_capacity(n);

    for Particle { radius, charge, position } in particles {
      radii.push(radius);
      charges.push(charge);
      positions.push(position.to_vec3a());
    }

    Self {
      radii,
      charges,
      positions,
    }
  }

  #[inline(always)]
  pub const fn count(&self)-> usize {
    self.charges.len()
  }

  #[inline(always)]
  pub fn insert(&mut self,Particle { position, charge, radius }: Particle) {
    self.radii.push(radius);
    self.charges.push(charge);
    self.positions.push(position.to_vec3a());
  }


  pub(crate) fn gen_mesh(&mut self,dev: &Device)-> Mesh {
    let mut builder=MeshBuilder::new();

    for i in 0..self.count() {
      builder.insert_indexed_buf_with(|indices,vertices| unsafe {
        self.get_unchecked(i)
        .draw(vertices,indices)
      });
    }

    builder.build(dev)
  }

  pub unsafe fn get_unchecked(&self,idx: usize)-> Particle {
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
  fn from(particles: Vec<Particle>)-> Self {
    Self::from_vec(particles)
  }
}

impl Default for Particles {
  #[inline(always)]
  fn default()-> Self {
    Particles::new()
  }
}


