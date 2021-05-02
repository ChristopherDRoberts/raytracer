use std::io::stderr;
use std::io::Write;
use std::ops::{Add, Sub, Neg, AddAssign, MulAssign, DivAssign};

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3{
    fn new(x: f64, y:f64, z:f64) -> Self{
        Vec3{x: x, y: y, z: z}
    }

    fn length_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Neg for Vec3{
    type Output = Self;
    fn neg(self) -> Self{
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Add for Vec3{
    type Output = Self;
    fn add(self, other: Self) -> Self{
        Vec3{x: self.x + other.x, y:self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vec3{
    type Output = Self;
    fn sub(self, other: Self) -> Self{
        Vec3{x: self.x - other.x, y:self.y - other.y, z: self.z - other.z}
    }
}

impl AddAssign for Vec3{
    fn add_assign(&mut self, rhs: Self){
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }    
}

impl MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, rhs: f64){
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3{
    fn div_assign(&mut self, rhs: f64){
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

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

fn approx_eq(x: f64, y: f64, tolerance: f64) -> bool{
    (x-y).abs() < tolerance
}

fn main() {
    let width = 256;
    let height = 256;

    print!("P3\n{} {}\n255\n", width, height);

    let blue = 63;
    for row in 0..height {
        eprint!("\rScanlines remaining: {}", height - 1 - row);
        stderr().flush().unwrap();
        for col in 0..width {
            let red = col;
            let green = width - row + 1;
            println!("{} {} {}", red, green, blue);
        }
    }
}
