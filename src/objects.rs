use crate::vec3::{Vec3, Point};
use crate::ray::Ray;
use crate::hittable::{HitRecord, HitResult, Hittable};

pub struct Sphere {
    center: Point,
    radius: f64
}

impl Sphere {
    pub fn sphere(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitResult {
        let mut rec: HitRecord = HitRecord::hit_record();
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = Vec3::dot(&oc, &r.direction());
        let c: f64 =  oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {return HitResult::hit_result(false, rec)}
        let sqrt_d: f64 = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range.
        let root: f64 = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {return HitResult::hit_result(false, rec)}
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return HitResult::hit_result(true, rec)
    }
}