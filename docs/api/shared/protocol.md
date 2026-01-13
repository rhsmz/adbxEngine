# プロトコル定義 API リファレンス

`adbx_shared`クレートのプロトコル定義に関するAPIリファレンスです。

## 概要

`protocol.rs`は、エディタとランタイム間の通信プロトコルを定義します。メッセージはJSONまたはMessagePack形式でシリアライゼーションされます。

## EditorMessage

エディタからランタイムへのメッセージを定義するenumです。

### バリアント

#### `LoadScene { scene_path: String }`
シーンの読み込みを要求します。

- **`scene_path`**: 読み込むシーンファイルのパス

**使用例:**
```rust
let message = EditorMessage::LoadScene {
    scene_path: "scenes/main.scene".to_string(),
};
```

#### `SaveScene { scene: SceneData }`
シーンの保存を要求します。

- **`scene`**: 保存するシーンデータ（`SceneData`型）

**使用例:**
```rust
let message = EditorMessage::SaveScene {
    scene: scene_data,
};
```

#### `SelectEntity { entity_id: u32 }`
エンティティの選択を通知します。

- **`entity_id`**: 選択されたエンティティのID

**使用例:**
```rust
let message = EditorMessage::SelectEntity {
    entity_id: 42,
};
```

#### `UpdateComponent { entity_id: u32, component_data: ComponentData }`
コンポーネントの更新を通知します。

- **`entity_id`**: 更新対象のエンティティID
- **`component_data`**: 更新するコンポーネントデータ

**使用例:**
```rust
let message = EditorMessage::UpdateComponent {
    entity_id: 42,
    component_data: ComponentData {
        type_name: "Transform".to_string(),
        data: serde_json::json!({
            "translation": [0.0, 0.0, 0.0],
            "rotation": [0.0, 0.0, 0.0, 1.0],
            "scale": [1.0, 1.0, 1.0],
        }),
    },
};
```

#### `LoadAsset { asset_path: String }`
アセットの読み込みを要求します。

- **`asset_path`**: 読み込むアセットファイルのパス

**使用例:**
```rust
let message = EditorMessage::LoadAsset {
    asset_path: "assets/textures/player.png".to_string(),
};
```

#### `HotReload { asset_path: String }`
アセットのホットリロードを要求します。

- **`asset_path`**: リロードするアセットファイルのパス

**使用例:**
```rust
let message = EditorMessage::HotReload {
    asset_path: "assets/scripts/player.lua".to_string(),
};
```

#### `ExecuteScript { script_content: String }`
スクリプトの実行を要求します。

- **`script_content`**: 実行するLuaスクリプトの内容

**使用例:**
```rust
let message = EditorMessage::ExecuteScript {
    script_content: "print('Hello, World!')".to_string(),
};
```

#### `AttachScript { entity_id: u32, script_path: String }`
エンティティにスクリプトをアタッチします。

- **`entity_id`**: スクリプトをアタッチするエンティティID
- **`script_path`**: アタッチするスクリプトファイルのパス

**使用例:**
```rust
let message = EditorMessage::AttachScript {
    entity_id: 42,
    script_path: "assets/scripts/player.lua".to_string(),
};
```

#### `DetachScript { entity_id: u32 }`
エンティティからスクリプトを削除します。

- **`entity_id`**: スクリプトを削除するエンティティID

**使用例:**
```rust
let message = EditorMessage::DetachScript {
    entity_id: 42,
};
```

## RuntimeMessage

ランタイムからエディタへのメッセージを定義するenumです。

### バリアント

#### `SceneLoaded { scene: SceneData }`
シーンの読み込みが完了したことを通知します。

- **`scene`**: 読み込まれたシーンデータ

**使用例:**
```rust
let message = RuntimeMessage::SceneLoaded {
    scene: loaded_scene,
};
```

#### `EntityUpdated { entity_id: u32 }`
エンティティの状態が更新されたことを通知します。

- **`entity_id`**: 更新されたエンティティID

**使用例:**
```rust
let message = RuntimeMessage::EntityUpdated {
    entity_id: 42,
};
```

#### `Error { message: String }`
エラーが発生したことを通知します。

- **`message`**: エラーメッセージ

**使用例:**
```rust
let message = RuntimeMessage::Error {
    message: "Failed to load scene".to_string(),
};
```

#### `HotReloaded { asset_path: String }`
アセットのホットリロードが完了したことを通知します。

- **`asset_path`**: リロードされたアセットファイルのパス

**使用例:**
```rust
let message = RuntimeMessage::HotReloaded {
    asset_path: "assets/scripts/player.lua".to_string(),
};
```

#### `ScriptExecuted { success: bool, error_message: Option<String> }`
スクリプトの実行が完了したことを通知します。

- **`success`**: 実行が成功したかどうか
- **`error_message`**: エラーが発生した場合のエラーメッセージ

**使用例:**
```rust
let message = RuntimeMessage::ScriptExecuted {
    success: true,
    error_message: None,
};
```

#### `ScriptAttached { entity_id: u32, script_path: String }`
スクリプトのアタッチが完了したことを通知します。

- **`entity_id`**: スクリプトがアタッチされたエンティティID
- **`script_path`**: アタッチされたスクリプトファイルのパス

**使用例:**
```rust
let message = RuntimeMessage::ScriptAttached {
    entity_id: 42,
    script_path: "assets/scripts/player.lua".to_string(),
};
```

#### `ScriptDetached { entity_id: u32 }`
スクリプトの削除が完了したことを通知します。

- **`entity_id`**: スクリプトが削除されたエンティティID

**使用例:**
```rust
let message = RuntimeMessage::ScriptDetached {
    entity_id: 42,
};
```

## シリアライゼーション

メッセージは`serde`を使用してシリアライゼーションされます。デフォルトではJSON形式ですが、MessagePack形式もサポートされています。

### JSON形式

```rust
use serde_json;

let message = EditorMessage::LoadScene {
    scene_path: "scenes/main.scene".to_string(),
};
let json = serde_json::to_string(&message)?;
```

### MessagePack形式

```rust
use rmp_serde;

let message = EditorMessage::LoadScene {
    scene_path: "scenes/main.scene".to_string(),
};
let msgpack = rmp_serde::to_vec(&message)?;
```

## 関連型

- [`SceneData`](../scene.md#scenedata): シーンデータ構造体
- [`ComponentData`](../scene.md#componentdata): コンポーネントデータ構造体
