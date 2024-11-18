use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

// Define un trait que actúa como una interfaz
pub trait Hittable {
    fn hit(
        &self,
        r: &Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut HitRecord,
    ) -> bool;
}