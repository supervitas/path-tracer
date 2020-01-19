use num::Float;

pub mod vec3;
pub mod mat4;
pub mod ray;
pub mod bbox;
pub mod color;
pub mod spherical;

pub fn lerp <T: Float> (start: T, end: T, amt: T) -> T {
    return (T::one()-amt)*start+amt*end
}
