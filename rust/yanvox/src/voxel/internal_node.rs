//! Internal node implementation for VDB-inspired voxel engine

use crate::math::{Vec3i, Bounds3i};
use crate::voxel::{VoxelData, NodeTrait, ChildNodeTrait, NodeDiagnostics, NodeType};
use serde::{Deserialize, Serialize};

/// Internal node that stores child nodes
/// The const generic LOG2 specifies the power of 2 for the number of children in each direction
/// e.g., LOG2 = 3 means 2^3 * 3 = 24 children
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalNode<T: VoxelData, N: ChildNodeTrait<T>, const LOG2: usize> {
    /// Background value for empty regions within this internal node
    pub background_value: T,
    /// Dense storage for child nodes
    /// Index corresponds to local coordinate within this internal node's bounds
    /// None represents no child node (background value)
    data: Vec<Option<N>>,
    /// Origin of this internal node
    pub origin: Vec3i,

    /// Level of this node
    pub level: u32,
}

impl<T: VoxelData, N: ChildNodeTrait<T>, const LOG2: usize> InternalNode<T, N, LOG2> {
    /// Create a new internal node
    fn from_level_and_coord(level: u32, coord: Vec3i, background_value: T) -> Self {
        let total_size = Self::child_capacity();
        let origin = <Self as ChildNodeTrait::<_>>::key(coord);
        Self {
            background_value,
            data: (0..total_size as usize).map(|_| None).collect(),
            origin,
            level,
        }
    }

    fn data_dimensions() -> Vec3i {
        Vec3i::new(1 << LOG2, 1 << LOG2, 1 << LOG2)
    }

    /// Calculate the dimensions based on LOG2
    fn calculate_dimensions() -> Vec3i {
        let log2 = <Self as ChildNodeTrait::<_>>::log2_cum();
        let per_axis = 1 << log2; // 2^LOG2
        Vec3i::new(per_axis, per_axis, per_axis)
    }

    /// Get the number of children this internal node can contain
    pub const fn child_capacity() -> usize {
        (1 << LOG2) * (1 << LOG2) * (1 << LOG2)
    }

    /// Get the dimensions of this internal node
    pub fn dimensions(&self) -> Vec3i {
        Self::calculate_dimensions()
    }

    /// Check if this internal node is at capacity
    pub fn is_at_capacity(&self) -> bool {
        self.active_count() >= Self::child_capacity()
    }

    /// Convert global coordinate to local index within the data vector
    fn coord_to_index(&self, coord: Vec3i) -> Option<usize> {        
        if !self.contains_coord(coord) {
            return None;
        }
        let log2 = <Self as ChildNodeTrait::<_>>::log2_cum();
        let child_log2 = <N as ChildNodeTrait::<_>>::log2_cum();

        log::debug!("coord: {coord:?}, log2: {log2}, child_log2: {child_log2}");

        let i = coord.x & (1 << log2) - 1;
        let j = coord.y & (1 << log2) - 1;
        let k = coord.z & (1 << log2) - 1;
        log::debug!("i: {i}, j: {j}, k: {k}");

        let i = i &! ((1 << child_log2) - 1);
        let j = j &! ((1 << child_log2) - 1);
        let k = k &! ((1 << child_log2) - 1);
        log::debug!("i: {i}, j: {j}, k: {k}");

        let i = i / (1 << child_log2);
        let j = j / (1 << child_log2);
        let k = k / (1 << child_log2);
        log::debug!("i: {i}, j: {j}, k: {k}");

        let index = i + j * (1 << LOG2) + k * (1 << LOG2) * (1 << LOG2); 
        Some(index as usize)
    }

    /// Convert local index back to global coordinate
    fn index_to_coord(&self, index: usize) -> Vec3i {
        let local_index = index as i32;
        let dimensions = Self::data_dimensions();
        let z = local_index / (dimensions.y * dimensions.x);
        let y = (local_index % (dimensions.y * dimensions.x)) / dimensions.x;
        let x = local_index % dimensions.x;
        log::debug!("x: {x}, y: {y}, z: {z}");

        let child_log2 = <N as ChildNodeTrait::<_>>::log2_cum();
        let x = x * (1 << child_log2);
        let y = y * (1 << child_log2);
        let z = z * (1 << child_log2);
        log::debug!("x: {x}, y: {y}, z: {z}");

        Vec3i::new(x, y, z) + self.origin
    }

