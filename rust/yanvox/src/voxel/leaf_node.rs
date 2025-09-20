//! Leaf node implementation for VDB-inspired voxel engine

use crate::math::{Vec3i, Bounds3i};
use crate::voxel::{VoxelData, NodeTrait, ChildNodeTrait, NodeDiagnostics, NodeType};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Leaf node that stores actual voxel data
/// The const generic LOG2 specifies the power of 2 for the number of children in each direction
/// e.g., LOG2 = 3 means 2^3 * 3 = 24 children
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeafNode<T: VoxelData, const LOG2: usize> {
    /// Background value for empty regions within this leaf
    pub background_value: T,
    /// Dense storage for voxel data
    /// Index corresponds to local coordinate within this leaf's bounds
    /// None represents no data (background value)
    data: Vec<Option<T>>,
    /// Origin of this leaf node
    pub origin: Vec3i,

    /// Bounds of this leaf node
    pub bounds: Bounds3i,
    /// Level of this node (highest level, typically the leaf level)
    pub level: u32,
    /// Number of active (non-background) voxels in this leaf
    active_count: usize,
}

impl<T: VoxelData, const LOG2: usize> LeafNode<T, LOG2> {
    /// Create a new leaf node
    pub fn new(level: u32, bounds: Bounds3i) -> Self {
        let dimensions = Self::calculate_dimensions();
        let total_size = dimensions.x * dimensions.y * dimensions.z;
        
        Self {
            background_value: T::background(),
            data: (0..total_size as usize).map(|_| None).collect(),
            origin: bounds.min.clone(),

            level,
            bounds,
            active_count: 0,
        }
    }

    /// Calculate the dimensions based on LOG2
    fn calculate_dimensions() -> Vec3i {
        let per_axis = 1 << LOG2; // 2^LOG2
        Vec3i::new(per_axis, per_axis, per_axis)
    }

    /// Get the number of children this leaf can contain
    pub const fn child_capacity() -> usize {
        3 * (1 << LOG2)
    }

    /// Get the dimensions of this leaf node
    pub fn dimensions(&self) -> Vec3i {
        Self::calculate_dimensions()
    }

    /// Check if this leaf is at capacity
    pub fn is_at_capacity(&self) -> bool {
        self.active_count >= Self::child_capacity()
    }

    /// Convert global coordinate to local index within the data vector
    fn coord_to_index(&self, coord: Vec3i) -> Option<usize> {
        if !self.bounds.contains(coord) {
            return None;
        }

        log::info!("coord: {coord:?}, LOG2: {:?}", LOG2);

        let i = coord.x & ((1 << LOG2) - 1);
        let j = coord.y & ((1 << LOG2) - 1);
        let k = coord.z & ((1 << LOG2) - 1);
        log::info!("i: {:?}, j: {:?}, k: {:?}", i, j, k);

        let index = i + j * (1 << LOG2) + k * (1 << LOG2) * (1 << LOG2);
        log::info!("index: {:?}", index);
        Some(index as usize)

/*
        let local_coord = coord - self.origin;
        let dimensions = self.dimensions();
        
        // Check if local coordinate is within dimensions
        if local_coord.x >= 0 && local_coord.x < dimensions.x &&
           local_coord.y >= 0 && local_coord.y < dimensions.y &&
           local_coord.z >= 0 && local_coord.z < dimensions.z {
            
            let index = (local_coord.z * dimensions.y * dimensions.x + 
                         local_coord.y * dimensions.x + 
                         local_coord.x) as usize;
            Some(index)
        } else {
            None
        }
        */
    }

    /// Convert local index back to global coordinate
    fn index_to_coord(&self, index: usize) -> Vec3i {
        let local_index = index as i32;
        let dimensions = self.dimensions();
        let z = local_index / (dimensions.y * dimensions.x);
        let y = (local_index % (dimensions.y * dimensions.x)) / dimensions.x;
        let x = local_index % dimensions.x;
        
        Vec3i::new(x, y, z) + self.origin
    }

    /// Check if a coordinate is within this leaf's bounds
    pub fn contains_coord(&self, coord: Vec3i) -> bool {
        self.bounds.contains(coord)
    }

    /// Get the density (active voxels / total capacity) of this leaf
    pub fn density(&self) -> f32 {
        self.active_count as f32 / Self::child_capacity() as f32
    }

    /// Check if this leaf is sparse (low density)
    pub fn is_sparse(&self, threshold: f32) -> bool {
        self.density() < threshold
    }

    /// Get all voxels in this leaf (including inactive ones)
    pub fn all_voxels(&self) -> impl Iterator<Item = (Vec3i, &T)> {
        self.data.iter()
            .enumerate()
            .filter_map(|(index, voxel)| {
                voxel.as_ref().map(|v| (self.index_to_coord(index), v))
            })
    }

