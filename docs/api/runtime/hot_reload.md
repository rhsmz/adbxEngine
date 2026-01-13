# ホットリロード API リファレンス

`adbx_runtime`クレートのホットリロード機能に関するAPIリファレンスです。

## 概要

ホットリロード機能は、アセットやスクリプトの変更を検知し、実行中のアプリケーションに自動的に反映する機能を提供します。

## CustomAssetRegistry

カスタムアセットタイプの登録システムを管理するリソースです。

### フィールド

- **`asset_loaders: HashMap<String, Arc<dyn CustomAssetLoader + Send + Sync>>`**: 登録されたアセットローダーのマップ
- **`asset_type_map: HashMap<String, TypeId>`**: 拡張子からアセット型へのマッピング

### メソッド

#### `new() -> Self`

新しい`CustomAssetRegistry`インスタンスを作成します。

```rust
use adbx_runtime::hot_reload::custom_asset_registry::CustomAssetRegistry;

let registry = CustomAssetRegistry::new();
```

#### `register_loader<L: CustomAssetLoader + 'static>(&mut self, loader: L)`

カスタムアセットローダーを登録します。

```rust
registry.register_loader(ConfigFileLoader);
```

#### `get_loader_by_extension(extension: &str) -> Option<Arc<dyn CustomAssetLoader + Send + Sync>>`

拡張子からアセットローダーを取得します。

```rust
if let Some(loader) = registry.get_loader_by_extension("config") {
    // ローダーを使用
}
```

#### `get_loader(type_name: &str) -> Option<Arc<dyn CustomAssetLoader + Send + Sync>>`

アセットタイプ名からローダーを取得します。

```rust
if let Some(loader) = registry.get_loader("ConfigFile") {
    // ローダーを使用
}
```

#### `is_extension_supported(extension: &str) -> bool`

指定された拡張子がサポートされているか確認します。

```rust
if registry.is_extension_supported("config") {
    println!("config拡張子がサポートされています");
}
```

## CustomAssetLoader

カスタムアセットローダーのトレイトです。

### メソッド

#### `load_asset(&self, path: &Path) -> Result<Box<dyn Any>, String>`

アセットを読み込みます。

```rust
let asset = loader.load_asset(&path)?;
```

#### `asset_type_name(&self) -> &str`

アセットタイプ名を返します。

```rust
let type_name = loader.asset_type_name();
```

#### `supported_extensions(&self) -> Vec<String>`

サポートする拡張子のリストを返します。

```rust
let extensions = loader.supported_extensions();
```

### 実装例

```rust
use adbx_runtime::hot_reload::custom_asset_registry::{CustomAssetLoader, CustomAssetRegistry};
use std::collections::HashMap;

pub struct ConfigFileLoader;

impl CustomAssetLoader for ConfigFileLoader {
    fn load_asset(&self, path: &std::path::Path) -> Result<Box<dyn std::any::Any>, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        let config: HashMap<String, String> = content
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
```

## アセット監視システム

### `asset_watcher`

アセットファイルの変更を監視するシステムです。

```rust
use adbx_runtime::hot_reload::asset_watcher::asset_watcher;

app.add_systems(Update, asset_watcher);
```

## スクリプトリロードシステム

### `script_reload`

Luaスクリプトの変更を検知してリロードするシステムです。

```rust
use adbx_runtime::hot_reload::script_reload::script_reload;

app.add_systems(Update, script_reload);
```

## カスタムアセットのホットリロード

### `handle_custom_asset_hot_reload`

カスタムアセットのホットリロードを処理する関数です。

```rust
use adbx_runtime::hot_reload::custom_asset_registry::handle_custom_asset_hot_reload;

handle_custom_asset_hot_reload(&registry, "assets/configs/settings.config")?;
```

## デフォルトローダーの登録

### `register_default_custom_asset_loaders`

デフォルトのカスタムアセットローダーを登録するシステムです。

```rust
use adbx_runtime::hot_reload::custom_asset_registry::register_default_custom_asset_loaders;

app.add_systems(Startup, register_default_custom_asset_loaders);
```

## 使用例

### カスタムアセットローダーの登録と使用

```rust
use adbx_runtime::hot_reload::custom_asset_registry::{CustomAssetRegistry, CustomAssetLoader};

// カスタムローダーを実装
struct MyCustomLoader;

impl CustomAssetLoader for MyCustomLoader {
    fn load_asset(&self, path: &std::path::Path) -> Result<Box<dyn std::any::Any>, String> {
        // アセットを読み込む処理
        Ok(Box::new(MyAsset {}))
    }
    
    fn asset_type_name(&self) -> &str {
        "MyAsset"
    }
    
    fn supported_extensions(&self) -> Vec<String> {
        vec!["myasset".to_string()]
    }
}

// ローダーを登録
let mut registry = CustomAssetRegistry::new();
registry.register_loader(MyCustomLoader);

// アセットを読み込む
if let Some(loader) = registry.get_loader_by_extension("myasset") {
    let asset = loader.load_asset(&path)?;
}
```

## 関連型

- [`CustomAssetRegistry`](custom_asset_registry.rs): カスタムアセットレジストリ
- [`CustomAssetLoader`](custom_asset_registry.rs): カスタムアセットローダートレイト
- [`asset_watcher`](asset_watcher.rs): アセット監視システム
- [`script_reload`](script_reload.rs): スクリプトリロードシステム
