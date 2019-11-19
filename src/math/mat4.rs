use std::ops::{Mul, Add, DivAssign, Neg, AddAssign};
use num::{Float, NumCast};
use core::ops;
use crate::math::vec3::Vector3;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug)]
pub struct Matrix4 <T: Float> {
    pub elements: [T; 16],
}

impl <T: Float> Matrix4 <T> where T: Float + DivAssign + AddAssign {
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

    pub fn identity() -> Self {
        let zero = T::zero();
        let one = T::one();

        let elements = [
            one, zero, zero, zero,
            zero, one, zero, zero,
            zero, zero, one, zero,
            zero, zero, zero, one
        ];

        Matrix4::from_array(elements)
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

    pub fn inverse (&mut self) {
        let mut me = self.elements;

        let n11 = me[ 0 ]; let n21 = me[ 1 ]; let n31 = me[ 2 ]; let n41 = me[ 3 ];
        let n12 = me[ 4 ]; let n22 = me[ 5 ]; let n32 = me[ 6 ]; let n42 = me[ 7 ];
        let n13 = me[ 8 ]; let n23 = me[ 9 ]; let n33 = me[ 10 ]; let n43 = me[ 11 ];
        let n14 = me[ 12 ]; let n24 = me[ 13 ]; let n34 = me[ 14 ]; let n44 = me[ 15 ];

        let t11 = n23 * n34 * n42 - n24 * n33 * n42 + n24 * n32 * n43 - n22 * n34 * n43 - n23 * n32 * n44 + n22 * n33 * n44;
        let t12 = n14 * n33 * n42 - n13 * n34 * n42 - n14 * n32 * n43 + n12 * n34 * n43 + n13 * n32 * n44 - n12 * n33 * n44;
        let t13 = n13 * n24 * n42 - n14 * n23 * n42 + n14 * n22 * n43 - n12 * n24 * n43 - n13 * n22 * n44 + n12 * n23 * n44;
        let t14 = n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33 + n12 * n24 * n33 + n13 * n22 * n34 - n12 * n23 * n34;

        let det = n11 * t11 + n21 * t12 + n31 * t13 + n41 * t14;

        if det == T::zero()  {
            println!("can't invert matrix, determinant is 0, return identity");
            self.elements =  Matrix4::identity().elements;
            return;
        }

        let det_inv = T::from(1.).unwrap() / det;

        me[ 0 ] = t11 * det_inv;
        me[ 1 ] = ( n24 * n33 * n41 - n23 * n34 * n41 - n24 * n31 * n43 + n21 * n34 * n43 + n23 * n31 * n44 - n21 * n33 * n44 ) * det_inv;
        me[ 2 ] = ( n22 * n34 * n41 - n24 * n32 * n41 + n24 * n31 * n42 - n21 * n34 * n42 - n22 * n31 * n44 + n21 * n32 * n44 ) * det_inv;
        me[ 3 ] = ( n23 * n32 * n41 - n22 * n33 * n41 - n23 * n31 * n42 + n21 * n33 * n42 + n22 * n31 * n43 - n21 * n32 * n43 ) * det_inv;

        me[ 4 ] = t12 * det_inv;
        me[ 5 ] = ( n13 * n34 * n41 - n14 * n33 * n41 + n14 * n31 * n43 - n11 * n34 * n43 - n13 * n31 * n44 + n11 * n33 * n44 ) * det_inv;
        me[ 6 ] = ( n14 * n32 * n41 - n12 * n34 * n41 - n14 * n31 * n42 + n11 * n34 * n42 + n12 * n31 * n44 - n11 * n32 * n44 ) * det_inv;
        me[ 7 ] = ( n12 * n33 * n41 - n13 * n32 * n41 + n13 * n31 * n42 - n11 * n33 * n42 - n12 * n31 * n43 + n11 * n32 * n43 ) * det_inv;

        me[ 8 ] = t13 * det_inv;
        me[ 9 ] = ( n14 * n23 * n41 - n13 * n24 * n41 - n14 * n21 * n43 + n11 * n24 * n43 + n13 * n21 * n44 - n11 * n23 * n44 ) * det_inv;
        me[ 10 ] = ( n12 * n24 * n41 - n14 * n22 * n41 + n14 * n21 * n42 - n11 * n24 * n42 - n12 * n21 * n44 + n11 * n22 * n44 ) * det_inv;
        me[ 11 ] = ( n13 * n22 * n41 - n12 * n23 * n41 - n13 * n21 * n42 + n11 * n23 * n42 + n12 * n21 * n43 - n11 * n22 * n43 ) * det_inv;

        me[ 12 ] = t14 * det_inv;
        me[ 13 ] = ( n13 * n24 * n31 - n14 * n23 * n31 + n14 * n21 * n33 - n11 * n24 * n33 - n13 * n21 * n34 + n11 * n23 * n34 ) * det_inv;
        me[ 14 ] = ( n14 * n22 * n31 - n12 * n24 * n31 - n14 * n21 * n32 + n11 * n24 * n32 + n12 * n21 * n34 - n11 * n22 * n34 ) * det_inv;
        me[ 15 ] = ( n12 * n23 * n31 - n13 * n22 * n31 + n13 * n21 * n32 - n11 * n23 * n32 - n12 * n21 * n33 + n11 * n22 * n33 ) * det_inv;
    }

    pub fn rotate_y(&mut self, theta: T) {
        let c = T::cos( theta );
        let s = T::sin( theta );

        let zero = T::zero();
        let one = T::one();

        self.set([
            c, zero, s, zero,
            zero, one, zero, zero,
            -s, zero, c, zero,
            zero, zero, zero, one
        ]);
    }

    pub fn rotate_z(&mut self, theta: T) {
        let c = T::cos(theta);
        let s = T::sin(theta);

        let zero = T::zero();
        let one = T::one();

        self.set([
            c, -s, zero, zero,
            s, c, zero, zero,
            zero, zero, one, zero,
            zero, zero, zero, one
        ]);
    }

    pub fn rotate_x(&mut self, theta: T) {
        let c = T::cos(theta);
        let s = T::sin(theta);

        let zero = T::zero();
        let one = T::one();

        self.set([
            one, zero, zero, zero,
            zero, c, -s, zero,
            zero, s, c, zero,
            zero, zero, zero, one
        ]);
    }

    pub fn from_axis_angle(axis: &Vector3<T>, angle: T) -> Self {
        let c = Float::cos( angle );
        let s = Float::sin( angle );
        let t = T::from(1.).unwrap() - c;
        let x = T::from(axis.x).unwrap();
        let y = T::from(axis.y).unwrap();
        let z = T::from(axis.z).unwrap();

        let tx = t * x;
        let ty = t * y;

        let zero = T::from(0.).unwrap();
        let one = T::from(1.).unwrap();

        Matrix4::new([
            tx * x + c, tx * y - s * z, tx * z + s * y, zero,
            tx * y + s * z, ty * y + c, ty * z - s * x, zero,
            tx * z - s * y, ty * z + s * x, t * z * z + c, zero,
            zero, zero, zero, one
        ])
    }

    pub fn look_at(&mut self, position: &Vector3<T>, target: &Vector3<T>, up: &Vector3<T>)  {
        let mut z = position - target ;

        z.normalize();
        let mut x = up.clone();
        x.cross(&z);

        x.normalize();
        let mut  y = z.clone();
        y.cross(&x);

        self.elements[ 0 ] = x.x; self.elements[ 4 ] = y.x; self.elements[ 8 ] = z.x;
        self.elements[ 1 ] = x.y; self.elements[ 5 ] = y.y; self.elements[ 9 ] = z.y;
        self.elements[ 2 ] = x.z; self.elements[ 6 ] = y.z; self.elements[ 10 ] = z.z;
    }
}

impl <T: Float> PartialEq for Matrix4<T> {
    fn eq(&self, other: &Matrix4<T>) -> bool {
        for (i, item) in self.elements.iter().enumerate() {
            if *item != other.elements[i] as T {
                return false
            }
        }

        true
    }
}
