# コンポーネント図

Adbx Engine Editorの主要コンポーネントの関係を説明します。

## UIコンポーネント階層

```kroki-plantuml
@startuml
class EditorApp {
    + UI Components
    + Systems
}

class HierarchyView {
    + expanded_entities
    + scroll_offset
}

class InspectorPanel {
    + selected_entity
    + update_throttle
    + last_update_time
    + pending_update
}

class AssetBrowser {
    + current_path
    + scroll_offset
}

class CodeEditor {
    + content
    + cursor_position
}

class ScriptEditor {
    + script_path
    + errors
}

class LogPanel {
    + logs
    + max_logs
    + is_visible
}

EditorApp --> HierarchyView
EditorApp --> InspectorPanel
EditorApp --> AssetBrowser
EditorApp --> CodeEditor
EditorApp --> ScriptEditor
EditorApp --> LogPanel
@enduml
```

## システム依存関係

```kroki-plantuml
@startuml
class Selection {
    + selected_entities
}

class SceneManager {
    + entity_id_map
    + current_scene_name
}

class OperationRecorder {
    + operations
    + max_history
}

class RealtimeSync {
    + sync_entity_selection()
    + sync_transform_changes()
}

class BuildGameSystem {
    + build_game()
    + BuildGameRequest
    + BuildProgress
}

class ComponentEditorRegistry {
    + register()
    + get()
}

Selection --> SceneManager
SceneManager --> OperationRecorder
RealtimeSync --> Selection
RealtimeSync --> SceneManager
BuildGameSystem --> SceneManager
ComponentEditorRegistry --> InspectorPanel
@enduml
```

## リソース初期化フロー

```kroki-plantuml
@startuml
participant AppInitialization
participant EditorApp
participant Systems
participant UI
participant Communication

AppInitialization -> EditorApp: init_resource()
AppInitialization -> Systems: init_resource()
AppInitialization -> UI: init_resource()
AppInitialization -> Communication: init_resource()
AppInitialization -> AppInitialization: init_resource(BuildGameMenuRequest)
@enduml
```

## 関連ドキュメント

- [システム概要](system-overview.md)
- [データフロー図](data-flow.md)
