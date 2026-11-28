
use probable_wave::prelude::*;
use rand::{
  SeedableRng,
  rngs::StdRng,
};



fn main()-> anyhow::Result<()> {
  env_logger::init();

  let mut rng=StdRng::seed_from_u64(0x1_5eed);

  let initial_state=QuantumState::new(4,3,1,SpinProjection::Up)?;

  let cloud=HydrogenCloud::sample(initial_state,16384,0.012,&mut rng);

  let scene=Scene::from_mesh_generator(
    move |dev| cloud.gen_mesh(dev)
  );


  Engine::new()
  .with_title("xd")
  .with_scene(scene)
  .run();

  Ok(())
}















