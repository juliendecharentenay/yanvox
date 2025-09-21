# yanvox

A VDB-inspired voxel engine library for high-resolution sparse volumes, written in Rust.

## Overview

yanvox is a sparse voxel data structure library designed for efficient storage and manipulation of 3D volumetric data. It provides a hierarchical tree structure optimized for sparse data.

## Features

- **Sparse Storage**: Only stores active voxels, saving memory for sparse data
- **Hierarchical Structure**: Multi-level tree organization for efficient access
- **Type-Safe**: Generic design supporting any voxel data type
- **Memory Efficient**: Automatic memory management with configurable compression
- **Coordinate System**: Flexible world-space to voxel-space coordinate conversion
- **Easy-to-Use API**: High-level convenience methods for common operations

## Examples

### Running the Example

```bash
cd rust
RUST_LOG=info cargo run --example voxel_volume_example
```

This example demonstrates:
- Creating a signed distance field (SDF) for a sphere
- Using the `fill_bounds` convenience method
- Automatic coordinate alignment
- Volume statistics and memory usage

### Signed Distance Fields

The library is particularly well-suited for SDFs:

```rust
struct SignedDistanceVoxel {
    value: f32,
}

impl VoxelData for SignedDistanceVoxel {
    fn is_active(&self) -> bool {
        self.value.abs() < 0.5 // Only store near-surface voxels
    }
    
    fn background() -> Self {
        SignedDistanceVoxel { value: 1.0 }
    }
}

// Generate sphere SDF
volume.fill_bounds(
    Vec3f::new(-2.0, -2.0, -2.0),
    Vec3f::new(2.0, 2.0, 2.0),
    |point| {
        let sdf_value = point.length() - 1.0; // Sphere radius 1.0
        Some(SignedDistanceVoxel { value: sdf_value })
    }
);
```

## API Reference

### Core Types

- `VoxelVolume<T>`: The main sparse voxel container
- `VoxelData`: Trait for voxel data types
- `VolumeConfig`: Configuration for voxel volumes
- `Vec3f`/`Vec3i`: 3D vector types
- `Bounds3f`/`Bounds3i`: 3D bounding box types

### Key Methods

#### Volume Creation
- `VoxelVolume::with_config(config)` - Create with configuration
- `VolumeConfig::new()` - Create default configuration

#### Voxel Operations
- `set_voxel_f(coord, value)` - Set voxel at world coordinate
- `get_voxel_f(coord)` - Get voxel at world coordinate
- `is_active_f(coord)` - Check if voxel is active

#### Batch Operations
- `fill_bounds(min, max, generator)` - Fill rectangular region
- `fill_region_bounds(bounds, generator)` - Fill using Bounds3f

#### Utility Methods
- `get_voxel_size()` - Get root voxel size
- `get_leaf_voxel_size()` - Get actual leaf voxel size
- `summary()` - Get volume statistics

## Configuration

### VolumeConfig

```rust
let config = VolumeConfig {
    compression: CompressionType::None,
    size: 0.1, // Root voxel size in world units
};
```

## Performance Considerations

- **Sparse Data**: Best performance with sparse data (many empty regions)
- **Voxel Size**: Smaller voxels provide higher resolution but use more memory
- **Batch Operations**: Use `fill_bounds` for better performance than individual voxel operations

## Memory Usage

The library provides memory usage estimates:

```rust
let summary = volume.summary();
println!("Memory usage: {} bytes ({:.2} MB)", 
         summary.memory_estimate, 
         summary.memory_estimate as f64 / 1_048_576.0);
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Roadmap

- [ ] Meshing using marching cubes algorithm
- [ ] Additional examples and tutorials
- [ ] Performance optimizations

## Acknowledgments

Inspired by OpenVDB and other sparse voxel data structures.
