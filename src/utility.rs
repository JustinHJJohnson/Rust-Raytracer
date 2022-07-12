use rand::{thread_rng, Rng};
use rand::distributions::OpenClosed01;
use crate::vec3::Vec3;

#[inline]
pub fn random_float() -> f64 {
    thread_rng().sample(OpenClosed01)
}

#[inline]
pub fn random_float_in_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p: Vec3 = Vec3::random_in_range(-1.0, 1.0);
        if p.length_squared() < 1.0 { return p }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if Vec3::dot(&in_unit_sphere, &normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}