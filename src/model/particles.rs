
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

const ELECTRON_RADIUS: f32=0.05;
const PROTON_RADIUS: f32=2.0 * ELECTRON_RADIUS;
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

  fn draw(&self,vertices: &mut Vec<Vertex>,indices: &mut Vec<u32>) {
    const LATITUDE_SEGMENTS: u32=16;
    const LONGITUDE_SEGMENTS: u32=32;

    let color=self.color();

    vertices.reserve((2 + (LATITUDE_SEGMENTS-1) * LONGITUDE_SEGMENTS) as usize);

    indices.reserve((6 * LONGITUDE_SEGMENTS * (LATITUDE_SEGMENTS-1)) as usize);

    let mut push_vertex=|normal: Vec3| {
      let world_position=self.position + self.radius*normal;

      vertices.push(Vertex::with_normal(
        world_position.extend(1.0),
        color,
        normal,
      ));
    };

    push_vertex(Vec3::Y);

    for lat in 1..LATITUDE_SEGMENTS {
      let phi=f32::PI * lat as f32 / LATITUDE_SEGMENTS as f32;

      let (sin_phi,cos_phi)=f32::sin_cos(phi);

      for lon in 0..LONGITUDE_SEGMENTS {
        let theta=f32::TAU * lon as f32 / LONGITUDE_SEGMENTS as f32;
        let (sin_theta,cos_theta)=f32::sin_cos(theta);

        let normal=Vec3::new(sin_phi*cos_theta,cos_phi,sin_phi*sin_theta);

        push_vertex(normal);
      }
    }

    let bottom=1 + (LATITUDE_SEGMENTS-1) * LONGITUDE_SEGMENTS;

    push_vertex(-Vec3::Y);

    for lon in 0..LONGITUDE_SEGMENTS {
      let current=1+lon;
      let next=1 + (lon+1) % LONGITUDE_SEGMENTS;

      indices.extend([0, next, current]);
    }

    // Connect adjacent latitude rings.
    for lat in 0..(LATITUDE_SEGMENTS-2) {
      let ring_a=1 + lat * LONGITUDE_SEGMENTS;
      let ring_b=ring_a + LONGITUDE_SEGMENTS;

      for lon in 0..LONGITUDE_SEGMENTS {
        let next_lon=(lon+1) % LONGITUDE_SEGMENTS;

        let a=ring_a+lon;
        let b=ring_b+lon;

        let next_a=ring_a+next_lon;
        let next_b=ring_b+next_lon;

        indices.extend([a,next_a,b,next_a,next_b,b]);
      }
    }

    let last_ring=1 + (LATITUDE_SEGMENTS-2) * LONGITUDE_SEGMENTS;

    for lon in 0..LONGITUDE_SEGMENTS {
      let current=last_ring + lon;
      let next=last_ring + (lon+1) % LONGITUDE_SEGMENTS;

      indices.extend([bottom,current,next]);
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