    /// Clear all voxel data from this leaf
    pub fn clear(&mut self) {
        for voxel in &mut self.data {
            *voxel = None;
        }
        self.active_count = 0;
    }

    /// Optimize this leaf by removing inactive voxels
    pub fn optimize(&mut self) {
        for voxel in &mut self.data {
            if let Some(value) = voxel.as_ref() {
                if !value.is_active() {
                    *voxel = None;
                }
            }
        }
        self.active_count = self.data.iter().filter(|v| v.is_some()).count();
    }

    /// Get memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        std::mem::size_of::<Self>() + 
        self.data.capacity() * std::mem::size_of::<Option<T>>()
    }
}

impl<T: VoxelData, const LOG2: usize> NodeTrait<T> for LeafNode<T, LOG2> {
    fn level(&self) -> u32 {
        self.level
    }

    fn log2_cum(&self) -> u32 {
      <Self as ChildNodeTrait::<T>>::log2_cum()
    }

    fn bounds(&self) -> Bounds3i {
        self.bounds
    }

    fn is_active(&self, coord: Vec3i) -> bool {
        if let Some(index) = self.coord_to_index(coord) {
            self.data.get(index)
                .and_then(|voxel| voxel.as_ref())
                .map_or(false, |value| value.is_active())
        } else {
            false
        }
    }

    fn active_count(&self) -> usize {
        self.active_count
    }

    fn total_count(&self) -> usize {
        self.data.iter().filter(|v| v.is_some()).count()
    }

    fn get_voxel(&self, coord: Vec3i) -> Option<&T> {
        if let Some(index) = self.coord_to_index(coord) {
            self.data.get(index)?.as_ref()
        } else {
            None
        }
    }

    fn set_voxel(&mut self, coord: Vec3i, value: T) -> Option<T> {
        if let Some(index) = self.coord_to_index(coord) {
            let was_active = self.data.get(index)
                .and_then(|v| v.as_ref())
                .map_or(false, |v| v.is_active());
            let is_active = value.is_active();
            
            let old_value = self.data.get_mut(index)?.replace(value);
            
            // Update active count
            if was_active && !is_active {
                self.active_count = self.active_count.saturating_sub(1);
            } else if !was_active && is_active {
                self.active_count += 1;
            }
            
            old_value
        } else {
            None
        }
    }

    fn remove_voxel(&mut self, coord: Vec3i) -> Option<T> {
        if let Some(index) = self.coord_to_index(coord) {
            if let Some(voxel) = self.data.get_mut(index) {
                if let Some(value) = voxel.take() {
                    if value.is_active() {
                        self.active_count = self.active_count.saturating_sub(1);
                    }
                    return Some(value);
                }
            }
        }
        None
    }

    // Iterator operations
    fn active_voxels(&self) -> Box<dyn Iterator<Item = (Vec3i, &T)> + '_> {
        Box::new(
            self.all_voxels()
                .filter(|(_, value)| value.is_active())
        )
    }

    fn all_voxels(&self) -> Box<dyn Iterator<Item = (Vec3i, &T)> + '_> {
        Box::new(   
            self.data.iter()
                .enumerate()
                .filter_map(|(index, voxel)| {
                    voxel.as_ref().map(|v| (self.index_to_coord(index), v))
                })
        )
    }

    // Background value operations
    fn background_value(&self) -> &T {
        &self.background_value
    }
}

// Implementation of ChildNodeTrait for LeafNode
impl<T: VoxelData, const LOG2: usize> ChildNodeTrait<T> for LeafNode<T, LOG2> {
    /// Returns the log2 of the number of children this leaf can contain
    fn log2() -> u32 {
        LOG2 as u32
    }

    /// Returns the cumulative log2 of the number of children the hierarcy can contain
    fn log2_cum() -> u32 {
      LOG2 as u32
    }

    fn create(key: Vec3i, level: u32) -> Self {
        let bounds3 = Bounds3i::new(key, key + Self::calculate_dimensions());
        Self::new(level, bounds3)
    }
}

// Implementation of NodeDiagnostics for LeafNode
impl<T: VoxelData, const LOG2: usize> NodeDiagnostics<T> for LeafNode<T, LOG2> {
    /// Returns the log2 of the child size (same as LOG2 for leaf nodes)
    fn log2_child_size(&self) -> u32 {
        LOG2 as u32
    }

    /// Returns the node type (always Leaf for leaf nodes)
    fn node_type(&self) -> NodeType {
        NodeType::Leaf
    }

