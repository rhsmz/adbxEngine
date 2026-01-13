# エディタシステム API リファレンス

`adbx_editor`クレートのシステムに関するAPIリファレンスです。

## 概要

エディタシステムは、エディタの動作を制御する各種システムを提供します。

## Selection

選択システムは、エンティティの選択状態を管理します。

### フィールド

- **`selected_entities: HashSet<Entity>`**: 選択されているエンティティのセット

### 使用例

```rust
use adbx_editor::systems::selection::Selection;

let selection = Selection::default();
```

## SceneManager

シーン管理システムは、シーンの読み込み・保存・管理を行います。

### フィールド

- **`entity_id_map: HashMap<u32, Entity>`**: エンティティIDのマッピング
- **`current_scene_name: String`**: 現在のシーン名

### 使用例

```rust
use adbx_editor::systems::scene_management::SceneManager;

let scene_manager = SceneManager::default();
```

## OperationRecorder

操作記録システムは、エディタ操作を記録し、AI理解可能な形式で提供します。

### フィールド

- **`operations: VecDeque<RecordedOperation>`**: 記録された操作のリスト
- **`max_history: usize`**: 最大履歴数
- **`is_recording: bool`**: 記録中かどうか

### メソッド

#### `record_operation`

操作を記録します。

```rust
recorder.record_operation(
    OperationType::EntityCreated,
    OperationContext { /* ... */ },
    serde_json::json!({ /* ... */ }),
);
```

#### `get_ai_readable_history`

操作履歴をAI理解可能な形式で取得します。

```rust
let history = recorder.get_ai_readable_history(Some(100));
```

### 使用例

```rust
use adbx_editor::systems::operation_recording::OperationRecorder;

let recorder = OperationRecorder::new(1000);
```

## RealtimeSync

リアルタイム同期システムは、エディタとランタイム間でエンティティ状態を同期します。

### システム

#### `sync_entity_selection`

エンティティ選択状態をランタイムに同期します。

```rust
use adbx_editor::systems::realtime_sync::sync_entity_selection;

app.add_systems(Update, sync_entity_selection);
```

#### `sync_transform_changes`

Transform変更をランタイムに同期します。

```rust
use adbx_editor::systems::realtime_sync::sync_transform_changes;

app.add_systems(Update, sync_transform_changes);
```

## BuildGame

ビルドゲームシステムは、ゲームをスタンドアロン実行可能ファイルとしてビルドします。

### リソース

- **`BuildGameRequest`**: ビルドリクエストの状態を管理
- **`BuildProgress`**: ビルド進捗を管理

### メソッド

#### `build_game`

ゲームをビルドします。

```rust
use adbx_editor::systems::build_game::build_game;

build_game(&project, &mut build_request, &mut build_progress)?;
```

## PackageGame

パッケージゲームシステムは、ビルドしたゲームをパッケージ化します。

### メソッド

#### `package_game`

ゲームをパッケージ化します。

```rust
use adbx_editor::systems::package_game::package_game;

package_game(&project_path, &build_output_path)?;
```

#### `package_game_platform`

プラットフォーム別にゲームをパッケージ化します。

```rust
use adbx_editor::systems::package_game::{package_game_platform, Platform};

package_game_platform(&project_path, &build_output_path, Platform::Windows)?;
```

## 関連型

- [`Selection`](selection.rs): 選択システム
- [`SceneManager`](scene_management.rs): シーン管理システム
- [`OperationRecorder`](operation_recording.rs): 操作記録システム
- [`RealtimeSync`](realtime_sync.rs): リアルタイム同期システム
- [`BuildGame`](build_game.rs): ビルドゲームシステム
- [`PackageGame`](package_game.rs): パッケージゲームシステム