# 設計書: adbxEngine

この設計書は、既存の実装とドキュメントを基に、adbxEngineの全体設計を再整理したものです。
設計の意図、責務分担、通信・データフロー、非機能要件を明確化し、今後の実装・拡張の指針を示します。

## 1. 目的とスコープ

- **目的**: エディタとランタイムを疎結合に保ちつつ、開発・実行・ホットリロードを一貫して支える構造を定義する。
- **対象**: `adbx_editor`, `adbx_runtime`, `adbx_shared` の3クレートと、それらの相互通信・データ形式。
- **非対象**: UIの詳細レイアウト、個別のシステム実装の内部アルゴリズム。

## 2. 全体像

```kroki-plantuml
@startuml
!define RECTANGLE class

package "adbx_editor" {
  RECTANGLE EditorApp
  RECTANGLE UISystems
  RECTANGLE CoreSystems
  RECTANGLE EditorComm
}

package "adbx_runtime" {
  RECTANGLE RuntimeApp
  RECTANGLE LuaVM
  RECTANGLE HotReload
  RECTANGLE RuntimeComm
}

package "adbx_shared" {
  RECTANGLE Protocol
  RECTANGLE SceneData
  RECTANGLE AssetSchema
  RECTANGLE ErrorTypes
}

EditorApp --> UISystems
EditorApp --> CoreSystems
EditorComm --> Protocol
RuntimeComm --> Protocol

EditorComm <--> RuntimeComm : TCP / mpsc

EditorApp --> SceneData
RuntimeApp --> SceneData
EditorApp --> AssetSchema
RuntimeApp --> AssetSchema
EditorApp --> ErrorTypes
RuntimeApp --> ErrorTypes

RuntimeApp --> LuaVM
RuntimeApp --> HotReload
@enduml
```

## 3. クレート責務

### 3.1 `adbx_shared` (コアプロトコル)

- **役割**: Editor/Runtime共通のデータ定義と通信プロトコルの唯一の正。
- **含むもの**: `SceneData`, `AssetMetadata`, `EditorMessage`, `RuntimeMessage`, `Error` 系列。
- **制約**: `adbx_editor` / `adbx_runtime` への依存は禁止。

### 3.2 `adbx_runtime` (ゲーム実行)

- **役割**: ゲームループ、Lua実行、アセット/スクリプトのホットリロード。
- **Lua**: ECSとの橋渡しを提供し、安全なLua APIを維持。
- **Hot Reload**: `AssetWatcher` によりファイル変更を監視し、再読み込みを通知。

### 3.3 `adbx_editor` (ツール/エディタ)

- **役割**: シーン編集、アセット管理、コード編集のGUIを提供。
- **UI**: Bevy UI / Eguiの即時モードに合わせて状態をResourceとして保持。
- **通信**: Runtimeを制御するクライアントとして振る舞う。

## 4. 通信設計

- **プロトコルの定義場所**: `adbx_shared/src/protocol.rs` に集約。
- **同一プロセス**: `mpsc` でメッセージを双方向送受信。
- **分離プロセス**: TCPで同期し、エディタ/ランタイムの責務境界を維持。

## 5. データ設計

### 5.1 シーン

- **永続化**: `serde` + `rmp-serde` (MessagePack) を標準形式。
- **デバッグ用途**: 必要に応じて `serde_json` を利用。

### 5.2 アセット

- **メタデータ**: `AssetMetadata` を基準に共有。
- **カスタムローダー**: `CustomAssetRegistry` によって拡張可能。

## 6. 実行フロー (要点)

- **起動**: `EditorApp` がリソース初期化 -> UI構築 -> Runtime接続。
- **編集**: UI入力 -> CoreSystems -> SceneData更新 -> Runtime同期。
- **ホットリロード**: Runtime監視 -> 変更検知 -> Editorへ通知 -> UI反映。

詳細なフローは `docs/architecture/data-flow.md` を参照。

## 7. 非機能要件

- **拡張性**: 共有データは `adbx_shared` に集約し、依存方向を単一化。
- **性能**: UIの重い処理はSystemsで実行し、描画は軽量化。
- **安全性**: Lua境界は `anyhow` を用いたエラーハンドリングを徹底。

## 8. 参照ドキュメント

- `docs/architecture/system-overview.md` (構成図とECS詳細フロー)
- `docs/architecture/data-flow.md` (通信/保存/ホットリロードの詳細)
- `docs/architecture/component-diagram.md` (UI・システム依存関係)
- `docs/architecture/performance.md` (性能最適化方針)
- `docs/architecture/diagrams.html` (Kroki統合ビュー)