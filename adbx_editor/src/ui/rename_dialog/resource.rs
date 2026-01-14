use bevy::prelude::*;
use std::path::PathBuf;

/// リネームダイアログのリクエスト
#[derive(Resource, Default)]
pub struct RenameDialogRequest {
    pub is_visible: bool,
    pub target_path: Option<PathBuf>,
    pub current_name: String,
    pub new_name: String,
    pub result: Option<RenameDialogResult>,
}

/// リネームダイアログの結果
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum RenameDialogResult {
    Renamed {
        old_path: PathBuf,
        new_path: PathBuf,
    },
    Cancelled,
}
