use crate::math::Vec3f;
use std::io::{Write, Result as IoResult};

/// A 3D vertex with position
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub position: Vec3f,
}

/// A triangle face with three vertex indices
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub indices: [usize; 3],
}

/// A mesh containing vertices and triangles
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    /// Create a new empty mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }

    /// Get the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get the number of triangles
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    /// Add a vertex to the mesh and return its index
    pub fn add_vertex(&mut self, vertex: Vertex) -> usize {
        self.vertices.push(vertex);
        self.vertices.len() - 1
    }

    /// Add a triangle to the mesh
    pub fn add_triangle(&mut self, triangle: Triangle) {
        self.triangles.push(triangle);
    }

    /// Clear all vertices and triangles
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.triangles.clear();
    }

    /// Check if the mesh is empty
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    /// Calculate the normal vector for a triangle using the right-hand rule
    fn calculate_triangle_normal(&self, triangle: &Triangle) -> Vec3f {
        let v0 = self.vertices[triangle.indices[0]].position;
        let v1 = self.vertices[triangle.indices[1]].position;
        let v2 = self.vertices[triangle.indices[2]].position;
        
        // Calculate two edge vectors
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        
        // Cross product to get normal (right-hand rule)
        let normal = edge1.cross(&edge2);
        
        normal.normalize()
    }

    /// Export mesh to ASCII STL format
    pub fn export_stl_ascii<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writeln!(writer, "solid yanvox_mesh")?;
        
        for triangle in &self.triangles {
            let normal = self.calculate_triangle_normal(triangle);
            let v0 = self.vertices[triangle.indices[0]].position;
            let v1 = self.vertices[triangle.indices[1]].position;
            let v2 = self.vertices[triangle.indices[2]].position;
            
            writeln!(writer, "  facet normal {} {} {}", normal.x, normal.y, normal.z)?;
            writeln!(writer, "    outer loop")?;
            writeln!(writer, "      vertex {} {} {}", v0.x, v0.y, v0.z)?;
            writeln!(writer, "      vertex {} {} {}", v1.x, v1.y, v1.z)?;
            writeln!(writer, "      vertex {} {} {}", v2.x, v2.y, v2.z)?;
            writeln!(writer, "    endloop")?;
            writeln!(writer, "  endfacet")?;
        }
        
        writeln!(writer, "endsolid yanvox_mesh")?;
        Ok(())
    }

    /// Export mesh to binary STL format
    pub fn export_stl_binary<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        // STL binary header (80 bytes)
        let header = b"yanvox_mesh_binary_export".to_vec();
        let mut header_padded = [0u8; 80];
        let copy_len = header.len().min(80);
        header_padded[..copy_len].copy_from_slice(&header[..copy_len]);
        writer.write_all(&header_padded)?;
        
        // Number of triangles (4 bytes, little-endian)
        let triangle_count = self.triangles.len() as u32;
        writer.write_all(&triangle_count.to_le_bytes())?;
        
        // Write each triangle
        for triangle in &self.triangles {
            let normal = self.calculate_triangle_normal(triangle);
            let v0 = self.vertices[triangle.indices[0]].position;
            let v1 = self.vertices[triangle.indices[1]].position;
            let v2 = self.vertices[triangle.indices[2]].position;
            
            // Normal vector (12 bytes: 3 × 4-byte floats)
            writer.write_all(&normal.x.to_le_bytes())?;
            writer.write_all(&normal.y.to_le_bytes())?;
            writer.write_all(&normal.z.to_le_bytes())?;
            
            // Three vertices (36 bytes: 3 × 3 × 4-byte floats)
            writer.write_all(&v0.x.to_le_bytes())?;
            writer.write_all(&v0.y.to_le_bytes())?;
            writer.write_all(&v0.z.to_le_bytes())?;
            
            writer.write_all(&v1.x.to_le_bytes())?;
            writer.write_all(&v1.y.to_le_bytes())?;
            writer.write_all(&v1.z.to_le_bytes())?;
            
            writer.write_all(&v2.x.to_le_bytes())?;
            writer.write_all(&v2.y.to_le_bytes())?;
            writer.write_all(&v2.z.to_le_bytes())?;
            
            // Attribute byte count (2 bytes, usually 0)
            writer.write_all(&[0u8; 2])?;
        }
        
        Ok(())
    }

    /// Export mesh to STL file (auto-detects format based on file extension)
    pub fn export_stl_file<P: AsRef<std::path::Path>>(&self, path: P) -> IoResult<()> {
        let path = path.as_ref();
        let mut file = std::fs::File::create(path)?;
        
        match path.extension().and_then(|s| s.to_str()) {
            Some("stl") => {
                // Default to binary for .stl extension
                self.export_stl_binary(&mut file)
            }
            Some("astl") | Some("ascii") => {
                // ASCII format for .astl or .ascii extensions
                self.export_stl_ascii(&mut file)
            }
            _ => {
                // Default to binary if extension is unclear
                self.export_stl_binary(&mut file)
            }
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}
