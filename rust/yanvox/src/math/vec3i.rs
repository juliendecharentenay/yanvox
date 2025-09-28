use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub, Mul};
use super::Vec3f;

/// 3D vector with integer coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3i {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn one() -> Self {
        Self { x: 1, y: 1, z: 1 }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    pub fn as_vec3f(&self) -> Vec3f {
        Vec3f::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

impl std::convert::From<(i32, i32, i32)> for Vec3i {
  fn from((x, y, z): (i32, i32, i32)) -> Self {
    Vec3i::new(x, y, z)
  }
}

impl Add for Vec3i {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3i {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<i32> for Vec3i {
    type Output = Self;
    fn mul(self, scalar: i32) -> Self {
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
        let v = Vec3i::new(1, 2, 3);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 3);
    }

    #[test]
    fn test_zero() {
        let v = Vec3i::zero();
        assert_eq!(v.x, 0);
        assert_eq!(v.y, 0);
        assert_eq!(v.z, 0);
    }

    #[test]
    fn test_one() {
        let v = Vec3i::one();
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 1);
        assert_eq!(v.z, 1);
    }

    #[test]
    fn test_min() {
        let v1 = Vec3i::new(5, 3, 8);
        let v2 = Vec3i::new(2, 7, 4);
        let result = v1.min(v2);
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 3);
        assert_eq!(result.z, 4);
    }

    #[test]
    fn test_min_identical() {
        let v1 = Vec3i::new(3, 3, 3);
        let v2 = Vec3i::new(3, 3, 3);
        let result = v1.min(v2);
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 3);
        assert_eq!(result.z, 3);
    }

    #[test]
    fn test_max() {
        let v1 = Vec3i::new(5, 3, 8);
        let v2 = Vec3i::new(2, 7, 4);
        let result = v1.max(v2);
        assert_eq!(result.x, 5);
        assert_eq!(result.y, 7);
        assert_eq!(result.z, 8);
    }

    #[test]
    fn test_max_identical() {
        let v1 = Vec3i::new(3, 3, 3);
        let v2 = Vec3i::new(3, 3, 3);
        let result = v1.max(v2);
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 3);
        assert_eq!(result.z, 3);
    }

    #[test]
    fn test_as_vec3f() {
        let v = Vec3i::new(1, 2, 3);
        let f = v.as_vec3f();
        assert_eq!(f.x, 1.0);
        assert_eq!(f.y, 2.0);
        assert_eq!(f.z, 3.0);
    }

    #[test]
    fn test_as_vec3f_negative() {
        let v = Vec3i::new(-1, -2, -3);
        let f = v.as_vec3f();
        assert_eq!(f.x, -1.0);
        assert_eq!(f.y, -2.0);
        assert_eq!(f.z, -3.0);
    }

    #[test]
    fn test_from_tuple() {
        let v = Vec3i::from((4, 5, 6));
        assert_eq!(v.x, 4);
        assert_eq!(v.y, 5);
        assert_eq!(v.z, 6);
    }

    #[test]
    fn test_from_tuple_negative() {
        let v = Vec3i::from((-4, -5, -6));
        assert_eq!(v.x, -4);
        assert_eq!(v.y, -5);
        assert_eq!(v.z, -6);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3i::new(1, 2, 3);
        let v2 = Vec3i::new(4, 5, 6);
        let result = v1 + v2;
        assert_eq!(result.x, 5);
        assert_eq!(result.y, 7);
        assert_eq!(result.z, 9);
    }

    #[test]
    fn test_add_zero() {
        let v1 = Vec3i::new(1, 2, 3);
        let v2 = Vec3i::zero();
        let result = v1 + v2;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 2);
        assert_eq!(result.z, 3);
    }

    #[test]
    fn test_add_negative() {
        let v1 = Vec3i::new(1, 2, 3);
        let v2 = Vec3i::new(-1, -2, -3);
        let result = v1 + v2;
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
        assert_eq!(result.z, 0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3i::new(5, 7, 9);
        let v2 = Vec3i::new(1, 2, 3);
        let result = v1 - v2;
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 5);
        assert_eq!(result.z, 6);
    }

    #[test]
    fn test_sub_zero() {
        let v1 = Vec3i::new(1, 2, 3);
        let v2 = Vec3i::zero();
        let result = v1 - v2;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 2);
        assert_eq!(result.z, 3);
    }

    #[test]
    fn test_sub_negative() {
        let v1 = Vec3i::new(1, 2, 3);
        let v2 = Vec3i::new(-1, -2, -3);
        let result = v1 - v2;
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 4);
        assert_eq!(result.z, 6);
    }

    #[test]
    fn test_mul_positive() {
        let v = Vec3i::new(1, 2, 3);
        let result = v * 2;
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 4);
        assert_eq!(result.z, 6);
    }

    #[test]
    fn test_mul_zero() {
        let v = Vec3i::new(1, 2, 3);
        let result = v * 0;
        assert_eq!(result.x, 0);
        assert_eq!(result.y, 0);
        assert_eq!(result.z, 0);
    }

    #[test]
    fn test_mul_negative() {
        let v = Vec3i::new(1, 2, 3);
        let result = v * -2;
        assert_eq!(result.x, -2);
        assert_eq!(result.y, -4);
        assert_eq!(result.z, -6);
    }

    #[test]
    fn test_mul_one() {
        let v = Vec3i::new(1, 2, 3);
        let result = v * 1;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 2);
        assert_eq!(result.z, 3);
    }

    #[test]
    fn test_commutative_add() {
        let v1 = Vec3i::new(1, 2, 3);
        let v2 = Vec3i::new(4, 5, 6);
        assert_eq!(v1 + v2, v2 + v1);
    }

    #[test]
    fn test_associative_add() {
        let v1 = Vec3i::new(1, 2, 3);
        let v2 = Vec3i::new(4, 5, 6);
        let v3 = Vec3i::new(7, 8, 9);
        assert_eq!((v1 + v2) + v3, v1 + (v2 + v3));
    }

    #[test]
    fn test_distributive_mul() {
        let v = Vec3i::new(1, 2, 3);
        let scalar1 = 2;
        let scalar2 = 3;
        assert_eq!(v * (scalar1 + scalar2), v * scalar1 + v * scalar2);
    }

    #[test]
    fn test_equality() {
        let v1 = Vec3i::new(1, 2, 3);
        let v2 = Vec3i::new(1, 2, 3);
        let v3 = Vec3i::new(1, 2, 4);
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
    }

    #[test]
    fn test_clone() {
        let v1 = Vec3i::new(1, 2, 3);
        let v2 = v1.clone();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_copy() {
        let v1 = Vec3i::new(1, 2, 3);
        let v2 = v1; // This should work because Vec3i implements Copy
        assert_eq!(v1, v2);
        assert_eq!(v1.x, 1); // v1 should still be usable
    }

    #[test]
    fn test_debug() {
        let v = Vec3i::new(1, 2, 3);
        let debug_str = format!("{:?}", v);
        assert!(debug_str.contains("1"));
        assert!(debug_str.contains("2"));
        assert!(debug_str.contains("3"));
    }

    #[test]
    fn test_serialize_deserialize() {
        let v = Vec3i::new(1, 2, 3);
        let serialized = serde_json::to_string(&v).unwrap();
        let deserialized: Vec3i = serde_json::from_str(&serialized).unwrap();
        assert_eq!(v, deserialized);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let v1 = Vec3i::new(1, 2, 3);
        let v2 = Vec3i::new(1, 2, 3);
        
        map.insert(v1, "first");
        assert_eq!(map.get(&v2), Some(&"first"));
    }

    #[test]
    fn test_edge_cases() {
        // Test with i32::MAX and i32::MIN
        let v_max = Vec3i::new(i32::MAX, i32::MAX, i32::MAX);
        let v_min = Vec3i::new(i32::MIN, i32::MIN, i32::MIN);
        
        assert_eq!(v_max.min(v_min), v_min);
        assert_eq!(v_max.max(v_min), v_max);
        
        // Test overflow in multiplication
        let v = Vec3i::new(1, 1, 1);
        let result = v * i32::MAX;
        assert_eq!(result.x, i32::MAX);
        assert_eq!(result.y, i32::MAX);
        assert_eq!(result.z, i32::MAX);
    }
}
