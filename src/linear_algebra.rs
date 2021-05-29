use crate::random::rand;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    v: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { v: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }
    pub fn z(&self) -> f64 {
        self.v[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let x = self.v[1] * other.v[2] - self.v[2] * other.v[1];
        let y = self.v[2] * other.v[0] - self.v[0] * other.v[2];
        let z = self.v[0] * other.v[1] - self.v[1] * other.v[0];
        Vec3::new(x, y, z)
    }

    pub fn unit_vector(&self) -> Vec3 {
        let l = self.length();
        Vec3::new(self.v[0] / l, self.v[1] / l, self.v[2] / l)
    }

    pub fn near_zero(&self) -> bool {
        let tolerance = 1e-8;
        (self.v[0].abs() < tolerance)
            && (self.v[1].abs() < tolerance)
            && (self.v[2].abs() < tolerance)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3::new(-self.v[0], -self.v[1], -self.v[2])
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3::new(
            self.v[0] + other.v[0],
            self.v[1] + other.v[1],
            self.v[2] + other.v[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3::new(
            self.v[0] - other.v[0],
            self.v[1] - other.v[1],
            self.v[2] - other.v[2],
        )
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3::new(
            self.v[0] * other.v[0],
            self.v[1] * other.v[1],
            self.v[2] * other.v[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.v[0] * rhs, self.v[1] * rhs, self.v[2] * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Vec3::new(self.v[0] / rhs, self.v[1] / rhs, self.v[2] / rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.v[0] += rhs.v[0];
        self.v[1] += rhs.v[1];
        self.v[2] += rhs.v[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.v[0] *= rhs;
        self.v[1] *= rhs;
        self.v[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.v[0] /= rhs;
        self.v[1] /= rhs;
        self.v[2] /= rhs;
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

pub fn refract(unit_vector: Vec3, normal: Vec3, refraction_ratio: f64) -> Vec3{
    let cosine = f64::min(1.0, -(unit_vector.dot(&normal)));
    let perp_vec = refraction_ratio*(unit_vector + cosine*normal);
    let parallel_vec = -f64::sqrt((1.0 - perp_vec.length_squared()).abs())*normal;
    
    perp_vec + parallel_vec
}

pub fn random_vector(min: f64, max: f64) -> Vec3 {
    let x = rand(min, max);
    let y = rand(min, max);
    let z = rand(min, max);
    Vec3::new(x, y, z)
}

pub fn random_vector_in_unit_sphere() -> Vec3 {
    loop {
        let v = random_vector(-1.0, 1.0);
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_vector_in_unit_sphere().unit_vector()
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod vec3_tests {
        use super::*;

        #[test]
        fn neg() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let v2 = -v1;

            assert!(approx_eq(v2.x(), -1.0, f64::EPSILON));
            assert!(approx_eq(v2.y(), -2.0, f64::EPSILON));
            assert!(approx_eq(v2.z(), -3.0, f64::EPSILON));
        }

        #[test]
        fn add() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let v2 = Vec3::new(1.0, 1.0, 1.0);
            let v3 = v1 + v2;

            assert!(approx_eq(v3.x(), 2.0, f64::EPSILON));
            assert!(approx_eq(v3.y(), 3.0, f64::EPSILON));
            assert!(approx_eq(v3.z(), 4.0, f64::EPSILON));
        }

        #[test]
        fn sub() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let v2 = Vec3::new(1.0, 2.0, 3.0);
            let v3 = v1 - v2;

            assert!(approx_eq(v3.x(), 0.0, f64::EPSILON));
            assert!(approx_eq(v3.y(), 0.0, f64::EPSILON));
            assert!(approx_eq(v3.z(), 0.0, f64::EPSILON));
        }

        #[test]
        fn add_assign() {
            let mut v1 = Vec3::new(0.0, 0.0, 0.0);
            let v2 = Vec3::new(1.0, 2.0, 3.0);
            v1 += v2;

            assert!(approx_eq(v1.x(), 1.0, f64::EPSILON));
            assert!(approx_eq(v1.y(), 2.0, f64::EPSILON));
            assert!(approx_eq(v1.z(), 3.0, f64::EPSILON));
        }

        #[test]
        fn mul_assign() {
            let mut v = Vec3::new(1.0, 2.0, 3.0);
            v *= 2.0;

            assert!(approx_eq(v.x(), 2.0, f64::EPSILON));
            assert!(approx_eq(v.y(), 4.0, f64::EPSILON));
            assert!(approx_eq(v.z(), 6.0, f64::EPSILON));
        }

        #[test]
        fn div_assign() {
            let mut v = Vec3::new(1.0, 2.0, 3.0);
            v /= 2.0;

            assert!(approx_eq(v.x(), 0.5, f64::EPSILON));
            assert!(approx_eq(v.y(), 1.0, f64::EPSILON));
            assert!(approx_eq(v.z(), 1.5, f64::EPSILON));
        }

        #[test]
        fn length() {
            let v = Vec3::new(2.0, 3.0, 4.0);

            assert!(approx_eq(v.length_squared(), 29.0, f64::EPSILON));
            assert!(approx_eq(v.length(), 5.385164807134504, f64::EPSILON));
        }

        #[test]
        fn dot() {
            let v = Vec3::new(1.0, 2.0, 3.0);
            let d = v.dot(&v);

            assert!(approx_eq(d, 14.0, f64::EPSILON));
        }

        #[test]
        fn cross() {
            let i = Vec3::new(1.0, 0.0, 0.0);
            let j = Vec3::new(0.0, 1.0, 0.0);
            let k = i.cross(&j);

            assert!(approx_eq(k.x(), 0.0, f64::EPSILON));
            assert!(approx_eq(k.y(), 0.0, f64::EPSILON));
            assert!(approx_eq(k.z(), 1.0, f64::EPSILON));
        }

        #[test]
        fn unit_vector() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let v2 = v1.unit_vector();

            assert!(approx_eq(v2.x(), 0.2672612419124244, f64::EPSILON));
            assert!(approx_eq(v2.y(), 0.5345224838248488, f64::EPSILON));
            assert!(approx_eq(v2.z(), 0.8017837257372732, f64::EPSILON));
            assert!(approx_eq(v2.length(), 1.0, f64::EPSILON));
        }
    }

    mod ray_tests {
        use super::*;

        #[test]
        fn at() {
            let origin = Vec3::new(1.0, 1.0, 1.0);
            let direction = Vec3::new(1.0, 0.5, -0.5);
            let ray = Ray::new(origin, direction);
            let p1 = ray.at(-1.0);
            let p2 = ray.at(1.0);

            assert!(approx_eq(p1.x(), 0.0, f64::EPSILON));
            assert!(approx_eq(p1.y(), 0.5, f64::EPSILON));
            assert!(approx_eq(p1.z(), 1.5, f64::EPSILON));

            assert!(approx_eq(p2.x(), 2.0, f64::EPSILON));
            assert!(approx_eq(p2.y(), 1.5, f64::EPSILON));
            assert!(approx_eq(p2.z(), 0.5, f64::EPSILON));
        }
    }

    fn approx_eq(x: f64, y: f64, tolerance: f64) -> bool {
        (x - y).abs() < tolerance
    }
}
