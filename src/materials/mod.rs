pub(crate) mod dialectric;
pub(crate) mod lambertian;
pub(crate) mod material;
pub(crate) mod metal;

pub use dialectric::Dielectric;
pub use lambertian::Lambertian;
pub use material::{Material, NoneMaterial};
pub use metal::Metal;
