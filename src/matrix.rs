use cast::transmute;
use cmp::Eq;
use num::from_int;
use ptr::to_unsafe_ptr;
use vec::raw::buf_as_slice;
use std::cmp::FuzzyEq;

use math::*;
use num_util::*;
use quaternion::{Quat, ToQuat};
use vector::{Vec2, Vec3, Vec4};

//
//  NxN Matrix
//
pub trait Matrix<T, V> {
    pure fn rows() -> uint;
    pure fn cols() -> uint;
    pure fn is_col_major() -> bool;
    
    pure fn col(i: uint) -> V;
    pure fn row(i: uint) -> V;
    
    pure fn mul_t(value: T) -> self;
    pure fn mul_v(other: &V) -> V;
    pure fn add_m(other: &self) -> self;
    pure fn sub_m(other: &self) -> self;
    pure fn mul_m(other: &self) -> self;
    
    // pure fn invert(other: &self) -> self;
    pure fn transpose() -> self;
    
    pure fn is_identity() -> bool;
    pure fn is_symmetric() -> bool;
    pure fn is_diagonal() -> bool;
    pure fn is_rotated() -> bool;
}

//
//  3x3 Matrix
//
pub trait Matrix3<T> {
    pure fn scale(vec: &Vec3<T>) -> self;
    pure fn to_Mat4() -> Mat4<T>;
}

//
//  4x4 Matrix
//
pub trait Matrix4<T> {
    pure fn scale(vec: &Vec3<T>) -> self;
    pure fn translate(vec: &Vec3<T>) -> self;
}






//
//  Mat2: A 2x2, column major matrix
//
pub struct Mat2<T> { x: Vec2<T>, y: Vec2<T> }

pub mod Mat2 {
    
    #[inline(always)]
    pub pure fn new<T:Copy>(c0r0: T, c0r1: T,
                            c1r0: T, c1r1: T) -> Mat2<T> {
        Mat2::from_cols(&Vec2::new(c0r0, c0r1),
                        &Vec2::new(c1r0, c1r1))
    }

    #[inline(always)]
    pub pure fn from_cols<T:Copy>(c0: &Vec2<T>, c1: &Vec2<T>) -> Mat2<T> {
        Mat2 { x: *c0,
               y: *c1 }
    }
    
    #[inline(always)]
    pub pure fn zero<T:Copy Num>() -> Mat2<T> {
        Mat2 { x: Vec2::zero(),
               y: Vec2::zero() }
    }
    
    #[inline(always)]
    pub pure fn identity<T:Copy Num>() -> Mat2<T> {
        Mat2 { x: Vec2::unit_x(),
               y: Vec2::unit_y() }
    }
}

pub impl<T:Copy Num Sqrt FuzzyEq> Mat2<T>: Matrix<T, Vec2<T>> {
    #[inline(always)]
    pure fn rows() -> uint { 2 }
    
    #[inline(always)]
    pure fn cols() -> uint { 2 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn col(i: uint) -> Vec2<T> { self[i] }
    
    #[inline(always)]
    pure fn row(i: uint) -> Vec2<T> {
        Vec2::new(self[0][i],
                  self[1][i])
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Mat2<T> {
        Mat2::from_cols(&self[0].mul_t(value),
                        &self[1].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(other: &Vec2<T>) -> Vec2<T> {
        Vec2::new(self[0][0] * other[0] + self[1][0] * other[1],
                  self[0][1] * other[0] + self[1][1] * other[1])
    }
    
    #[inline(always)]
    pure fn add_m(other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(&self[0].add_v(&other[0]),
                        &self[1].add_v(&other[1]))
    }
    
    #[inline(always)]
    pure fn sub_m(other: &Mat2<T>) -> Mat2<T> {
        Mat2::from_cols(&self[0].sub_v(&other[0]),
                        &self[1].sub_v(&other[1]))
    }
    
    #[inline(always)]
    pure fn mul_m(other: &Mat2<T>) -> Mat2<T> {
        Mat2::new(self[0][0] * other[0][0] + self[1][0] * other[0][1],
                  self[0][1] * other[0][0] + self[1][1] * other[0][1],
                  
                  self[0][0] * other[1][0] + self[1][0] * other[1][1],
                  self[0][1] * other[1][0] + self[1][1] * other[1][1])
    }
    
    // TODO - inversion is harrrd D:
    // #[inline(always)]
    // pure fn invert(other: &Mat2<T>) -> Mat2<T> {}
    
    #[inline(always)]
    pure fn transpose() -> Mat2<T> {
        Mat2::new(self[0][0], self[1][0],
                  self[0][1], self[1][1])
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(&Mat2::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric() -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[1][0].fuzzy_eq(&self[0][1])
    }
    
    #[inline(always)]
    pure fn is_diagonal() -> bool {
        self[0][1].fuzzy_eq(&from_int(0)) &&
        self[1][0].fuzzy_eq(&from_int(0))
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat2::identity())
    }
}

pub impl<T:Copy> Mat2<T>: Index<uint, Vec2<T>> {
    #[inline(always)]
    pure fn index(i: uint) -> Vec2<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat2<T>, *Vec2<T>>(
                to_unsafe_ptr(&self)), 2) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy Neg<T>> Mat2<T>: Neg<Mat2<T>> {
    #[inline(always)]
    pure fn neg() -> Mat2<T> {
        Mat2::from_cols(&-self[0], &-self[1])
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy FuzzyEq> Mat2<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Mat2<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Mat2<T>) -> bool {
        !(self == *other)
    }
}

impl<T:Copy Eq> Mat2<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Mat2<T>) -> bool {
        self[0].exact_eq(&other[0]) &&
        self[1].exact_eq(&other[1])
    }
}

pub impl<T:Copy FuzzyEq> Mat2<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat2<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1])
    }
}






