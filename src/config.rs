use crate::CameraConfig;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MaterialRef {
    Inline(crate::MaterialConfig),

    Reference(String),
}

impl MaterialRef {
    pub fn resolve(
        &self,
        materials: &HashMap<String, crate::MaterialConfig>,
    ) -> Result<crate::MaterialConfig, String> {
        match self {
            MaterialRef::Inline(material) => Ok(material.clone()),
            MaterialRef::Reference(name) => materials
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Material '{}' not found in materials section", name)),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub camera: CameraConfig,
    pub scene: SceneConfig,
}

impl Config {
    pub fn to_scene(
        &self,
    ) -> Result<(crate::Camera, crate::HittableList, crate::HittableList), String> {
        let camera = crate::Camera::from(self.camera.clone());
        let (world, lights) = self.scene.process()?;
        Ok((camera, world, lights))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SceneConfig {
    pub objects: Vec<crate::ObjectConfig>,
    pub lights: Vec<crate::ObjectConfig>,
    #[serde(default)]
    pub materials: std::collections::HashMap<String, crate::MaterialConfig>,
}
impl SceneConfig {
    pub fn process(&self) -> Result<(crate::HittableList, crate::HittableList), String> {
        let mut world = crate::HittableList::new();
        let mut lights = crate::HittableList::new();

        for object in &self.objects {
            let object = object.to_hittable(&self.materials)?;
            world.add(object);
        }

        for light_config in &self.lights {
            let empty_light = light_to_empty(light_config.clone());
            println!("empty_light={:?}", empty_light);
            let empty_light = empty_light.to_hittable(&self.materials)?;
            world.add(empty_light);

            println!("light_config={:?}", light_config);
            let light = light_config.to_hittable(&self.materials)?;
            lights.add(light);
        }

        Ok((world, lights))
    }
}

fn light_to_empty(config: crate::ObjectConfig) -> crate::ObjectConfig {
    match config {
        crate::ObjectConfig::Quad {
            corner,
            u,
            v,
            transform,
            ..
        } => crate::ObjectConfig::Quad {
            corner,
            u,
            v,
            transform,
            material: MaterialRef::Inline(crate::MaterialConfig::None),
        },
        crate::ObjectConfig::Sphere {
            center,
            radius,
            transform,
            ..
        } => crate::ObjectConfig::Sphere {
            center,
            radius,
            transform,
            material: MaterialRef::Inline(crate::MaterialConfig::None),
        },
        crate::ObjectConfig::Box {
            min,
            max,
            transform,
            ..
        } => crate::ObjectConfig::Box {
            min,
            max,
            transform,
            material: MaterialRef::Inline(crate::MaterialConfig::None),
        },
    }
}
