# エディタの拡張方法

Adbx Engine Editorを拡張する方法を説明します。

## カスタムコンポーネントエディタの追加

カスタムコンポーネントエディタを追加するには、`ComponentEditorRegistry`にエディタを登録します。

```rust
use adbx_editor::ui::inspector::ComponentEditorRegistry;

registry.register_editor("MyComponent", |commands, entity, component| {
    // エディタUIを構築
});
```

## カスタムアセットタイプの追加

カスタムアセットタイプを追加するには、`CustomAssetLoader`を実装し、`CustomAssetRegistry`に登録します。

```rust
use adbx_runtime::hot_reload::custom_asset_registry::{CustomAssetLoader, CustomAssetRegistry};

struct MyAssetLoader;

impl CustomAssetLoader for MyAssetLoader {
    fn load_asset(&self, path: &Path) -> Result<Box<dyn Any>, String> {
        // アセットを読み込む
    }
    
    fn asset_type_name(&self) -> &str {
        "MyAsset"
    }
    
    fn supported_extensions(&self) -> Vec<String> {
        vec!["myasset".to_string()]
    }
}

registry.register_loader(MyAssetLoader);
```

## ビルドシステムの拡張

ビルドシステムを拡張して、カスタムビルドステップを追加できます。

### ビルドフックの追加

```rust
use adbx_editor::systems::build_game::{BuildGameRequest, BuildProgress};

pub fn custom_build_step(
    project_path: &PathBuf,
    build_output_path: &PathBuf,
) -> Result<(), String> {
    // カスタムビルド処理
    Ok(())
}
```

### ビルド設定のカスタマイズ

`build_config.json`を拡張して、カスタム設定を追加できます。

## 関連ドキュメント

- [カスタムコンポーネントエディタの追加](custom-components.md)
- [カスタムアセットタイプの追加](custom-assets.md)
