use super::{AssetBrowser, AssetFileInfo, AssetType};
use std::path::Path;

/// AssetBrowserの拡張トレイト（ディレクトリスキャン機能）
pub trait AssetBrowserExt {
    fn scan_directory(&mut self);
    fn detect_asset_type(path: &Path) -> AssetType;
}

impl AssetBrowserExt for AssetBrowser {
    fn scan_directory(&mut self) {
        self.asset_files.clear();

        if !self.current_path.exists() {
            return;
        }

        if let Ok(entries) = std::fs::read_dir(&self.current_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();

                let is_directory = path.is_dir();
                let asset_type = if is_directory {
                    AssetType::Other
                } else {
                    Self::detect_asset_type(&path)
                };

                self.asset_files.push(AssetFileInfo {
                    path,
                    name,
                    asset_type,
                    is_directory,
                });
            }
        }

        // 名前でソート
        self.asset_files.sort_by(|a, b| {
            // ディレクトリを先に
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });
    }

    fn detect_asset_type(path: &Path) -> AssetType {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "glb" | "gltf" | "obj" | "fbx" => AssetType::Mesh,
                "png" | "jpg" | "jpeg" | "bmp" | "tga" | "dds" => AssetType::Texture,
                "mat" | "material" => AssetType::Material,
                "lua" => AssetType::Script,
                "scene" | "scn" => AssetType::Scene,
                "txt" | "md" | "json" | "yaml" | "yml" | "xml" | "csv" | "log" => AssetType::Text,
                _ => AssetType::Other,
            }
        } else {
            AssetType::Other
        }
    }
}
