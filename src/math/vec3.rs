use std::fmt;
use std::ops::{Mul, Add, DivAssign};
use num::Float;

#[derive(Clone, Debug)]
pub struct Vector3 <T: Float> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl <T: Float> Vector3 <T> where T: Float + DivAssign {
    pub fn new(x: T, y: T, z: T) -> Self {
        return Vector3 {x, y, z};
    }

    pub fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

    pub fn dot(&self, v2: Vector3<T>) -> T {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }

    pub fn magnitude(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn divide(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.divide(magnitude);
    }
}

impl<T: fmt::Display> fmt::Display for Vector3<T>
    where T: Float, <T as Mul>::Output: Add<Output=T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
