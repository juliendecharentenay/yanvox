use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub, Mul};
use super::Vec3i;

/// 3D vector with floating point coordinates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn scale(self, f: f32) -> Self {
      Self {
        x: self.x * f,
        y: self.y * f,
        z: self.z * f,
      }
    }

    pub fn cross(&self, other: &Self) -> Self {
      Self {
        x:  self.y * other.z - self.z * other.y,
        y:  self.z * other.x - self.x * other.z,
        z:  self.x * other.y - self.y * other.x,
      }
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Self::zero()
        }
    }

    pub fn as_vec3i(&self) -> Vec3i {
      Vec3i::new(self.x as i32, self.y as i32, self.z as i32)
    }
}

impl Add for Vec3f {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3f {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_zero() {
        let v = Vec3f::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_one() {
        let v = Vec3f::one();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.z, 1.0);
    }

    #[test]
    fn test_length() {
        let v = Vec3f::new(3.0, 4.0, 0.0);
        assert!((v.length() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_length_zero() {
        let v = Vec3f::zero();
        assert!((v.length() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_length_negative() {
        let v = Vec3f::new(-3.0, -4.0, 0.0);
        assert!((v.length() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_length_3d() {
        let v = Vec3f::new(1.0, 2.0, 2.0);
        let expected = (1.0_f32 + 4.0_f32 + 4.0_f32).sqrt(); // sqrt(9) = 3.0
        assert!((v.length() - expected).abs() < f32::EPSILON);
    }

    #[test]
    fn test_scale() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        let result = v.scale(2.0);
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_scale_zero() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        let result = v.scale(0.0);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_scale_negative() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        let result = v.scale(-2.0);
        assert_eq!(result.x, -2.0);
        assert_eq!(result.y, -4.0);
        assert_eq!(result.z, -6.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);
        let result = v1.cross(&v2);
        assert!((result.x - 0.0).abs() < f32::EPSILON);
        assert!((result.y - 0.0).abs() < f32::EPSILON);
        assert!((result.z - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_cross_product_anticommutative() {
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);
        let result1 = v1.cross(&v2);
        let result2 = v2.cross(&v1);
        assert!((result1.x + result2.x).abs() < f32::EPSILON);
        assert!((result1.y + result2.y).abs() < f32::EPSILON);
        assert!((result1.z + result2.z).abs() < f32::EPSILON);
    }

    #[test]
    fn test_cross_product_parallel() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(2.0, 4.0, 6.0); // v2 = 2 * v1
        let result = v1.cross(&v2);
        assert!((result.x - 0.0).abs() < f32::EPSILON);
        assert!((result.y - 0.0).abs() < f32::EPSILON);
        assert!((result.z - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_cross_product_3d() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(4.0, 5.0, 6.0);
        let result = v1.cross(&v2);
        // Expected: (2*6-3*5, 3*4-1*6, 1*5-2*4) = (-3, 6, -3)
        assert!((result.x - (-3.0)).abs() < f32::EPSILON);
        assert!((result.y - 6.0).abs() < f32::EPSILON);
        assert!((result.z - (-3.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_normalize() {
        let v = Vec3f::new(3.0, 4.0, 0.0);
        let normalized = v.normalize();
        assert!((normalized.length() - 1.0).abs() < f32::EPSILON);
        assert!((normalized.x - 0.6).abs() < f32::EPSILON);
        assert!((normalized.y - 0.8).abs() < f32::EPSILON);
        assert!((normalized.z - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_normalize_zero() {
        let v = Vec3f::zero();
        let normalized = v.normalize();
        assert_eq!(normalized, Vec3f::zero());
    }

    #[test]
    fn test_normalize_negative() {
        let v = Vec3f::new(-3.0, -4.0, 0.0);
        let normalized = v.normalize();
        assert!((normalized.length() - 1.0).abs() < f32::EPSILON);
        assert!((normalized.x - (-0.6)).abs() < f32::EPSILON);
        assert!((normalized.y - (-0.8)).abs() < f32::EPSILON);
        assert!((normalized.z - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_normalize_3d() {
        let v = Vec3f::new(1.0, 1.0, 1.0);
        let normalized = v.normalize();
        assert!((normalized.length() - 1.0).abs() < f32::EPSILON);
        let expected = 1.0 / 3.0_f32.sqrt();
        assert!((normalized.x - expected).abs() < f32::EPSILON);
        assert!((normalized.y - expected).abs() < f32::EPSILON);
        assert!((normalized.z - expected).abs() < f32::EPSILON);
    }

    #[test]
    fn test_as_vec3i() {
        let v = Vec3f::new(1.5, 2.7, 3.9);
        let i = v.as_vec3i();
        assert_eq!(i.x, 1);
        assert_eq!(i.y, 2);
        assert_eq!(i.z, 3);
    }

    #[test]
    fn test_as_vec3i_negative() {
        let v = Vec3f::new(-1.5, -2.7, -3.9);
        let i = v.as_vec3i();
        assert_eq!(i.x, -1);
        assert_eq!(i.y, -2);
        assert_eq!(i.z, -3);
    }

    #[test]
    fn test_as_vec3i_zero() {
        let v = Vec3f::zero();
        let i = v.as_vec3i();
        assert_eq!(i.x, 0);
        assert_eq!(i.y, 0);
        assert_eq!(i.z, 0);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert!((result.x - 5.0).abs() < f32::EPSILON);
        assert!((result.y - 7.0).abs() < f32::EPSILON);
        assert!((result.z - 9.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_add_zero() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::zero();
        let result = v1 + v2;
        assert!((result.x - 1.0).abs() < f32::EPSILON);
        assert!((result.y - 2.0).abs() < f32::EPSILON);
        assert!((result.z - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_add_negative() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(-1.0, -2.0, -3.0);
        let result = v1 + v2;
        assert!((result.x - 0.0).abs() < f32::EPSILON);
        assert!((result.y - 0.0).abs() < f32::EPSILON);
        assert!((result.z - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3f::new(5.0, 7.0, 9.0);
        let v2 = Vec3f::new(1.0, 2.0, 3.0);
        let result = v1 - v2;
        assert!((result.x - 4.0).abs() < f32::EPSILON);
        assert!((result.y - 5.0).abs() < f32::EPSILON);
        assert!((result.z - 6.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_sub_zero() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::zero();
        let result = v1 - v2;
        assert!((result.x - 1.0).abs() < f32::EPSILON);
        assert!((result.y - 2.0).abs() < f32::EPSILON);
        assert!((result.z - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_sub_negative() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(-1.0, -2.0, -3.0);
        let result = v1 - v2;
        assert!((result.x - 2.0).abs() < f32::EPSILON);
        assert!((result.y - 4.0).abs() < f32::EPSILON);
        assert!((result.z - 6.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_mul_positive() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        let result = v * 2.0;
        assert!((result.x - 2.0).abs() < f32::EPSILON);
        assert!((result.y - 4.0).abs() < f32::EPSILON);
        assert!((result.z - 6.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_mul_zero() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        let result = v * 0.0;
        assert!((result.x - 0.0).abs() < f32::EPSILON);
        assert!((result.y - 0.0).abs() < f32::EPSILON);
        assert!((result.z - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_mul_negative() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        let result = v * -2.0;
        assert!((result.x - (-2.0)).abs() < f32::EPSILON);
        assert!((result.y - (-4.0)).abs() < f32::EPSILON);
        assert!((result.z - (-6.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_mul_one() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        let result = v * 1.0;
        assert!((result.x - 1.0).abs() < f32::EPSILON);
        assert!((result.y - 2.0).abs() < f32::EPSILON);
        assert!((result.z - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_commutative_add() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(4.0, 5.0, 6.0);
        let result1 = v1 + v2;
        let result2 = v2 + v1;
        assert!((result1.x - result2.x).abs() < f32::EPSILON);
        assert!((result1.y - result2.y).abs() < f32::EPSILON);
        assert!((result1.z - result2.z).abs() < f32::EPSILON);
    }

    #[test]
    fn test_associative_add() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(4.0, 5.0, 6.0);
        let v3 = Vec3f::new(7.0, 8.0, 9.0);
        let result1 = (v1 + v2) + v3;
        let result2 = v1 + (v2 + v3);
        assert!((result1.x - result2.x).abs() < f32::EPSILON);
        assert!((result1.y - result2.y).abs() < f32::EPSILON);
        assert!((result1.z - result2.z).abs() < f32::EPSILON);
    }

    #[test]
    fn test_distributive_mul() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        let scalar1 = 2.0;
        let scalar2 = 3.0;
        let result1 = v * (scalar1 + scalar2);
        let result2 = v * scalar1 + v * scalar2;
        assert!((result1.x - result2.x).abs() < f32::EPSILON);
        assert!((result1.y - result2.y).abs() < f32::EPSILON);
        assert!((result1.z - result2.z).abs() < f32::EPSILON);
    }

    #[test]
    fn test_equality() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(1.0, 2.0, 3.0);
        let v3 = Vec3f::new(1.0, 2.0, 4.0);
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn test_equality_epsilon() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(1.0 + f32::EPSILON, 2.0, 3.0);
        // These should NOT be equal due to exact equality
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_clone() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = v1.clone();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_copy() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = v1; // This should work because Vec3f implements Copy
        assert_eq!(v1, v2);
        assert!((v1.x - 1.0).abs() < f32::EPSILON); // v1 should still be usable
    }

    #[test]
    fn test_debug() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        let debug_str = format!("{:?}", v);
        assert!(debug_str.contains("1"));
        assert!(debug_str.contains("2"));
        assert!(debug_str.contains("3"));
    }

    #[test]
    fn test_serialize_deserialize() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        let serialized = serde_json::to_string(&v).unwrap();
        let deserialized: Vec3f = serde_json::from_str(&serialized).unwrap();
        assert_eq!(v, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_negative() {
        let v = Vec3f::new(-1.0, -2.0, -3.0);
        let serialized = serde_json::to_string(&v).unwrap();
        let deserialized: Vec3f = serde_json::from_str(&serialized).unwrap();
        assert_eq!(v, deserialized);
    }

    #[test]
    fn test_scale_vs_mul() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        let scalar = 2.0;
        let result1 = v.scale(scalar);
        let result2 = v * scalar;
        assert!((result1.x - result2.x).abs() < f32::EPSILON);
        assert!((result1.y - result2.y).abs() < f32::EPSILON);
        assert!((result1.z - result2.z).abs() < f32::EPSILON);
    }

    #[test]
    fn test_edge_cases() {
        // Test with f32::MAX and f32::MIN
        let v_max = Vec3f::new(f32::MAX, f32::MAX, f32::MAX);
        let v_min = Vec3f::new(f32::MIN, f32::MIN, f32::MIN);
        
        // Test that we can create these vectors
        assert_eq!(v_max.x, f32::MAX);
        assert_eq!(v_min.x, f32::MIN);
        
        // Test infinity
        let v_inf = Vec3f::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        assert!(v_inf.x.is_infinite());
        assert!(v_inf.y.is_infinite());
        assert!(v_inf.z.is_infinite());
        
        // Test NaN
        let v_nan = Vec3f::new(f32::NAN, f32::NAN, f32::NAN);
        assert!(v_nan.x.is_nan());
        assert!(v_nan.y.is_nan());
        assert!(v_nan.z.is_nan());
    }

    #[test]
    fn test_normalize_edge_cases() {
        // Test with very small numbers
        let v_small = Vec3f::new(f32::EPSILON, f32::EPSILON, f32::EPSILON);
        let normalized = v_small.normalize();
        assert!((normalized.length() - 1.0).abs() < f32::EPSILON);
        
        // Test with very large numbers (but not so large that they cause overflow)
        let v_large = Vec3f::new(1e6, 1e6, 1e6);
        let normalized = v_large.normalize();
        assert!((normalized.length() - 1.0).abs() < 1e-5); // Use a more lenient tolerance for large numbers
    }

    #[test]
    fn test_cross_product_properties() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(4.0, 5.0, 6.0);
        let v3 = Vec3f::new(7.0, 8.0, 9.0);
        
        // Test distributive property: v1 × (v2 + v3) = v1 × v2 + v1 × v3
        let result1 = v1.cross(&(v2 + v3));
        let result2 = v1.cross(&v2) + v1.cross(&v3);
        assert!((result1.x - result2.x).abs() < f32::EPSILON);
        assert!((result1.y - result2.y).abs() < f32::EPSILON);
        assert!((result1.z - result2.z).abs() < f32::EPSILON);
        
        // Test scalar multiplication: (a * v1) × v2 = a * (v1 × v2)
        let scalar = 2.0;
        let result1 = (v1 * scalar).cross(&v2);
        let result2 = v1.cross(&v2) * scalar;
        assert!((result1.x - result2.x).abs() < f32::EPSILON);
        assert!((result1.y - result2.y).abs() < f32::EPSILON);
        assert!((result1.z - result2.z).abs() < f32::EPSILON);
    }
}