    /// Check if a coordinate is within this internal node's bounds
    pub fn contains_coord(&self, coord: Vec3i) -> bool {
        self.bounds().contains(coord)
    }

    /// Get the density (active child nodes / total capacity) of this internal node
    pub fn density(&self) -> f32 {
        self.active_count() as f32 / Self::child_capacity() as f32
    }

    /// Check if this internal node is sparse (low density)
    pub fn is_sparse(&self, threshold: f32) -> bool {
        self.density() < threshold
    }

    /// Get all child nodes in this internal node (including inactive ones)
    pub fn all_children(&self) -> impl Iterator<Item = (Vec3i, &N)> {
        self.data.iter()
            .enumerate()
            .filter_map(|(index, child)| {
                child.as_ref().map(|c| (self.index_to_coord(index), c))
            })
    }

    /// Clear all child node data from this internal node
    pub fn clear(&mut self) {
        for child in &mut self.data {
            *child = None;
        }
    }

    /// Get memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        std::mem::size_of::<Self>() + 
        self.data.capacity() * std::mem::size_of::<Option<N>>()
    }

    /// Internal method to create a new child node for a coordinate
    fn create_child(&mut self, coord: Vec3i) -> &mut N {
        let index = self.coord_to_index(coord.clone()).expect("Child key should be within bounds");
        let child = N::create(coord, self.level + 1, self.background_value.clone());
        self.data[index] = Some(child);
        self.data[index].as_mut().unwrap()
    }

    /// Internal method to find child node for a coordinate
    fn find_child(&self, coord: Vec3i) -> Option<&N> {
        if let Some(index) = self.coord_to_index(coord) {
            self.data.get(index)?.as_ref()
        } else {
            None
        }
    }

    /// Internal method to find child node for a coordinate (mutable)
    fn find_child_mut(&mut self, coord: Vec3i) -> Option<&mut N> {
        if let Some(index) = self.coord_to_index(coord) {
            self.data.get_mut(index)?.as_mut()
        } else {
            None
        }
    }
}

impl<T: VoxelData, N: ChildNodeTrait<T>, const LOG2: usize> NodeTrait<T> for InternalNode<T, N, LOG2> {
    fn level(&self) -> u32 {
        self.level
    }

    fn log2_cum(&self) -> u32 {
        <Self as ChildNodeTrait::<T>>::log2_cum()
    }

    fn bounds(&self) -> Bounds3i {
        Bounds3i::new(self.origin, self.origin + Self::calculate_dimensions())
    }

    fn is_active(&self, coord: Vec3i) -> bool {
        if let Some(child) = self.find_child(coord) {
            child.is_active(coord)
        } else {
            false
        }
    }

    fn active_count(&self) -> usize {
        self.data.iter()
            .filter_map(|child| child.as_ref())
            .map(|child| child.active_count())
            .sum()
    }

    fn total_count(&self) -> usize {
        self.data.iter()
            .filter_map(|child| child.as_ref())
            .map(|child| child.total_count())
            .sum()
    }

    fn get_voxel(&self, coord: Vec3i) -> &T {
        if let Some(child) = self.find_child(coord) {
            child.get_voxel(coord)
        } else {
            &self.background_value
        }
    }

    fn set_voxel(&mut self, coord: Vec3i, value: T) -> Option<T> {
        if let Some(child) = self.find_child_mut(coord) {
            // Internal nodes delegate to children
            child.set_voxel(coord, value)
        } else if self.background_value != value {
            // Create a new child if the background value is different
            let child = self.create_child(coord);
            child.set_voxel(coord, value)
        } else {
            // Do nothing if the background value is the same
            None
        }
    }

