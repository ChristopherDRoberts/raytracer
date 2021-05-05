use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        let l = self.length();
        Vec3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neg() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = -v1;

        assert!(approx_eq(v2.x, -1.0, f64::EPSILON));
        assert!(approx_eq(v2.y, -2.0, f64::EPSILON));
        assert!(approx_eq(v2.z, -3.0, f64::EPSILON));
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        let v3 = v1 + v2;

        assert!(approx_eq(v3.x, 2.0, f64::EPSILON));
        assert!(approx_eq(v3.y, 3.0, f64::EPSILON));
        assert!(approx_eq(v3.z, 4.0, f64::EPSILON));
    }

    #[test]
    fn sub() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let v3 = v1 - v2;

        assert!(approx_eq(v3.x, 0.0, f64::EPSILON));
        assert!(approx_eq(v3.y, 0.0, f64::EPSILON));
        assert!(approx_eq(v3.z, 0.0, f64::EPSILON));
    }

    #[test]
    fn add_assign() {
        let mut v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        v1 += v2;

        assert!(approx_eq(v1.x, 1.0, f64::EPSILON));
        assert!(approx_eq(v1.y, 2.0, f64::EPSILON));
        assert!(approx_eq(v1.z, 3.0, f64::EPSILON));
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;

        assert!(approx_eq(v.x, 2.0, f64::EPSILON));
        assert!(approx_eq(v.y, 4.0, f64::EPSILON));
        assert!(approx_eq(v.z, 6.0, f64::EPSILON));
    }

    #[test]
    fn div_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v /= 2.0;

        assert!(approx_eq(v.x, 0.5, f64::EPSILON));
        assert!(approx_eq(v.y, 1.0, f64::EPSILON));
        assert!(approx_eq(v.z, 1.5, f64::EPSILON));
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

        assert!(approx_eq(k.x, 0.0, f64::EPSILON));
        assert!(approx_eq(k.y, 0.0, f64::EPSILON));
        assert!(approx_eq(k.z, 1.0, f64::EPSILON));
    }

    #[test]
    fn unit_vector() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = v1.unit_vector();

        assert!(approx_eq(v2.x, 0.2672612419124244, f64::EPSILON));
        assert!(approx_eq(v2.y, 0.5345224838248488, f64::EPSILON));
        assert!(approx_eq(v2.z, 0.8017837257372732, f64::EPSILON));
        assert!(approx_eq(v2.length(), 1.0, f64::EPSILON));
    }

    fn approx_eq(x: f64, y: f64, tolerance: f64) -> bool {
        (x - y).abs() < tolerance
    }
}
