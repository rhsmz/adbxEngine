use crate::communication::*;
use crate::hot_reload::{asset_watcher::*, custom_asset_registry::*, script_reload::*};
use crate::lua::{
    commands_bridge::*, component_bridge::*, event_bridge::*, resource_bridge::*, script_cache::*,
    transform_bridge::*, vm::*,
};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

/// Adbx Runtimeプラグイン
pub struct AdbxRuntimePlugin;

impl Plugin for AdbxRuntimePlugin {
    fn build(&self, app: &mut App) {
        // Commandsブリッジ、Transformブリッジ、Resourceブリッジ、Eventブリッジ、Componentブリッジを初期化
        let commands_bridge = Arc::new(Mutex::new(LuaCommandsBridge::default()));
        let transform_bridge = Arc::new(Mutex::new(TransformBridge::default()));
        let resource_bridge = Arc::new(Mutex::new(ResourceBridge::default()));
        let event_bridge = Arc::new(Mutex::new(EventBridge::default()));
        let component_bridge = Arc::new(Mutex::new(ComponentBridge::default()));

        app
            // リフレクション登録
            .register_type::<adbx_shared::components::SceneEntity>()
            .register_type::<adbx_shared::components::LuaScript>()
            .register_type::<adbx_shared::components::LuaScriptState>()
            // CustomEventをMessageとして登録
            .add_message::<CustomEvent>()
            // Commandsブリッジ
            .insert_resource(LuaCommandsBridge::default())
            // Transformブリッジ
            .insert_resource(TransformBridge::default())
            // Resourceブリッジ
            .insert_resource(ResourceBridge::default())
            // Eventブリッジ
            .insert_resource(EventBridge::default())
            // Componentブリッジ
            .insert_resource(ComponentBridge::default())
            // Lua VMリソース（ブリッジを渡す）
            .insert_resource(LuaVm::new(
                commands_bridge.clone(),
                transform_bridge.clone(),
                resource_bridge.clone(),
                event_bridge.clone(),
                component_bridge.clone(),
            ))
            // スクリプトキャッシュ
            .init_resource::<ScriptCache>()
            // スクリプト監視リソース
            .init_resource::<ScriptWatcher>()
            // カスタムアセットレジストリ
            .init_resource::<CustomAssetRegistry>()
            // デフォルトのカスタムアセットローダーを登録
            .add_systems(
                Startup,
                crate::hot_reload::custom_asset_registry::register_default_custom_asset_loaders,
            )
            // ランタイム-エディタ通信リソース
            .init_resource::<RuntimeEditorCommunication>()
            // エディタからのメッセージを処理するシステム
            .add_systems(Update, handle_editor_messages)
            // Luaスクリプトシステム
            .add_systems(Update, load_lua_scripts)
            .add_systems(Update, execute_lua_scripts)
            // Luaコマンド実行システム
            .add_systems(Update, execute_lua_commands)
            // Transform更新システム
            .add_systems(Update, apply_transform_updates)
            // Resource操作システム
            .add_systems(Update, process_resource_operations)
            // Event操作システム
            .add_systems(Update, (process_event_operations, receive_events))
            // Component操作システム
            .add_systems(Update, process_component_operations)
            // ホットリロードシステム
            .add_systems(Update, (watch_and_reload_scripts, handle_asset_changes));

        // アセット監視のセットアップ
        setup_asset_watcher(app);
    }
}