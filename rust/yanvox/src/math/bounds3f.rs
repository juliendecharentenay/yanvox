use serde::{Deserialize, Serialize};
use super::Vec3f;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let min = Vec3f::new(0.0, 0.0, 0.0);
        let max = Vec3f::new(10.0, 10.0, 10.0);
        let bounds = Bounds3f::new(min, max);
        assert_eq!(bounds.min, min);
        assert_eq!(bounds.max, max);
    }

    #[test]
    fn test_empty() {
        let bounds = Bounds3f::empty();
        assert_eq!(bounds.min.x, f32::MAX);
        assert_eq!(bounds.min.y, f32::MAX);
        assert_eq!(bounds.min.z, f32::MAX);
        assert_eq!(bounds.max.x, f32::MIN);
        assert_eq!(bounds.max.y, f32::MIN);
        assert_eq!(bounds.max.z, f32::MIN);
    }

    #[test]
    fn test_from_point() {
        let point = Vec3f::new(5.0, 6.0, 7.0);
        let bounds = Bounds3f::from_point(point);
        assert_eq!(bounds.min, point);
        assert_eq!(bounds.max, point);
    }

    #[test]
    fn test_expand_with_point_inside() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(10.0, 10.0, 10.0));
        let point = Vec3f::new(5.0, 5.0, 5.0);
        let expanded = bounds.expand(point);
        assert_eq!(expanded.min, bounds.min);
        assert_eq!(expanded.max, bounds.max);
    }

    #[test]
    fn test_expand_with_point_outside_min() {
        let bounds = Bounds3f::new(Vec3f::new(5.0, 5.0, 5.0), Vec3f::new(10.0, 10.0, 10.0));
        let point = Vec3f::new(2.0, 3.0, 4.0);
        let expanded = bounds.expand(point);
        assert_eq!(expanded.min, point);
        assert_eq!(expanded.max, bounds.max);
    }

    #[test]
    fn test_expand_with_point_outside_max() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let point = Vec3f::new(8.0, 9.0, 10.0);
        let expanded = bounds.expand(point);
        assert_eq!(expanded.min, bounds.min);
        assert_eq!(expanded.max, point);
    }

    #[test]
    fn test_expand_with_point_mixed() {
        let bounds = Bounds3f::new(Vec3f::new(2.0, 2.0, 2.0), Vec3f::new(8.0, 8.0, 8.0));
        let point = Vec3f::new(1.0, 9.0, 5.0);
        let expanded = bounds.expand(point);
        assert_eq!(expanded.min, Vec3f::new(1.0, 2.0, 2.0));
        assert_eq!(expanded.max, Vec3f::new(8.0, 9.0, 8.0));
    }

    #[test]
    fn test_expand_bounds_overlapping() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let bounds2 = Bounds3f::new(Vec3f::new(3.0, 3.0, 3.0), Vec3f::new(8.0, 8.0, 8.0));
        let expanded = bounds1.expand_bounds(bounds2);
        assert_eq!(expanded.min, Vec3f::new(0.0, 0.0, 0.0));
        assert_eq!(expanded.max, Vec3f::new(8.0, 8.0, 8.0));
    }

    #[test]
    fn test_expand_bounds_disjoint() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(2.0, 2.0, 2.0));
        let bounds2 = Bounds3f::new(Vec3f::new(5.0, 5.0, 5.0), Vec3f::new(7.0, 7.0, 7.0));
        let expanded = bounds1.expand_bounds(bounds2);
        assert_eq!(expanded.min, Vec3f::new(0.0, 0.0, 0.0));
        assert_eq!(expanded.max, Vec3f::new(7.0, 7.0, 7.0));
    }

    #[test]
    fn test_expand_bounds_contained() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(10.0, 10.0, 10.0));
        let bounds2 = Bounds3f::new(Vec3f::new(3.0, 3.0, 3.0), Vec3f::new(7.0, 7.0, 7.0));
        let expanded = bounds1.expand_bounds(bounds2);
        assert_eq!(expanded.min, bounds1.min);
        assert_eq!(expanded.max, bounds1.max);
    }

    #[test]
    fn test_contains_point_inside() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(10.0, 10.0, 10.0));
        let point = Vec3f::new(5.0, 5.0, 5.0);
        assert!(bounds.contains(point));
    }

    #[test]
    fn test_contains_point_on_min_boundary() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(10.0, 10.0, 10.0));
        let point = Vec3f::new(0.0, 0.0, 0.0);
        assert!(bounds.contains(point));
    }

    #[test]
    fn test_contains_point_on_max_boundary() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(10.0, 10.0, 10.0));
        let point = Vec3f::new(10.0, 10.0, 10.0);
        assert!(!bounds.contains(point)); // max is exclusive
    }

    #[test]
    fn test_contains_point_outside() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(10.0, 10.0, 10.0));
        let point = Vec3f::new(15.0, 15.0, 15.0);
        assert!(!bounds.contains(point));
    }

    #[test]
    fn test_contains_point_partially_outside() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(10.0, 10.0, 10.0));
        let point = Vec3f::new(5.0, 15.0, 5.0);
        assert!(!bounds.contains(point));
    }

    #[test]
    fn test_contains_point_epsilon() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(10.0, 10.0, 10.0));
        let point = Vec3f::new(10.0 - f32::EPSILON * 10.0, 5.0, 5.0); // Use larger epsilon for test
        assert!(bounds.contains(point));
        let point_outside = Vec3f::new(10.0 + f32::EPSILON, 5.0, 5.0);
        assert!(!bounds.contains(point_outside));
    }

    #[test]
    fn test_intersects_overlapping() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let bounds2 = Bounds3f::new(Vec3f::new(3.0, 3.0, 3.0), Vec3f::new(8.0, 8.0, 8.0));
        assert!(bounds1.intersects(bounds2));
    }

    #[test]
    fn test_intersects_touching() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let bounds2 = Bounds3f::new(Vec3f::new(5.0, 5.0, 5.0), Vec3f::new(10.0, 10.0, 10.0));
        assert!(!bounds1.intersects(bounds2)); // touching at boundary doesn't intersect
    }

    #[test]
    fn test_intersects_disjoint() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(2.0, 2.0, 2.0));
        let bounds2 = Bounds3f::new(Vec3f::new(5.0, 5.0, 5.0), Vec3f::new(7.0, 7.0, 7.0));
        assert!(!bounds1.intersects(bounds2));
    }

    #[test]
    fn test_intersects_contained() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(10.0, 10.0, 10.0));
        let bounds2 = Bounds3f::new(Vec3f::new(3.0, 3.0, 3.0), Vec3f::new(7.0, 7.0, 7.0));
        assert!(bounds1.intersects(bounds2));
    }

    #[test]
    fn test_intersects_identical() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let bounds2 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        assert!(bounds1.intersects(bounds2));
    }

    #[test]
    fn test_size() {
        let bounds = Bounds3f::new(Vec3f::new(2.0, 3.0, 4.0), Vec3f::new(8.0, 9.0, 10.0));
        let size = bounds.size();
        assert!((size.x - 6.0).abs() < f32::EPSILON);
        assert!((size.y - 6.0).abs() < f32::EPSILON);
        assert!((size.z - 6.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_size_zero() {
        let bounds = Bounds3f::new(Vec3f::new(5.0, 5.0, 5.0), Vec3f::new(5.0, 5.0, 5.0));
        let size = bounds.size();
        assert!((size.x - 0.0).abs() < f32::EPSILON);
        assert!((size.y - 0.0).abs() < f32::EPSILON);
        assert!((size.z - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_size_negative() {
        let bounds = Bounds3f::new(Vec3f::new(10.0, 10.0, 10.0), Vec3f::new(5.0, 5.0, 5.0));
        let size = bounds.size();
        assert!((size.x - (-5.0)).abs() < f32::EPSILON);
        assert!((size.y - (-5.0)).abs() < f32::EPSILON);
        assert!((size.z - (-5.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_volume() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(3.0, 4.0, 5.0));
        let volume = bounds.volume();
        assert!((volume - 60.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_volume_zero() {
        let bounds = Bounds3f::new(Vec3f::new(5.0, 5.0, 5.0), Vec3f::new(5.0, 5.0, 5.0));
        let volume = bounds.volume();
        assert!((volume - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_volume_negative() {
        let bounds = Bounds3f::new(Vec3f::new(10.0, 10.0, 10.0), Vec3f::new(5.0, 5.0, 5.0));
        let volume = bounds.volume();
        assert!((volume - (-125.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_volume_large() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(1000.0, 1000.0, 1000.0));
        let volume = bounds.volume();
        assert!((volume - 1_000_000_000.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_volume_small() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.1, 0.1, 0.1));
        let volume = bounds.volume();
        assert!((volume - 0.001).abs() < f32::EPSILON);
    }

    #[test]
    fn test_equality() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let bounds2 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let bounds3 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(6.0, 6.0, 6.0));
        assert_eq!(bounds1, bounds2);
        assert_ne!(bounds1, bounds3);
    }

    #[test]
    fn test_equality_epsilon() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let bounds2 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0 + f32::EPSILON * 10.0, 5.0, 5.0));
        // These should NOT be equal due to exact equality
        assert_ne!(bounds1, bounds2);
    }

    #[test]
    fn test_clone() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let bounds2 = bounds1.clone();
        assert_eq!(bounds1, bounds2);
    }

    #[test]
    fn test_copy() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let bounds2 = bounds1; // This should work because Bounds3f implements Copy
        assert_eq!(bounds1, bounds2);
        assert!((bounds1.min.x - 0.0).abs() < f32::EPSILON); // bounds1 should still be usable
    }

    #[test]
    fn test_debug() {
        let bounds = Bounds3f::new(Vec3f::new(1.0, 2.0, 3.0), Vec3f::new(4.0, 5.0, 6.0));
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
        let bounds = Bounds3f::new(Vec3f::new(1.0, 2.0, 3.0), Vec3f::new(4.0, 5.0, 6.0));
        let serialized = serde_json::to_string(&bounds).unwrap();
        let deserialized: Bounds3f = serde_json::from_str(&serialized).unwrap();
        assert_eq!(bounds, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_negative() {
        let bounds = Bounds3f::new(Vec3f::new(-1.0, -2.0, -3.0), Vec3f::new(4.0, 5.0, 6.0));
        let serialized = serde_json::to_string(&bounds).unwrap();
        let deserialized: Bounds3f = serde_json::from_str(&serialized).unwrap();
        assert_eq!(bounds, deserialized);
    }

    #[test]
    fn test_edge_cases() {
        // Test with f32::MAX and f32::MIN
        let bounds_max = Bounds3f::new(
            Vec3f::new(f32::MIN, f32::MIN, f32::MIN),
            Vec3f::new(f32::MAX, f32::MAX, f32::MAX)
        );
        assert_eq!(bounds_max.min.x, f32::MIN);
        assert_eq!(bounds_max.max.x, f32::MAX);
        
        // Test empty bounds
        let empty = Bounds3f::empty();
        assert!(!empty.contains(Vec3f::new(0.0, 0.0, 0.0)));
        assert!(!empty.intersects(Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(1.0, 1.0, 1.0))));
        
        // Test infinity
        let bounds_inf = Bounds3f::new(
            Vec3f::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
            Vec3f::new(f32::INFINITY, f32::INFINITY, f32::INFINITY)
        );
        assert!(bounds_inf.contains(Vec3f::new(0.0, 0.0, 0.0)));
        assert!(bounds_inf.contains(Vec3f::new(f32::MAX, f32::MAX, f32::MAX)));
    }

    #[test]
    fn test_expand_empty_bounds() {
        let empty = Bounds3f::empty();
        let point = Vec3f::new(5.0, 6.0, 7.0);
        let expanded = empty.expand(point);
        assert_eq!(expanded.min, point);
        assert_eq!(expanded.max, point);
    }

    #[test]
    fn test_expand_bounds_with_empty() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let empty = Bounds3f::empty();
        let expanded = bounds.expand_bounds(empty);
        assert_eq!(expanded.min, bounds.min);
        assert_eq!(expanded.max, bounds.max);
    }

    #[test]
    fn test_contains_edge_cases() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        
        // Test points just inside and outside boundaries
        assert!(bounds.contains(Vec3f::new(0.0, 0.0, 0.0))); // on min boundary
        assert!(!bounds.contains(Vec3f::new(5.0, 5.0, 5.0))); // on max boundary (exclusive)
        assert!(bounds.contains(Vec3f::new(4.999999, 4.999999, 4.999999))); // just inside
        assert!(!bounds.contains(Vec3f::new(5.0, 4.0, 4.0))); // just outside on x
        assert!(!bounds.contains(Vec3f::new(4.0, 5.0, 4.0))); // just outside on y
        assert!(!bounds.contains(Vec3f::new(4.0, 4.0, 5.0))); // just outside on z
    }

    #[test]
    fn test_intersects_epsilon() {
        let bounds1 = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(5.0, 5.0, 5.0));
        let bounds2 = Bounds3f::new(Vec3f::new(5.0 - f32::EPSILON * 10.0, 5.0 - f32::EPSILON * 10.0, 5.0 - f32::EPSILON * 10.0), 
                                   Vec3f::new(10.0, 10.0, 10.0));
        assert!(bounds1.intersects(bounds2));
        
        let bounds3 = Bounds3f::new(Vec3f::new(5.0 + f32::EPSILON, 5.0 + f32::EPSILON, 5.0 + f32::EPSILON), 
                                   Vec3f::new(10.0, 10.0, 10.0));
        assert!(!bounds1.intersects(bounds3));
    }

    #[test]
    fn test_volume_precision() {
        let bounds = Bounds3f::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.1, 0.1, 0.1));
        let volume = bounds.volume();
        let expected = 0.001;
        assert!((volume - expected).abs() < f32::EPSILON * 10.0); // Allow for some floating point error
    }

    #[test]
    fn test_negative_bounds() {
        let bounds = Bounds3f::new(Vec3f::new(-5.0, -5.0, -5.0), Vec3f::new(5.0, 5.0, 5.0));
        assert!(bounds.contains(Vec3f::new(0.0, 0.0, 0.0)));
        assert!(bounds.contains(Vec3f::new(-2.0, -2.0, -2.0)));
        assert!(bounds.contains(Vec3f::new(2.0, 2.0, 2.0)));
        assert!(!bounds.contains(Vec3f::new(-6.0, 0.0, 0.0)));
        assert!(!bounds.contains(Vec3f::new(6.0, 0.0, 0.0)));
        
        let size = bounds.size();
        assert!((size.x - 10.0).abs() < f32::EPSILON);
        assert!((size.y - 10.0).abs() < f32::EPSILON);
        assert!((size.z - 10.0).abs() < f32::EPSILON);
        
        let volume = bounds.volume();
        assert!((volume - 1000.0).abs() < f32::EPSILON);
    }
}
