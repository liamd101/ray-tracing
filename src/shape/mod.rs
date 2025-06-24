pub mod quadrilateral;
pub use quadrilateral::{new_box, Quadrilateral};

pub mod sphere;
pub use sphere::Sphere;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ObjectConfig {
    Sphere {
        center: crate::Point3,
        radius: f32,
        material: crate::MaterialRef,
    },
    Quad {
        corner: crate::Point3,
        u: crate::Point3,
        v: crate::Point3,
        material: crate::MaterialRef,
    },
}

impl ObjectConfig {
    pub fn to_hittable(
        &self,
        materials: &std::collections::HashMap<String, crate::MaterialConfig>,
    ) -> std::result::Result<std::sync::Arc<dyn crate::Hittable>, String> {
        match self {
            &ObjectConfig::Sphere {
                center,
                radius,
                ref material,
            } => {
                // let material = crate::MaterialRef::Reference(material.clone());
                let material = material.resolve(materials)?;
                Ok(std::sync::Arc::new(Sphere::stationary(
                    center,
                    radius,
                    material.into(),
                )))
            }
            &ObjectConfig::Quad {
                corner,
                u,
                v,
                ref material,
            } => {
                let material = material.resolve(materials)?;
                Ok(std::sync::Arc::new(Quadrilateral::new(
                    corner,
                    u,
                    v,
                    material.into(),
                )))
            }
        }
    }
}
