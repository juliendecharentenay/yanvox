use crate::voxel::VoxelData;

/// Integer voxel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntVoxel(pub i32);

impl VoxelData for IntVoxel {
    fn is_active(&self) -> bool {
        self.0 != 0
    }

    fn background() -> Self { Self(0) }
}

impl Default for IntVoxel {
    fn default() -> Self {
        Self(0)
    }
}
