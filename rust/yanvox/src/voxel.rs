use super::*;
use math::{Vec3i, Vec3f, Bounds3i, Bounds3f};
use serde::{Deserialize, Serialize};

mod root_node; use root_node::RootNode;
mod internal_node; use internal_node::InternalNode;
mod leaf_node; use leaf_node::LeafNode;

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

trait ChildNodeTrait<T: VoxelData>: NodeTrait<T> {
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


pub struct VoxelVolume<T: VoxelData> {
    root: Box<dyn NodeTrait<T>>,
    config: VolumeConfig,
}

/// Summary information for a VoxelVolume
#[derive(Debug, Clone)]
pub struct VoxelVolumeSummary {
    root_length: f32,
    leaf_length: f32,
    bounds: Bounds3i,
    active_voxels: usize,
    total_voxels: usize,
    memory_estimate: usize,
    world_bounds: Bounds3f,
}

impl<T: VoxelData + Clone + 'static> VoxelVolume<T> {
    /// Create a new voxel volume with a configuration
    pub fn with_config(config: VolumeConfig) -> Self {
        let root: Box<dyn NodeTrait<T>> = match config.volume_config_type {
            VolumeConfigType::Default => Box::new(RootNode::<T, LeafNode<T, 2>>::default()),
            VolumeConfigType::Hashx5x4 => Box::new(RootNode::<T, InternalNode<T, LeafNode<T, 4>, 5>>::default()),
            VolumeConfigType::Hashx2x1 => Box::new(RootNode::<T, InternalNode<T, LeafNode<T, 1>, 2>>::default()),
        };
        Self {
            root,
            config,
        }
    }
    
    fn cv_coord(&self, coord: Vec3f) -> Vec3i {
      let leaf_voxel_size = &self.config.leaf_voxel_size;
      coord.scale(1.0_f32 / leaf_voxel_size).as_vec3i()
    }

    // Basic voxel operations
    /// Get a voxel at a given coordinate
    pub fn get_voxel(&self, coord: Vec3i) -> &T {
        self.root.get_voxel(coord)
    }

    pub fn get_voxel_f(&self, coord: Vec3f) -> &T {
      self.get_voxel(self.cv_coord(coord))
    }

    /// Set a voxel at a given coordinate
    pub fn set_voxel(&mut self, coord: Vec3i, value: T) -> Option<T> {
        self.root.set_voxel(coord, value)
    }

    pub fn set_voxel_f(&mut self, coord: Vec3f, value: T) -> Option<T> {
      self.set_voxel(self.cv_coord(coord), value)
    }

    /// Remove a voxel at a given coordinate
    pub fn remove_voxel(&mut self, coord: Vec3i) -> Option<T> {
        self.root.remove_voxel(coord)
    }

    pub fn remove_voxel_f(&mut self, coord: Vec3f) -> Option<T> {
      self.remove_voxel(self.cv_coord(coord))
    }

    /// Check if a voxel at a given coordinate is active
    pub fn is_active(&self, coord: Vec3i) -> bool {
        self.root.is_active(coord)
    }

    pub fn is_active_f(&self, coord: Vec3f) -> bool {
        self.is_active(self.cv_coord(coord))
    }

    /// Get the number of active voxels
    pub fn active_count(&self) -> usize {
        self.root.active_count()
    }

    /// Get the total number of voxels
    pub fn total_count(&self) -> usize {
        self.root.total_count()
    }

    // Batch operations

    /*
    /// Clear a region
    pub fn clear_region(&mut self, bounds: Bounds3i) -> usize {
        self.root.clear_region(bounds)
    }

    /// Copy a region
    pub fn copy_region(&self, bounds: Bounds3i) -> Vec<(Vec3i, T)> {
        self.root.copy_region(bounds)
    }

    /// Paste a region
    pub fn paste_region(&mut self, data: &[(Vec3i, T)]) -> usize {
        self.root.paste_region(data)
    }
        */

    /// Estimate memory usage in bytes
    pub fn estimate_memory_usage(&self) -> usize {
        // Base memory for the struct itself
        let mut memory = std::mem::size_of::<Self>();
        
        // Add memory for active voxels (rough estimate)
        let active_voxels = self.active_count();
        let voxel_size = std::mem::size_of::<T>();
        memory += active_voxels * voxel_size;
        
        // Add overhead for the tree structure (rough estimate)
        // This is a conservative estimate - actual usage depends on tree structure
        let tree_overhead = active_voxels * std::mem::size_of::<usize>() * 2;
        memory += tree_overhead;
        
        memory
    }
    
    /// Get the bounds of the voxel volume
    pub fn bounds(&self) -> Bounds3i {
        self.root.bounds()
    }
    
