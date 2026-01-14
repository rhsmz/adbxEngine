# 既知の問題と警告

このドキュメントでは、現在のコードベースで確認されている警告と未使用機能について説明します。

## コンパイル時の警告

現在、約170個の警告が生成されています。主な警告は以下の通りです：

### 未使用の列挙型バリアント

以下の列挙型のバリアントが定義されていますが、現在は使用されていません：

- `FileDialogResult::File`, `FileDialogResult::Folder`, `FileDialogResult::Cancelled`
- `LogLevel::Info`, `LogLevel::Warn`, `LogLevel::Error`, `LogLevel::Debug`
- `RenameDialogResult::Cancelled`

これらは将来の実装のために予約されています。

### 未使用のメソッドと関数

以下のメソッドと関数が定義されていますが、現在は使用されていません：

- `ComponentEditorRegistry::iter()` - すべてのエディタを取得するメソッド
- `ComponentEditor::component_name()` - コンポーネント名を取得するメソッド
- `InspectorPanel`の遅延更新関連フィールド（`last_update_time`、`update_throttle`、`pending_update`）
- `spawn_transform_vector3_edit_field()` - Transform編集フィールドを生成する関数
- `validate_transform_value()` - Transform値を検証する関数
- `toggle_log_panel()` - ログパネルの表示/非表示を切り替える関数
- `LogPanel::add_log()` - ログを追加するメソッド
- `validate_rename_name()` - リネーム名を検証する関数
- `generate_rename_path()` - リネームパスを生成する関数
- `attach_script_to_entity()` - エンティティにスクリプトをアタッチする関数

### 未使用のフィールド

以下のリソースのフィールドが定義されていますが、現在は読み取られていません：

- `InspectorPanel::last_update_time`, `update_throttle`, `pending_update`
- `LogPanel::max_logs`, `is_visible`
- `PreviewContent::content`, `preview_type`
- `RenameDialogResult::Renamed::old_path`, `new_path`
- `SceneView::camera_entity`
- `ScriptEditor::entity`
- `ScriptError::column`

## 起動時の問題

### BuildGameMenuRequestリソースの初期化

`BuildGameMenuRequest`リソースが`app_initialization.rs`で初期化されていない場合、起動時にパニックが発生します。

**エラーメッセージ:**
```
Parameter `ResMut<'_, BuildGameMenuRequest>` failed validation: Resource does not exist
```

**解決方法:**
`app_initialization.rs`の`initialize_app`関数に以下を追加してください：

```rust
app.init_resource::<crate::systems::menu::build_game_menu::BuildGameMenuRequest>();
```

## アセットディレクトリの警告

`adbx_editor/assets`ディレクトリが存在しない場合、以下の警告が表示されます：

```
Skip creating file watcher because path "C:\Users\...\adbx_editor\assets" does not exist.
AssetSourceId::Default does not have an AssetWatcher configured. Consider adding an "assets" directory.
```

この警告は無視しても問題ありませんが、アセット監視機能を使用する場合は、ディレクトリを作成してください。

## 警告の対処方法

### 一時的な対処

警告を抑制するには、該当するコードに`#[allow(dead_code)]`属性を追加します：

```rust
#[allow(dead_code)]
pub struct MyUnusedStruct {
    // ...
}
```

### 根本的な対処

1. **未使用機能の実装**: 将来使用予定の機能を実装する
2. **未使用コードの削除**: 不要になったコードを削除する
3. **機能の完成**: 部分的に実装された機能を完成させる

## 推奨事項

- 定期的に`cargo clippy`を実行して、警告を確認してください
- 未使用のコードは、将来の実装が確実な場合のみ残してください
- リソースの初期化は、`app_initialization.rs`で一元管理してください

## 関連ドキュメント

- [トラブルシューティング](../user-guide/troubleshooting.md)
- [エディタの拡張方法](extending-editor.md)
