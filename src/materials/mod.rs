pub(crate) mod dialectric;
pub(crate) mod lambertian;
pub(crate) mod lights;
pub(crate) mod material;
pub(crate) mod metal;
pub(crate) mod isotropic;

pub use dialectric::Dielectric;
pub use lambertian::Lambertian;
pub use lights::DiffuseLight;
pub use material::{Material, NoneMaterial};
pub use metal::Metal;
pub use isotropic::Isotropic;
