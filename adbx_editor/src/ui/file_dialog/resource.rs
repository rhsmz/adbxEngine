use bevy::prelude::*;
use std::path::PathBuf;

/// ファイルダイアログのリクエスト
#[derive(Resource, Default)]
pub struct FileDialogRequest {
    pub dialog_type: Option<FileDialogType>,
    pub result: Option<FileDialogResult>,
}

/// ファイルダイアログの種類
#[derive(Debug, Clone)]
pub enum FileDialogType {
    OpenFile {
        title: String,
        filters: Vec<FileFilter>,
    },
    SaveFile {
        title: String,
        default_name: Option<String>,
        filters: Vec<FileFilter>,
    },
    PickFolder {
        title: String,
    },
}

/// ファイルフィルター
#[derive(Debug, Clone)]
pub struct FileFilter {
    pub name: String,
    pub extensions: Vec<String>,
}

/// ファイルダイアログの結果
#[derive(Debug, Clone)]
pub enum FileDialogResult {
    File(PathBuf),
    Folder(PathBuf),
    Cancelled,
}
