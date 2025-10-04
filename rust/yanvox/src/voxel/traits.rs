use crate::math::{Vec3i, Vec3f, Bounds3i, Bounds3f};

pub trait VoxelData: Clone + std::cmp::PartialEq {
    /// Check if this voxel is "active" (non-empty)
    fn is_active(&self) -> bool;

    /// Retrieve background value
    fn background() -> Self;
}

/// Unified trait that all nodes implement
/// This allows seamless traversal from root to leaf
pub trait NodeTrait<T: VoxelData> {
    // Basic node information
    fn level(&self) -> u32;
    fn log2_cum(&self) -> u32;
    fn bounds(&self) -> Bounds3i;
    fn is_active(&self, coord: Vec3i) -> bool;
    fn active_count(&self) -> usize;
    fn total_count(&self) -> usize;
    
    // Data operations (implemented by leaf nodes, return None for internal nodes)
    fn get_voxel(&self, coord: Vec3i) -> &T;
    fn set_voxel(&mut self, coord: Vec3i, value: T) -> Option<T>;
    fn remove_voxel(&mut self, coord: Vec3i) -> Option<T>;

    // Iterator operations
    fn active_voxels(&self) -> Box<dyn Iterator<Item = (Vec3i, &T)> + '_>;
    fn all_voxels(&self) -> Box<dyn Iterator<Item = (Vec3i, &T)> + '_>;    
}

pub trait ChildNodeTrait<T: VoxelData>: NodeTrait<T> {
    fn log2() -> u32;
    fn log2_cum() -> u32;

    /// Calculate the key (lower left corner)
    /// 
    /// This is used to find the child node for a given coordinate
    /// and to create a new child node
    /// 
    /// The key is the lower left corner of the child node's bounds
    /// 
    /// The key is used to index the child node in the parent node's data structure
    fn key(coord: Vec3i) -> Vec3i {
        let size = 1 << <Self as ChildNodeTrait<T>>::log2_cum();
        let key =Vec3i::new(
            coord.x &! (size - 1),
            coord.y &! (size - 1),
            coord.z &! (size - 1),
        );
        log::debug!("Key for location {coord:?} => {key:?}");
        key
    }
    
    // Add factory method
    fn create(coord: Vec3i, level: u32, background_value: T) -> Self;
}

// Separate trait for advanced users
pub trait NodeDiagnostics<T: VoxelData> {
    fn log2_child_size(&self) -> u32;
    fn node_type(&self) -> NodeType;
    fn depth(&self) -> u32;
    fn child_count(&self) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Root,
    Internal,
    Leaf,
}

