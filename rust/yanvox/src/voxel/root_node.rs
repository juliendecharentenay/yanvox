//! Root node implementation with internal hierarchy management
use super::*;
use math::{Vec3i, Bounds3i};
use voxel::{VoxelData, NodeTrait};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Root node of a hierarchical voxel data structure.
/// 
/// The `RootNode` serves as the top-level container in a multi-level voxel hierarchy,
/// managing child nodes that contain the actual voxel data. It acts as a coordinator
/// that delegates operations to appropriate child nodes based on spatial coordinates.
/// 
/// # Type Parameters
/// 
/// * `T` - The voxel data type that implements `VoxelData`
/// * `N` - The child node type that implements `ChildNodeTrait<T>`
/// 
/// # Fields
/// 
/// * `level` - The hierarchical level of this node (always 0 for root nodes)
/// * `background_value` - The default value used for inactive/empty voxels
/// * `children` - A hash map storing child nodes keyed by their spatial coordinates
/// 
/// # Example
/// 
/// ```ignore
/// use yanvox::voxel::{RootNode, LeafNode};
/// use yanvox::math::Vec3i;
/// 
/// // Create a root node with f32 voxels and 6-level leaf nodes
/// let mut root = RootNode::<f32, LeafNode<f32, 6>>::default();
/// 
/// // Set a voxel value
/// root.set_voxel(Vec3i::new(10, 20, 30), 1.5);
/// 
/// // Check if a voxel is active
/// assert!(root.is_active(Vec3i::new(10, 20, 30)));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootNode<T: VoxelData, N: ChildNodeTrait<T>> {
    /// The hierarchical level of this node (always 0 for root nodes)
    pub level: u32,
    /// The default value used for empty child nodes
    pub background_value: T,
    /// Child nodes stored in a hash map keyed by their spatial coordinates
    children: HashMap<Vec3i, N>,
}

impl<T: VoxelData, N: ChildNodeTrait<T>> Default for RootNode<T, N> {
    /// Creates a new empty root node with default values.
    /// 
    /// This implementation:
    /// - Sets the level to 0 (root level)
    /// - Initializes the background value using `T::background()`
    /// - Validates that the background value is inactive (panics if not)
    /// - Creates an empty children hash map
    /// 
    /// # Panics
    /// 
    /// Panics if the background value returned by `T::background()` is active.
    /// This ensures that the background value represents an "empty" state.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// use yanvox::voxel::{RootNode, LeafNode};
    /// 
    /// let root = RootNode::<f32, LeafNode<f32, 6>>::default();
    /// assert_eq!(root.level(), 0);
    /// assert_eq!(root.background_value(), &0.0);
    /// assert_eq!(root.active_count(), 0);
    /// ```
    fn default() -> Self {
        let background_value = T::background();
        if background_value.is_active() { panic!("Background value is active"); }
        Self {
            level: 0,
            background_value,
            children: HashMap::new(),
        }
    }
}

impl<T: VoxelData, N: ChildNodeTrait<T>> RootNode<T, N> {
    /// Creates a new child node for the given coordinate.
    /// 
    /// This method creates a new child node at the appropriate level and inserts it into the children map.
    /// 
    /// # Arguments
    /// 
    /// * `coord` - The 3D coordinate to create a child node for
    /// 
    /// # Returns
    /// 
    /// Returns a mutable reference to the new child node.
    fn create_child(&mut self, coord: Vec3i) -> &mut N {
        let child_key = <N as ChildNodeTrait::<T>>::key(coord);
        let child = N::create(coord, self.level+1, self.background_value.clone());
        self.children.insert(child_key, child);
        self.children.get_mut(&child_key).unwrap()
    }

    /// Calculates the child key (lower-left corner) for a given coordinate.
    /// 
    /// The child key identifies which child node should contain the given coordinate.
    /// This is used for spatial partitioning in the hierarchical structure.
    /// 
    /// # Arguments
    /// 
    /// * `coord` - The 3D coordinate to find the child key for
    /// 
    /// # Returns
    /// 
    /// Returns the lower-left corner coordinate of the child node that should
    /// contain the given coordinate.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// use yanvox::math::Vec3i;
    /// use yanvox::voxel::{RootNode, LeafNode};
    /// // For a 6-level leaf node (2^6 = 64 voxels per side)
    /// let root = RootNode::<f32, LeafNode<f32, 6>>::default();
    /// assert_eq!(root.calculate_child_key(Vec3i::new(0, 0, 0)), Vec3i::new(0, 0, 0));
    /// assert_eq!(root.calculate_child_key(Vec3i::new(33, 2, 3)), Vec3i::new(32, 0, 0));
    /// ```
    fn calculate_child_key(&self, coord: Vec3i) -> Vec3i {
        <N as ChildNodeTrait::<T>>::key(coord)
    }

