
use wgpu::*;
use rand::Rng;
use glam::prelude::*;
use std::f64::consts as f64;

use crate::{
  physics::QuantumState,
  style::{
    Color,
  },
  renderer::{
    Mesh,
    Vertex,
    MeshBuilder,
  }
};




const DIRECTIONS: [Vec3;6]=[
  Vec3::X,
  Vec3::NEG_X,
  Vec3::Y,
  Vec3::NEG_Y,
  Vec3::Z,
  Vec3::NEG_Z,
];

const FACES: [[u32;3];8]=[
  [2, 4, 0],
  [2, 0, 5],
  [2, 5, 1],
  [2, 1, 4],
  [3, 0, 4],
  [3, 5, 0],
  [3, 1, 5],
  [3, 4, 1],
];



pub struct HydrogenCloud {
  samples: Vec<Vec3>,
  colors: Vec<Color>,
  marker_radius: f32,
}

impl HydrogenCloud {
  #[inline]
  pub const fn new(samples: Vec<Vec3>,marker_radius: f32)-> Self {
    assert!(marker_radius.is_finite() && marker_radius>0.0);

    Self {
      samples,
      marker_radius,
      colors: Vec::new(),
    }
  }

  #[inline(always)]
  pub const fn sample_count(&self)-> usize {
    self.samples.len()
  }

  /// coordinates are in bohr radii
  pub fn sample(state: QuantumState,count: usize,marker_radius: f32,rng: &mut impl Rng)-> Self {
    assert!(marker_radius.is_finite() && marker_radius > 0.0);

    let n=state.principal().get();
    let l=state.azimuthal();
    let m=state.magnetic().unsigned_abs();

    let n_squared=(n*n) as f32;

    let mut position=random_direction(rng) * (n*n) as f32;
    let mut density=state.density(position);

    while density==0.0 || !density.is_finite() {
      position=random_direction(rng)*n_squared;
      density=state.density(position);
    }

    let step_size=0.2*n_squared;

    let burn_in=10_000;
    let thinning=8;


    let angular_norm=angular_norm(l,m);
    let color_scale=1.5*5.0_f64.powi(n as i32) / angular_norm;
    let mut colors=Vec::<Color>::with_capacity(count);


    let mut samples=Vec::<Vec3>::with_capacity(count);

    for iteration in 0..(burn_in + thinning*count) {
      let proposal=position + random_direction(rng)*(random_unit(rng) as f32 * step_size);
      let proposal_density=state.density(proposal);

      if proposal_density>0.0 && proposal_density.is_finite() && random_unit(rng)<(proposal_density/density).min(1.0) {
        position=proposal;
        density=proposal_density;
      }

      if iteration>=burn_in && (iteration-burn_in)%thinning == 0 {
        samples.push(position);
        colors.push(heatmap_fire(density*color_scale));
      }
    }

    Self {
      colors,
      samples,
      marker_radius,
    }
  }

  pub fn gen_mesh(&self,dev: &Device)-> Mesh {
    let mut builder=MeshBuilder::new();

    builder.insert_indexed_buf_with(|indices,vertices| {
      for (sample_idx,&sample) in self.samples.iter().enumerate() {
        let sample=vec3(sample.x,sample.z,-sample.y);
        let color=unsafe {
          *self.colors
          .get_unchecked(sample_idx)
        };


        let base=vertices.len() as u32;
        for dir in DIRECTIONS {
          let position=sample + dir*self.marker_radius;

          vertices.push(Vertex::with_normal(
            position.extend(1.0),
            color,
            dir,
          ));
        }

        for [a,b,c] in FACES {
          indices.extend([base+a, base+b, base+c]);
        }
      }
    });

    builder.build(dev)
  }
}


#[inline]
fn angular_norm(l: u32,m: u32)-> f64 {
  let (l,m)=(l as u64, m as u64);

  debug_assert!(m <= l);
  let mut factorial_ratio=1.0;
  let mut i=1u64;

  while i<=m {
    factorial_ratio /= ((l-m+i) * (l+i)) as f64;
    i+=1;
  }

  (2*l + 1) as f64 / (4.0*f64::PI) * factorial_ratio
}

const FIRE_STOPS: [Vec3;6]=[
  vec3(0.0,0.0,0.0), // black
  vec3(0.5, 0.0, 0.99), // purple
  vec3(0.8, 0.0, 0.0), // red
  vec3(1.0, 0.5, 0.0), // orange
  vec3(1.0, 1.0, 0.0), // yellow
  vec3(1.0,1.0,1.0), // white
];

#[inline]
fn heatmap_fire(value: f64)-> Color {
  let x=5.0*f32::clamp(value as f32,0.0,1.0);

  let i=(x as usize).min(4);
  let t=x - i as f32;

  let rgb=FIRE_STOPS[i].lerp(FIRE_STOPS[i+1], t);

  Color::from_vec3(rgb)
}



#[inline]
fn random_unit(rng: &mut impl Rng)-> f64 {
  const SCALE: f64=1.0 / ((1u64<<53) as f64);
  (rng.next_u64()>>11) as f64 * SCALE
}

#[inline]
fn random_direction(rng: &mut impl Rng)-> Vec3 {
  let z=1.0 - 2.0*random_unit(rng);
  let phi=f64::TAU*random_unit(rng);

  let xy=f64::sqrt(1.0 - z*z).max(0.0);
  let (sin_phi,cos_phi)=phi.sin_cos();

  vec3(
    (xy*cos_phi) as f32,
    (xy*sin_phi) as f32,
    z as f32,
  )
}







