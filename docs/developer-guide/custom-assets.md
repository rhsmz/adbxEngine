# カスタムアセットタイプの追加

カスタムアセットタイプを追加する方法を説明します。

## 実装手順

1. `CustomAssetLoader`トレイトを実装
2. `CustomAssetRegistry`にローダーを登録
3. ホットリロード時に自動的に使用されます

## 実装例

```rust
use adbx_runtime::hot_reload::custom_asset_registry::{CustomAssetLoader, CustomAssetRegistry};
use std::path::Path;
use std::any::Any;

struct MyAssetLoader;

impl CustomAssetLoader for MyAssetLoader {
    fn load_asset(&self, path: &Path) -> Result<Box<dyn Any>, String> {
        // アセットを読み込む
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read asset: {}", e))?;
        
        // アセットデータを処理
        Ok(Box::new(MyAsset { content }))
    }
    
    fn asset_type_name(&self) -> &str {
        "MyAsset"
    }
    
    fn supported_extensions(&self) -> Vec<String> {
        vec!["myasset".to_string()]
    }
}

// 登録（システム内で実行）
fn register_my_asset_loader(mut registry: ResMut<CustomAssetRegistry>) {
    registry.register_loader(MyAssetLoader);
}
```

## デフォルトのアセットローダー

以下のアセットローダーがデフォルトで登録されています：

- **ConfigFileLoader**: `.config`と`.cfg`拡張子のファイルを読み込みます

## ホットリロード

カスタムアセットローダーを登録すると、対応する拡張子のファイルが変更された際に自動的にホットリロードされます。

`handle_custom_asset_hot_reload`関数が、ファイル変更を検出して適切なローダーを使用してアセットを再読み込みします。

## 関連ドキュメント

- [エディタの拡張方法](extending-editor.md)
- [ランタイムホットリロード API](../../api/runtime/hot_reload.md)
