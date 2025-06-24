pub mod quadrilateral;
pub use quadrilateral::{new_box, Quadrilateral};

pub mod sphere;
pub use sphere::Sphere;

use crate::{MaterialRef, Point3, RotateY, Translate, Vec3};
use std::sync::Arc;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ObjectConfig {
    Sphere {
        center: Point3,
        radius: f32,
        material: MaterialRef,
        #[serde(default)]
        transform: TransformConfig,
    },
    Quad {
        corner: Point3,
        u: Point3,
        v: Point3,
        material: MaterialRef,
        #[serde(default)]
        transform: TransformConfig,
    },
    Box {
        min: Point3,
        max: Point3,
        material: MaterialRef,
        #[serde(default)]
        transform: TransformConfig,
    },
}

impl ObjectConfig {
    pub fn to_hittable(
        &self,
        materials: &std::collections::HashMap<String, crate::MaterialConfig>,
    ) -> std::result::Result<std::sync::Arc<dyn crate::Hittable>, String> {
        let base_object: std::sync::Arc<dyn crate::Hittable> = match self {
            &ObjectConfig::Sphere {
                center,
                radius,
                ref material,
                ..
            } => {
                // let material = crate::MaterialRef::Reference(material.clone());
                let material = material.resolve(materials)?;
                std::sync::Arc::new(Sphere::stationary(center, radius, material.into()))
            }
            &ObjectConfig::Quad {
                corner,
                u,
                v,
                ref material,
                ..
            } => {
                let material = material.resolve(materials)?;
                std::sync::Arc::new(Quadrilateral::new(corner, u, v, material.into()))
            }
            ObjectConfig::Box {
                min,
                max,
                ref material,
                transform: _,
            } => {
                let resolved_material = material.resolve(materials)?;
                Arc::new(new_box(*min, *max, resolved_material.into()))
            }
        };

        let transformed_object = match self {
            ObjectConfig::Sphere { transform, .. }
            | ObjectConfig::Quad { transform, .. }
            | ObjectConfig::Box { transform, .. } => transform.apply_to_hittable(base_object),
        };

        Ok(transformed_object)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct TransformConfig {
    #[serde(default)]
    pub translate: Option<Vec3>,
    #[serde(default)]
    pub rotate_y: Option<f32>, // degrees
    #[serde(default)]
    pub rotate_x: Option<f32>, // degrees
    #[serde(default)]
    pub rotate_z: Option<f32>, // degrees
    #[serde(default)]
    pub scale: Option<Vec3>,
}

impl Default for TransformConfig {
    fn default() -> Self {
        Self {
            translate: None,
            rotate_y: None,
            rotate_x: None,
            rotate_z: None,
            scale: None,
        }
    }
}

impl TransformConfig {
    pub fn apply_to_hittable(
        &self,
        hittable: Arc<dyn crate::Hittable>,
    ) -> Arc<dyn crate::Hittable> {
        let mut result = hittable;

        // Apply rotations first (in order: X, Y, Z)
        //if let Some(angle_x) = self.rotate_x {
        //    result = Arc::new(RotateX::new(result, angle_x));
        //}
        if let Some(angle_y) = self.rotate_y {
            result = Arc::new(RotateY::new(result, angle_y));
        }
        //if let Some(angle_z) = self.rotate_z {
        //    result = Arc::new(RotateZ::new(result, angle_z));
        //}

        // Apply scale (if you have it implemented)
        // if let Some(scale_vec) = self.scale {
        //     result = Arc::new(Scale::new(result, scale_vec));
        // }

        // Apply translation last
        if let Some(translation) = self.translate {
            result = Arc::new(Translate::new(result, translation));
        }

        result
    }
}