    /// Fill a rectangular region defined by world-space bounds with values generated by a function
    /// 
    /// # Arguments
    /// * `min` - Minimum corner of the region in world space
    /// * `max` - Maximum corner of the region in world space  
    /// * `generator` - Function that takes a world coordinate and returns a voxel value
    /// 
    /// # Returns
    /// Number of voxels that were set
    pub fn fill_bounds(&mut self, min: Vec3f, max: Vec3f, generator: impl Fn(Vec3f) -> Option<T>) -> usize {
        let mut count = 0;
        let min = self.world_to_voxel_coord(min);
        let max = self.world_to_voxel_coord(max);
        for coord in VoxelCoordIterator::new(min, max) {
            let world_coord = self.voxel_to_world_coord(coord);
            if let Some(value) = generator(world_coord) {
                self.set_voxel(coord, value);
                count += 1;
            }
        }
        count
    }

    /// Fill a rectangular region defined by a Bounds3f with values generated by a function
    /// 
    /// # Arguments
    /// * `bounds` - Bounds3f defining the region in world space
    /// * `generator` - Function that takes a world coordinate and returns a voxel value
    /// 
    /// # Returns
    /// Number of voxels that were set
    pub fn fill_region_bounds(&mut self, bounds: Bounds3f, generator: impl Fn(Vec3f) -> Option<T>) -> usize {
        self.fill_bounds(bounds.min, bounds.max, generator)
    }

    // ===== UTILITY FUNCTIONS =====

    /// Convert a world-space coordinate to the corresponding voxel coordinate
    /// 
    /// # Arguments
    /// * `world_coord` - Coordinate in world space
    /// 
    /// # Returns
    /// The corresponding voxel coordinate
    fn world_to_voxel_coord(&self, world_coord: Vec3f) -> Vec3i {
        self.cv_coord(world_coord)
    }

    /// Convert a voxel coordinate to the corresponding world-space coordinate
    /// 
    /// # Arguments
    /// * `voxel_coord` - Coordinate in voxel space
    /// 
    /// # Returns
    /// The corresponding world-space coordinate
    fn voxel_to_world_coord(&self, voxel_coord: Vec3i) -> Vec3f {
        let leaf_voxel_size = &self.config.leaf_voxel_size;
        voxel_coord.as_vec3f().scale(*leaf_voxel_size)
    }

    /// Get the root voxel size
    /// 
    /// # Returns
    /// The root voxel size in world units
    pub fn get_root_voxel_size(&self) -> f32 {
        let log2 = self.root.log2_cum() as i32;
        2.0_f32.powi(log2) * self.config.leaf_voxel_size
    }

    /// Get the actual leaf voxel size (the smallest voxel size in the hierarchy)
    /// 
    /// # Returns
    /// The leaf voxel size in world units
    pub fn get_leaf_voxel_size(&self) -> f32 {
        self.config.leaf_voxel_size
    }

    /// Get the voxel center coordinate for a given world coordinate
    /// 
    /// This snaps the world coordinate to the nearest voxel center
    /// 
    /// # Arguments
    /// * `world_coord` - Coordinate in world space
    /// 
    /// # Returns
    /// The voxel center coordinate in world space
    pub fn snap_to_voxel_center(&self, world_coord: Vec3f) -> Vec3f {
        let step_size = self.get_leaf_voxel_size();
        Vec3f::new(
            (world_coord.x / step_size).round() * step_size,
            (world_coord.y / step_size).round() * step_size,
            (world_coord.z / step_size).round() * step_size,
        )
    }

    /// Get a summary of the voxel volume
    pub fn summary(&self) -> VoxelVolumeSummary {
        let total_voxels = self.root.total_count();
        let active_voxels = self.root.active_count();
        let memory_estimate = self.estimate_memory_usage();
        let bounds = self.bounds();
        let world_bounds = Bounds3f::new(
            self.voxel_to_world_coord(bounds.min),
            self.voxel_to_world_coord(bounds.max),
        );
        VoxelVolumeSummary {
            root_length: self.get_root_voxel_size(),
            leaf_length: self.get_leaf_voxel_size(),
            bounds,
            active_voxels,
            total_voxels,
            memory_estimate,
            world_bounds,
        }
    }
}

impl std::fmt::Display for VoxelVolumeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VoxelVolume Summary:\n")?;
        write!(f, "  Size at root: {:.2} units\n", self.root_length)?;
        write!(f, "  Size at leaf: {:.2} units\n", self.leaf_length)?;
        write!(f, "  Bounds: [{}, {}, {}] to [{}, {}, {}]\n", 
               self.bounds.min.x, self.bounds.min.y, self.bounds.min.z,
               self.bounds.max.x, self.bounds.max.y, self.bounds.max.z)?;
        write!(f, "  World bounds: [{:.2}, {:.2}, {:.2}] to [{:.2}, {:.2}, {:.2}]\n", 
               self.world_bounds.min.x, self.world_bounds.min.y, self.world_bounds.min.z,
               self.world_bounds.max.x, self.world_bounds.max.y, self.world_bounds.max.z)?;
        write!(f, "  Active voxels: {} / {} ({:.1}%)\n", 
               self.active_voxels, self.total_voxels, 
               if self.total_voxels > 0 { (self.active_voxels as f64 / self.total_voxels as f64) * 100.0 } else { 0.0 })?;
        write!(f, "  Memory usage: ~{} bytes ({:.2} MB)\n", 
               self.memory_estimate, self.memory_estimate as f64 / 1_048_576.0)?;
        
        Ok(())
    }
}

