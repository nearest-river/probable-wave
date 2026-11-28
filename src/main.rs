

use probable_wave::prelude::*;


fn main()-> anyhow::Result<()> {
  env_logger::init();


  let initial_state=QuantumState::new(4,3,1,SpinProjection::Up)?;
  let atom=HydrogenAtom::new(initial_state);

  let mut scene=Scene::new();

  scene.with_atom(atom);


  Engine::new()
  .with_scene(scene)
  .with_title("xd")
  .run();

  Ok(())
}















