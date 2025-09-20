use crate::voxel::VoxelData;

/// Floating point voxel
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatVoxel(pub f32);

impl VoxelData for FloatVoxel {
    fn is_active(&self) -> bool {
        self.0 != 0.0
    }
    fn background() -> Self { Self(0.0) }
}

impl Default for FloatVoxel {
    fn default() -> Self {
        Self(0.0)
    }
}
