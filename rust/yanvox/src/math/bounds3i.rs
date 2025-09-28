use serde::{Deserialize, Serialize};
use super::Vec3i;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let min = Vec3i::new(0, 0, 0);
        let max = Vec3i::new(10, 10, 10);
        let bounds = Bounds3i::new(min, max);
        assert_eq!(bounds.min, min);
        assert_eq!(bounds.max, max);
    }

    #[test]
    fn test_empty() {
        let bounds = Bounds3i::empty();
        assert_eq!(bounds.min.x, i32::MAX);
        assert_eq!(bounds.min.y, i32::MAX);
        assert_eq!(bounds.min.z, i32::MAX);
        assert_eq!(bounds.max.x, i32::MIN);
        assert_eq!(bounds.max.y, i32::MIN);
        assert_eq!(bounds.max.z, i32::MIN);
    }

    #[test]
    fn test_from_point() {
        let point = Vec3i::new(5, 6, 7);
        let bounds = Bounds3i::from_point(point);
        assert_eq!(bounds.min, point);
        assert_eq!(bounds.max, point);
    }

    #[test]
    fn test_expand_with_point_inside() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(10, 10, 10));
        let point = Vec3i::new(5, 5, 5);
        let expanded = bounds.expand(point);
        assert_eq!(expanded.min, bounds.min);
        assert_eq!(expanded.max, bounds.max);
    }

    #[test]
    fn test_expand_with_point_outside_min() {
        let bounds = Bounds3i::new(Vec3i::new(5, 5, 5), Vec3i::new(10, 10, 10));
        let point = Vec3i::new(2, 3, 4);
        let expanded = bounds.expand(point);
        assert_eq!(expanded.min, point);
        assert_eq!(expanded.max, bounds.max);
    }

    #[test]
    fn test_expand_with_point_outside_max() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        let point = Vec3i::new(8, 9, 10);
        let expanded = bounds.expand(point);
        assert_eq!(expanded.min, bounds.min);
        assert_eq!(expanded.max, point);
    }

    #[test]
    fn test_expand_with_point_mixed() {
        let bounds = Bounds3i::new(Vec3i::new(2, 2, 2), Vec3i::new(8, 8, 8));
        let point = Vec3i::new(1, 9, 5);
        let expanded = bounds.expand(point);
        assert_eq!(expanded.min, Vec3i::new(1, 2, 2));
        assert_eq!(expanded.max, Vec3i::new(8, 9, 8));
    }

    #[test]
    fn test_expand_bounds_overlapping() {
        let bounds1 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        let bounds2 = Bounds3i::new(Vec3i::new(3, 3, 3), Vec3i::new(8, 8, 8));
        let expanded = bounds1.expand_bounds(bounds2);
        assert_eq!(expanded.min, Vec3i::new(0, 0, 0));
        assert_eq!(expanded.max, Vec3i::new(8, 8, 8));
    }

    #[test]
    fn test_expand_bounds_disjoint() {
        let bounds1 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(2, 2, 2));
        let bounds2 = Bounds3i::new(Vec3i::new(5, 5, 5), Vec3i::new(7, 7, 7));
        let expanded = bounds1.expand_bounds(bounds2);
        assert_eq!(expanded.min, Vec3i::new(0, 0, 0));
        assert_eq!(expanded.max, Vec3i::new(7, 7, 7));
    }

    #[test]
    fn test_expand_bounds_contained() {
        let bounds1 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(10, 10, 10));
        let bounds2 = Bounds3i::new(Vec3i::new(3, 3, 3), Vec3i::new(7, 7, 7));
        let expanded = bounds1.expand_bounds(bounds2);
        assert_eq!(expanded.min, bounds1.min);
        assert_eq!(expanded.max, bounds1.max);
    }

    #[test]
    fn test_contains_point_inside() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(10, 10, 10));
        let point = Vec3i::new(5, 5, 5);
        assert!(bounds.contains(point));
    }

    #[test]
    fn test_contains_point_on_min_boundary() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(10, 10, 10));
        let point = Vec3i::new(0, 0, 0);
        assert!(bounds.contains(point));
    }

    #[test]
    fn test_contains_point_on_max_boundary() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(10, 10, 10));
        let point = Vec3i::new(10, 10, 10);
        assert!(!bounds.contains(point)); // max is exclusive
    }

    #[test]
    fn test_contains_point_outside() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(10, 10, 10));
        let point = Vec3i::new(15, 15, 15);
        assert!(!bounds.contains(point));
    }

    #[test]
    fn test_contains_point_partially_outside() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(10, 10, 10));
        let point = Vec3i::new(5, 15, 5);
        assert!(!bounds.contains(point));
    }

    #[test]
    fn test_intersects_overlapping() {
        let bounds1 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        let bounds2 = Bounds3i::new(Vec3i::new(3, 3, 3), Vec3i::new(8, 8, 8));
        assert!(bounds1.intersects(bounds2));
    }

    #[test]
    fn test_intersects_touching() {
        let bounds1 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        let bounds2 = Bounds3i::new(Vec3i::new(5, 5, 5), Vec3i::new(10, 10, 10));
        assert!(!bounds1.intersects(bounds2)); // touching at boundary doesn't intersect
    }

    #[test]
    fn test_intersects_disjoint() {
        let bounds1 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(2, 2, 2));
        let bounds2 = Bounds3i::new(Vec3i::new(5, 5, 5), Vec3i::new(7, 7, 7));
        assert!(!bounds1.intersects(bounds2));
    }

    #[test]
    fn test_intersects_contained() {
        let bounds1 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(10, 10, 10));
        let bounds2 = Bounds3i::new(Vec3i::new(3, 3, 3), Vec3i::new(7, 7, 7));
        assert!(bounds1.intersects(bounds2));
    }

    #[test]
    fn test_intersects_identical() {
        let bounds1 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        let bounds2 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        assert!(bounds1.intersects(bounds2));
    }

    #[test]
    fn test_size() {
        let bounds = Bounds3i::new(Vec3i::new(2, 3, 4), Vec3i::new(8, 9, 10));
        let size = bounds.size();
        assert_eq!(size.x, 6);
        assert_eq!(size.y, 6);
        assert_eq!(size.z, 6);
    }

    #[test]
    fn test_size_zero() {
        let bounds = Bounds3i::new(Vec3i::new(5, 5, 5), Vec3i::new(5, 5, 5));
        let size = bounds.size();
        assert_eq!(size.x, 0);
        assert_eq!(size.y, 0);
        assert_eq!(size.z, 0);
    }

    #[test]
    fn test_size_negative() {
        let bounds = Bounds3i::new(Vec3i::new(10, 10, 10), Vec3i::new(5, 5, 5));
        let size = bounds.size();
        assert_eq!(size.x, -5);
        assert_eq!(size.y, -5);
        assert_eq!(size.z, -5);
    }

    #[test]
    fn test_volume() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(3, 4, 5));
        let volume = bounds.volume();
        assert_eq!(volume, 3 * 4 * 5);
    }

    #[test]
    fn test_volume_zero() {
        let bounds = Bounds3i::new(Vec3i::new(5, 5, 5), Vec3i::new(5, 5, 5));
        let volume = bounds.volume();
        assert_eq!(volume, 0);
    }

    #[test]
    fn test_volume_negative() {
        let bounds = Bounds3i::new(Vec3i::new(10, 10, 10), Vec3i::new(5, 5, 5));
        let volume = bounds.volume();
        assert_eq!(volume, -125); // (-5) * (-5) * (-5)
    }

    #[test]
    fn test_volume_large() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(1000, 1000, 1000));
        let volume = bounds.volume();
        assert_eq!(volume, 1_000_000_000);
    }

    #[test]
    fn test_equality() {
        let bounds1 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        let bounds2 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        let bounds3 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(6, 6, 6));
        assert_eq!(bounds1, bounds2);
        assert_ne!(bounds1, bounds3);
    }

    #[test]
    fn test_clone() {
        let bounds1 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        let bounds2 = bounds1.clone();
        assert_eq!(bounds1, bounds2);
    }

    #[test]
    fn test_copy() {
        let bounds1 = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        let bounds2 = bounds1; // This should work because Bounds3i implements Copy
        assert_eq!(bounds1, bounds2);
        assert_eq!(bounds1.min.x, 0); // bounds1 should still be usable
    }

    #[test]
    fn test_debug() {
        let bounds = Bounds3i::new(Vec3i::new(1, 2, 3), Vec3i::new(4, 5, 6));
        let debug_str = format!("{:?}", bounds);
        assert!(debug_str.contains("1"));
        assert!(debug_str.contains("2"));
        assert!(debug_str.contains("3"));
        assert!(debug_str.contains("4"));
        assert!(debug_str.contains("5"));
        assert!(debug_str.contains("6"));
    }

    #[test]
    fn test_serialize_deserialize() {
        let bounds = Bounds3i::new(Vec3i::new(1, 2, 3), Vec3i::new(4, 5, 6));
        let serialized = serde_json::to_string(&bounds).unwrap();
        let deserialized: Bounds3i = serde_json::from_str(&serialized).unwrap();
        assert_eq!(bounds, deserialized);
    }

    #[test]
    fn test_edge_cases() {
        // Test with i32::MAX and i32::MIN
        let bounds_max = Bounds3i::new(
            Vec3i::new(i32::MIN, i32::MIN, i32::MIN),
            Vec3i::new(i32::MAX, i32::MAX, i32::MAX)
        );
        assert_eq!(bounds_max.min.x, i32::MIN);
        assert_eq!(bounds_max.max.x, i32::MAX);
        
        // Test empty bounds
        let empty = Bounds3i::empty();
        assert!(!empty.contains(Vec3i::new(0, 0, 0)));
        assert!(!empty.intersects(Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(1, 1, 1))));
    }

    #[test]
    fn test_expand_empty_bounds() {
        let empty = Bounds3i::empty();
        let point = Vec3i::new(5, 6, 7);
        let expanded = empty.expand(point);
        assert_eq!(expanded.min, point);
        assert_eq!(expanded.max, point);
    }

    #[test]
    fn test_expand_bounds_with_empty() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        let empty = Bounds3i::empty();
        let expanded = bounds.expand_bounds(empty);
        assert_eq!(expanded.min, bounds.min);
        assert_eq!(expanded.max, bounds.max);
    }

    #[test]
    fn test_contains_edge_cases() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(5, 5, 5));
        
        // Test points just inside and outside boundaries
        assert!(bounds.contains(Vec3i::new(0, 0, 0))); // on min boundary
        assert!(!bounds.contains(Vec3i::new(5, 5, 5))); // on max boundary (exclusive)
        assert!(bounds.contains(Vec3i::new(4, 4, 4))); // just inside
        assert!(!bounds.contains(Vec3i::new(5, 4, 4))); // just outside on x
        assert!(!bounds.contains(Vec3i::new(4, 5, 4))); // just outside on y
        assert!(!bounds.contains(Vec3i::new(4, 4, 5))); // just outside on z
    }
}
