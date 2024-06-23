pub(crate) mod dialectric;
pub(crate) mod lambertian;
pub(crate) mod material;
pub(crate) mod metal;
pub(crate) mod texture;
pub(crate) mod checkerboard;

pub use dialectric::Dielectric;
pub use lambertian::Lambertian;
pub use material::{Material, NoneMaterial};
pub use metal::Metal;
pub use texture::{SolidColor, Texture};
pub use checkerboard::Checkerboard;