//
//  Mat3: A 3x3, column major matrix
//
pub struct Mat3<T> { x: Vec3<T>, y: Vec3<T>, z: Vec3<T> }

pub mod Mat3 {
    
    #[inline(always)]
    pub pure fn new<T:Copy>(c0r0:T, c0r1:T, c0r2:T,
                            c1r0:T, c1r1:T, c1r2:T,
                            c2r0:T, c2r1:T, c2r2:T) -> Mat3<T> {
        Mat3::from_cols(&Vec3::new(c0r0, c0r1, c0r2),
                        &Vec3::new(c1r0, c1r1, c1r2),
                        &Vec3::new(c2r0, c2r1, c2r2))
    }
    
    #[inline(always)]
    pub pure fn from_cols<T:Copy>(c0: &Vec3<T>, c1: &Vec3<T>, c2: &Vec3<T>) -> Mat3<T> {
        Mat3 { x: *c0,
               y: *c1,
               z: *c2 }
    }
    
    #[inline(always)]
    pub pure fn zero<T:Num>() -> Mat3<T> {
        Mat3 { x: Vec3::zero(),
               y: Vec3::zero(),
               z: Vec3::zero() }
    }
    
    #[inline(always)]
    pub pure fn identity<T:Num>() -> Mat3<T> {
        Mat3 { x: Vec3::unit_x(),
               y: Vec3::unit_y(),
               z: Vec3::unit_z() }
    }
}

pub impl<T:Copy Num Sqrt FuzzyEq> Mat3<T>: Matrix<T, Vec3<T>> {
    #[inline(always)]
    pure fn rows() -> uint { 3 }
    
