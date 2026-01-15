# はじめに

Adbx Engine Editorの基本的な使い方を説明します。

## インストール

### 前提条件

- Rust 1.70以上
- Cargo
- Docker（Krokiダイアグラム生成用、オプション）

### ビルド

プロジェクトはCargo workspaceを使用しており、以下の主要コンポーネントを含みます：

- `adbx_editor`: エディタアプリケーション
- `adbx_runtime`: ゲームランタイム
- `adbx_shared`: 共有ライブラリ

```bash
# Taskfileを使用（推奨）
task build

# または直接Cargoを使用
cargo build

# リリースビルドの場合
cargo build --release
```

### 開発環境のセットアップ

初回のビルド前に、以下の準備を行ってください：

1. **アセットディレクトリの作成**: エディタとランタイムの監視機能を有効にするため、以下のディレクトリを作成してください。
   ```bash
   mkdir adbx_editor/assets
   mkdir adbx_runtime/assets
   ```

2. **コード品質チェック**: 定期的にClippyを実行して警告を確認してください。
   ```bash
   # Taskfileを使用（推奨）
   task clippy-all

   # または直接Cargoを使用
   cargo clippy --workspace -- -D warnings
   ```

3. **テスト実行**: すべてのテストを実行して機能を検証してください。
   ```bash
   # Taskfileを使用（推奨）
   task test

   # または直接Cargoを使用
   cargo test
   ```

**注意**: ビルドには`file_watcher` featureが有効なBevy 0.17.3が必要です。このfeatureはアセットのホットリロード機能を提供します。

### 実行

```bash
# Taskfileを使用（推奨）
task run

# または直接Cargoを使用
cargo run --bin adbx_editor

# リリースビルドを実行する場合
cargo run --bin adbx_editor --release
```

### 初回起動時の注意事項

エディタを初回起動する際、以下の点に注意してください：

1. **アセットディレクトリの作成**: エディタは`adbx_editor/assets`ディレクトリを監視します。このディレクトリが存在しない場合、以下の警告が表示されますが、エディタは正常に動作します：
   ```
   Skip creating file watcher because path "C:\Users\...\adbx_editor\assets" does not exist.
   AssetSourceId::Default does not have an AssetWatcher configured. Consider adding an "assets" directory.
   ```
   アセット監視機能を有効にする場合は、ディレクトリを作成してください。

2. **リソース初期化**: すべての必要なリソースが自動的に初期化されます。`BuildGameMenuRequest`リソースは`app_initialization.rs`で初期化されているため、ビルド機能が正常に動作します。

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

ビルドされたゲームは、`target/release/game_package/`ディレクトリに配置され、以下のファイルが含まれます：
- 実行可能ファイル（`adbx_runtime.exe`または`adbx_runtime`）
- アセットディレクトリ（`assets/`）
- シーンディレクトリ（`scenes/`）
- 設定ファイル（`build_config.json`）

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
