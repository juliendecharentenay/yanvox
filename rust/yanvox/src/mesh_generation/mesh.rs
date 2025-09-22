use crate::math::Vec3f;

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
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}
