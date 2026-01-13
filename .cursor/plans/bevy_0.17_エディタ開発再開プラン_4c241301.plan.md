---
name: Bevy 0.17 エディタ開発再開プラン
overview: 現在の実装状況を確認し、コンパイルエラーの修正と未実装機能の完成を進めます。インスペクターの構文エラー修正、ハイブコーディングエディタのAI統合、エディタ-ランタイム通信の完成、プロジェクト管理機能の実装を行います。
todos:
  - id: fix-inspector-syntax
    content: inspector.rsの構文エラー修正（845行目の閉じ括弧追加）
    status: completed
  - id: fix-warnings
    content: 未使用インポートと変数の警告を修正
    status: completed
    dependencies:
      - fix-inspector-syntax
  - id: ai-integration
    content: ハイブコーディングエディタへのAI統合（OpenAI/Claude API、コード補完、コード生成）
    status: completed
    dependencies:
      - fix-warnings
  - id: complete-communication
    content: エディタ-ランタイム通信の完成（IPC/ネットワーク、リアルタイム同期）
    status: completed
    dependencies:
      - fix-warnings
  - id: complete-project-management
    content: プロジェクト管理機能の完成（作成/開く/保存、シーン保存/読み込み）
    status: completed
    dependencies:
      - fix-warnings
  - id: testing-optimization
    content: 機能テスト、パフォーマンス最適化、エラーハンドリング改善
    status: completed
    dependencies:
      - ai-integration
      - complete-communication
      - complete-project-management
---

# Bevy 0.17 ゲームエンジンエディタ開発再開プラン

## 現在の実装状況

### 完了している機能

- **フェーズ1**: プロジェクト基盤構築（ワークスペース、クレート構造、依存関係）
- **フェーズ2**: 共有ライブラリ（シーン定義、アセット定義、プロトコル）
- **フェーズ3**: ランタイム（Luaスクリプトシステム、ホットリロード機能）
- **フェーズ4**: エディタUI基盤
- シーンビュー（カメラ操作、Gizmo）
- ヒエラルキービュー（Entity階層表示、選択、操作）
- インスペクターパネル（Component表示、Transform編集）
- アセットブラウザー
- スクリプトエディタ
- コードエディタ（基本実装）
- ログパネル
- エディタレイアウト

### 修正・実装が必要な項目

#### 1. コンパイルエラーの修正

- [adbx_editor/src/ui/inspector.rs](adbx_editor/src/ui/inspector.rs): 845行目に`if keyboard_input.pressed(...)`ブロックの閉じ括弧が不足

#### 2. ハイブコーディングエディタのAI統合

- [adbx_editor/src/ui/code_editor.rs](adbx_editor/src/ui/code_editor.rs): AI統合API（OpenAI/Claude）の実装
- コード補完機能
- AIによるコード生成・編集・リファクタリング
- エディタ操作のAI理解可能な形式での記録

#### 3. エディタ-ランタイム通信の完成

- [adbx_editor/src/communication.rs](adbx_editor/src/communication.rs): IPC/ネットワーク通信の実装確認と完成
- リアルタイム同期機能

#### 4. プロジェクト管理機能の完成

- [adbx_editor/src/project.rs](adbx_editor/src/project.rs): プロジェクトの作成/開く/保存機能の完成
- シーンの保存/読み込み機能の確認と完成

#### 5. 警告の修正

- 未使用インポートの削除（`adbx_shared/src/protocol.rs`, `adbx_runtime/src/lua/script_cache.rs`等）

## 実装手順

### ステップ1: コンパイルエラーの修正

1. `inspector.rs`の845行目に不足している閉じ括弧を追加
2. コンパイル確認

### ステップ2: 警告の修正

1. 未使用インポートの削除
2. 未使用変数の修正

### ステップ3: ハイブコーディングエディタのAI統合

1. AI APIクライアントの実装（OpenAI/Claude）
2. コードエディタへのAI機能統合
3. コード補完機能の実装
4. AI操作記録システムの実装

### ステップ4: エディタ-ランタイム通信の完成

1. 通信プロトコルの確認と完成
2. リアルタイム同期機能の実装

### ステップ5: プロジェクト管理機能の完成

1. プロジェクト作成/開く/保存機能の実装
2. シーン保存/読み込み機能の確認と完成

### ステップ6: テストと最適化

1. 各機能の動作確認
2. パフォーマンス最適化
3. エラーハンドリングの改善