# 設定ファイルと構成

Adbx Engine Editorの各種設定ファイルと構成について説明します。

## 設定ファイルの種類

### プロジェクト設定ファイル (*.config, *.cfg)

カスタムアセットとして読み込まれる設定ファイルです。`ConfigFileLoader`によって自動的にパースされ、ホットリロードされます。

#### ファイル形式

キーと値のペアを`=`で区切って記述します。コメントは`#`で開始します。

```config
# ゲーム設定
game_title = My Awesome Game
version = 1.0.0
window_width = 1920
window_height = 1080

# オーディオ設定
master_volume = 0.8
music_volume = 0.6
sfx_volume = 0.7

# 物理設定
gravity = -9.81
time_scale = 1.0
```

#### 使用例

```rust
use bevy::prelude::*;
use adbx_runtime::hot_reload::custom_asset_registry::CustomAssetRegistry;
use std::collections::HashMap;

fn load_game_config(
    registry: Res<CustomAssetRegistry>,
    // 設定ファイルが変更されたことを検知するロジック
) {
    // ConfigFileLoaderによってパースされた設定を取得
    // 実際の使用方法はアプリケーションの要件による
    println!("Config files are automatically loaded by ConfigFileLoader");
}
```

### ビルド設定ファイル (build_config.json)

ゲームビルド時の設定を定義するJSONファイルです。プロジェクトルートに配置します。

#### ファイル形式

```json
{
  "game_name": "MyGame",
  "main_scene": "MainScene",
  "window_title": "My Game",
  "window_width": 1920,
  "window_height": 1080,
  "assets": [
    "assets/textures/",
    "assets/audio/",
    "assets/models/"
  ],
  "scripts": [
    "scripts/main.lua",
    "scripts/gameplay.lua"
  ]
}
```

#### 設定項目

- `game_name`: ゲームの名前（出力ディレクトリ名に使用）
- `main_scene`: 起動時に読み込むメインシーン
- `window_title`: ウィンドウタイトル
- `window_width/height`: ウィンドウサイズ
- `assets`: ビルドに含めるアセットディレクトリ
- `scripts`: ビルドに含めるスクリプトファイル

### エディタ設定ファイル

エディタの設定は自動的に保存されます。

#### レイアウトファイル (~/.adbx_editor/layout.json)

エディタのUIレイアウト設定を保存します。初回起動時はデフォルトレイアウトが使用され、変更すると自動的に保存されます。

#### 設定ファイル (~/.adbx_editor/settings.json)

エディタの各種設定を保存します。`save_editor_settings_system`によって管理されます。

## 設定ファイルの読み込みフロー

### ConfigFileLoader の動作

1. ファイル変更を検出（`file_watcher` featureが必要）
2. 拡張子が`.config`または`.cfg`かを確認
3. ファイルをテキストとして読み込み
4. 行ごとにパース（`key = value`形式）
5. コメント行（`#`開始）と空行をスキップ
6. `HashMap<String, String>`としてデータを構築
7. アプリケーションで使用可能

### ホットリロードの制約

- ConfigFileLoaderはシンプルなキーバリュー形式のみサポート
- 複雑なデータ構造が必要な場合は、カスタムアセットローダーを作成
- 設定ファイルの変更は即座に反映されますが、エラーハンドリングはアプリケーション側で実装

## ディレクトリ構成

### 推奨されるプロジェクト構成

```
my_game/
├── assets/
│   ├── textures/
│   ├── audio/
│   └── models/
├── scripts/
│   ├── main.lua
│   └── gameplay.lua
├── config/
│   ├── game.config
│   └── audio.cfg
├── scenes/
│   └── main.scene
├── build_config.json
└── README.md
```

### エディタ用ディレクトリ

```
~/.adbx_editor/
├── layout.json          # UIレイアウト設定
├── settings.json        # エディタ設定
└── projects/           # プロジェクト固有設定
```

## 注意事項

- ConfigFileLoaderは基本的な設定ファイル向けです
- 複雑な設定が必要な場合はJSONやYAMLなどの形式を検討してください
- レイアウトファイルは手動編集せず、エディタのUIから変更してください
- ビルド設定ファイルはプロジェクトルートに配置する必要があります

## 関連ドキュメント

- [カスタムアセットタイプの追加](developer-guide/custom-assets.md)
- [ホットリロードAPI](api/runtime/hot_reload.md)
- [プロジェクト管理API](api/editor/project.md)