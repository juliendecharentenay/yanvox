use crate::voxel::{VoxelVolume, SignedDistance};
use crate::math::{Vec3i, Vec3f};
use super::mesh::{Mesh, Vertex, Triangle};
use thiserror::Error;

/// Error types for algorithm
#[derive(Debug, Error)]
pub enum AlgorithmError {
}

/// Core marching cubes algorithm implementation
pub struct MarchingCubesAlgorithm {
    // Configuration parameters could be added here
}

impl MarchingCubesAlgorithm {
    pub fn new() -> Self {
        Self {}
    }

    /// Generate a mesh using the marching cubes algorithm
    pub fn generate_mesh<T: SignedDistance + Clone + 'static>(
        &self,
        volume: &VoxelVolume<T>,
        iso_level: f32,
    ) -> Result<Mesh, AlgorithmError> {
        let mut mesh = Mesh::new();

        // Use the active voxels iterator from VoxelVolume
        for (coord, _voxel) in volume.active_voxels() {
            self.process_cube(volume, &mut mesh, coord, iso_level)?;
        }

        Ok(mesh)
    }

    /// Process a single cube for marching cubes (simplified version)
    fn process_cube<T: SignedDistance + Clone + 'static>(
        &self,
        volume: &VoxelVolume<T>,
        mesh: &mut Mesh,
        coord: Vec3i,
        iso_level: f32,
    ) -> Result<(), AlgorithmError> {
        // Get the 8 corner values of the cube
        if let Some(corner_values) = self.get_cube_corner_values(volume, coord) {

            // Simple case: if we have a mix of values above and below iso level, create a triangle
            let below_count = corner_values.iter().filter(|&&v| v < iso_level).count();
        
            if below_count > 0 && below_count < 8 {
                // Create a simple triangle at the center of the cube for now
                let leaf_size = volume.get_leaf_voxel_size();
                let center = coord.as_vec3f().scale(leaf_size) + Vec3f::new(leaf_size * 0.5, leaf_size * 0.5, leaf_size * 0.5);
            
                // Create 3 vertices for a simple triangle
                let v1 = mesh.add_vertex(Vertex { position: center + Vec3f::new(-0.1, 0.0, 0.0) });
                let v2 = mesh.add_vertex(Vertex { position: center + Vec3f::new(0.1, 0.0, 0.0) });
                let v3 = mesh.add_vertex(Vertex { position: center + Vec3f::new(0.0, 0.1, 0.0) });
            
                mesh.add_triangle(Triangle { indices: [v1, v2, v3] });
            }
        }

        Ok(())
    }

    /// Get the signed distance values at the 8 corners of a cube
    fn get_cube_corner_values<T: SignedDistance + Clone + 'static>(
        &self,
        volume: &VoxelVolume<T>,
        coord: Vec3i,
    ) -> Option<[f32; 8]> {
        let offsets = [
            Vec3i::new(0, 0, 0), Vec3i::new(1, 0, 0),
            Vec3i::new(1, 1, 0), Vec3i::new(0, 1, 0),
            Vec3i::new(0, 0, 1), Vec3i::new(1, 0, 1),
            Vec3i::new(1, 1, 1), Vec3i::new(0, 1, 1),
        ];

        let mut values = [0.0; 8];
        for (i, offset) in offsets.iter().enumerate() {
            let corner_coord = coord + *offset;
            let voxel = volume.get_voxel(corner_coord);
            if ! voxel.is_active() { return None; }
            values[i] = voxel.signed_distance();
        }
        Some(values)
    }
}
