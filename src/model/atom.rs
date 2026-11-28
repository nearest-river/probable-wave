
use wgpu::Device;
use std::f32::consts as f32;

use crate::{
  physics::{
    QuantumState,
    HydrogenCloud,
  },
  renderer::{
    Mesh,
  },
};





pub struct HydrogenAtom {
  pub(crate) state: QuantumState,
}


impl HydrogenAtom {
  pub const fn new(state: QuantumState)-> Self {
    HydrogenAtom { state }
  }

  pub(crate) fn gen_mesh(&self,dev: &Device)-> Mesh {
    let mut rng=rand::rng();

    HydrogenCloud::sample(self.state,36000,0.025 * self.state.principal().get() as f32,&mut rng)
    .gen_mesh(dev)
  }
}






























