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
    + expanded_entities: HashSet<Entity>
    + scroll_offset: f32
    + drag_state: HierarchyDragState
}

class InspectorPanel {
    + content: InspectorContent
    + input_state: InspectorInputState
    + registry: ComponentEditorRegistry
    ---
    - last_update_time: f64 (unused)
    - update_throttle: f64 (unused)
    - pending_update: bool (unused)
}

class AssetBrowser {
    + assets: Vec<AssetItem>
    + current_path: PathBuf
    + scroll_offset: f32
    + selected_asset: Option<AssetId>
}

class CodeEditor {
    + content: String
    + cursor_position: (usize, usize)
    + ai_integration: AiIntegration
    + syntax_highlighting: bool
}

class ScriptEditor {
    + script_content: String
    + script_errors: Vec<ScriptError>
    ---
    - entity: Option<Entity> (unused)
}

class LogPanel {
    + logs: VecDeque<LogEntry>
    ---
    - max_logs: usize (unused)
    - is_visible: bool (unused)
}

class SceneView {
    ---
    - camera_entity: Option<Entity> (unused)
}

class RealtimePreview {
    + content: PreviewContent
    ---
    - preview_type: PreviewType (unused in PreviewContent)
}

class RenameDialog {
    + request: RenameDialogRequest
    ---
    - old_path: PathBuf (unused in result)
    - new_path: PathBuf (unused in result)
}

EditorApp --> HierarchyView
EditorApp --> InspectorPanel
EditorApp --> AssetBrowser
EditorApp --> CodeEditor
EditorApp --> ScriptEditor
EditorApp --> LogPanel
EditorApp --> SceneView
EditorApp --> RealtimePreview
EditorApp --> RenameDialog

note right of InspectorPanel : 遅延更新フィールドは未使用
note right of LogPanel : max_logs, is_visible は未使用
note right of ScriptEditor : entity フィールドは未使用
note right of SceneView : camera_entity は未使用
@enduml
```

### UIパネルの相互作用フロー

```kroki-plantuml
@startuml
title UI Panel Interactions

actor User
participant HierarchyView
participant InspectorPanel
participant AssetBrowser
participant CodeEditor
participant SceneView
participant LogPanel

== Entity Selection Flow ==
User -> HierarchyView: Click entity
HierarchyView -> InspectorPanel: Entity selected
InspectorPanel -> InspectorPanel: Load component editors
InspectorPanel -> User: Show editable fields

== Asset Management Flow ==
User -> AssetBrowser: Select asset
AssetBrowser -> CodeEditor: Open for editing
CodeEditor -> LogPanel: Report syntax errors

== Scene Editing Flow ==
User -> SceneView: Interact with gizmos
SceneView -> HierarchyView: Update selection
HierarchyView -> InspectorPanel: Update entity data

== Error Reporting Flow ==
CodeEditor -> LogPanel: Validation errors
ScriptEditor -> LogPanel: Script errors
BuildSystem -> LogPanel: Build results

note right: All UI panels communicate through\nECS resources and events
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
