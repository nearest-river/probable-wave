
mod particles;

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
  pub fn new()-> Self {
    let particles=Particles::default();

    Self {
      particles,
      mesh: None,
      time: None,
      time_0: None,
      on_tick: None,
    }
  }

  #[inline(always)]
  pub fn from_particles(particles: impl Into<Particles>)-> Self {
    let particles=particles.into();
    Self {
      particles,
      mesh: None,
      time: None,
      time_0: None,
      on_tick: None,
    }
  }

  #[inline(always)]
  pub fn on_tick(&mut self,f: impl Fn(Query<'_>)+'static) {
    self.on_tick=Some(Box::from(f));
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


    if let Some(on_tick)=self.on_tick.as_deref() {
      on_tick(Query {
        delta_time: dt,
        particles: &mut self.particles,
        time: now.duration_since(time_0).as_secs_f32(),
      });

      self.update(renderer);
    }

    if self.mesh.is_none() {
      self.mesh=Some(self.particles.gen_mesh(&renderer.dev));
    }

    // SAFETY(nate): trust me bro.
    let mesh=unsafe { self.mesh.as_ref().unwrap_unchecked() };

    renderer.render(mesh);
  }

  #[inline]
  fn update(&mut self,renderer: &Renderer) {
    self.mesh=Some(self.particles.gen_mesh(&renderer.dev));
  }
}







