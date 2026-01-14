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

- [アーキテクチャ概要](docs/architecture.md) - アーキテクチャ全体像と図一覧
- [アーキテクチャドキュメント](docs/architecture/) - システムアーキテクチャと設計（Krokiダイアグラム含む）
- [**📊 アーキテクチャダイアグラム（ブラウザ表示）**](docs/architecture/diagrams.html) - すべてのダイアグラムをブラウザで確認
- [ユーザーガイド](docs/user-guide/) - エディタの使い方と機能説明
- [開発者ガイド](docs/developer-guide/) - エディタの拡張方法とAPI
- [設定ファイル](docs/configuration.md) - 設定ファイルと構成の詳細
- [APIリファレンス](docs/api/) - すべてのAPIの詳細な説明
- [コントリビューションガイド](docs/CONTRIBUTING.md) - ドキュメントの更新方法と責務
- [既知の問題](docs/developer-guide/known-issues.md) - 既知の警告と未使用機能

### ダイアグラムのブラウザ確認

Krokiダイアグラムをブラウザで簡単に確認するには：

```bash
# HTMLファイルを開く
start docs\architecture\diagrams.html
```

このHTMLファイルでは、すべてのアーキテクチャダイアグラムを統合して表示し、Krokiサービスの状態確認やリアルタイム読み込みが可能です。

## 依存関係

- Bevy 0.17（`file_watcher` feature有効）
- mlua（Lua統合）
- serde（シリアライゼーション）
- syntect（シンタックスハイライト）
- rmp-serde（MessagePackサポート）
- thiserror（エラーハンドリング）

## ライセンス

このプロジェクトはMIT OR Apache-2.0ライセンスの下で公開されています。

### サードパーティライセンス

このプロジェクトは以下のオープンソースライブラリを使用しています：

#### コアライブラリ

- **Bevy 0.17**: MIT OR Apache-2.0ライセンス
  - BevyはMITライセンスまたはApache-2.0ライセンスのいずれかを選択できます
  - 詳細は[Bevyのライセンス](https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT)を参照してください

- **mlua 0.9**: MITライセンス
  - Lua統合ライブラリ
  - 詳細は[mluaのライセンス](https://github.com/khvzak/mlua/blob/master/LICENSE)を参照してください

- **serde 1.0**: MIT OR Apache-2.0ライセンス
  - シリアライゼーションフレームワーク
  - 詳細は[serdeのライセンス](https://github.com/serde-rs/serde/blob/main/LICENSE-MIT)を参照してください

- **serde_json 1.0**: MIT OR Apache-2.0ライセンス
  - JSONシリアライゼーション
  - 詳細は[serde_jsonのライセンス](https://github.com/serde-rs/json/blob/main/LICENSE-MIT)を参照してください

- **rmp-serde 1.1**: MIT OR Apache-2.0ライセンス
  - MessagePackシリアライゼーション
  - 詳細は[rmp-serdeのライセンス](https://github.com/3Hren/msgpack-rust/blob/master/LICENSE)を参照してください

- **thiserror 1.0**: MIT OR Apache-2.0ライセンス
  - エラーハンドリングライブラリ
  - 詳細は[thiserrorのライセンス](https://github.com/dtolnay/thiserror/blob/master/LICENSE-APACHE)を参照してください

- **anyhow 1.0**: MIT OR Apache-2.0ライセンス
  - エラーハンドリングライブラリ
  - 詳細は[anyhowのライセンス](https://github.com/dtolnay/anyhow/blob/master/LICENSE-APACHE)を参照してください

#### UI・エディタ関連

- **syntect 5.3**: MIT OR Apache-2.0ライセンス
  - シンタックスハイライトライブラリ
  - 詳細は[syntectのライセンス](https://github.com/trishume/syntect/blob/master/LICENSE)を参照してください

- **rfd 0.14**: MIT OR Apache-2.0ライセンス
  - ファイルダイアログライブラリ
  - 詳細は[rfdのライセンス](https://github.com/PolyMeilex/rfd/blob/master/LICENSE)を参照してください

- **arboard 3.3**: MIT OR Apache-2.0ライセンス
  - クリップボード操作ライブラリ
  - 詳細は[arboardのライセンス](https://github.com/1Password/arboard/blob/main/LICENSE)を参照してください

#### ネットワーク・非同期処理

- **reqwest 0.12**: MIT OR Apache-2.0ライセンス
  - HTTPクライアントライブラリ（`ai` feature使用時）
  - 詳細は[reqwestのライセンス](https://github.com/seanmonstar/reqwest/blob/main/LICENSE-MIT)を参照してください

- **tokio 1.0**: MIT OR Apache-2.0ライセンス
  - 非同期ランタイム（`ai` feature使用時）
  - 詳細は[tokioのライセンス](https://github.com/tokio-rs/tokio/blob/master/LICENSE)を参照してください

#### ユーティリティ

- **regex 1.11**: MIT OR Apache-2.0ライセンス
  - 正規表現ライブラリ
  - 詳細は[regexのライセンス](https://github.com/rust-lang/regex/blob/master/LICENSE-MIT)を参照してください

- **dirs 5.0**: MIT OR Apache-2.0ライセンス
  - ディレクトリパス取得ライブラリ
  - 詳細は[dirsのライセンス](https://github.com/dirs-dev/dirs-rs/blob/main/LICENSE-MIT)を参照してください

#### ライセンス表記について

上記のライブラリはすべてMITライセンスまたはApache-2.0ライセンスのいずれか、または両方のデュアルライセンスの下で公開されています。各ライブラリの完全なライセンス条項は、それぞれのGitHubリポジトリまたはcrates.ioページで確認できます。
