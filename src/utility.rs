use rand::{thread_rng, Rng};
use rand::distributions::OpenClosed01;

#[inline]
pub fn random_float() -> f64 {
    thread_rng().sample(OpenClosed01)
}