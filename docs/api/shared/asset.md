# アセット定義 API リファレンス

`adbx_shared`クレートのアセット定義に関するAPIリファレンスです。

## 概要

`asset.rs`は、アセットのタイプ、メタデータ、参照を定義します。

## AssetType

アセットのタイプを表すenumです。

### バリアント

- **`Image`**: 画像アセット（PNG、JPEG、GIFなど）
- **`Audio`**: オーディオアセット（MP3、WAV、OGGなど）
- **`Mesh`**: 3Dメッシュアセット（OBJ、GLTFなど）
- **`Material`**: マテリアルアセット
- **`Script`**: スクリプトアセット（Lua、Pythonなど）
- **`Scene`**: シーンアセット
- **`Unknown`**: 不明なタイプ

### 使用例

```rust
use adbx_shared::asset::AssetType;

let asset_type = AssetType::Image;
match asset_type {
    AssetType::Image => println!("画像アセット"),
    AssetType::Audio => println!("オーディオアセット"),
    AssetType::Mesh => println!("メッシュアセット"),
    AssetType::Material => println!("マテリアルアセット"),
    AssetType::Script => println!("スクリプトアセット"),
    AssetType::Scene => println!("シーンアセット"),
    AssetType::Unknown => println!("不明なタイプ"),
}
```

## AssetMetadata

アセットのメタデータを表す構造体です。

### フィールド

- **`id: String`**: アセットの一意なID
- **`path: String`**: アセットファイルのパス
- **`asset_type: AssetType`**: アセットのタイプ
- **`name: String`**: アセットの名前
- **`size: u64`**: アセットファイルのサイズ（バイト単位）
- **`last_modified: u64`**: 最終更新時刻（Unixタイムスタンプ）

### 使用例

```rust
use adbx_shared::asset::{AssetMetadata, AssetType};

let metadata = AssetMetadata {
    id: "player_texture".to_string(),
    path: "assets/textures/player.png".to_string(),
    asset_type: AssetType::Image,
    name: "Player Texture".to_string(),
    size: 1024 * 1024, // 1MB
    last_modified: 1234567890,
};
```

## AssetReference

アセットへの参照を表す構造体です。

### フィールド

- **`asset_id: String`**: 参照先のアセットID
- **`path: String`**: アセットファイルのパス

### 使用例

```rust
use adbx_shared::asset::AssetReference;

let reference = AssetReference {
    asset_id: "player_texture".to_string(),
    path: "assets/textures/player.png".to_string(),
};
```

## アセットタイプの判定

ファイル拡張子からアセットタイプを判定する例：

```rust
use adbx_shared::asset::AssetType;

fn get_asset_type_from_path(path: &str) -> AssetType {
    let extension = path.split('.').last().unwrap_or("");
    match extension.to_lowercase().as_str() {
        "png" | "jpg" | "jpeg" | "gif" | "bmp" => AssetType::Image,
        "mp3" | "wav" | "ogg" | "flac" => AssetType::Audio,
        "obj" | "gltf" | "glb" | "fbx" => AssetType::Mesh,
        "lua" | "py" | "js" => AssetType::Script,
        "scene" | "json" => AssetType::Scene,
        _ => AssetType::Unknown,
    }
}
```

## シリアライゼーション

アセットメタデータと参照は`serde`を使用してシリアライゼーションできます。

### JSON形式

```rust
use serde_json;
use adbx_shared::asset::AssetMetadata;

let metadata = AssetMetadata {
    id: "player_texture".to_string(),
    path: "assets/textures/player.png".to_string(),
    asset_type: AssetType::Image,
    name: "Player Texture".to_string(),
    size: 1024 * 1024,
    last_modified: 1234567890,
};

// シリアライゼーション
let json = serde_json::to_string_pretty(&metadata)?;

// デシリアライゼーション
let deserialized: AssetMetadata = serde_json::from_str(&json)?;
```

## 関連型

- [`EditorMessage::LoadAsset`](../protocol.md#loadasset--asset_path-string-): アセット読み込みメッセージ
- [`EditorMessage::HotReload`](../protocol.md#hotreload--asset_path-string-): ホットリロードメッセージ
