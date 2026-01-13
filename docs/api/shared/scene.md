# シーン定義 API リファレンス

`adbx_shared`クレートのシーン定義に関するAPIリファレンスです。

## 概要

`scene.rs`は、シーンデータの構造とシリアライゼーション形式を定義します。シーンはエンティティの階層構造とコンポーネントデータを含みます。

## SceneData

シーンデータを表す構造体です。

### フィールド

- **`name: String`**: シーンの名前
- **`entities: Vec<EntityData>`**: シーンに含まれるエンティティのリスト

### 使用例

```rust
use adbx_shared::scene::SceneData;

let scene = SceneData {
    name: "MainScene".to_string(),
    entities: vec![
        EntityData {
            id: 1,
            name: "Player".to_string(),
            parent: None,
            components: vec![],
        },
    ],
};
```

## EntityData

エンティティデータを表す構造体です。

### フィールド

- **`id: u32`**: エンティティの一意なID
- **`name: String`**: エンティティの名前
- **`parent: Option<u32>`**: 親エンティティのID（ルートエンティティの場合は`None`）
- **`components: Vec<ComponentData>`**: エンティティにアタッチされたコンポーネントのリスト

### 使用例

```rust
use adbx_shared::scene::EntityData;

let entity = EntityData {
    id: 1,
    name: "Player".to_string(),
    parent: None,
    components: vec![
        ComponentData {
            type_name: "Transform".to_string(),
            data: serde_json::json!({
                "translation": [0.0, 0.0, 0.0],
                "rotation": [0.0, 0.0, 0.0, 1.0],
                "scale": [1.0, 1.0, 1.0],
            }),
        },
    ],
};
```

## ComponentData

コンポーネントデータを表す構造体です。型情報とシリアライズされた値を含みます。

### フィールド

- **`type_name: String`**: コンポーネントの型名（例: `"Transform"`, `"Sprite"`）
- **`data: serde_json::Value`**: シリアライズされたコンポーネントデータ

### 使用例

```rust
use adbx_shared::scene::ComponentData;

let transform = ComponentData {
    type_name: "Transform".to_string(),
    data: serde_json::json!({
        "translation": [10.0, 20.0, 30.0],
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [1.0, 1.0, 1.0],
    }),
};
```

## SceneMetadata

シーンのメタデータを表す構造体です。

### フィールド

- **`name: String`**: シーンの名前
- **`path: String`**: シーンファイルのパス
- **`last_modified: u64`**: 最終更新時刻（Unixタイムスタンプ）

### 使用例

```rust
use adbx_shared::scene::SceneMetadata;

let metadata = SceneMetadata {
    name: "MainScene".to_string(),
    path: "scenes/main.scene".to_string(),
    last_modified: 1234567890,
};
```

## シリアライゼーション

シーンデータはJSONまたはMessagePack形式でシリアライゼーションできます。

### JSON形式

```rust
use serde_json;
use adbx_shared::scene::SceneData;

let scene = SceneData {
    name: "MainScene".to_string(),
    entities: vec![],
};

// シリアライゼーション
let json = serde_json::to_string_pretty(&scene)?;

// デシリアライゼーション
let deserialized: SceneData = serde_json::from_str(&json)?;
```

### MessagePack形式

```rust
use rmp_serde;
use adbx_shared::scene::SceneData;

let scene = SceneData {
    name: "MainScene".to_string(),
    entities: vec![],
};

// シリアライゼーション
let msgpack = rmp_serde::to_vec(&scene)?;

// デシリアライゼーション
let deserialized: SceneData = rmp_serde::from_slice(&msgpack)?;
```

## エンティティ階層

エンティティの階層構造は`parent`フィールドで表現されます。親エンティティのIDが`None`の場合は、そのエンティティはルートエンティティです。

### 階層構造の例

```rust
let entities = vec![
    EntityData {
        id: 1,
        name: "Root".to_string(),
        parent: None, // ルートエンティティ
        components: vec![],
    },
    EntityData {
        id: 2,
        name: "Child".to_string(),
        parent: Some(1), // 親エンティティID = 1
        components: vec![],
    },
    EntityData {
        id: 3,
        name: "Grandchild".to_string(),
        parent: Some(2), // 親エンティティID = 2
        components: vec![],
    },
];
```

## 関連型

- [`EditorMessage::LoadScene`](../protocol.md#loadscene--scene_path-string-): シーン読み込みメッセージ
- [`EditorMessage::SaveScene`](../protocol.md#savescene--scene-scenedata-): シーン保存メッセージ
