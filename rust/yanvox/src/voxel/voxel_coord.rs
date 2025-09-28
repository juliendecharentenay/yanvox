use crate::math::{Vec3i, Vec3f};

/// Represents a voxel coordinate in both voxel space and world space
#[derive(Debug)]
pub struct VoxelCoord {
    /// Coordinate in voxel space (integer grid)
    pub voxel: Vec3i,
    /// Coordinate in world space (floating point)
    pub world: Vec3f,
}

impl VoxelCoord {
    /// Create a new VoxelCoord from both voxel and world coordinates
    pub fn new(voxel: Vec3i, world: Vec3f) -> Self {
        Self { voxel, world }
    }
    
    /// Create a VoxelCoord from voxel coordinates, computing world coordinates
    pub fn from_voxel(voxel: Vec3i, leaf_voxel_size: f32) -> Self {
        let world = voxel.as_vec3f().scale(leaf_voxel_size);
        Self { voxel, world }
    }
}