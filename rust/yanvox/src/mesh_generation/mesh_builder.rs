use crate::voxel::{VoxelVolume, SignedDistance};
use super::mesh::Mesh;
use super::algorithm::MarchingCubesAlgorithm;
use thiserror::Error;

/// Error types for mesh building
#[derive(Debug, Error)]
pub enum MeshBuilderError {
    #[error("No iso level specified")]
    NoIsoLevel,
    #[error("Invalid iso level: {0}")]
    InvalidIsoLevel(f32),
    #[error("Mesh generation failed: {0}")]
    GenerationFailed(String),
}

/// Builder for creating meshes from voxel volumes using marching cubes
pub struct MeshBuilder<'a, T: SignedDistance> {
    voxel_volume: &'a VoxelVolume<T>,
    iso_level: Option<f32>,
    algorithm: MarchingCubesAlgorithm,
}

impl<'a, T: SignedDistance + Clone + 'static> MeshBuilder<'a, T> {
    /// Create a new mesh builder with a voxel volume
    pub fn new(voxel_volume: &'a VoxelVolume<T>) -> Self {
        Self {
            voxel_volume,
            iso_level: None,
            algorithm: MarchingCubesAlgorithm::new(),
        }
    }

    /// Set the iso level for surface extraction
    pub fn with_iso_level(mut self, iso_level: f32) -> Self {
        self.iso_level = Some(iso_level);
        self
    }

    /// Build the mesh using the configured parameters
    pub fn build(self) -> Result<Mesh, MeshBuilderError> {
        let iso_level = self.iso_level.ok_or(MeshBuilderError::NoIsoLevel)?;

        // Validate iso level
        if !iso_level.is_finite() {
            return Err(MeshBuilderError::InvalidIsoLevel(iso_level));
        }

        // Delegate to the algorithm
        self.algorithm.generate_mesh(self.voxel_volume, iso_level)
            .map_err(|e| MeshBuilderError::GenerationFailed(e.to_string()))
    }
}