// ===== VOXEL COORDINATE ITERATOR =====
/// Iterator over voxel-space coordinates
struct VoxelCoordIterator {
    min: Vec3i,
    max: Vec3i,
    current: Vec3i,
    finished: bool,
}

impl VoxelCoordIterator {
    fn new(min: Vec3i, max: Vec3i) -> Self {
        Self {
            min,
            max,
            current: min.clone(),
            finished: false,
        }
    }
}

impl Iterator for VoxelCoordIterator {
    type Item = Vec3i;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let result = self.current;

        // Advance to next voxel
        self.current.x += 1;
        if self.current.x >= self.max.x {
            self.current.x = self.min.x;
            self.current.y += 1;
            if self.current.y >= self.max.y {
                self.current.y = self.min.y;
                self.current.z += 1;
                if self.current.z >= self.max.z {
                    self.finished = true;
                }
            }
        }

        Some(result)
    }
}

/*
impl<T: VoxelData, const INTERNAL_CHILDREN: usize, const LEAF_CHILDREN: usize> VoxelVolume<T, INTERNAL_CHILDREN, LEAF_CHILDREN> {
    // Construction
    pub fn new() -> Self;
    pub fn with_config(config: VolumeConfig<T>) -> Self;
    pub fn with_background(background_value: T) -> Self;
    
    // Basic voxel operations
    pub fn get_voxel(&self, coord: Vec3i) -> T;
    pub fn set_voxel(&mut self, coord: Vec3i, value: T) -> T;
    pub fn remove_voxel(&mut self, coord: Vec3i) -> Option<T>;
    pub fn is_active(&self, coord: Vec3i) -> bool;
    
    // Batch operations
    pub fn fill_region(&mut self, bounds: Bounds3i, value: T) -> usize;
    pub fn clear_region(&mut self, bounds: Bounds3i) -> usize;
    pub fn copy_region(&self, bounds: Bounds3i) -> Vec<(Vec3i, T)>;
    pub fn paste_region(&mut self, data: &[(Vec3i, T)]) -> usize;
    
    // Query operations
    pub fn query_bounds(&self, bounds: Bounds3i) -> impl Iterator<Item = (Vec3i, T)>;
    pub fn query_radius(&self, center: Vec3i, radius: f32) -> impl Iterator<Item = (Vec3i, T)>;
    pub fn query_sphere(&self, center: Vec3i, radius: f32) -> impl Iterator<Item = (Vec3i, T)>;
    
    // Iteration
    pub fn active_voxels(&self) -> impl Iterator<Item = (Vec3i, T)>;
    pub fn all_voxels(&self) -> impl Iterator<Item = (Vec3i, T)>;
    pub fn active_voxels_in_bounds(&self, bounds: Bounds3i) -> impl Iterator<Item = (Vec3i, T)>;
    
    // Statistics and metadata
    pub fn active_count(&self) -> usize;
    pub fn total_count(&self) -> usize;
    pub fn memory_usage(&self) -> usize;
    pub fn bounds(&self) -> Bounds3i;
    pub fn is_empty(&self) -> bool;
    pub fn background_value(&self) -> T;
    
    // Configuration
    pub fn config(&self) -> &VolumeConfig<T>;
    pub fn set_background(&mut self, value: T);
    pub fn optimize(&mut self); // Defragmentation, compression, etc.
    
    // Utility
    pub fn clear(&mut self);
    pub fn clone_region(&self, bounds: Bounds3i) -> Self;
    pub fn merge(&mut self, other: &Self) -> usize;
}
    */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeConfig {
    pub compression: CompressionType,
    pub volume_config_type: VolumeConfigType,
    pub leaf_voxel_size: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VolumeConfigType {
    Default,
    Hashx5x4,
    Hashx2x1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionType {
    None,
    LZ4,
    Zstd,
    // Custom(Box<dyn CompressionAlgorithm>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voxel_volume_length_calculations() {
        let config = VolumeConfig {
          compression: CompressionType::None,
          leaf_voxel_size: 2.5,
          volume_config_type: VolumeConfigType::Default,
        };

        /*
        #[derive(Default)]
        struct InternalDefault {}
        impl std::convert::From<InternalDefault> for u32 {
          fn from(_: InternalDefault) -> u32 { 1u32 }
        }
        */

        let voxel_volume = VoxelVolume::<u32>::with_config(config);
        let summary = voxel_volume.summary();
        assert_eq!(summary.root_length, 10.0);
        assert_eq!(summary.leaf_length, 2.5);
    }
}
