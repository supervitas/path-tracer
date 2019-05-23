use std::fmt;
use std::ops::{Mul, Add};

pub struct Vector3 <T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T
}

//impl<S> Vector3<S> where S: Mul<Output=S> + Add<Output=S> + Copy {
//    pub fn dot(&self, v2: Vector3<S>) -> S {
//        self.x * v2.x + self.y * v2.y + self.z * v2.z
//    }
//}

impl<S> Vector3<S> where S: Into<f64> + Copy {
    pub fn dot<U>(&self, v2: Vector3<U>) -> f64 where U: Into<f64> + Copy {
        self.x.into() * v2.x.into() + self.y.into() * v2.y.into() + self.z.into() * v2.z.into()
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
