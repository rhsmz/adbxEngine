use crate::lua::commands_bridge::LuaCommandsBridge;
use mlua::{Lua, Result as LuaResult};
use std::sync::Arc;
use std::sync::Mutex;

/// Entity操作のバインディング
pub fn register_entity_bindings(
    lua: &Lua,
    commands_bridge: Arc<Mutex<LuaCommandsBridge>>,
) -> LuaResult<()> {
    let globals = lua.globals();

    let bridge_clone = commands_bridge.clone();
    // Entity操作
    let entity_table = lua.create_table()?;
    entity_table.set(
        "spawn",
        lua.create_function(move |_lua, _: ()| {
            let mut bridge = bridge_clone.lock().unwrap();
            bridge
                .command_queue
                .push_back(crate::lua::commands_bridge::LuaCommand::SpawnEntity {
                    components: Vec::new(),
                });
            Ok(0u32) // プレースホルダー（実際のEntity IDは後で返す）
        })?,
    )?;

    let bridge_clone2 = commands_bridge.clone();
    entity_table.set(
        "despawn",
        lua.create_function(move |_lua, entity_id: u32| {
            let mut bridge = bridge_clone2.lock().unwrap();
            bridge
                .command_queue
                .push_back(crate::lua::commands_bridge::LuaCommand::DespawnEntity { entity_id });
            Ok(())
        })?,
    )?;
    globals.set("Entity", entity_table)?;

    Ok(())
}
