use crate::*;
use bincode::{config, Decode, Encode};

use std::ops::{Mul, Add, Sub};

#[derive(Encode, Decode, PartialEq, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }

    pub fn scalar_multiply(&self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }

    pub fn magnitude(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }

    pub fn squared_magnitude(&self)  -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn normalize(&self) -> Vec3 {
        let magnitude = self.magnitude();
        return Vec3 {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude
        }
    }
    
    pub fn dot_product(&self, other: Vec3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross_product(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn lerp(&self, other: &Vec3, amount_between: f32) -> Vec3 {
        Vec3 {
            x: (1.0 - amount_between) * self.x + amount_between * other.x,
            y: (1.0 - amount_between) * self.z + amount_between * other.z,
            z: (1.0 - amount_between) * self.z + amount_between * other.z,
        }
    }
}

#[derive(Encode, Decode, PartialEq, Debug)]
pub struct Matrix3x3 {
    m11:f32,
    m12:f32,
    m13:f32,
  
    m21:f32,
    m22:f32,
    m23:f32,
  
    m31:f32,
    m32:f32,
    m33:f32,
  }

impl Matrix3x3 {
    pub fn new(  
        m11: f32,
        m12: f32,
        m13: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m31: f32,
        m32: f32,
        m33: f32) -> Matrix3x3 {
        Matrix3x3 {
            m11,
            m12,
            m13,

            m21,
            m22,
            m23,
            
            m31,
            m32,
            m33
        }
    }
}

impl Mul for Matrix3x3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Matrix3x3 {
        Matrix3x3 {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32,
            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33,

            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32,
            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33,

            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32,
            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33,
        }
    }
}


#[cfg(test)]
mod three_d_math_tests {
    use crate::*;
    #[test]
    fn vector_addition_test() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(a + b, Vec3::new(3.0, 3.0, 3.0))
    }

    #[test]
    fn vector_scalar_multiply_test() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = a.scalar_multiply(2.0);
        assert_eq!(b, Vec3::new(2.0, 2.0, 2.0));
        let c = b.scalar_multiply(0.0);
        assert_eq!(c, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vector_vector_multiplication_test() {
        let a = Vec3::new(5.0, 5.0, 5.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(a * b, Vec3::new(10.0, 10.0, 10.0))
    }

    #[test]
    fn matrix3x3_multiplication_test() {
        let a = Matrix3x3::new(1.0, 1.0, 1.0,
                                2.0, 2.0, 2.0,
                                3.0, 3.0, 3.0);

        let b = Matrix3x3::new(1.0, 1.0, 1.0,
                                2.0, 2.0, 2.0,
                                3.0, 3.0, 3.0);

        let result = Matrix3x3::new(6.0,  6.0,  6.0, 
                                     12.0, 12.0, 12.0,
                                     18.0, 18.0, 18.0);
        assert_eq!(a * b, result);
    }

}

