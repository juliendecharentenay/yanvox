//! Mathematical utilities and data structures for 3D operations

use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub, Mul};

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

/// 3D axis-aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Bounds3i {
    pub min: Vec3i,
    pub max: Vec3i,
}

impl Bounds3i {
    pub fn new(min: Vec3i, max: Vec3i) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self {
            min: Vec3i::new(i32::MAX, i32::MAX, i32::MAX),
            max: Vec3i::new(i32::MIN, i32::MIN, i32::MIN),
        }
    }

    pub fn from_point(point: Vec3i) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    pub fn expand(self, point: Vec3i) -> Self {
        Self {
            min: self.min.min(point),
            max: self.max.max(point),
        }
    }

    pub fn expand_bounds(self, other: Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    pub fn contains(self, point: Vec3i) -> bool {
        point.x >= self.min.x && point.x < self.max.x &&
        point.y >= self.min.y && point.y < self.max.y &&
        point.z >= self.min.z && point.z < self.max.z
    }

    pub fn intersects(self, other: Self) -> bool {
        self.min.x < other.max.x && self.max.x > other.min.x &&
        self.min.y < other.max.y && self.max.y > other.min.y &&
        self.min.z < other.max.z && self.max.z > other.min.z
    }

    pub fn size(self) -> Vec3i {
        self.max - self.min
    }

    pub fn volume(self) -> i64 {
        let size = self.size();
        size.x as i64 * size.y as i64 * size.z as i64
    }
}

/// 3D axis-aligned bounding box with floating point coordinates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Bounds3f {
    pub min: Vec3f,
    pub max: Vec3f,
}

impl Bounds3f {
    pub fn new(min: Vec3f, max: Vec3f) -> Self {
        Self { min, max }
    }

    pub fn empty() -> Self {
        Self {
            min: Vec3f::new(f32::MAX, f32::MAX, f32::MAX),
            max: Vec3f::new(f32::MIN, f32::MIN, f32::MIN),
        }
    }

    pub fn from_point(point: Vec3f) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    pub fn expand(self, point: Vec3f) -> Self {
        Self {
            min: Vec3f::new(
                self.min.x.min(point.x),
                self.min.y.min(point.y),
                self.min.z.min(point.z),
            ),
            max: Vec3f::new(
                self.max.x.max(point.x),
                self.max.y.max(point.y),
                self.max.z.max(point.z),
            ),
        }
    }

    pub fn expand_bounds(self, other: Self) -> Self {
        Self {
            min: Vec3f::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vec3f::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }

    pub fn contains(self, point: Vec3f) -> bool {
        point.x >= self.min.x && point.x < self.max.x &&
        point.y >= self.min.y && point.y < self.max.y &&
        point.z >= self.min.z && point.z < self.max.z
    }

    pub fn intersects(self, other: Self) -> bool {
        self.min.x < other.max.x && self.max.x > other.min.x &&
        self.min.y < other.max.y && self.max.y > other.min.y &&
        self.min.z < other.max.z && self.max.z > other.min.z
    }

    pub fn size(self) -> Vec3f {
        Vec3f::new(
            self.max.x - self.min.x,
            self.max.y - self.min.y,
            self.max.z - self.min.z,
        )
    }

    pub fn volume(self) -> f32 {
        let size = self.size();
        size.x * size.y * size.z
    }
}

/// Type aliases for common use cases
pub type Vec3 = Vec3i;
pub type Bounds3 = Bounds3i;