    /// Returns the depth of this node (same as level for leaf nodes)
    fn depth(&self) -> u32 {
        self.level
    }

    /// Returns the number of child voxels (not child nodes, since this is a leaf)
    fn child_count(&self) -> usize {
        self.active_count
    }
}

// Implement Default for common voxel types
impl<T: VoxelData, const LOG2: usize> Default for LeafNode<T, LOG2> {
    fn default() -> Self {
        Self::new(0, Bounds3i::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_child_node_trait() {
        let _leaf = LeafNode::<f32, 6>::new(5, Bounds3i::empty());
        
        // Test that the trait is implemented correctly
        assert_eq!(LeafNode::<f32, 6>::log2(), 6);
        assert_eq!(LeafNode::<f32, 3>::log2(), 3);
        assert_eq!(LeafNode::<f32, 9>::log2(), 9);
    }

    #[test]
    fn test_node_diagnostics() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(8, 8, 8));
        let mut leaf = LeafNode::<f32, 6>::new(5, bounds);
        
        // Test diagnostics
        assert_eq!(leaf.log2_child_size(), 6);
        assert_eq!(leaf.node_type(), NodeType::Leaf);
        assert_eq!(leaf.depth(), 5);
        assert_eq!(leaf.child_count(), 0); // No active voxels initially
        
        // Add some voxels and test again
        leaf.set_voxel(Vec3i::new(1, 1, 1), 42.0);
        leaf.set_voxel(Vec3i::new(2, 2, 2), 24.0);
        
        assert_eq!(leaf.child_count(), 2); // Now has 2 active voxels
    }

    #[test]
    fn test_leaf_node_creation() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(8, 8, 8));
        let leaf = LeafNode::<f32, 6>::new(5, bounds); // 2^6 = 64 children
        
        assert_eq!(leaf.level(), 5);
        assert_eq!(leaf.bounds(), bounds);
        assert_eq!(leaf.active_count(), 0);
        assert_eq!(LeafNode::<f32, 6>::child_capacity(), 192);
        assert_eq!(leaf.dimensions(), Vec3i::new(64, 64, 64)); // 2^6 in each directions
    }

    #[test]
    fn test_voxel_operations() {
        let bounds = Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(8, 8, 8));
        let mut leaf = LeafNode::<f32, 6>::new(5, bounds);
        
        // Set a voxel
        let result = leaf.set_voxel(Vec3i::new(1, 1, 1), 42.0);
        assert_eq!(result, None);
        assert_eq!(leaf.active_count(), 1);
        assert!(leaf.is_active(Vec3i::new(1, 1, 1)));
        
        // Get the voxel
        assert_eq!(leaf.get_voxel(Vec3i::new(1, 1, 1)), Some(&42.0));
        
        // Remove the voxel
        let removed = leaf.remove_voxel(Vec3i::new(1, 1, 1));
        assert_eq!(removed, Some(42.0));
        assert_eq!(leaf.active_count(), 0);
        assert!(!leaf.is_active(Vec3i::new(1, 1, 1)));
    }

    #[test]
    fn test_different_powers() {
        // Check capacity
        assert_eq!(LeafNode::<f32, 3>::child_capacity(), 24);
        assert_eq!(LeafNode::<f32, 5>::child_capacity(), 96);
        assert_eq!(LeafNode::<f32, 6>::child_capacity(), 192);
        
        assert_eq!(LeafNode::<f32, 3>::calculate_dimensions(), Vec3i::new(8, 8, 8));
        assert_eq!(LeafNode::<f32, 5>::calculate_dimensions(), Vec3i::new(32, 32, 32));
        assert_eq!(LeafNode::<f32, 6>::calculate_dimensions(), Vec3i::new(64, 64, 64));
    }

    #[test]
    fn convert_coord_to_index() {
      let bounds = Bounds3i::new(Vec3i::new(8, 16, 32), Vec3i::new(16, 24, 40));
      let mut leaf = LeafNode::<f32, 3>::new(3, bounds);

      assert!(!leaf.contains_coord(Vec3i::new(-5, -1, 4)));
      assert!(leaf.contains_coord(Vec3i::new(13, 21, 39)));
      assert_eq!(leaf.coord_to_index(Vec3i::new(15, 23, 33)), Some(127)); // = 7 + 7 * 8 + 1 * 8 * 8
    }

    #[test]
    fn convert_index_to_coord() {
      let bounds = Bounds3i::new(Vec3i::new(8, 24, 32), Vec3i::new(16, 32, 40));
      let mut leaf = LeafNode::<f32, 3>::new(3, bounds);
      assert_eq!(leaf.index_to_coord(7+2*8+4*8*8), Vec3i::new(15, 26, 36));
    }
}
