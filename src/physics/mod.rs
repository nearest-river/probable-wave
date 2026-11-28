
mod cloud;
mod quantum;

pub use cloud::HydrogenCloud;

pub use quantum::{
  QuantumState,
  SpinProjection,
  QuantumStateError,
};

use std::f64::consts as f64;


pub const PLANCK_CONSTANT: f64=6.626_070_15e-34;
pub const REDUCED_PLANCK_CONSTANT: f64=PLANCK_CONSTANT/f64::TAU;

pub const ELECTRON_VOLT: f64=1.602_176_634e-19;
pub const GROUND_STATE_ENERGY_EV: f64=-13.605_693_122_99;

pub const GROUND_STATE_ENERGY_J: f64=GROUND_STATE_ENERGY_EV*ELECTRON_VOLT;
pub const GROUND_STATE_ANGULAR_FREQUENCY: f64=GROUND_STATE_ENERGY_J/REDUCED_PLANCK_CONSTANT;
pub const BOHR_RADIUS: f64=5.291_772_109_03e-11;








