use crate::vec3::Vec3;

use Vec3 as Point3;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    // P(t) = origin + t * direction
    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}
