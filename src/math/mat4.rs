use std::ops::{Mul, Add, DivAssign, Neg};
use num::{Float};
use core::ops;
use crate::math::vec3::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Matrix4 <T: Float> {
    pub elements: [T; 16],
}

impl <T: Float> Matrix4 <T> where T: Float + DivAssign {
    pub fn new(elements: [T; 16]) -> Self {
        Matrix4 {
            elements: [
                elements[0], elements[4], elements[8], elements[12],
                elements[1], elements[5], elements[9], elements[13],
                elements[2], elements[6], elements[10], elements[14],
                elements[3], elements[7], elements[11], elements[15]
            ]
        }
    }

    pub fn translation(&mut self, x: T, y: T, z: T) {
        self.elements[3] = x;
        self.elements[7] = y;
        self.elements[11] = z;
    }

    pub fn from_array(elements: [T; 16]) -> Self {
        Matrix4 {
            elements
        }
    }

    pub fn set(&mut self, elements: [T; 16])  {
        self.elements = elements;
    }

    pub fn multiply(&mut self, other: &Matrix4<T>) {
        let a11 = self.elements[ 0 ]; let a12 = self.elements[ 4 ]; let a13 = self.elements[ 8 ]; let a14 = self.elements[ 12 ];
        let a21 = self.elements[ 1 ]; let a22 = self.elements[ 5 ]; let a23 = self.elements[ 9 ]; let a24 = self.elements[ 13 ];
        let a31 = self.elements[ 2 ]; let a32 = self.elements[ 6 ]; let a33 = self.elements[ 10 ]; let a34 = self.elements[ 14 ];
        let a41 = self.elements[ 3 ]; let a42 = self.elements[ 7 ]; let a43 = self.elements[ 11 ]; let a44 = self.elements[ 15 ];

        let b11 = other.elements[ 0 ]; let b12 = other.elements[ 4 ]; let b13 = other.elements[ 8 ]; let b14 = other.elements[ 12 ];
        let b21 = other.elements[ 1 ]; let b22 = other.elements[ 5 ]; let b23 = other.elements[ 9 ]; let b24 = other.elements[ 13 ];
        let b31 = other.elements[ 2 ]; let b32 = other.elements[ 6 ]; let b33 = other.elements[ 10 ]; let b34 = other.elements[ 14 ];
        let b41 = other.elements[ 3 ]; let b42 = other.elements[ 7 ]; let b43 = other.elements[ 11 ]; let b44 = other.elements[ 15 ];

        self.elements[ 0 ] = a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41;
        self.elements[ 4 ] = a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42;
        self.elements[ 8 ] = a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43;
        self.elements[ 12 ] = a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44;

        self.elements[ 1 ] = a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41;
        self.elements[ 5 ] = a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42;
        self.elements[ 9 ] = a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43;
        self.elements[ 13 ] = a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44;

        self.elements[ 2 ] = a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41;
        self.elements[ 6 ] = a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42;
        self.elements[ 10 ] = a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43;
        self.elements[ 14 ] = a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44;

        self.elements[ 3 ] = a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41;
        self.elements[ 7 ] = a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42;
        self.elements[ 11 ] = a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43;
        self.elements[ 15 ] = a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44;
    }
}

impl Matrix4<f32> {
    pub fn identity() -> Self {
        Matrix4::new (
            [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ])
    }

    pub fn look_at(position: &Vector3<f32>, target: &Vector3<f32>) -> Self {
        let mut forward = position - target;
        forward.normalize();

        let mut right = Vector3::new(0.,1.,0.);
        right.cross(&forward);
        let mut up = forward.clone();
        up.cross(&right);

        let mut result = Matrix4::identity();

        result.elements[0] = right.x;
        result.elements[1] = right.y;
        result.elements[2] = right.z;
        result.elements[4] = up.x;
        result.elements[5] = up.y;
        result.elements[6] = up.z;
        result.elements[8] = forward.x;
        result.elements[9] = forward.y;
        result.elements[10] = forward.z;

        result.elements[12] = position.x;
        result.elements[13] = position.y;
        result.elements[14] = position.z;

        result
    }

    pub fn rotate_x(&mut self, theta: f32) {
        let c = f32::cos( theta );
        let  s = f32::sin( theta );

        self.set([
            1., 0., 0., 0.,
            0., c, - s, 0.,
            0., s, c, 0.,
            0., 0., 0., 1.
        ]);
    }

    pub fn rotate_y(&mut self, theta: f32) {
        let c = f32::cos( theta );
        let  s = f32::sin( theta );

        self.set([
            c, 0., s, 0.,
            0., 1., 0., 0.,
            - s, 0., c, 0.,
            0., 0., 0., 1.
        ]);
    }

    pub fn rotate_z(&mut self, theta: f32) {
        let c = f32::cos(theta);
        let s = f32::sin(theta);

        self.set([
            c, -s, 0., 0.,
            s, c, 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.
        ]);
    }
}

impl <T: Float> PartialEq for Matrix4<T> {
    fn eq(&self, other: &Matrix4<T>) -> bool {
        let mut equal = true;

        for (i, item) in self.elements.iter().enumerate() {
            if *item != other.elements[i] as T {
                equal = false;
            }
        }

        equal
    }
}
