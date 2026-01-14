use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// クリップボードリソース
#[derive(Resource, Default)]
#[allow(dead_code)]
pub struct Clipboard {
    pub text_content: String, // テキストコンテンツ（コードエディタ用）
    pub entity_data: Option<EntityClipboardData>, // Entityデータ（Entityコピー/ペースト用）
}

/// Entityクリップボードデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityClipboardData {
    pub entities: Vec<SerializedEntity>, // シリアライズされたEntityのリスト
}

/// シリアライズされたEntity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedEntity {
    pub name: String,
    pub transform: SerializedTransform,
    pub components: Vec<ComponentData>, // その他のコンポーネントデータ
}

/// シリアライズされたTransform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedTransform {
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

/// コンポーネントデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentData {
    pub component_type: String,
    pub data: serde_json::Value,
}

#[allow(dead_code)]
impl Clipboard {
    pub fn new() -> Self {
        Self {
            text_content: String::new(),
            entity_data: None,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text_content = text;
        self.entity_data = None; // テキストを設定したらEntityデータをクリア
    }

    pub fn set_entity_data(&mut self, data: EntityClipboardData) {
        self.entity_data = Some(data);
    }

    pub fn get_text(&self) -> &str {
        &self.text_content
    }

    pub fn get_entity_data(&self) -> Option<&EntityClipboardData> {
        self.entity_data.as_ref()
    }

    pub fn has_text(&self) -> bool {
        !self.text_content.is_empty()
    }

    pub fn has_entity_data(&self) -> bool {
        self.entity_data.is_some()
    }
}
