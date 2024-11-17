use crate::vec3::{Point3, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}
impl Ray {

    pub fn new() -> Self {
        Ray { orig: Point3::new(), dir: Vec3::new() }
    }

    // Constructor para crear una nueva instancia de Vec3 con valores definidos
    pub fn new_values(origin: Point3, direction: Vec3) -> Self {
        Ray { orig: origin, dir: direction }
    }
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(self, t : f64) -> Point3 {
        self.orig + t * self.dir
    }
}