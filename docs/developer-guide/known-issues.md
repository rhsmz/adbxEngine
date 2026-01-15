# 既知の問題と警告

このドキュメントでは、現在のコードベースで確認されている警告と未使用機能について説明します。

## コンパイル時の警告

現在、170個程度の警告が生成されています（`cargo build`実行時）。主な警告は以下の通りです：

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

## 対応状況

| 項目 | ステータス | 対応内容 | 理由 |
|------|-----------|----------|------|
| 列挙型バリアント | 保留 | `#[allow(dead_code)]`適用済み | 将来の実装予約 |
| 未使用メソッド/関数 | 保留 | `#[allow(dead_code)]`適用済み | 将来の実装予約 |
| 未使用フィールド | 保留 | `#[allow(dead_code)]`適用済み | 将来の実装予約 |
| BuildGameMenuRequest初期化 | 完了 | `app_initialization.rs`で初期化済み | v0.1.0で修正 |
| アセットディレクトリ警告 | 完了 | `adbx_editor/assets/`および`adbx_runtime/assets/`作成済み | 監視機能有効化 |
| TCP未接続警告 | 完了 | 自動再接続システム実装済み | 通信状態管理強化 |
| Camera order曖昧性警告 | 完了 | SceneViewカメラのorderを-1に設定 | カメラ重複解消 |

## 起動時の問題

### BuildGameMenuRequestリソースの初期化

`BuildGameMenuRequest`リソースが`app_initialization.rs`で初期化されていない場合、ビルドメニュー操作時にパニックが発生します。

**エラーメッセージ:**
```
thread 'main' (7000) panicked at C:\Users\rsmz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bevy_ecs-0.17.3\src\error\handler.rs:125:1:
Encountered an error in system `adbx_editor::systems::menu::build_game_menu::handle_build_game_request`: Parameter `ResMut<'_, BuildGameMenuRequest>` failed validation: Resource does not exist
```

**修正状況:**
この問題はv0.1.0で修正されました。`app_initialization.rs`の`initialize_app`関数に以下の行が追加されています：

```rust
app.init_resource::<crate::systems::menu::build_game_menu::BuildGameMenuRequest>();
```

ビルド機能が正常に動作するようになりました。

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
