# データフロー図

Adbx Engine Editorの主要なデータフローを説明します。

## エディタ-ランタイム通信フロー

```kroki-plantuml
@startuml
participant Editor
participant Communication
participant Runtime

Editor -> Communication: EditorMessage
Communication -> Runtime: TCP / mpsc
Runtime -> Communication: RuntimeMessage
Communication -> Editor: TCP / mpsc
@enduml
```

## シーン読み込み/保存フロー

```kroki-plantuml
@startuml
participant Editor
participant SceneManager
participant FileSystem
participant Runtime

Editor -> SceneManager: load_scene()
SceneManager -> FileSystem: read file
FileSystem -> SceneManager: SceneData
SceneManager -> Runtime: LoadScene message
Runtime -> Runtime: Create SceneEntity components
Runtime -> Editor: SceneLoaded message

note right: SceneEntity, LuaScript, LuaScriptState\ncomponents are shared via adbx_shared crate
@enduml
```

## ホットリロードフロー

### 標準アセットのホットリロード

```kroki-plantuml
@startuml
title Standard Asset Hot Reload Flow

participant BevyAssetWatcher as "Bevy AssetWatcher"
participant FileSystem
participant AssetServer as "Bevy AssetServer"
participant Runtime
participant Editor

== File Change Detection ==
BevyAssetWatcher -> FileSystem: Monitor file changes
FileSystem -> BevyAssetWatcher: File modified event

== Asset Reload ==
BevyAssetWatcher -> AssetServer: AssetEvent::Modified
AssetServer -> AssetServer: Reload asset
AssetServer -> Runtime: Asset available

== Editor Notification ==
Runtime -> Editor: Asset reloaded notification
Editor -> Editor: Update UI (AssetBrowser, etc.)

note right: Requires file_watcher feature in Cargo.toml
@enduml
```

### カスタムアセットのホットリロード

```kroki-plantuml
@startuml
title Custom Asset Hot Reload Flow

participant FileWatcher
participant CustomAssetRegistry as "CustomAssetRegistry"
participant AssetLoader as "CustomAssetLoader"
participant Runtime
participant Editor

== File Change Detection ==
FileWatcher -> FileWatcher: Detect file change (*.config, *.cfg, etc.)

== Loader Resolution ==
FileWatcher -> CustomAssetRegistry: get_loader_by_extension()
CustomAssetRegistry -> CustomAssetRegistry: Find registered loader
CustomAssetRegistry -> FileWatcher: Return loader (e.g., ConfigFileLoader)

== Asset Loading ==
FileWatcher -> AssetLoader: load_asset()
AssetLoader -> FileSystem: Read file content
FileSystem -> AssetLoader: File content
AssetLoader -> AssetLoader: Parse content
AssetLoader -> FileWatcher: Parsed asset data

== Runtime Update ==
FileWatcher -> Runtime: Hot reload complete
Runtime -> Editor: Custom asset reloaded

note right: Supports extensions: config, cfg\nCustom loaders can be registered via CustomAssetRegistry
@enduml
```

### アセットタイプ別の処理フロー

```kroki-plantuml
@startuml
title Asset Type Processing Flow

start
:File change detected;

if (Is standard Bevy asset?) then (yes)
    :Bevy AssetWatcher handles reload;
    :Standard asset types (Image, Mesh, etc.);
else (no)
    if (Custom loader registered?) then (yes)
        :Use CustomAssetLoader;
        :Process via registered loader;
        note right: ConfigFileLoader for .config/.cfg
    else (no)
        :No loader available;
        :Log debug message;
    endif
endif

:Notify editor of changes;
:Update UI components;
end
@enduml
```

## アプリケーション初期化フロー

```kroki-plantuml
@startuml
participant Main
participant AppInitialization
participant SystemRegistration
participant EditorPlugin
participant RuntimePlugin

Main -> AppInitialization: initialize_app()
AppInitialization -> AppInitialization: init resources
AppInitialization -> RuntimePlugin: add_plugins()
AppInitialization -> EditorPlugin: add_plugins()
Main -> SystemRegistration: register_systems()
SystemRegistration -> SystemRegistration: add_systems()
Main -> Main: app.run()
@enduml
```

## 関連ドキュメント

- [システム概要](system-overview.md)
- [コンポーネント図](component-diagram.md)
