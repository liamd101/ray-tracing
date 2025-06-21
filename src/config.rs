use serde::{Deserialize, Serialize};
use crate::CameraConfig;

#[derive(Deserialize, Serialize, Debug)]
pub struct SceneConfig {
    pub camera: CameraConfig,
}
