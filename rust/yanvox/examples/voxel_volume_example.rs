use log::info;
use yanvox::voxel::{VoxelData, VoxelVolume, VolumeConfig, CompressionType};
use yanvox::math::Vec3f;

/// A voxel that stores a signed distance value
/// Only active when within EPSILON distance of the surface
#[derive(Debug, Clone, PartialEq)]
struct SignedDistanceVoxel {
    value: f32,
}

impl SignedDistanceVoxel {
    /// Epsilon distance for determining if a voxel is active
    const EPSILON: f32 = 0.5;
    
    /// Create a signed distance voxel from a sphere SDF
    fn from_sphere(point: Vec3f, radius: f32) -> Self {
        let sdf_value = point.length() - radius;
        SignedDistanceVoxel { value: sdf_value }
    }
}

impl VoxelData for SignedDistanceVoxel {
    /// A voxel is active if it's within EPSILON distance of the surface
    fn is_active(&self) -> bool {
        self.value.abs() < Self::EPSILON
    }

    /// Background value is +EPSILON (external value)
    fn background() -> Self {
        SignedDistanceVoxel { value: Self::EPSILON }
    }
}

fn main() {
    // Initialize logging
    env_logger::init();
    
    info!("Hello, VoxelVolume example!");
    
    // Create a volume configuration
    let config = VolumeConfig {
        compression: CompressionType::None,
        size: 0.1, // 0.1 unit cube
    };
    
    // Create a VoxelVolume using SignedDistanceVoxel
    let mut volume: VoxelVolume<SignedDistanceVoxel> = VoxelVolume::with_config(config);
    
    info!("Created VoxelVolume with SignedDistanceVoxel (epsilon = {})", SignedDistanceVoxel::EPSILON);
    
    // Generate sphere SDF in the volume
    // Domain: -2.0 to +2.0 in all dimensions
    // Sphere: radius 1.0, centered at origin
    let sphere_radius = 1.0;
    let domain_min = -2.0;
    let domain_max = 2.0;
    let voxel_size = 0.1;
    
    info!("Generating sphere SDF:");
    info!("  Sphere radius: {}", sphere_radius);
    info!("  Domain: [{}, {}, {}] to [{}, {}, {}]", domain_min, domain_min, domain_min, domain_max, domain_max, domain_max);
    info!("  Voxel size: {}", voxel_size);
    
    let voxels_set = volume.fill_bounds(
        Vec3f::new(domain_min, domain_min, domain_min), 
        Vec3f::new(domain_max, domain_max, domain_max), 
        |point| Some(SignedDistanceVoxel::from_sphere(point, sphere_radius))
    );
    info!("Sphere generation complete");
    info!("Voxels set: {}", voxels_set);
    info!("Volume summary:\n{}", volume.summary());
    
    // TODO: Add more VoxelVolume usage examples here
}