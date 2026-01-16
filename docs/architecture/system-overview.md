# システム概要

Adbx Engine Editorのシステムアーキテクチャの概要です。

## アーキテクチャ図

### 全体構成

```kroki-plantuml
@startuml
!define RECTANGLE class

package "adbx_editor" {
    RECTANGLE EditorApp {
        + UI Components
        + ECS Systems
        + Communication
    }

    package "UI Components" as UI {
        RECTANGLE InspectorPanel
        RECTANGLE HierarchyView
        RECTANGLE AssetBrowser
        RECTANGLE LogPanel
        RECTANGLE ScriptEditor
        RECTANGLE CodeEditor
    }

    package "ECS Systems" as Systems {
        RECTANGLE SelectionSystem
        RECTANGLE SceneManagement
        RECTANGLE GizmoSystem
        RECTANGLE MenuSystem
        RECTANGLE RealtimeSync
        RECTANGLE BuildSystem
    }
}

package "adbx_runtime" {
    RECTANGLE RuntimeApp {
        + Lua VM
        + Hot Reload
        + Communication
    }

    package "Hot Reload" as HotReload {
        RECTANGLE AssetWatcher
        RECTANGLE CustomAssetRegistry
        RECTANGLE ScriptReload
    }
}

package "adbx_shared" {
    RECTANGLE Protocol {
        + EditorMessage
        + RuntimeMessage
    }
    RECTANGLE Scene {
        + SceneData
        + EntityData
    }
    RECTANGLE Components {
        + SceneEntity
        + LuaScript
        + LuaScriptState
    }
    RECTANGLE Asset {
        + AssetType
        + AssetMetadata
    }
    RECTANGLE Error {
        + AdbxError
        + ErrorSeverity
    }
}

EditorApp --> Protocol
EditorApp --> Scene
EditorApp --> Asset
EditorApp --> Error
EditorApp --> RuntimeApp : TCP / mpsc

UI --> Systems : ECS Resources
Systems --> UI : Updates

RuntimeApp --> Protocol
RuntimeApp --> Scene
RuntimeApp --> Asset
RuntimeApp --> Error
RuntimeApp --> EditorApp : TCP / mpsc

HotReload --> AssetWatcher : Monitors file changes
HotReload --> CustomAssetRegistry : Manages custom loaders

@enduml
```

### ECSシステムの詳細フロー

```kroki-plantuml
@startuml
title ECS System Flow in Editor

actor User
participant EditorApp
participant UI as "UI Systems"
participant Core as "Core Systems"
participant Runtime as "Runtime (adbx_runtime)"

== Startup Phase ==
EditorApp -> EditorApp: Initialize Resources
EditorApp -> UI: Setup UI Components
UI -> Core: Register Component Editors

== Update Loop ==
User -> UI: User Interaction
UI -> Core: Process Input Events

alt Selection System
    UI -> Core: Entity Selection
    Core -> UI: Update Inspector
end

alt Scene Management
    UI -> Core: Scene Operations
    Core -> Runtime: Sync Scene Data
end

alt Hot Reload
    Runtime -> Runtime: Monitor File Changes
    Runtime -> Core: Asset Reload Events
    Core -> UI: Update Asset Browser
end

alt Build System
    UI -> Core: Build Request
    Core -> Core: Package Game
    Core -> UI: Build Progress/Result
end

== Communication ==
EditorApp <-> Runtime: Real-time Sync
Runtime -> EditorApp: Lua Script Updates

@enduml
```

### リソースとシステムの依存関係

```kroki-plantuml
@startuml
title Resource Dependencies in ECS

class EditorApp {
    +editor_state: EditorState
    +project: Project
    +selection: Selection
}

class InspectorPanel {
    +content: InspectorContent
    +input_state: InspectorInputState
}

class SceneManager {
    +current_scene: Option<Entity>
    +scene_data: SceneData
}

class AssetBrowser {
    +assets: Vec<AssetItem>
    +selected_asset: Option<AssetId>
}

class LogPanel {
    +logs: VecDeque<LogEntry>
    +max_logs: usize
}

class ScriptEditor {
    +current_script: Option<PathBuf>
    +script_content: String
}

EditorApp --> InspectorPanel : updates
EditorApp --> SceneManager : manages
EditorApp --> AssetBrowser : displays
EditorApp --> LogPanel : logs to
EditorApp --> ScriptEditor : edits

InspectorPanel --> SceneManager : reads entity data
AssetBrowser --> SceneManager : provides assets
ScriptEditor --> LogPanel : reports errors

note right of LogPanel
    max_logs field unused
    in current implementation
end note

note right of InspectorPanel
    Delayed update fields
    (last_update_time, update_throttle, pending_update)
    currently unused
end note

@enduml
```

## コンポーネント概要

### adbx_editor

エディタアプリケーション。以下の主要コンポーネントを含みます：

- **UI Components**: ヒエラルキービュー、インスペクターパネル、アセットブラウザー、コードエディタなど
- **Systems**: 選択システム、シーン管理、操作記録、リアルタイム同期など
- **Communication**: ランタイムとの通信（TCP/mpsc）

### adbx_runtime

ゲームランタイム。以下の主要コンポーネントを含みます：

- **Lua VM**: Luaスクリプトの実行環境
- **Hot Reload**: アセットとスクリプトのホットリロード機能
- **Communication**: エディタとの通信（TCP/mpsc）

### adbx_shared

共有ライブラリ。以下の主要コンポーネントを含みます：

- **Protocol**: エディタ-ランタイム通信プロトコル
- **Scene**: シーンデータ定義
- **Asset**: アセット定義
- **Error**: エラーハンドリング

## 通信方式

### 同一プロセスモード

エディタとランタイムが同じプロセスで実行される場合、`mpsc`チャネルを使用してメッセージを送受信します。

### 分離実行モード

エディタとランタイムが別々のプロセスで実行される場合、TCP通信を使用してメッセージを送受信します。

## 関連ドキュメント

- [データフロー図](data-flow.md)
- [コンポーネント図](component-diagram.md)
- [パフォーマンス最適化](performance.md)
