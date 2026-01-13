use mlua::{Lua, Result as LuaResult};

/// Log操作のバインディング
pub fn register_log_bindings(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    // Log操作
    let log_table = lua.create_table()?;
    log_table.set(
        "info",
        lua.create_function(|_lua, msg: String| {
            bevy::log::info!("[Lua] {}", msg);
            Ok(())
        })?,
    )?;
    log_table.set(
        "warn",
        lua.create_function(|_lua, msg: String| {
            bevy::log::warn!("[Lua] {}", msg);
            Ok(())
        })?,
    )?;
    log_table.set(
        "error",
        lua.create_function(|_lua, msg: String| {
            bevy::log::error!("[Lua] {}", msg);
            Ok(())
        })?,
    )?;
    globals.set("Log", log_table)?;

    Ok(())
}
