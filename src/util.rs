use rand::{prelude::Distribution, distributions::Uniform};

/// Custom PI for portability.
const PI: f64 = 3.1415926535897932385;

/// Converts degrees to radians.
#[inline(always)]
pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Returns a random real in [0, 1).
#[inline(always)]
pub fn rand() -> f64 {
    let distr = Uniform::new(0.0, 1.0);
    distr.sample(&mut rand::thread_rng())
}

#[inline(always)]
/// Returns a random real in [min,max).
pub fn rand_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand()
}
