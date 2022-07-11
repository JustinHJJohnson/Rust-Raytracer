use crate::vec3::{Vec3, Point};
use crate::ray::{Ray};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal}
    }

    pub fn hit_record() -> HitRecord {
        HitRecord { p: (Vec3::ZERO), normal: (Vec3::ZERO), t: (0.0), front_face: (false) }
    }
}

pub struct HitResult {
    pub hit: bool,
    pub hit_record: HitRecord
}

impl HitResult {
    pub fn hit_result(hit: bool, hit_record: HitRecord) -> HitResult {
        HitResult{hit, hit_record}
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitResult;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn hittable_list() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitResult {
        let mut output_rec: HitRecord = HitRecord::hit_record();
        let mut hit_anything: bool = false;
        let mut closest_so_far:f64 = t_max;

        for object in &self.objects {
            let result: HitResult = object.hit(r, t_min, closest_so_far);
            if result.hit {
                hit_anything = true;
                closest_so_far = result.hit_record.t;
                output_rec = result.hit_record;
            }
        }

        return HitResult::hit_result(hit_anything, output_rec);
    }
}