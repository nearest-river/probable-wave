
use rand::Rng;
use glam::prelude::*;
use std::{
  f32::consts as f32,
  f64::consts as f64,
};

pub const PLANCK_CONSTANT: f64=6.626_070_15e-34;
pub const REDUCED_PLANCK_CONSTANT: f64=PLANCK_CONSTANT/f64::TAU;

pub const ELECTRON_VOLT: f64=1.602_176_634e-19;
pub const GROUND_STATE_ENERGY_EV: f64=-13.605_693_122_99;

pub const GROUND_STATE_ENERGY_J: f64=GROUND_STATE_ENERGY_EV*ELECTRON_VOLT;
pub const GROUND_STATE_ANGULAR_FREQUENCY: f64=GROUND_STATE_ENERGY_J/REDUCED_PLANCK_CONSTANT;
pub const BOHR_RADIUS: f64=5.291_772_109_03e-11;



pub struct Hydrogen1S;

impl Hydrogen1S {
  #[inline]
  pub fn density(position: Vec3)-> f64 {
    f64::exp(-2.0 * position.length() as f64) / f64::PI
  }

  pub fn sample(rng: &mut impl Rng)-> Vec3 {
    let u1=Self::unit(rng).max(f64::MIN_POSITIVE);
    let u2=Self::unit(rng).max(f64::MIN_POSITIVE);
    let u3=Self::unit(rng).max(f64::MIN_POSITIVE);

    let r=-0.5*(f64::ln(u1*u2*u3));

    let z=1.0 - 2.0*Self::unit(rng);
    let phi=f64::TAU * Self::unit(rng);

    let xy=f64::sqrt((1.0 - z*z).max(0.0));
    let (sin_phi,cos_phi)=f64::sin_cos(phi);

    vec3(
      (r*xy*cos_phi) as f32,
      (r*xy*sin_phi) as f32,
      (r*z) as f32
    )
  }

  #[inline]
  fn unit(rng: &mut impl Rng)-> f64 {
    const SCALE: f64=1.0 / ((1u64<<53) as f64);

    (rng.next_u64() >> 11) as f64 * SCALE
  }
}





























