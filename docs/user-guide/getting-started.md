# はじめに

Adbx Engine Editorの基本的な使い方を説明します。

## インストール

### 前提条件

- Rust 1.70以上
- Cargo
- Docker（Krokiダイアグラム生成用、オプション）

### ビルド

```bash
# Taskfileを使用（推奨）
task build

# または直接Cargoを使用
cargo build
```

### 実行

```bash
# Taskfileを使用（推奨）
task run

# または直接Cargoを使用
cargo run --bin adbx_editor
```

### 初回起動時の注意事項

エディタを初回起動する際、以下の点に注意してください：

1. **アセットディレクトリの作成**: エディタは`adbx_editor/assets`ディレクトリを監視します。このディレクトリが存在しない場合、警告が表示されますが、エディタは正常に動作します。

2. **リソース初期化**: すべての必要なリソースが自動的に初期化されます。起動時にパニックが発生する場合は、リソースの初期化が不足している可能性があります。

3. **レイアウトファイル**: 初回起動時はデフォルトレイアウトが使用されます。レイアウトファイルは`~/.adbx_editor/layout.json`に保存されます。

## 基本操作

### プロジェクトの作成

1. メニューバーから「File」→「New Project」を選択
2. プロジェクト名と保存場所を指定
3. 「Create」をクリック

### プロジェクトの開く

1. メニューバーから「File」→「Open Project」を選択
2. プロジェクトフォルダを選択

### シーンの作成

1. ヒエラルキービューで右クリック
2. 「Create Entity」を選択
3. エンティティ名を入力

### エンティティの編集

1. ヒエラルキービューでエンティティを選択
2. インスペクターパネルでコンポーネントを編集

### シーンの保存

1. メニューバーから「File」→「Save Scene」を選択
2. シーン名を入力
3. 「Save」をクリック

### ゲームのビルド

1. メニューバーから「Build」→「Build Game」を選択
2. ビルド進捗が表示されます
3. ビルド完了後、`target/release/game_package/`にパッケージ化されたゲームが生成されます

ビルドされたゲームは、`target/release/game_package/`ディレクトリに配置され、実行可能ファイルと必要なアセットが含まれます。

#### ビルド設定

プロジェクトディレクトリに`build_config.json`を作成して、ビルド設定をカスタマイズできます：

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
  ]
}
```

## 関連ドキュメント

- [エディタ機能の説明](editor-features.md)
- [Luaスクリプティングガイド](scripting-guide.md)
- [トラブルシューティング](troubleshooting.md)
