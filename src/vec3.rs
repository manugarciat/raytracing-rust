use std::ops::{Add, Div, Index, IndexMut, Mul, MulAssign, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3], //no es publico
}

impl Vec3 {
    // Constructor para crear una nueva instancia de Vec3 (0,0,0)
    pub fn new_zero() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    // Constructor para crear una nueva instancia de Vec3 con valores definidos
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }

    pub fn length_squared(&self) -> f64 {
        self.e.iter().map(|&x| x * x).sum()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

// Negativiza el vector
impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, escalar: f64) {
        self.e[0] *= escalar;
        self.e[1] *= escalar;
        self.e[2] *= escalar;
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, divisor: f64) {
        *self *= 1.0 / divisor;
    }
}

// point3 is just an alias for vec3, but useful for geometric clarity in the code.
pub(crate) type Point3 = Vec3;

// Vector Utility Functions
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            e: [self.x() - other.x(), self.y() - other.y(), self.z() - other.z()]
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Vec3 { e: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()] }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 { e: [self.x() * other.x(), self.y() * other.y(), self.z() * other.z()] }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, escalar: f64) -> Self::Output {
        Vec3 { e: [self.x() * escalar, self.y() * escalar, self.z() * escalar] }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Self::Output {
        vector * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, divisor: f64) -> Self::Output {
        self * (1.0 / divisor)
    }
}

// operaciones entre vectores
pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.e.iter().zip(b.e.iter()).map(|(x, y)| x * y).sum()
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(a.y() * b.z() - a.z() * b.y(),
              a.z() * b.x() - a.x() * b.z(),
              a.x() * b.y() - a.y() * b.x())
}

/// Normaliza el vector
pub fn unit_vector(v: Vec3) -> Vec3 {
    let l = v.length();
    return v / l;
}