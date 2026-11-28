
mod atom;

use std::time::Instant;
pub use atom::HydrogenAtom;

use crate::{
  physics::QuantumState,
  renderer::{
    Mesh,
    Renderer,
  },
};


pub struct Scene {
  mesh: Option<Mesh>,
  time: Option<Instant>,
  time_0: Option<Instant>,
  atom: Option<HydrogenAtom>,
}


impl Scene {
  #[inline]
  pub fn new()-> Self {
    Self {
      atom: None,
      mesh: None,
      time: None,
      time_0: None,
    }
  }

  #[inline(always)]
  pub fn with_atom(&mut self,atom: HydrogenAtom) {
    self.atom=Some(atom);
  }

pub(crate) fn render(
    &mut self,
    renderer: &mut Renderer,
    clip: [f32; 3],
    intensity: f32,
    egui_renderer: &mut egui_wgpu::Renderer,
    paint_jobs: &[egui::ClippedPrimitive],
    textures_delta: &egui::TexturesDelta,
) {    let now=Instant::now();
    let _dt=self.time
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


    if self.mesh.is_none() && let Some(atom)=&self.atom {
      self.mesh=Some(atom.gen_mesh(&renderer.dev));
    }

    // SAFETY(nate): trust me bro.
    let mesh=unsafe { self.mesh.as_ref().unwrap_unchecked() };
renderer.render(
    mesh,
    time,
    clip,
    intensity,
    egui_renderer,
    paint_jobs,
    textures_delta,
);
  }

  #[inline(always)]
  pub fn set_quantum_state(&mut self,state: QuantumState,renderer: &Renderer) {
    if let Some(atom)=&mut self.atom {
      atom.state=state;
      self.mesh=Some(atom.gen_mesh(&renderer.dev));
    }
  }
}







