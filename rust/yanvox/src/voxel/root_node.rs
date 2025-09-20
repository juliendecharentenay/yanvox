//! Root node implementation with internal hierarchy management
use super::*;
use math::{Vec3i, Bounds3i};
use voxel::{VoxelData, NodeTrait};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootNode<T: VoxelData, N: ChildNodeTrait<T>> {
    pub level: u32,
    pub background_value: T,
    children: HashMap<Vec3i, N>,
}

impl<T: VoxelData, N: ChildNodeTrait<T>> Default for RootNode<T, N> {
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
    /// Internal method to find or create child node for a coordinate
    fn get_or_create_child(&mut self, coord: Vec3i) -> &mut N {
        let child_key = <N as ChildNodeTrait::<T>>::key(coord);
        
        if !self.children.contains_key(&child_key) {
            // Create new child node
            let child = N::create(coord, self.level+1);
            self.children.insert(child_key, child);
        }
        
        self.children.get_mut(&child_key).unwrap()
    }

    /// Calculate the key (lower left corner) identifying
    ///  which child should contain the given coordinate
    fn calculate_child_key(&self, coord: Vec3i) -> Vec3i {
        <N as ChildNodeTrait::<T>>::key(coord)
    }

    /// Internal method to find child node for a coordinate
    fn find_child(&self, coord: Vec3i) -> Option<&N> {
        let child_key = <N as ChildNodeTrait::<T>>::key(coord);
        self.children.get(&child_key)
    }

    /// Internal method to find child node for a coordinate (mutable)
    fn find_child_mut(&mut self, coord: Vec3i) -> Option<&mut N> {
        let child_key = <N as ChildNodeTrait::<T>>::key(coord);
        self.children.get_mut(&child_key)
    }
}

impl<T: VoxelData, N: ChildNodeTrait<T>> NodeTrait<T> for RootNode<T, N> {
    fn level(&self) -> u32 {
        self.level
    }

    fn log2_cum(&self) -> u32 {
      <N as ChildNodeTrait::<T>>::log2_cum()
    }

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

    fn is_active(&self, coord: Vec3i) -> bool {
        if let Some(child) = self.find_child(coord) {
            child.is_active(coord)
        } else {
            false
        }
    }

    fn active_count(&self) -> usize {
        self.children.values()
            .map(|child| child.active_count())
            .sum()
    }

    fn total_count(&self) -> usize {
        self.children.values()
            .map(|child| child.total_count())
            .sum()
    }

    fn get_voxel(&self, coord: Vec3i) -> Option<&T> {
        if let Some(child) = self.find_child(coord) {
            child.get_voxel(coord)
        } else {
            None
        }
    }

    fn set_voxel(&mut self, coord: Vec3i, value: T) -> Option<T> {
        // Root nodes delegate to children
        let child = self.get_or_create_child(coord);
        child.set_voxel(coord, value)
    }

    fn remove_voxel(&mut self, coord: Vec3i) -> Option<T> {
        if let Some(child) = self.find_child_mut(coord) {
            child.remove_voxel(coord)
        } else {
            None
        }
    }

    // Iterator operations
    fn active_voxels(&self) -> Box<dyn Iterator<Item = (Vec3i, &T)> + '_> {
        Box::new(
            self.children.values()
                .flat_map(|child| child.active_voxels())
        )
    }

    fn all_voxels(&self) -> Box<dyn Iterator<Item = (Vec3i, &T)> + '_> {
        Box::new(
            self.children.values()
                .flat_map(|child| child.all_voxels())
        )
    }

    // Background value operations
    fn background_value(&self) -> &T {
        &self.background_value
    }
}

impl<T: VoxelData, N: ChildNodeTrait<T>> NodeDiagnostics<T> for RootNode<T, N> {
    fn log2_child_size(&self) -> u32 {
        N::log2()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Root
    }

    fn depth(&self) -> u32 {
        self.level
    }

    fn child_count(&self) -> usize {
        self.children.len()
    }
}

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
        assert_eq!(root.get_voxel(Vec3i::new(1, 2, 3)), Some(&2.0));
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
        assert_eq!(root.get_voxel(Vec3i::new(1, 2, 3)), Some(&2.0));
        assert_eq!(root.active_count(), 1);
        assert_eq!(root.total_count(), 1);
    
        let removed = root.remove_voxel(Vec3i::new(1, 2, 3));
        assert_eq!(removed, Some(2.0));
        assert_eq!(root.children.len(), 1);
        //assert_eq!(root.bounds(), Bounds3i::new(Vec3i::new(0, 0, 0), Vec3i::new(64, 64, 64)));
        assert_eq!(root.is_active(Vec3i::new(1, 2, 3)), false);
        assert_eq!(root.get_voxel(Vec3i::new(1, 2, 3)), None);
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
