use super::*;

/// Trait for voxel data that can provide signed distance values for mesh generation
pub trait SignedDistance: VoxelData {
    /// Get the signed distance value at this voxel
    /// Positive values indicate outside the surface, negative values indicate inside
    fn signed_distance(&self) -> f32;
}