    /// Finds an existing child node for the given coordinate.
    /// 
    /// This method performs a read-only lookup to find a child node that contains
    /// the specified coordinate.
    /// 
    /// # Arguments
    /// 
    /// * `coord` - The 3D coordinate to find a child node for
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&N)` if a child node exists for the coordinate's region,
    /// or `None` if no child node has been created for that region yet.
    fn find_child(&self, coord: Vec3i) -> Option<&N> {
        let child_key = <N as ChildNodeTrait::<T>>::key(coord);
        self.children.get(&child_key)
    }

    /// Finds an existing child node for the given coordinate (mutable version).
    /// 
    /// This method performs a mutable lookup to find a child node that contains
    /// the specified coordinate.
    /// 
    /// # Arguments
    /// 
    /// * `coord` - The 3D coordinate to find a child node for
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&mut N)` if a child node exists for the coordinate's region,
    /// or `None` if no child node has been created for that region yet.
    fn find_child_mut(&mut self, coord: Vec3i) -> Option<&mut N> {
        let child_key = <N as ChildNodeTrait::<T>>::key(coord);
        self.children.get_mut(&child_key)
    }
}

impl<T: VoxelData, N: ChildNodeTrait<T>> NodeTrait<T> for RootNode<T, N> {
    /// Returns the hierarchical level of this node.
    /// 
    /// For root nodes, this is always 0, representing the top level of the hierarchy.
    /// 
    /// # Returns
    /// 
    /// The level of this node in the hierarchy (always 0 for root nodes).
    fn level(&self) -> u32 {
        self.level
    }

    /// Returns the cumulative log2 size of child nodes.
    /// 
    /// This represents the total size of child nodes in the hierarchy below this node.
    /// For root nodes, this delegates to the child node type's `log2_cum()` method.
    /// 
    /// # Returns
    /// 
    /// The cumulative log2 size of child nodes.
    fn log2_cum(&self) -> u32 {
      <N as ChildNodeTrait::<T>>::log2_cum()
    }

    /// Returns the bounding box of all active voxels in this node and its children.
    /// 
    /// The bounds represent the spatial extent of all voxel data stored in the hierarchy.
    /// If no children exist, returns an empty bounds.
    /// 
    /// # Returns
    /// 
    /// A `Bounds3i` representing the spatial extent of all voxel data.
    /// Returns `Bounds3i::empty()` if no children exist.
    /// 
    /// # Performance
    /// 
    /// This method iterates through all child nodes to compute the union of their bounds.
    fn bounds(&self) -> Bounds3i {
        if self.children.is_empty() {
            Bounds3i::empty()
        } else {
            self.children.values()
                .map(|child| child.bounds())
                .fold(Bounds3i::empty(), |acc, bounds| {
                    if acc == Bounds3i::empty() {
                        bounds
                    } else {
                        acc.expand_bounds(bounds)
                    }
                })
        }
    }

    /// Checks if a voxel at the given coordinate is active.
    /// 
    /// An active voxel is one that contains non-background data. This method
    /// delegates to the appropriate child node if one exists for the coordinate.
    /// 
    /// # Arguments
    /// 
    /// * `coord` - The 3D coordinate to check
    /// 
    /// # Returns
    /// 
    /// Returns `true` if a child node exists for the coordinate and the voxel
    /// is active, `false` otherwise.
    fn is_active(&self, coord: Vec3i) -> bool {
        self.get_voxel(coord).is_active()
    }

    /// Returns the total number of active voxels in this node and all children.
    /// 
    /// Active voxels are those containing non-background data. This method
    /// recursively counts active voxels across all child nodes.
    /// 
    /// # Returns
    /// 
    /// The total number of active voxels in the entire hierarchy.
    fn active_count(&self) -> usize {
        self.children.values()
            .map(|child| child.active_count())
            .sum()
    }

