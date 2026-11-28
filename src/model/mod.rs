
mod particles;

use wgpu::Device;
use std::time::Instant;

use crate::renderer::{
  Mesh,
  Renderer,
};

pub use particles::{
  Particle,
  Particles,
};


pub struct Scene {
  mesh: Option<Mesh>,
  time: Option<Instant>,
  time_0: Option<Instant>,
  mesh_generator: Box<dyn Fn(&Device)-> Mesh>,
}


impl Scene {

  #[inline]
  pub fn from_mesh_generator(mesh_generator: impl Fn(&Device)-> Mesh+'static)-> Self {
    Self {
      mesh: None,
      time: None,
      time_0: None,
      mesh_generator: Box::from(mesh_generator),
    }
  }

  pub(crate) fn render(&mut self,renderer: &mut Renderer) {
    let now=Instant::now();
    let dt=self.time
    .replace(now)
    .map_or(0.0,|t_0| now.duration_since(t_0).as_secs_f32());

    let time_0=match self.time_0 {
      Some(time_0)=> time_0,
      None=> {
        self.time_0=Some(now);
        now
      }
    };

    let time=now.duration_since(time_0).as_secs_f32();

    drop((time,dt));

    if self.mesh.is_none() {
      self.mesh=Some((self.mesh_generator)(&renderer.dev));
    }

    // SAFETY(nate): trust me bro.
    let mesh=unsafe { self.mesh.as_ref().unwrap_unchecked() };

    renderer.render(mesh);
  }

  #[inline]
  fn update(&mut self,renderer: &Renderer) {
    self.mesh=Some((self.mesh_generator)(&renderer.dev));
  }
}