    fn remove_voxel(&mut self, coord: Vec3i) -> Option<T> {
        if let Some(child) = self.find_child_mut(coord) {
            child.remove_voxel(coord)
        } else {
            None
        }
    }

    // Iterator operations
    fn active_voxels(&self) -> Box<dyn Iterator<Item = (Vec3i, &T)> + '_> {
        Box::new(
            self.data.iter()
                .filter_map(|child| child.as_ref())
                .flat_map(|child| child.active_voxels())
        )
    }

    fn all_voxels(&self) -> Box<dyn Iterator<Item = (Vec3i, &T)> + '_> {
        Box::new(
            self.data.iter()
                .filter_map(|child| child.as_ref())
                .flat_map(|child| child.all_voxels())
        )
    }

    /*
    // Background value operations
    fn background_value(&self) -> &T {
        &self.background_value
    }
        */
}

// Implementation of ChildNodeTrait for InternalNode
impl<T: VoxelData, N: ChildNodeTrait<T>, const LOG2: usize> ChildNodeTrait<T> for InternalNode<T, N, LOG2> {
    /// Returns the log2 of the number of children this internal node can contain
    fn log2() -> u32 {
        LOG2 as u32
    }

    /// Returns the cumulative log2 of the number of children the hierarchy can contain
    fn log2_cum() -> u32 {
        LOG2 as u32 + <N as ChildNodeTrait::<_>>::log2_cum()
    }

    fn create(coord: Vec3i, level: u32, background_value: T) -> Self {
        Self::from_level_and_coord(level, coord, background_value)
    }
}

// Implementation of NodeDiagnostics for InternalNode
impl<T: VoxelData, N: ChildNodeTrait<T>, const LOG2: usize> NodeDiagnostics<T> for InternalNode<T, N, LOG2> {
    /// Returns the log2 of the child size (same as LOG2 for internal nodes)
    fn log2_child_size(&self) -> u32 {
        LOG2 as u32
    }

    /// Returns the node type (always Internal for internal nodes)
    fn node_type(&self) -> NodeType {
        NodeType::Internal
    }

    /// Returns the depth of this node (same as level for internal nodes)
    fn depth(&self) -> u32 {
        self.level
    }

    /// Returns the number of child nodes
    fn child_count(&self) -> usize {
        self.data.iter().filter(|e| e.is_some()).count()
    }
}