    #[inline(always)]
    pure fn cols() -> uint { 3 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn col(i: uint) -> Vec3<T> { self[i] }
    
    #[inline(always)]
    pure fn row(i: uint) -> Vec3<T> {
        Vec3::new(self[0][i],
                  self[1][i],
                  self[2][i])
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Mat3<T> {
        Mat3::from_cols(&self[0].mul_t(value),
                        &self[1].mul_t(value),
                        &self[2].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self[0][0] * other[0] + self[1][0] * other[1] + self[2][0] * other[2],
                  self[0][1] * other[0] + self[1][1] * other[1] + self[2][1] * other[2],
                  self[0][2] * other[0] + self[1][2] * other[1] + self[2][2] * other[2])
    }
    
    #[inline(always)]
    pure fn add_m(other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(&self[0].add_v(&other[0]),
                        &self[1].add_v(&other[1]),
                        &self[2].add_v(&other[2]))
    }
    
    #[inline(always)]
    pure fn sub_m(other: &Mat3<T>) -> Mat3<T> {
        Mat3::from_cols(&self[0].sub_v(&other[0]),
                        &self[1].sub_v(&other[1]),
                        &self[2].sub_v(&other[2]))
    }
    
    #[inline(always)]
    pure fn mul_m(other: &Mat3<T>) -> Mat3<T> {
        Mat3::new(self[0][0] * other[0][0] + self[1][0] * other[0][1] + self[2][0] * other[0][2],
                  self[0][1] * other[0][0] + self[1][1] * other[0][1] + self[2][1] * other[0][2],
                  self[0][2] * other[0][0] + self[1][2] * other[0][1] + self[2][2] * other[0][2],
                  
                  self[0][0] * other[1][0] + self[1][0] * other[1][1] + self[2][0] * other[1][2],
                  self[0][1] * other[1][0] + self[1][1] * other[1][1] + self[2][1] * other[1][2],
                  self[0][2] * other[1][0] + self[1][2] * other[1][1] + self[2][2] * other[1][2],
                  
                  self[0][0] * other[2][0] + self[1][0] * other[2][1] + self[2][0] * other[2][2],
                  self[0][1] * other[2][0] + self[1][1] * other[2][1] + self[2][1] * other[2][2],
                  self[0][2] * other[2][0] + self[1][2] * other[2][1] + self[2][2] * other[2][2])
    }
    
    // TODO - inversion is harrrd D:
    // #[inline(always)]
    // pure fn invert(other: &Mat3) -> Mat3 {}
    
    #[inline(always)]
    pure fn transpose() -> Mat3<T> {
        Mat3::new(self[0][0], self[1][0], self[2][0],
                  self[0][1], self[1][1], self[2][1],
                  self[0][2], self[1][2], self[2][2])
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(&Mat3::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric() -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[0][2].fuzzy_eq(&self[2][0]) &&
        
        self[1][0].fuzzy_eq(&self[0][1]) &&
        self[1][2].fuzzy_eq(&self[2][1]) &&
        
        self[2][0].fuzzy_eq(&self[0][2]) &&
        self[2][1].fuzzy_eq(&self[1][2])
    }
    
    #[inline(always)]
    pure fn is_diagonal() -> bool {
        self[0][1].fuzzy_eq(&from_int(0)) &&
        self[0][2].fuzzy_eq(&from_int(0)) &&
        
        self[1][0].fuzzy_eq(&from_int(0)) &&
        self[1][2].fuzzy_eq(&from_int(0)) &&
        
        self[2][0].fuzzy_eq(&from_int(0)) &&
        self[2][1].fuzzy_eq(&from_int(0))
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat3::identity())
    }
}

pub impl<T:Copy Num Sqrt FuzzyEq> Mat3<T>: Matrix3<T> {
    #[inline(always)]
    pure fn scale(vec: &Vec3<T>) -> Mat3<T> {
        self.mul_m(&Mat3::new(      vec.x, from_int(0), from_int(0),
                              from_int(0),       vec.y, from_int(0),
                              from_int(0), from_int(0),      vec.z))
    }
    
    #[inline(always)]
    pure fn to_Mat4() -> Mat4<T> {
        Mat4::new( self[0][0],  self[0][1],   self[0][2], from_int(0),
                   self[1][0],  self[1][1],   self[1][2], from_int(0),
                   self[2][0],  self[2][1],   self[2][2], from_int(0),
                  from_int(0), from_int(0),  from_int(0), from_int(1))
    }
}

pub impl<T:Copy Num NumCast Ord> Mat3<T>: ToQuat<T> {
    pure fn to_Quat() -> Quat<T> {
        // Implemented using a mix of ideas from jMonkeyEngine and Ken Shoemake's
        // paper on Quaternions: http://www.cs.ucr.edu/~vbz/resources/Quatut.pdf
        
        let mut s: float;
        let w: float, x: float, y: float, z: float;
        let trace: float = cast(self[0][0] + self[1][1] + self[2][2]);
        
        if trace >= from_int(0) {
            s = (trace + 1f).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = (self[1][2] - self[2][1]).cast::<float>() * s;
            y = (self[2][0] - self[0][2]).cast::<float>() * s;
            z = (self[0][1] - self[1][0]).cast::<float>() * s;
        } else if (self[0][0] > self[1][1]) && (self[0][0] > self[2][2]) {
            s = (1f + (self[0][0] - self[1][1] - self[2][2]).cast::<float>()).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = (self[0][1] - self[1][0]).cast::<float>() * s;
            y = (self[2][0] - self[0][2]).cast::<float>() * s;
            z = (self[1][2] - self[2][1]).cast::<float>() * s;
        } else if self[1][1] > self[2][2] {
            s = (1f + (self[1][1] - self[0][0] - self[2][2]).cast::<float>()).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = (self[0][1] - self[1][0]).cast::<float>() * s;
            y = (self[1][2] - self[2][1]).cast::<float>() * s;
            z = (self[2][0] - self[0][2]).cast::<float>() * s;
        } else {
            s = (1f + (self[2][2] - self[0][0] - self[1][1]).cast::<float>()).sqrt();
            w = 0.5 * s;
            s = 0.5 / s;
            x = (self[2][0] - self[0][2]).cast::<float>() * s;
            y = (self[1][2] - self[2][1]).cast::<float>() * s;
            z = (self[0][1] - self[1][0]).cast::<float>() * s;
        }
        
        Quat::new(cast(w), cast(x), cast(y), cast(z))
    }
}

pub impl<T:Copy> Mat3<T>: Index<uint, Vec3<T>> {
    #[inline(always)]
    pure fn index(i: uint) -> Vec3<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat3<T>, *Vec3<T>>(
                to_unsafe_ptr(&self)), 3) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy Neg<T>> Mat3<T>: Neg<Mat3<T>> {
    #[inline(always)]
    pure fn neg() -> Mat3<T> {
        Mat3::from_cols(&-self[0], &-self[1], &-self[2])
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy FuzzyEq> Mat3<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Mat3<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Mat3<T>) -> bool {
        !(self == *other)
    }
}

pub impl<T:Copy Eq> Mat3<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Mat3<T>) -> bool {
        self[0].exact_eq(&other[0]) &&
        self[1].exact_eq(&other[1]) &&
        self[2].exact_eq(&other[2])
    }
}

