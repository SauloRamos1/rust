use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    // Constructor
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }
    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn dot(self, o: Self) -> f32 {
        self.e[0] * o.e[0] + self.e[1] * o.e[1] + self.e[2] * o.e[2]
    }
    pub fn cross(self, o: Self) -> Self {
        Self {
            e: [
                self.e[1] * o.e[2] - self.e[2] * o.e[1],
                self.e[2] * o.e[0] - self.e[0] * o.e[2],
                self.e[0] * o.e[1] - self.e[1] * o.e[0],
            ],
        }
    }
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
}

// -v
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

// v + v
impl Add for Vec3 {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self {
            e: [self.e[0] + o.e[0], self.e[1] + o.e[1], self.e[2] + o.e[2]],
        }
    }
}

// v - v
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self {
            e: [self.e[0] - o.e[0], self.e[1] - o.e[1], self.e[2] - o.e[2]],
        }
    }
}

// v * t (scalar)
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, t: f32) -> Self {
        Self {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [self * v.e[0], self * v.e[1], self * v.e[2]],
        }
    }
}

// v / t
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, t: f32) -> Self {
        Self {
            e: [self.e[0] / t, self.e[1] / t, self.e[2] / t],
        }
    }
}

// use: println!("{}", v);
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
