# カスタムアセットタイプの追加

カスタムアセットタイプを追加する方法を説明します。

## 実装手順

1. `CustomAssetLoader`トレイトを実装
2. `CustomAssetRegistry`にローダーを登録

## 実装例

```rust
use adbx_runtime::hot_reload::custom_asset_registry::{CustomAssetLoader, CustomAssetRegistry};

struct MyAssetLoader;

impl CustomAssetLoader for MyAssetLoader {
    fn load_asset(&self, path: &Path) -> Result<Box<dyn Any>, String> {
        // アセットを読み込む
        Ok(Box::new(MyAsset {}))
    }
    
    fn asset_type_name(&self) -> &str {
        "MyAsset"
    }
    
    fn supported_extensions(&self) -> Vec<String> {
        vec!["myasset".to_string()]
    }
}

// 登録
registry.register_loader(MyAssetLoader);
```

## 関連ドキュメント

- [エディタの拡張方法](extending-editor.md)
- [ランタイムホットリロード API](../../api/runtime/hot_reload.md)