pub impl<T:Copy FuzzyEq> Mat3<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat3<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2])
    }
}

pub impl<T:Copy> Mat3<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr() -> *T {
        self[0].to_ptr()
    }
}






//
//  Mat4: A 4x4, column major matrix
//
pub struct Mat4<T> { x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T> }

pub mod Mat4 {
    
    #[inline(always)]
    pub pure fn new<T:Copy>(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
                            c1r0: T, c1r1: T, c1r2: T, c1r3: T,
                            c2r0: T, c2r1: T, c2r2: T, c2r3: T,
                            c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> Mat4<T>  {
        Mat4::from_cols(&Vec4::new(c0r0, c0r1, c0r2, c0r3),
                        &Vec4::new(c1r0, c1r1, c1r2, c1r3),
                        &Vec4::new(c2r0, c2r1, c2r2, c2r3),
                        &Vec4::new(c3r0, c3r1, c3r2, c3r3))
    }
    
    #[inline(always)]
    pub pure fn from_cols<T:Copy>(c0: &Vec4<T>, c1: &Vec4<T>, c2: &Vec4<T>, c3: &Vec4<T>) -> Mat4<T> {
        Mat4 { x: *c0,
               y: *c1,
               z: *c2,
               w: *c3 }
    }
    
    #[inline(always)]
    pub pure fn zero<T:Num>() -> Mat4<T> {
        Mat4 { x: Vec4::zero(),
               y: Vec4::zero(),
               z: Vec4::zero(),
               w: Vec4::zero() }
    }
    
    #[inline(always)]
    pub pure fn identity<T:Num>() -> Mat4<T> {
        Mat4 { x: Vec4::unit_x(),
               y: Vec4::unit_y(),
               z: Vec4::unit_z(),
               w: Vec4::unit_w() }
    }
}

pub impl<T:Copy Num Sqrt FuzzyEq> Mat4<T>: Matrix<T, Vec4<T>> {
    #[inline(always)]
    pure fn rows() -> uint { 4 }
    
    #[inline(always)]
    pure fn cols() -> uint { 4 }
    
    #[inline(always)]
    pure fn is_col_major() -> bool { true }
    
    #[inline(always)]
    pure fn col(i: uint) -> Vec4<T> { self[i] }
    
    #[inline(always)]
    pure fn row(i: uint) -> Vec4<T> {
        Vec4::new(self[0][i],
                  self[1][i],
                  self[2][i],
                  self[3][i])
    }
    
    #[inline(always)]
    pure fn mul_t(value: T) -> Mat4<T> {
        Mat4::from_cols(&self[0].mul_t(value),
                        &self[1].mul_t(value),
                        &self[2].mul_t(value),
                        &self[3].mul_t(value))
    }
    
