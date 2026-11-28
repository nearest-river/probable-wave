
use crate::physics;
use glam::prelude::*;
use std::{
  num::NonZeroU32,
  f64::consts as f64,
};




#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct QuantumState {
  n: NonZeroU32,
  l: u32,
  m: i32,
  s: SpinProjection,
}

#[repr(i8)]
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum SpinProjection {
  Up,
  Down,
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,thiserror::Error)]
pub enum QuantumStateError {
  #[error("principal quantum number cannot be 0")]
  ZeroPrincipal,
  #[error("azimuthal quantum number must be less than principal quantum number")]
  InvalidOrbital,
  #[error("magnetic quantum number must be within (-l,l)")]
  InvalidMagnetic
}

impl QuantumState {
  pub const ELECTRON_SPIN: f64=0.5;

  pub const fn new(n: u32,l: u32,m: i32,s: SpinProjection)-> Result<Self,QuantumStateError> {
    let Some(n)=NonZeroU32::new(n) else {
      return Err(QuantumStateError::ZeroPrincipal)
    };

    if l >= n.get() {
      return Err(QuantumStateError::InvalidOrbital);
    }

    if m.unsigned_abs() > l {
      return Err(QuantumStateError::InvalidMagnetic);
    }

    Ok(Self {
      n,
      l,
      m,
      s
    })
  }

  #[inline(always)]
  pub const fn principal(self)-> NonZeroU32 {
    self.n
  }

  #[inline(always)]
  pub const fn azimuthal(self)-> u32 {
    self.l
  }

  #[inline(always)]
  pub const fn magnetic(self)-> i32 {
    self.m
  }

  #[inline(always)]
  pub const fn spin(self)-> f64 {
    Self::ELECTRON_SPIN
  }

  #[inline(always)]
  pub const fn magnetic_spin(self)-> f64 {
    self.s.magnetic_spin()
  }

  #[inline(always)]
  pub const fn spin_projection(self)-> SpinProjection {
    self.s
  }

  #[inline(always)]
  pub const fn energy_ev(self)-> f64 {
    physics::GROUND_STATE_ENERGY_EV / self.n.get().pow(2) as f64
  }

  #[inline(always)]
  pub const fn is_1s(self)-> bool {
    self.n.get()==1 && self.l==0 && self.m==0
  }

  pub fn density(self,position: Vec3)-> f64 {
    let r=position.length() as f64;
    let cosine=if r==0.0 {
      1.0
    } else {
      position.z as f64 / r
    };

    let radial=radial_wavefunction(self.n.get(),self.l,r);
    let angular=spherical_harmonic_abs_squared(self.l,self.m,cosine);

    radial*radial*angular
  }
}


fn associated_laguerre(k: u32,alpha: u32,x: f64)-> f64 {
  match k {
    0=> return 1.0,
    1=> return 1.0 + alpha as f64 - x,
    _=> {}
  }

  let alpha=alpha as f64;

  let mut prev=1.0;
  let mut curr=1.0 + alpha - x;

  for i in 2..=k {
    let i=i as f64;

    let next=curr*((2.0*i - 1.0 + alpha - x) - prev*(i-1.0+alpha)) / i;

    prev=curr;
    curr=next;
  }

  curr
}

fn associated_legendre(l: u32,m: u32,x: f64)-> f64 {
  debug_assert!(m<=l);

  let x=x.clamp(-1.0,1.0);
  let sqrt=(1.0 - x*x).sqrt();

  let mut pmm=1.0;

  for i in 1..=m {
    pmm*=-((2*i - 1) as f64)*sqrt;
  }

  if l==m {
    return pmm;
  }

  let mut pm1=x*(2*m + 1) as f64 * pmm;

  if l == m+1 {
    return pm1;
  }

  let mut prev=pmm;
  let mut curr=pm1;

  for ll in (m+2)..=l {
    let ll=ll as f64;
    let mf=m as f64;

    let next=((2.0*ll - 1.0)*x*curr - prev*(ll+mf-1.0)) / (ll-mf);

    prev=curr;
    curr=next;
  }

  pm1=curr;
  pm1
}

// -----------------------------------------------------------------------------
// Factorials and normalization
// -----------------------------------------------------------------------------

fn ln_factorial(n: u32)-> f64 {
  (1..=n)
  .map(|i| f64::ln(i as f64))
  .sum()
}

fn radial_wavefunction(n: u32,l: u32,r: f64)-> f64 {
  let n=n as f64;
  let l=l as f64;

  let k=n as u32 - l as u32 - 1;
  let alpha=2*l as u32 + 1;

  let log_norm=1.5*f64::ln(2.0/n) + 0.5*(ln_factorial(k)-f64::ln(2.0*n) - ln_factorial((n+l) as u32));

  let x=2.0*r/n;

  f64::exp(log_norm) * f64::exp(-r/n) * x.powf(l) * associated_laguerre(k, alpha, x)
}

fn spherical_harmonic_abs_squared(l: u32,m: i32,cos_theta: f64)-> f64 {
  let m=m.unsigned_abs();

  let log_norm=f64::ln((2*l + 1) as f64 / (4.0*f64::PI)) + ln_factorial(l-m) - ln_factorial(l+m);

  let p=associated_legendre(l,m,cos_theta);

  log_norm.exp()*p*p
}





impl SpinProjection {
  #[inline(always)]
  pub const fn magnetic_spin(self)-> f64 {
    match self {
      Self::Up=> 0.5,
      Self::Down=> -0.5,
    }
  }
}






