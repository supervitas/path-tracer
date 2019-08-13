use std::fmt;
use std::ops::{Mul, Add, DivAssign, Sub};
use num::Float;
use core::ops;

#[derive(Clone, Debug)]
pub struct Vector3 <T: Float> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl <T: Float> Vector3 <T> where T: Float + DivAssign {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 {x, y, z}
    }

    pub fn set(&mut self, x: T, y: T, z: T) -> &mut Self {
        self.x = x;
        self.y = y;
        self.z = z;

        self
    }

    pub fn cross(&mut self, v2: &Vector3<T>) -> &mut Self {
        let ax = self.x;
        let ay = self.y;
        let az = self.z;

        self.x = ay * v2.z - az * v2.y;
        self.y = az * v2.x - ax * v2.z;
        self.z = ax * v2.y - ay * v2.x;

        self
    }

    pub fn dot(&self, v2: &Vector3<T>) -> T {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }

    pub fn magnitude(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn divide(&mut self, scalar: T) -> &mut Self {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;

        self
    }

    pub fn normalize(&mut self) -> &mut Self {
        let magnitude = self.magnitude();
        self.divide(magnitude);

        self
    }
}

impl <T: Float> ops::Add<&Vector3<T>> for &Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, other: &Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl <T: Float> ops::Sub<&Vector3<T>> for &Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, other: &Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl <T: Float> PartialEq for Vector3<T> {
    fn eq(&self, other: &Vector3<T>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: fmt::Display> fmt::Display for Vector3<T>
    where T: Float, <T as Mul>::Output: Add<Output=T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