    #[inline(always)]
    pure fn mul_v(other: &Vec4<T>) -> Vec4<T> {
        Vec4::new(self[0][0] * other[0] + self[1][0] * other[1] + self[2][0] * other[2] + self[3][0] * other[3],
                  self[0][1] * other[0] + self[1][1] * other[1] + self[2][1] * other[2] + self[3][1] * other[3],
                  self[0][2] * other[0] + self[1][2] * other[1] + self[2][2] * other[2] + self[3][2] * other[3],
                  self[0][3] * other[0] + self[1][3] * other[1] + self[2][3] * other[2] + self[3][3] * other[3])
    }
    
    #[inline(always)]
    pure fn add_m(other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(&self[0].add_v(&other[0]),
                        &self[1].add_v(&other[1]),
                        &self[2].add_v(&other[2]),
                        &self[3].add_v(&other[3]))
    }
    
    #[inline(always)]
    pure fn sub_m(other: &Mat4<T>) -> Mat4<T> {
        Mat4::from_cols(&self[0].sub_v(&other[0]),
                        &self[1].sub_v(&other[1]),
                        &self[2].sub_v(&other[2]),
                        &self[3].sub_v(&other[3]))
    }
    
    #[inline(always)]
    pure fn mul_m(other: &Mat4<T>) -> Mat4<T> {
        Mat4::new(self[0][0] * other[0][0] + self[1][0] * other[0][1] + self[2][0] * other[0][2] + self[3][0] * other[0][3],
                  self[0][1] * other[0][0] + self[1][1] * other[0][1] + self[2][1] * other[0][2] + self[3][1] * other[0][3],
                  self[0][2] * other[0][0] + self[1][2] * other[0][1] + self[2][2] * other[0][2] + self[3][2] * other[0][3],
                  self[0][3] * other[0][0] + self[1][3] * other[0][1] + self[2][3] * other[0][2] + self[3][3] * other[0][3],
                  
                  self[0][0] * other[1][0] + self[1][0] * other[1][1] + self[2][0] * other[1][2] + self[3][0] * other[1][3],
                  self[0][1] * other[1][0] + self[1][1] * other[1][1] + self[2][1] * other[1][2] + self[3][1] * other[1][3],
                  self[0][2] * other[1][0] + self[1][2] * other[1][1] + self[2][2] * other[1][2] + self[3][2] * other[1][3],
                  self[0][3] * other[1][0] + self[1][3] * other[1][1] + self[2][3] * other[1][2] + self[3][3] * other[1][3],
                  
                  self[0][0] * other[2][0] + self[1][0] * other[2][1] + self[2][0] * other[2][2] + self[3][0] * other[2][3],
                  self[0][1] * other[2][0] + self[1][1] * other[2][1] + self[2][1] * other[2][2] + self[3][1] * other[2][3],
                  self[0][2] * other[2][0] + self[1][2] * other[2][1] + self[2][2] * other[2][2] + self[3][2] * other[2][3],
                  self[0][3] * other[2][0] + self[1][3] * other[2][1] + self[2][3] * other[2][2] + self[3][3] * other[2][3],
                  
                  self[0][0] * other[3][0] + self[1][0] * other[3][1] + self[2][0] * other[3][2] + self[3][0] * other[3][3],
                  self[0][1] * other[3][0] + self[1][1] * other[3][1] + self[2][1] * other[3][2] + self[3][1] * other[3][3],
                  self[0][2] * other[3][0] + self[1][2] * other[3][1] + self[2][2] * other[3][2] + self[3][2] * other[3][3],
                  self[0][3] * other[3][0] + self[1][3] * other[3][1] + self[2][3] * other[3][2] + self[3][3] * other[3][3])
    }
    
    // TODO - inversion is harrrd D:
    // #[inline(always)]
    // pure fn invert(other: &Mat4<T>) -> Mat4<T> {}
    
    #[inline(always)]
    pure fn transpose() -> Mat4<T> {
        Mat4::new(self[0][0], self[1][0], self[2][0], self[3][0],
                  self[0][1], self[1][1], self[2][1], self[3][1],
                  self[0][2], self[1][2], self[2][2], self[3][2],
                  self[0][3], self[1][3], self[2][3], self[3][3])
    }
    
    #[inline(always)]
    pure fn is_identity() -> bool {
        self.fuzzy_eq(&Mat4::identity())
    }
    
