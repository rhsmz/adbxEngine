use bevy::prelude::*;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

/// カスタムアセットタイプの登録システム
#[derive(Resource, Default)]
pub struct CustomAssetRegistry {
    pub asset_loaders: HashMap<String, Arc<dyn CustomAssetLoader + Send + Sync>>,
    pub asset_type_map: HashMap<String, TypeId>, // 拡張子 -> アセット型のマッピング
}

/// カスタムアセットローダーのトレイト
pub trait CustomAssetLoader: Send + Sync {
    /// アセットを読み込む
    fn load_asset(&self, path: &std::path::Path) -> Result<Box<dyn std::any::Any>, String>;

    /// アセットタイプ名を返す
    fn asset_type_name(&self) -> &str;

    /// サポートする拡張子のリストを返す
    fn supported_extensions(&self) -> Vec<String>;
}

impl CustomAssetRegistry {
    pub fn new() -> Self {
        Self {
            asset_loaders: HashMap::new(),
            asset_type_map: HashMap::new(),
        }
    }

    /// カスタムアセットローダーを登録
    pub fn register_loader<L: CustomAssetLoader + 'static>(&mut self, loader: L) {
        let type_name = loader.asset_type_name().to_string();
        let extensions = loader.supported_extensions();

        // 拡張子マッピングを登録
        for ext in &extensions {
            self.asset_type_map.insert(ext.clone(), TypeId::of::<L>());
        }

        // ローダーを登録
        let type_name_clone = type_name.clone();
        self.asset_loaders.insert(type_name, Arc::new(loader));

        bevy::log::info!(
            "Registered custom asset loader: {} (extensions: {:?})",
            type_name_clone,
            extensions
        );
    }

    /// 拡張子からアセットローダーを取得
    pub fn get_loader_by_extension(
        &self,
        extension: &str,
    ) -> Option<Arc<dyn CustomAssetLoader + Send + Sync>> {
        // 拡張子からTypeIdを取得
        if let Some(_type_id) = self.asset_type_map.get(extension) {
            // 拡張子に対応するローダーを検索
            // 各ローダーのサポート拡張子を確認して、一致するものを返す
            for (_name, loader) in &self.asset_loaders {
                let supported_extensions = loader.supported_extensions();
                if supported_extensions.iter().any(|ext| ext == extension) {
                    return Some(loader.clone());
                }
            }
        }
        None
    }

    /// アセットタイプ名からローダーを取得
    pub fn get_loader(&self, type_name: &str) -> Option<Arc<dyn CustomAssetLoader + Send + Sync>> {
        self.asset_loaders.get(type_name).cloned()
    }

    /// 登録されているすべてのローダー名を取得
    pub fn get_registered_loaders(&self) -> Vec<String> {
        self.asset_loaders.keys().cloned().collect()
    }

    /// 指定された拡張子がサポートされているか確認
    pub fn is_extension_supported(&self, extension: &str) -> bool {
        self.get_loader_by_extension(extension).is_some()
    }
}

/// カスタムアセットのホットリロード処理
pub fn handle_custom_asset_hot_reload(
    registry: &CustomAssetRegistry,
    asset_path: &str,
) -> Result<(), String> {
    let path = std::path::Path::new(asset_path);

    if !path.exists() {
        return Err(format!("Asset file does not exist: {}", asset_path));
    }

    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
        if let Some(loader) = registry.get_loader_by_extension(extension) {
            match loader.load_asset(path) {
                Ok(_asset) => {
                    bevy::log::info!(
                        "Custom asset hot reloaded: {} (type: {})",
                        asset_path,
                        loader.asset_type_name()
                    );
                    Ok(())
                }
                Err(e) => {
                    bevy::log::error!("Failed to hot reload custom asset {}: {}", asset_path, e);
                    Err(e)
                }
            }
        } else {
            // カスタムアセットローダーが見つからない場合は、標準アセットとして処理される可能性があるため、エラーにしない
            bevy::log::debug!(
                "No custom loader found for extension: {} (file: {})",
                extension,
                asset_path
            );
            Ok(())
        }
    } else {
        Err(format!("Invalid asset path (no extension): {}", asset_path))
    }
}

/// カスタム設定ファイルローダーの実装例
/// この例では、`.config`拡張子のファイルを読み込むローダーを実装します
pub struct ConfigFileLoader;

impl CustomAssetLoader for ConfigFileLoader {
    fn load_asset(&self, path: &std::path::Path) -> Result<Box<dyn std::any::Any>, String> {
        // ファイルを読み込む
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        // 簡易的な設定ファイルパーサー（実際の実装では、より高度なパーサーを使用）
        let config: std::collections::HashMap<String, String> = content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    return None;
                }
                if let Some(pos) = line.find('=') {
                    let key = line[..pos].trim().to_string();
                    let value = line[pos + 1..].trim().to_string();
                    Some((key, value))
                } else {
                    None
                }
            })
            .collect();

        Ok(Box::new(config))
    }

    fn asset_type_name(&self) -> &str {
        "ConfigFile"
    }

    fn supported_extensions(&self) -> Vec<String> {
        vec!["config".to_string(), "cfg".to_string()]
    }
}

/// デフォルトのカスタムアセットローダーを登録するシステム
pub fn register_default_custom_asset_loaders(mut registry: ResMut<CustomAssetRegistry>) {
    // 設定ファイルローダーを登録
    registry.register_loader(ConfigFileLoader);

    bevy::log::info!("Default custom asset loaders registered");
}
