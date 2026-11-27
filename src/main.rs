
use glam::prelude::*;
use std::f32::consts as f32;
use probable_wave::prelude::*;



fn main()-> anyhow::Result<()> {
  env_logger::init();

  let mut scene=Scene::from_particles(vec![
    Particle::proton(),
    Particle::electron_at(vec3(0.5,0.0,0.0)),
  ]);

  scene.on_tick(|Query { delta_time, particles, .. }| {
    particles.positions[1]=Mat3A::from_angle(f32::TAU*delta_time)*particles.positions[1];
  });


  Engine::new()
  .with_title("xd")
  .with_scene(scene)
  .run();



  Ok(())
}





