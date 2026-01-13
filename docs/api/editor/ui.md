# エディタUI API リファレンス

`adbx_editor`クレートのUIコンポーネントに関するAPIリファレンスです。

## 概要

エディタUIは、Bevy標準UIを使用して実装されたモダンなエディタインターフェースを提供します。

## HierarchyView

ヒエラルキービューは、エンティティの階層構造を表示するパネルです。仮想スクロール機能をサポートしています。

### フィールド

- **`expanded_entities: HashSet<Entity>`**: 展開されているエンティティのセット
- **`scroll_offset: f32`**: スクロール位置
- **`item_height: f32`**: 各アイテムの高さ
- **`total_items: usize`**: 総アイテム数

### 使用例

```rust
use adbx_editor::ui::hierarchy::HierarchyView;

let hierarchy_view = HierarchyView::default();
```

## InspectorPanel

インスペクターパネルは、選択されたエンティティのコンポーネントを表示・編集するパネルです。遅延更新機能をサポートしています。

### フィールド

- **`selected_entity: Option<Entity>`**: 選択されているエンティティ
- **`last_update_time: f64`**: 最後の更新時刻
- **`update_throttle_ms: f64`**: 更新スロットル（ミリ秒）

### 使用例

```rust
use adbx_editor::ui::inspector::InspectorPanel;

let inspector = InspectorPanel::default();
```

## AssetBrowser

アセットブラウザーは、プロジェクト内のアセットを表示・管理するパネルです。仮想スクロール機能をサポートしています。

### フィールド

- **`current_path: PathBuf`**: 現在のディレクトリパス
- **`scroll_offset: f32`**: スクロール位置
- **`item_height: f32`**: 各アイテムの高さ
- **`total_items: usize`**: 総アイテム数

### 使用例

```rust
use adbx_editor::ui::asset_browser::AssetBrowser;

let asset_browser = AssetBrowser::new();
```

## CodeEditor

コードエディタは、コード編集機能を提供するコンポーネントです。シンタックスハイライト、コード補完、Undo/Redo機能をサポートしています。

### フィールド

- **`content: String`**: エディタの内容
- **`cursor_position: usize`**: カーソル位置
- **`language: String`**: 言語（自動判定）

### 使用例

```rust
use adbx_editor::ui::code_editor::CodeEditor;

let code_editor = CodeEditor::default();
```

## ScriptEditor

スクリプトエディタは、Luaスクリプトを編集・実行するコンポーネントです。

### 使用例

```rust
use adbx_editor::ui::script_editor::ScriptEditor;

let script_editor = ScriptEditor::default();
```

## SceneView

シーンビューは、3Dシーンを表示するビューです。カメラ操作とGizmo機能をサポートしています。

### 使用例

```rust
use adbx_editor::ui::scene_view::SceneView;

let scene_view = SceneView::default();
```

## DockingSystem

ドッキングシステムは、パネルの移動・リサイズ・ドッキングを管理するシステムです。

### 使用例

```rust
use adbx_editor::ui::docking::DockingSystem;

let docking = DockingSystem::default();
```

## 関連型

- [`HierarchyView`](hierarchy.rs): ヒエラルキービュー
- [`InspectorPanel`](inspector.rs): インスペクターパネル
- [`AssetBrowser`](asset_browser.rs): アセットブラウザー
- [`CodeEditor`](code_editor.rs): コードエディタ