    /// Returns the total number of voxels (active and inactive) in this node and all children.
    /// 
    /// This includes both active voxels (containing data) and inactive voxels
    /// (containing background values). This method recursively counts all voxels
    /// across all child nodes.
    /// 
    /// # Returns
    /// 
    /// The total number of voxels in the entire hierarchy.
    fn total_count(&self) -> usize {
        self.children.values()
            .map(|child| child.total_count())
            .sum()
    }

    /// Retrieves a voxel value at the given coordinate.
    /// 
    /// This method looks up the voxel data at the specified coordinate by
    /// delegating to the appropriate child node. If no child node exists
    /// for the coordinate, returns the background value.
    /// 
    /// # Arguments
    /// 
    /// * `coord` - The 3D coordinate to retrieve the voxel from
    /// 
    /// # Returns
    /// 
    /// Returns a reference to the voxel value at the coordinate.
    fn get_voxel(&self, coord: Vec3i) -> &T {
        if let Some(child) = self.find_child(coord) {
            child.get_voxel(coord)
        } else {
            &self.background_value
        }
    }

    /// Sets a voxel value at the given coordinate.
    /// 
    /// This method creates or updates voxel data at the specified coordinate.
    /// If no child node exists for the coordinate, one will be created automatically.
    /// 
    /// # Arguments
    /// 
    /// * `coord` - The 3D coordinate to set the voxel at
    /// * `value` - The value to store at the coordinate
    /// 
    /// # Returns
    /// 
    /// Returns `Some(T)` containing the previous value if one existed,
    /// or `None` if this is a new voxel.
    fn set_voxel(&mut self, coord: Vec3i, value: T) -> Option<T> {
        if let Some(child) = self.find_child_mut(coord) {
            // Root nodes delegate to existing children
            child.set_voxel(coord, value)
        } else if self.background_value != value {
            // Create a new child node if the background value is different
            let child = self.create_child(coord);
            child.set_voxel(coord, value)
        } else {
            // Do nothing if the background value is the same
            None
        }
    }

    /// Removes a voxel at the given coordinate.
    /// 
    /// This method removes voxel data at the specified coordinate by delegating
    /// to the appropriate child node. If no child node exists for the coordinate,
    /// returns `None`.
    /// 
    /// # Arguments
    /// 
    /// * `coord` - The 3D coordinate to remove the voxel from
    /// 
    /// # Returns
    /// 
    /// Returns `Some(T)` containing the removed value if one existed,
    /// or `None` if no voxel existed at that coordinate.
    fn remove_voxel(&mut self, coord: Vec3i) -> Option<T> {
        if let Some(child) = self.find_child_mut(coord) {
            child.remove_voxel(coord)
        } else {
            None
        }
    }

    /// Returns an iterator over all active voxels in this node and its children.
    /// 
    /// Active voxels are those containing non-background data. The iterator
    /// yields tuples of `(Vec3i, &T)` representing the coordinate and value
    /// of each active voxel.
    /// 
    /// # Returns
    /// 
    /// A boxed iterator over all active voxels in the hierarchy.
    /// 
    /// # Performance
    /// 
    /// This method creates an iterator that traverses all child nodes,
    /// which may be expensive for large hierarchies.
    fn active_voxels(&self) -> Box<dyn Iterator<Item = (Vec3i, &T)> + '_> {
        Box::new(
            self.children.values()
                .flat_map(|child| child.active_voxels())
        )
    }

    /// Returns an iterator over all voxels (active and inactive) in this node and its children.
    /// 
    /// This includes both active voxels (containing data) and inactive voxels
    /// (containing background values). The iterator yields tuples of `(Vec3i, &T)`
    /// representing the coordinate and value of each voxel.
    /// 
    /// # Returns
    /// 
    /// A boxed iterator over all voxels in the hierarchy.
    /// 
    /// # Performance
    /// 
    /// This method creates an iterator that traverses all child nodes,
    /// which may be expensive for large hierarchies.
    fn all_voxels(&self) -> Box<dyn Iterator<Item = (Vec3i, &T)> + '_> {
        Box::new(
            self.children.values()
                .flat_map(|child| child.all_voxels())
        )
    }
}

impl<T: VoxelData, N: ChildNodeTrait<T>> NodeDiagnostics<T> for RootNode<T, N> {
    /// Returns the log2 size of child nodes.
    /// 
    /// This represents the size of child nodes in the hierarchy below this node.
    /// For root nodes, this delegates to the child node type's `log2()` method.
    /// 
    /// # Returns
    /// 
    /// The log2 size of child nodes (e.g., 6 means 2^6 = 64 voxels per side).
    fn log2_child_size(&self) -> u32 {
        N::log2()
    }

