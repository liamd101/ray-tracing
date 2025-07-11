pub(crate) mod dialectric;
pub(crate) mod isotropic;
pub(crate) mod lambertian;
pub(crate) mod lights;
pub(crate) mod material;
pub(crate) mod metal;
pub(crate) mod glossy;

pub use dialectric::Dielectric;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use lights::DiffuseLight;
pub use material::{Material, NoneMaterial, ScatterRecord};
pub use metal::Metal;
pub use glossy::Glossy;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MaterialConfig {
    None,
    Lambertian { color: crate::Color },
    Metal { color: crate::Color, fuzz: f32 },
    Dieletric { refraction_index: f32 },
    DiffuseLight { color: crate::Color },
}

use std::sync::Arc;
impl From<MaterialConfig> for Arc<dyn Material> {
    fn from(value: MaterialConfig) -> Self {
        match value {
            MaterialConfig::None => Arc::new(NoneMaterial {}),
            MaterialConfig::Lambertian { color } => Arc::new(Lambertian::new(color)),
            MaterialConfig::Metal { color, fuzz } => Arc::new(Metal::new(color, fuzz)),
            MaterialConfig::Dieletric { refraction_index } => {
                Arc::new(Dielectric::new(refraction_index))
            }
            MaterialConfig::DiffuseLight { color } => Arc::new(DiffuseLight::from_color(color)),
        }
    }
}
