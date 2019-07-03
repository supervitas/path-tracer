use std::fmt;
use std::ops::{Mul, Add};

#[derive(Debug)]
pub struct Vector3 <T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<S> Vector3<S> where S: Mul<Output=S> + Add<Output=S> + Copy {
    pub fn dot(&self, v2: Vector3<S>) -> S {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }
}

impl <T> Vector3 <T> where T: Copy {
    pub fn new(x:T, y: T, z: T) -> Self {
        return Vector3{x,y,z};;
    }
}

impl<T: Add<Output = T>> Add for Vector3<T>  where T: Copy + Mul, <T as Mul>::Output: Add<Output=T> {
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<T: fmt::Display> fmt::Display for Vector3<T>
    where T: Copy + Mul, <T as Mul>::Output: Add<Output=T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