    /// Returns the type of this node.
    /// 
    /// For root nodes, this always returns `NodeType::Root`.
    /// 
    /// # Returns
    /// 
    /// Always returns `NodeType::Root` for root nodes.
    fn node_type(&self) -> NodeType {
        NodeType::Root
    }

    /// Returns the depth of this node in the hierarchy.
    /// 
    /// For root nodes, this is always 0, representing the top level.
    /// 
    /// # Returns
    /// 
    /// The depth of this node (always 0 for root nodes).
    fn depth(&self) -> u32 {
        self.level
    }

    /// Returns the number of direct child nodes.
    /// 
    /// This counts only the immediate children of this root node,
    /// not the total number of nodes in the entire hierarchy.
    /// 
    /// # Returns
    /// 
    /// The number of direct child nodes stored in this root node.
    fn child_count(&self) -> usize {
        self.children.len()
    }
}

/// Test module for RootNode functionality.
/// 
/// This module contains comprehensive tests for the RootNode implementation,
/// covering:
/// - Basic initialization and default behavior
/// - Voxel operations (get, set, remove)
/// - Child node management and creation
/// - Spatial partitioning and coordinate calculations
/// - Active/inactive voxel counting
/// - Bounds calculation
/// 
/// The tests use f32 as the voxel data type and LeafNode<f32, 6> as the child node type,
/// providing a 6-level hierarchy (2^6 = 64 voxels per side per child node).
#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_root_node() {
        let root = RootNode::<f32, LeafNode<f32, 6>>::default();
        assert_eq!(root.level(), 0);
        assert_eq!(root.children.len(), 0);
        assert_eq!(root.bounds(), Bounds3i::empty());
        assert_eq!(root.is_active(Vec3i::new(0, 0, 0)), false);
        assert_eq!(root.active_count(), 0);
        assert_eq!(root.total_count(), 0);
    }

    #[test]
    fn test_root_node_with_children() {
        let mut root = RootNode::<f32, LeafNode<f32, 6>>::default();
        root.set_voxel(Vec3i::new(1, 2, 3), 2.0);
        assert_eq!(root.children.len(), 1);
        // assert_eq!(root.bounds(), Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(64, 64, 64)));
        assert_eq!(root.is_active(Vec3i::new(1, 2, 3)), true);
        assert_eq!(root.get_voxel(Vec3i::new(1, 2, 3)), &2.0);
        assert_eq!(root.active_count(), 1);
        assert_eq!(root.total_count(), 1);
    }

    #[test]
    fn test_root_node_with_children_and_remove() {
        let mut root = RootNode::<f32, LeafNode<f32, 6>>::default();
        root.set_voxel(Vec3i::new(1, 2, 3), 2.0);
        assert_eq!(root.children.len(), 1);
        // assert_eq!(root.bounds(), Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(64, 64, 64)));
        assert_eq!(root.is_active(Vec3i::new(1, 2, 3)), true);
        assert_eq!(root.get_voxel(Vec3i::new(1, 2, 3)), &2.0);
        assert_eq!(root.active_count(), 1);
        assert_eq!(root.total_count(), 1);
    
        let removed = root.remove_voxel(Vec3i::new(1, 2, 3));
        assert_eq!(removed, Some(2.0));
        assert_eq!(root.children.len(), 1);
        //assert_eq!(root.bounds(), Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(64, 64, 64)));
        assert_eq!(root.is_active(Vec3i::new(1, 2, 3)), false);
        assert_eq!(root.get_voxel(Vec3i::new(1, 2, 3)), &0.0);
        assert_eq!(root.active_count(), 0);
        assert_eq!(root.total_count(), 0);
    }

    #[test]
    fn test_child_key_calculation() {
      let root = RootNode::<f32, LeafNode<f32, 5>>::default();

      assert_eq!(root.log2_child_size(), 5);
      assert_eq!(root.calculate_child_key(Vec3i::new(0, 0, 0)), Vec3i::new(0, 0, 0));
      assert_eq!(root.calculate_child_key(Vec3i::new(33, 2, 3)), Vec3i::new(32, 0, 0));
      assert_eq!(root.calculate_child_key(Vec3i::new(70, 38, 3)), Vec3i::new(64, 32, 0));
      assert_eq!(root.calculate_child_key(Vec3i::new(31, -31, -65)), Vec3i::new(0, -32, -96));
    }
}
