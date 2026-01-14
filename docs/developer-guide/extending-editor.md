# エディタの拡張方法

Adbx Engine Editorを拡張する方法を説明します。

## カスタムコンポーネントエディタの追加

カスタムコンポーネントエディタを追加するには、`ComponentEditor`トレイトを実装し、`ComponentEditorRegistry`にエディタを登録します。

詳細は[カスタムコンポーネントエディタの追加](custom-components.md)を参照してください。

## カスタムアセットタイプの追加

カスタムアセットタイプを追加するには、`CustomAssetLoader`トレイトを実装し、`CustomAssetRegistry`に登録します。

詳細は[カスタムアセットタイプの追加](custom-assets.md)を参照してください。

## ビルドシステムの拡張

ビルドシステムを拡張して、カスタムビルドステップを追加できます。

### ビルドリクエストの処理

ビルドシステムは`BuildGameRequest`と`BuildProgress`リソースを使用します。カスタムビルドステップを追加するには、`build_game`関数を拡張するか、新しいシステムを追加します。

```rust
use adbx_editor::systems::build_game::{build_game, BuildGameRequest, BuildProgress};
use adbx_editor::project::Project;
use std::path::PathBuf;

pub fn custom_build_step(
    project: &Project,
    build_request: &mut BuildGameRequest,
    build_progress: &mut BuildProgress,
) -> Result<(), String> {
    // カスタムビルド処理
    build_progress.progress_text = "Running custom build step...".to_string();
    build_progress.progress_percent = 0.5;
    
    // ビルド処理を実行
    Ok(())
}
```

### ビルド設定のカスタマイズ

`build_config.json`を拡張して、カスタム設定を追加できます。設定ファイルは`adbx_runtime::game_config::BuildConfig`構造体にマッピングされます。

```json
{
  "game_name": "MyGame",
  "main_scene": "MainScene",
  "window_title": "My Game",
  "window_width": 1920,
  "window_height": 1080,
  "assets": [
    "assets/textures/",
    "assets/audio/"
  ],
  "custom_setting": "custom_value"
}
```

### リソースの初期化

カスタムビルドシステムを追加する場合、必要なリソースを`app_initialization.rs`で初期化してください：

```rust
app.init_resource::<crate::systems::build_game::BuildGameRequest>();
app.init_resource::<crate::systems::build_game::BuildProgress>();
app.init_resource::<crate::systems::menu::build_game_menu::BuildGameMenuRequest>();
```

## システムの追加

新しいシステムを追加するには、`EditorPlugin`の`build`メソッドでシステムを登録します：

```rust
impl Plugin for MyCustomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, my_custom_system);
    }
}
```

## リソースの追加

新しいリソースを追加するには、`app_initialization.rs`の`initialize_app`関数でリソースを初期化します：

```rust
app.init_resource::<MyCustomResource>();
```

## 関連ドキュメント

- [カスタムコンポーネントエディタの追加](custom-components.md)
- [カスタムアセットタイプの追加](custom-assets.md)
