//! Mathematical utilities and data structures for 3D operations

mod vec3i;
mod vec3f;
mod bounds3i;
mod bounds3f;

pub use vec3i::Vec3i;
pub use vec3f::Vec3f;
pub use bounds3i::Bounds3i;
pub use bounds3f::Bounds3f;

/// Type aliases for common use cases
pub type Vec3 = Vec3i;
pub type Bounds3 = Bounds3i;
