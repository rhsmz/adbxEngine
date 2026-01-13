---
name: Bevy 0.17 ゲームエンジンエディタ開発計画
overview: Bevy 0.17用のUnityレベルのゲームエンジンエディタを構築します。エディタとランタイムを分離したクレート構成で、Feathers UI、AI統合ハイブコーディングエディタ、Luaスクリプトシステム、ホットリロード機能を実装します。
todos: []
---

# Bevy 0.17 ゲームエンジンエディタ開発計画

## プロジェクト構造

プロジェクトは以下の3つのクレートで構成されます：

- **`adbx_editor`**: エディタアプリケーション（Feathers UI、ハイブコーディングエディタ）
- **`adbx_runtime`**: ゲームランタイム（Luaスクリプトシステム、ホットリロード）
- **`adbx_shared`**: 共有ライブラリ（データ構造、プロトコル定義）

## 実装フェーズ

### フェーズ1: プロジェクト基盤構築

#### 1.1 ワークスペースとクレート構造の作成

- `Cargo.toml`（ワークスペース設定）
- `adbx_editor/Cargo.toml`（エディタクレート）
- `adbx_runtime/Cargo.toml`（ランタイムクレート）
- `adbx_shared/Cargo.toml`（共有ライブラリ）

#### 1.2 基本依存関係の設定

- Bevy 0.17（`file_watcher` feature有効）
- Feathers UI（Bevy 0.17対応版）
- mlua（Lua統合）
- その他必要な依存関係

### フェーズ2: 共有ライブラリ（`adbx_shared`）

#### 2.1 データ構造定義

- シーン定義（Entity、Component、Hierarchy）
- アセットメタデータ
- エディタ-ランタイム通信プロトコル

#### 2.2 シリアライゼーション

- シーンの保存/読み込み形式（JSON/MessagePack）
- アセット参照の管理

### フェーズ3: ランタイム（`adbx_runtime`）

#### 3.1 Luaスクリプトシステム

- `LuaScript`コンポーネントの定義
- mluaを使用したLua VMの初期化
- Bevy ECS APIのLuaバインディング：
  - Entity操作（spawn、despawn、query）
  - Component操作（get、set、add、remove）
  - Resource操作
  - Event送受信
  - Transform、Sprite、その他主要コンポーネントへのアクセス

#### 3.2 スクリプトホットリロード

- ファイル監視システム（`AssetWatcher`）
- Luaスクリプトの動的再読み込み
- 実行中のスクリプト状態の保持

#### 3.3 アセットホットリロード

- Bevy標準のホットリロード機能の活用
- カスタムアセットタイプのサポート

### フェーズ4: エディタ（`adbx_editor`）

#### 4.1 Feathers UI基盤

- エディタウィンドウレイアウト
- メニューバー、ツールバー
- ドッキング可能なパネルシステム

#### 4.2 シーンビュー

- 3D/2Dシーンのレンダリング
- カメラ操作（Orbit、Pan、Zoom）
- GizmoによるEntity操作

#### 4.3 ヒエラルキービュー

- Entity階層の表示
- Entityの選択、追加、削除
- ドラッグ&ドロップによる親子関係の変更

#### 4.4 インスペクターパネル

- 選択EntityのComponent表示
- Componentプロパティの編集
- カスタムエディタの登録システム

#### 4.5 アセットブラウザー

- アセット一覧表示
- アセットのプレビュー
- アセットのインポート/エクスポート

#### 4.6 ハイブコーディングエディタ

- コードエディタコンポーネント（syntect/egui_editor使用）
- AI統合API（OpenAI/Claude等）
- コード補完機能
- リアルタイムプレビュー
- AIによるコード生成・編集・リファクタリング
- エディタ操作のAI理解可能な形式での記録

#### 4.7 スクリプトエディタ

- Luaスクリプトの編集
- シンタックスハイライト
- エラーハイライト
- Entityへのスクリプトアタッチ機能

#### 4.8 プロジェクト管理

- プロジェクトの作成/開く/保存
- シーンの保存/読み込み
- 設定の管理

### フェーズ5: 統合と最適化

#### 5.1 エディタ-ランタイム通信

- エディタとランタイムの分離実行
- IPCまたはネットワーク通信による連携
- リアルタイム同期

#### 5.2 パフォーマンス最適化

- 大規模シーンでのパフォーマンス改善
- エディタUIの応答性向上
- Luaスクリプト実行の最適化

#### 5.3 エラーハンドリングとロギング

- 包括的なエラーハンドリング
- デバッグログシステム
- ユーザーフレンドリーなエラーメッセージ

## 技術スタック

- **ゲームエンジン**: Bevy 0.17
- **UIフレームワーク**: Feathers（Bevy 0.17内蔵）
- **Lua統合**: mlua
- **シリアライゼーション**: serde（JSON/MessagePack）
- **コードエディタ**: syntect + egui_editor または Monaco Editor統合
- **AI統合**: OpenAI API / Anthropic Claude API
- **ファイル監視**: Bevy AssetWatcher（file_watcher feature）

## 主要ファイル構造

```
adbxEngine/
├── Cargo.toml                    # ワークスペース設定
├── adbx_editor/                  # エディタクレート
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs              # エディタエントリーポイント
│   │   ├── editor_app.rs        # エディタアプリケーション
│   │   ├── ui/
│   │   │   ├── mod.rs
│   │   │   ├── scene_view.rs   # シーンビュー
│   │   │   ├── hierarchy.rs     # ヒエラルキービュー
│   │   │   ├── inspector.rs     # インスペクター
│   │   │   ├── asset_browser.rs # アセットブラウザー
│   │   │   ├── code_editor.rs   # ハイブコーディングエディタ
│   │   │   └── script_editor.rs # スクリプトエディタ
│   │   └── systems/
│   │       ├── mod.rs
│   │       ├── selection.rs     # 選択システム
│   │       └── gizmo.rs         # Gizmo操作
│   └── assets/
├── adbx_runtime/                 # ランタイムクレート
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── lua/
│   │   │   ├── mod.rs
│   │   │   ├── vm.rs            # Lua VM管理
│   │   │   ├── bindings.rs      # Bevy APIバインディング
│   │   │   └── component.rs     # Component操作
│   │   ├── hot_reload/
│   │   │   ├── mod.rs
│   │   │   ├── asset_watcher.rs # アセット監視
│   │   │   └── script_reload.rs # スクリプト再読み込み
│   │   └── plugin.rs            # Bevyプラグイン
│   └── assets/
└── adbx_shared/                  # 共有ライブラリ
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── scene.rs             # シーン定義
        ├── asset.rs              # アセット定義
        └── protocol.rs           # 通信プロトコル
```

## 実装の優先順位

1. **最優先**: プロジェクト基盤、ランタイムのLua統合、基本的なエディタUI
2. **高優先度**: シーンビュー、ヒエラルキー、インスペクター、ホットリロード
3. **中優先度**: ハイブコーディングエディタ、アセットブラウザー
4. **低優先度**: 高度なGizmo操作、カスタムエディタ拡張