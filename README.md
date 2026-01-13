# Adbx Engine Editor

Bevy 0.17用のUnityレベルのゲームエンジンエディタです。

## 機能

- **Unityレベルのエディタ機能**: Bevy標準UIを使用したモダンなエディタインターフェース
  - ドッキング可能なパネルシステム
  - ヒエラルキービュー（仮想スクロール対応）
  - インスペクターパネル（遅延更新対応）
  - アセットブラウザー（仮想スクロール対応）
- **ハイブコーディングエディタ**: 生成AIを統合したコード編集機能
  - シンタックスハイライト（syntect使用）
  - コード補完機能
  - リアルタイムプレビュー
- **Luaスクリプトシステム**: Entityにアタッチ可能なLuaスクリプト機能
  - スクリプトのコンパイル結果キャッシュ
  - 実行頻度の制限
  - 不要なスクリプト実行のスキップ
- **ホットリロード**: アセット、スクリプト、シーンのリアルタイムリロード
  - カスタムアセットタイプのサポート
- **パフォーマンス最適化**
  - 大規模シーンでのパフォーマンス改善
  - エディタUIの応答性向上
  - Luaスクリプト実行の最適化
- **包括的なエラーハンドリング**
  - エラータイプの統一とコンテキスト情報
  - ユーザーフレンドリーなエラーメッセージ（日本語対応）

## プロジェクト構造

```
adbxEngine/
├── adbx_editor/      # エディタアプリケーション
├── adbx_runtime/     # ゲームランタイム
├── adbx_shared/      # 共有ライブラリ
├── docs/             # ドキュメント
│   ├── api/          # APIリファレンス
│   ├── architecture/ # アーキテクチャドキュメント
│   ├── user-guide/   # ユーザーガイド
│   └── developer-guide/ # 開発者ガイド
└── tests/            # テストコード
    ├── integration/  # 統合テスト
    └── e2e/          # E2Eテスト
```

## クイックスタート

### 前提条件

- Rust 1.70以上
- Cargo

### インストール

```bash
git clone <repository-url>
cd adbxEngine
```

### ビルドと実行

```bash
# Taskfileを使用（推奨）
task run

# または直接Cargoを使用
cargo run --bin adbx_editor
```

## 主要なコマンド

このプロジェクトでは[Task](https://taskfile.dev/)を使用して開発タスクを管理しています。

### 基本的なコマンド

```bash
# エディタを実行（デフォルト）
task run

# プロジェクトをビルド
task build

# リリースモードでビルド
task build-release

# すべてのテストを実行
task test

# コードをフォーマット
task fmt

# Clippyでリント
task clippy
```

### 開発用コマンド

```bash
# ファイル変更を監視して自動ビルド
task watch

# ファイル変更を監視して自動テスト
task watch-test

# ファイル変更を監視して自動実行
task watch-run

# 開発環境のセットアップ（フォーマット、Clippy、テスト）
task dev
```

### テストコマンド

```bash
# すべてのテスト
task test

# 特定のクレートのテスト
task test-shared
task test-runtime
task test-editor

# 統合テスト
task test-integration

# E2Eテスト
task test-e2e
```

### ドキュメントコマンド

```bash
# ドキュメントを生成して開く
task doc

# すべてのクレートのドキュメントを生成
task doc-all
```

### ビルドコマンド

```bash
# ゲームをスタンドアロン実行可能ファイルとしてビルド
task build-game

# ビルドしたゲームをパッケージ化
task package-game
```

### その他のコマンド

```bash
# コードのコンパイルチェック
task check

# ビルド成果物を削除
task clean

# 依存関係を更新
task update

# 依存関係ツリーを表示
task tree
```

すべての利用可能なタスクを確認するには：

```bash
task --list
```

## ゲームのビルド

エディタで作成したゲームをスタンドアロン実行可能ファイルとしてビルドできます。

### エディタからビルド

1. メニューバーから「Build」→「Build Game」を選択
2. ビルド進捗が表示されます
3. ビルド完了後、`target/release/game_package/`にパッケージ化されたゲームが生成されます

### コマンドラインからビルド

```bash
# ゲームをビルド
task build-game

# ランタイムを直接実行
task run-runtime -- --project ./my_project
```

### ビルド設定

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

## ドキュメント

詳細なドキュメントは`docs/`ディレクトリにあります：

- [APIリファレンス](docs/api/) - すべてのAPIの詳細な説明
- [アーキテクチャドキュメント](docs/architecture/) - システムアーキテクチャと設計
- [ユーザーガイド](docs/user-guide/) - エディタの使い方
- [開発者ガイド](docs/developer-guide/) - エディタの拡張方法

## 依存関係

- Bevy 0.17（`file_watcher` feature有効）
- mlua（Lua統合）
- serde（シリアライゼーション）
- syntect（シンタックスハイライト）
- rmp-serde（MessagePackサポート）
- thiserror（エラーハンドリング）

## ライセンス

MIT OR Apache-2.0