// Implement Default for common voxel types
impl<T: VoxelData, N: ChildNodeTrait<T>, const LOG2: usize> Default for InternalNode<T, N, LOG2> {
    fn default() -> Self {
        Self::from_level_and_coord(0, Vec3i::zero(), T::background())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    use crate::voxel::LeafNode;

    #[test]
    fn test_child_node_trait() {
        assert_eq!(InternalNode::<f32, LeafNode<f32, 1>, 2>::log2(), 2);
        assert_eq!(InternalNode::<f32, LeafNode<f32, 2>, 3>::log2(), 3);
        assert_eq!(InternalNode::<f32, LeafNode<f32, 3>, 4>::log2(), 4);

        assert_eq!(<InternalNode::<f32, LeafNode<f32, 1>, 2> as ChildNodeTrait::<f32>>::log2_cum(), 3);
        assert_eq!(<InternalNode::<f32, LeafNode<f32, 2>, 3> as ChildNodeTrait::<f32>>::log2_cum(), 5);
        assert_eq!(<InternalNode::<f32, LeafNode<f32, 3>, 4> as ChildNodeTrait::<f32>>::log2_cum(), 7);
    }

    #[test]
    fn test_node_diagnostics() {
        let mut internal = InternalNode::<f32, LeafNode<f32, 2>, 3>::from_level_and_coord(5, Vec3i::zero(), 0.0);
        
        // Test diagnostics
        assert_eq!(internal.log2_child_size(), 3);
        assert_eq!(internal.node_type(), NodeType::Internal);
        assert_eq!(internal.depth(), 5);
        assert_eq!(internal.child_count(), 0); // No active children initially
    }

    #[test]
    fn convert_coord_to_index() {
        let internal = InternalNode::<f32, LeafNode<f32, 2>, 3>::from_level_and_coord(5, Vec3i::zero(), 0.0);
        
        assert_eq!(internal.level(), 5);
        assert_eq!(internal.bounds(), Bounds3i::new(Vec3i::zero(), Vec3i::new(32, 32, 32)));
        assert_eq!(internal.active_count(), 0);
        assert_eq!(internal.dimensions(), Vec3i::new(32, 32, 32)); // 2^(2+3 in each direction
        assert_eq!(InternalNode::<f32, LeafNode<f32, 2>, 3>::child_capacity(), 512);

        assert_eq!(internal.coord_to_index(Vec3i::new(0, 0, 0)), Some(0));
        assert_eq!(internal.coord_to_index(Vec3i::new(1, 1, 1)), Some(0));
        assert_eq!(internal.coord_to_index(Vec3i::new(4, 2, 2)), Some(1));
        assert_eq!(internal.coord_to_index(Vec3i::new(2, 4, 2)), Some(8));
        assert_eq!(internal.coord_to_index(Vec3i::new(2, 2, 4)), Some(64));
        assert_eq!(internal.coord_to_index(Vec3i::new(30, 15, 10)), Some(159)); // = 7 + 3 x 8 + 2 x 8 x 8

        assert_eq!(internal.coord_to_index(Vec3i::new(40, 5, 1)), None);
        assert_eq!(internal.coord_to_index(Vec3i::new(32, 1, 1)), None);

        let internal = InternalNode::<f32, LeafNode<f32, 2>, 3>::from_level_and_coord(5, Vec3i::new(32, 64, 96), 0.0);
        assert_eq!(internal.coord_to_index(Vec3i::new(32, 64, 96)), Some(0));
        assert_eq!(internal.coord_to_index(Vec3i::new(33, 65, 97)), Some(0));
        assert_eq!(internal.coord_to_index(Vec3i::new(62, 79, 106)), Some(159));
        assert_eq!(internal.coord_to_index(Vec3i::new(15, 45, 85)), None);
        assert_eq!(internal.coord_to_index(Vec3i::new(100, 120, 150)), None);
    }

    #[test]
    fn convert_index_to_coord() {
        let internal = InternalNode::<f32, LeafNode<f32, 2>, 3>::from_level_and_coord(5, Vec3i::zero(), 0.0);
        assert_eq!(internal.index_to_coord(0), Vec3i::new(0, 0, 0));
        assert_eq!(internal.index_to_coord(1), Vec3i::new(4, 0, 0));
        assert_eq!(internal.index_to_coord(8), Vec3i::new(0, 4, 0));
        assert_eq!(internal.index_to_coord(64), Vec3i::new(0, 0, 4));
        assert_eq!(internal.index_to_coord(159), Vec3i::new(28, 12, 8));

        let internal = InternalNode::<f32, LeafNode<f32, 2>, 3>::from_level_and_coord(5, Vec3i::new(32, 64, 96), 0.0);
        assert_eq!(internal.index_to_coord(0), Vec3i::new(32, 64, 96));
        assert_eq!(internal.index_to_coord(159), Vec3i::new(60, 76, 104));
    }


    #[test]
    fn test_different_powers() {
        // Check capacity
        /*
        assert_eq!(InternalNode::<f32, (), f32, 3>::child_capacity(), 24);
        assert_eq!(InternalNode::<f32, (), f32, 5>::child_capacity(), 96);
        assert_eq!(InternalNode::<f32, (), f32, 6>::child_capacity(), 192);
        
        assert_eq!(InternalNode::<f32, (), f32, 3>::calculate_dimensions(), Vec3i::new(8, 8, 8));
        assert_eq!(InternalNode::<f32, (), f32, 5>::calculate_dimensions(), Vec3i::new(32, 32, 32));
        assert_eq!(InternalNode::<f32, (), f32, 6>::calculate_dimensions(), Vec3i::new(64, 64, 64));
        */
    }

}
