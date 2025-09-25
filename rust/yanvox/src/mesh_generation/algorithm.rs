use crate::voxel::{VoxelVolume, SignedDistance};
use crate::math::{Vec3i, Vec3f};
use super::mesh::{Mesh, Vertex, Triangle};
use super::marching_cubes::{CORNER_OFFSETS, EDGE_VERTEX_INDICES, EDGE_MASKS, TRIANGLE_TABLE};
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

    /// Process a single cube for marching cubes with proper edge vertex interpolation
    fn process_cube<T: SignedDistance + Clone + 'static>(
        &self,
        volume: &VoxelVolume<T>,
        mesh: &mut Mesh,
        coord: Vec3i,
        iso_level: f32,
    ) -> Result<(), AlgorithmError> {
        // Get the 8 corner values of the cube
        if let Some(corner_values) = self.get_cube_corner_values(volume, coord) {
            // Calculate the cube configuration index
            let cube_index = self.calculate_cube_index(&corner_values, iso_level);
            
            // Skip if no surface intersection
            if cube_index == 0 || cube_index == 255 {
                return Ok(());
            }
            
            // Get the edge mask for this configuration
            let edge_mask = EDGE_MASKS[cube_index as usize];
            
            // Calculate vertex positions on active edges
            let mut edge_vertices = [Vec3f::new(0.0, 0.0, 0.0); 12];
            let leaf_size = volume.get_leaf_voxel_size();
            
            for edge in 0..12 {
                if (edge_mask & (1 << edge)) != 0u16 {
                    edge_vertices[edge] = self.interpolate_edge_vertex(
                        &corner_values,
                        coord,
                        edge,
                        iso_level,
                        leaf_size
                    );
                }
            }
            
            // Generate triangles using the triangulation table
            let triangle_data = TRIANGLE_TABLE[cube_index as usize];
            let mut i = 0;
            while i < 16 && triangle_data[i] != -1i32 {
                let v1_idx = triangle_data[i] as usize;
                let v2_idx = triangle_data[i + 1] as usize;
                let v3_idx = triangle_data[i + 2] as usize;
                
                // Add vertices to mesh and create triangle
                let v1 = mesh.add_vertex(Vertex { position: edge_vertices[v1_idx] });
                let v2 = mesh.add_vertex(Vertex { position: edge_vertices[v2_idx] });
                let v3 = mesh.add_vertex(Vertex { position: edge_vertices[v3_idx] });
                
                mesh.add_triangle(Triangle { indices: [v1, v2, v3] });
                
                i += 3;
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
        let offsets = self.corner_offsets();

        let mut values = [0.0; 8];
        for (i, offset) in offsets.iter().enumerate() {
            let corner_coord = coord + *offset;
            let voxel = volume.get_voxel(corner_coord);
            if ! voxel.is_active() { return None; }
            values[i] = voxel.signed_distance();
        }
        Some(values)
    }

    /// Calculate the cube configuration index based on corner values and iso level
    fn calculate_cube_index(&self, corner_values: &[f32; 8], iso_level: f32) -> u8 {
        let mut cube_index = 0u8;
        
        for (i, &value) in corner_values.iter().enumerate() {
            if value < iso_level {
                cube_index |= 1 << i;
            }
        }
        
        cube_index
    }

    /// Interpolate vertex position on an edge based on iso level
    fn interpolate_edge_vertex(
        &self,
        corner_values: &[f32; 8],
        coord: Vec3i,
        edge: usize,
        iso_level: f32,
        leaf_size: f32,
    ) -> Vec3f {
        let edge_indices = EDGE_VERTEX_INDICES[edge];
        let v1_idx = edge_indices[0] as usize;
        let v2_idx = edge_indices[1] as usize;
        
        let val1 = corner_values[v1_idx];
        let val2 = corner_values[v2_idx];
        
        // Calculate interpolation factor
        let t = if (val2 - val1).abs() < 1e-6 {
            0.5 // Avoid division by zero
        } else {
            (iso_level - val1) / (val2 - val1)
        };
        
        // Clamp t to valid range
        let t = t.clamp(0.0, 1.0);
        
        // Get corner positions
        let corner_offsets: [Vec3i; 8] = self.corner_offsets();
        
        let pos1 = (coord + corner_offsets[v1_idx]).as_vec3f().scale(leaf_size);
        let pos2 = (coord + corner_offsets[v2_idx]).as_vec3f().scale(leaf_size);
        
        // Interpolate between the two corner positions
        pos1 + (pos2 - pos1).scale(t)
    }

    fn corner_offsets(&self) -> [Vec3i; 8] {
      [
        CORNER_OFFSETS[0].into(),
        CORNER_OFFSETS[1].into(),
        CORNER_OFFSETS[2].into(),
        CORNER_OFFSETS[3].into(),
        CORNER_OFFSETS[4].into(),
        CORNER_OFFSETS[5].into(),
        CORNER_OFFSETS[6].into(),
        CORNER_OFFSETS[7].into(),
      ]
    }
}
