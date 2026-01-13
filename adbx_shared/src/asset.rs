use serde::{Deserialize, Serialize};

/// アセットタイプ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    Image,
    Audio,
    Mesh,
    Material,
    Script,
    Scene,
    Unknown,
}

/// アセットメタデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub id: String,
    pub path: String,
    pub asset_type: AssetType,
    pub name: String,
    pub size: u64,
    pub last_modified: u64,
}

/// アセット参照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReference {
    pub asset_id: String,
    pub path: String,
}
