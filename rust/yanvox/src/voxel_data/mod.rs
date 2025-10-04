use crate::voxel::VoxelData;

mod signed_distance;
mod bool_voxel;
mod float_voxel;
mod int_voxel;

pub use signed_distance::SignedDistance;

// Implement VoxelData for common primitive types
impl VoxelData for f32 {
    fn is_active(&self) -> bool {
        *self != 0.0
    }
    fn background() -> Self { 0.0 }
}

impl VoxelData for f64 {
    fn is_active(&self) -> bool {
        *self != 0.0
    }

    fn background() -> Self { 0.0 }
}

impl VoxelData for i32 {
    fn is_active(&self) -> bool {
        *self != 0
    }
    fn background() -> Self { 0 }
}

impl VoxelData for u32 {
    fn is_active(&self) -> bool {
        *self != 0
    }
    fn background() -> Self { 0 }
}

impl VoxelData for bool {
    fn is_active(&self) -> bool {
        *self
    }
    fn background() -> Self { false }
}
