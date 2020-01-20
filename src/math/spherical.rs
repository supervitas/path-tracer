use std::fmt;
use std::ops::{Mul, Add, DivAssign};
use num::{Float};
use crate::math::vec3::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Spherical <T: Float> {
    pub radius: T,
    pub polar_angle: T, // phi
    pub azimuth_angle: T // theta
}

impl <T: Float> Spherical <T> where T: Float + DivAssign {
    pub fn new(radius: T, polar_angle: T, azimuth_angle: T) -> Self {
        Spherical {radius, polar_angle, azimuth_angle}
    }

    pub fn from_cartesian(x: T, y: T, z: T) -> Self {
        let radius = Float::sqrt(x * x + y * y + z * z);
        let azimuth_angle = Float::atan2(x,z);
        let polar_angle = Float::acos(y / radius);


        Spherical { radius, polar_angle, azimuth_angle }
    }

    pub fn to_cartesian(&self) -> Vector3<T> {
        let sin_phi_radius = Float::sin( self.polar_angle ) * self.radius;

        let x = sin_phi_radius * Float::sin( self.azimuth_angle );
        let y = Float::cos( self.polar_angle ) * self.radius;
        let z = sin_phi_radius * Float::cos( self.azimuth_angle );

        Vector3::new(x, y, z)
    }
}


impl <T: Float> PartialEq for Spherical<T> {
    fn eq(&self, other: &Spherical<T>) -> bool {
        self.radius == other.radius && self.polar_angle == other.polar_angle && self.azimuth_angle == other.azimuth_angle
    }
}

impl<T: fmt::Display> fmt::Display for Spherical<T>
    where T: Float, <T as Mul>::Output: Add<Output=T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "radius: {}, polar: {}, azimuth: {}", self.radius, self.polar_angle, self.azimuth_angle)
    }
}
