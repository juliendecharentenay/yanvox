use crate::voxel::VoxelData;

/// Simple boolean voxel (occupied/empty)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolVoxel(pub bool);

impl VoxelData for BoolVoxel {
    fn is_active(&self) -> bool {
        self.0
    }

    fn background() -> Self {
      Self(false)
    }
}

impl Default for BoolVoxel {
    fn default() -> Self {
        Self(false)
    }
}
