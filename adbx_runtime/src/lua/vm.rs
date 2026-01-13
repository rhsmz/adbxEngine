use crate::lua::bindings::register_bevy_bindings;
use crate::lua::commands_bridge::LuaCommandsBridge;
use crate::lua::component::{LuaScript, LuaScriptState};
use crate::lua::component_bridge::ComponentBridge;
use crate::lua::event_bridge::EventBridge;
use crate::lua::resource_bridge::ResourceBridge;
use crate::lua::script_cache::ScriptCache;
use crate::lua::transform_bridge::TransformBridge;
use bevy::prelude::*;
use mlua::{Lua, Result as LuaResult};
use std::sync::{Arc, Mutex};

/// Lua VMリソース（スレッドセーフ）
#[derive(Resource, Clone)]
pub struct LuaVm {
    pub lua: Arc<Mutex<Lua>>,
}

impl LuaVm {
    pub fn new(
        commands_bridge: Arc<Mutex<LuaCommandsBridge>>,
        transform_bridge: Arc<Mutex<TransformBridge>>,
        resource_bridge: Arc<Mutex<ResourceBridge>>,
        event_bridge: Arc<Mutex<EventBridge>>,
        component_bridge: Arc<Mutex<ComponentBridge>>,
    ) -> Self {
        let lua = Lua::new();
        register_bevy_bindings(
            &lua,
            commands_bridge,
            transform_bridge,
            resource_bridge,
            event_bridge,
            component_bridge,
        )
        .expect("Failed to register Bevy bindings");
        Self {
            lua: Arc::new(Mutex::new(lua)),
        }
    }
}

/// Luaスクリプトを読み込んで実行するシステム
pub fn load_lua_scripts(
    mut commands: Commands,
    query: Query<Entity, Added<LuaScript>>,
    script_query: Query<&LuaScript>,
    lua_vm: Res<LuaVm>,
) {
    for entity in query.iter() {
        if let Ok(script) = script_query.get(entity) {
            // LuaScriptStateコンポーネントを追加
            let lua = lua_vm.lua.lock().unwrap();
            let state = match load_and_execute_script(&lua, entity, &script.script_content) {
                Ok(_) => LuaScriptState::Loaded,
                Err(e) => {
                    bevy::log::error!("Failed to load Lua script for entity {:?}: {}", entity, e);
                    LuaScriptState::Error
                }
            };
            drop(lua);
            commands.entity(entity).insert(state);
        }
    }
}

/// Luaスクリプトを実行するシステム（最適化版）
/// スクリプトはキャッシュされ、初回のみコンパイルされます
/// 実行頻度の制限と不要な実行のスキップを実装
pub fn execute_lua_scripts(
    mut query: Query<(Entity, &LuaScript, &mut LuaScriptState)>,
    lua_vm: Res<LuaVm>,
    mut script_cache: ResMut<ScriptCache>,
    time: Res<Time>,
) {
    let lua = lua_vm.lua.lock().unwrap();
    let current_time = time.elapsed_secs_f64() as f64;

    for (entity, script, mut state) in query.iter_mut() {
        if *state == LuaScriptState::Error {
            continue;
        }

        // 実行をスキップするかどうかをチェック
        if let Some(&should_skip) = script_cache.skip_execution.get(&entity) {
            if should_skip {
                continue; // update関数がない場合は実行をスキップ
            }
        }

        // 実行頻度の制限をチェック
        if let Some(&last_execution) = script_cache.execution_timestamps.get(&entity) {
            let config = script_cache
                .execution_configs
                .get(&entity)
                .copied()
                .unwrap_or_default();
            let min_interval = 1.0 / config.max_executions_per_second;

            if (current_time - last_execution) < min_interval {
                continue; // 実行頻度制限によりスキップ
            }
        }

        *state = LuaScriptState::Running;

        // スクリプトをキャッシュから取得またはコンパイル
        let result = script_cache.get_or_compile(entity, &script.script_content, &lua);

        match result {
            Ok(Some(execution_state)) => {
                // update関数があれば呼び出す
                let update_result = match execution_state {
                    crate::lua::script_cache::ScriptExecutionState::HasUpdate => {
                        // 実行時刻を更新
                        script_cache
                            .execution_timestamps
                            .insert(entity, current_time);

                        lua.scope(|_scope| {
                            let globals = lua.globals();
                            globals.set("entity_id", entity.index())?;
                            globals.set("delta_time", time.delta_secs())?;

                            // update関数を取得して呼び出す
                            if let Ok(update_fn) = globals.get::<_, mlua::Function>("update") {
                                update_fn.call::<_, ()>(())?;
                            }

                            Ok::<(), mlua::Error>(())
                        })
                    }
                    crate::lua::script_cache::ScriptExecutionState::NoUpdate => {
                        // update関数がない場合は実行をスキップするようにマーク
                        script_cache.skip_execution.insert(entity, true);
                        Ok(())
                    }
                };

                match update_result {
                    Ok(_) => {
                        *state = LuaScriptState::Loaded;
                    }
                    Err(e) => {
                        bevy::log::error!("Lua script update error for entity {:?}: {}", entity, e);
                        *state = LuaScriptState::Error;
                    }
                }
            }
            Ok(None) => {
                // スクリプトが空またはコンパイルできない
                *state = LuaScriptState::Error;
            }
            Err(e) => {
                bevy::log::error!(
                    "Failed to compile Lua script for entity {:?}: {}",
                    entity,
                    e
                );
                *state = LuaScriptState::Error;
            }
        }
    }
}

fn load_and_execute_script(lua: &Lua, entity: Entity, script_content: &str) -> LuaResult<()> {
    lua.scope(|_scope| {
        let globals = lua.globals();
        globals.set("entity_id", entity.index())?;
        lua.load(script_content).exec()?;
        Ok(())
    })
}
