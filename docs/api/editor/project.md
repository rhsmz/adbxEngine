# プロジェクト管理 API リファレンス

`adbx_editor`クレートのプロジェクト管理に関するAPIリファレンスです。

## 概要

プロジェクト管理は、プロジェクトの作成、読み込み、保存、シーンの管理などの機能を提供します。

## Project

プロジェクト管理リソースです。

### フィールド

- **`project_path: Option<PathBuf>`**: プロジェクトのパス
- **`name: String`**: プロジェクト名
- **`scenes: Vec<SceneData>`**: プロジェクトに含まれるシーンのリスト
- **`assets: Vec<AssetMetadata>`**: プロジェクトに含まれるアセットのリスト
- **`serialization_format: SerializationFormat`**: シリアライゼーション形式

### 使用例

```rust
use adbx_editor::project::Project;

let project = Project::default();
```

## SerializationFormat

シリアライゼーション形式を表すenumです。

### バリアント

- **`Json`**: JSON形式
- **`MessagePack`**: MessagePack形式

### 使用例

```rust
use adbx_editor::project::SerializationFormat;

let format = SerializationFormat::Json;
```

## プロジェクト操作

### `create_project`

新しいプロジェクトを作成します。

```rust
use adbx_editor::project::create_project;

let project = create_project(
    "MyProject".to_string(),
    PathBuf::from("projects/my_project"),
)?;
```

### `load_project`

既存のプロジェクトを読み込みます。

```rust
use adbx_editor::project::load_project;

let project = load_project(PathBuf::from("projects/my_project"))?;
```

## シーン操作

### `save_scene`

シーンを保存します（デフォルト形式：JSON）。

```rust
use adbx_editor::project::save_scene;

save_scene(&scene_data, &project_path)?;
```

### `save_scene_with_format`

シーンを指定された形式で保存します。

```rust
use adbx_editor::project::{save_scene_with_format, SerializationFormat};

save_scene_with_format(&scene_data, &project_path, SerializationFormat::MessagePack)?;
```

### `load_scene`

シーンを読み込みます。

```rust
use adbx_editor::project::load_scene;

let scene = load_scene(&scene_path)?;
```

## 関連型

- [`SceneData`](../../shared/scene.md#scenedata): シーンデータ構造体
- [`AssetMetadata`](../../shared/asset.md#assetmetadata): アセットメタデータ構造体