    #[inline(always)]
    pure fn is_symmetric() -> bool {
        self[0][1].fuzzy_eq(&self[1][0]) &&
        self[0][2].fuzzy_eq(&self[2][0]) &&
        self[0][3].fuzzy_eq(&self[3][0]) &&
        
        self[1][0].fuzzy_eq(&self[0][1]) &&
        self[1][2].fuzzy_eq(&self[2][1]) &&
        self[1][3].fuzzy_eq(&self[3][1]) &&
        
        self[2][0].fuzzy_eq(&self[0][2]) &&
        self[2][1].fuzzy_eq(&self[1][2]) &&
        self[2][3].fuzzy_eq(&self[3][2]) &&
        
        self[3][0].fuzzy_eq(&self[0][3]) &&
        self[3][1].fuzzy_eq(&self[1][3]) &&
        self[3][2].fuzzy_eq(&self[2][3])
    }
    
    #[inline(always)]
    pure fn is_diagonal() -> bool {
        self[0][1].fuzzy_eq(&from_int(0)) &&
        self[0][2].fuzzy_eq(&from_int(0)) &&
        self[0][3].fuzzy_eq(&from_int(0)) &&
        
        self[1][0].fuzzy_eq(&from_int(0)) &&
        self[1][2].fuzzy_eq(&from_int(0)) &&
        self[1][3].fuzzy_eq(&from_int(0)) &&
        
        self[2][0].fuzzy_eq(&from_int(0)) &&
        self[2][1].fuzzy_eq(&from_int(0)) &&
        self[2][3].fuzzy_eq(&from_int(0)) &&
        
        self[3][0].fuzzy_eq(&from_int(0)) &&
        self[3][1].fuzzy_eq(&from_int(0)) &&
        self[3][2].fuzzy_eq(&from_int(0))
    }
    
    #[inline(always)]
    pure fn is_rotated() -> bool {
        !self.fuzzy_eq(&Mat4::identity())
    }
}

pub impl<T:Copy Num Sqrt FuzzyEq> Mat4<T>: Matrix4<T> {
    #[inline(always)]
    pure fn scale(vec: &Vec3<T>) -> Mat4<T> {
        self.mul_m(&Mat4::new(      vec.x, from_int(0), from_int(0), from_int(0),
                              from_int(0),       vec.y, from_int(0), from_int(0),
                              from_int(0), from_int(0),       vec.z, from_int(0),
                              from_int(0), from_int(0), from_int(0), from_int(1)))
    }
    
    #[inline(always)]
    pure fn translate(vec: &Vec3<T>) -> Mat4<T> {
        Mat4::from_cols(&self[0],
                        &self[1],
                        &self[2],
                        &Vec4::new(self[3][0] + vec.x,
                                   self[3][1] + vec.y,
                                   self[3][2] + vec.z,
                                   self[3][3]))
    }
}

pub impl<T:Copy> Mat4<T>: Index<uint, Vec4<T>> {
    #[inline(always)]
    pure fn index(i: uint) -> Vec4<T> {
        unsafe { do buf_as_slice(
            transmute::<*Mat4<T>, *Vec4<T>>(
                to_unsafe_ptr(&self)), 4) |slice| { slice[i] }
        }
    }
}

pub impl<T:Copy Neg<T>> Mat4<T>: Neg<Mat4<T>> {
    #[inline(always)]
    pure fn neg() -> Mat4<T> {
        Mat4::from_cols(&-self[0], &-self[1], &-self[2], &-self[3])
    }
}

// TODO: make work for T:Integer
pub impl<T:Copy FuzzyEq> Mat4<T>: Eq {
    #[inline(always)]
    pure fn eq(other: &Mat4<T>) -> bool {
        self.fuzzy_eq(other)
    }
    
    #[inline(always)]
    pure fn ne(other: &Mat4<T>) -> bool {
        !(self == *other)
    }
}

pub impl<T:Copy Eq> Mat4<T>: ExactEq {
    #[inline(always)]
    pure fn exact_eq(other: &Mat4<T>) -> bool {
        self[0].exact_eq(&other[0]) &&
        self[1].exact_eq(&other[1]) &&
        self[2].exact_eq(&other[2]) &&
        self[3].exact_eq(&other[3])
    }
}

pub impl<T:Copy FuzzyEq> Mat4<T>: FuzzyEq {
    #[inline(always)]
    pure fn fuzzy_eq(other: &Mat4<T>) -> bool {
        self[0].fuzzy_eq(&other[0]) &&
        self[1].fuzzy_eq(&other[1]) &&
        self[2].fuzzy_eq(&other[2]) &&
        self[3].fuzzy_eq(&other[3])
    }
}

pub impl<T:Copy> Mat4<T>: ToPtr<T> {
    #[inline(always)]
    pure fn to_ptr() -> *T {
        self[0].to_ptr()
    }
}