use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// シーン定義
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneData {
    pub name: String,
    pub entities: Vec<EntityData>,
}

/// Entityデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityData {
    pub id: u32,
    pub name: String,
    pub parent: Option<u32>,
    pub components: Vec<ComponentData>,
}

/// Componentデータ（型情報とシリアライズされた値）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentData {
    pub type_name: String,
    pub data: serde_json::Value,
}

/// シーンメタデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMetadata {
    pub name: String,
    pub path: String,
    pub last_modified: u64,
}
