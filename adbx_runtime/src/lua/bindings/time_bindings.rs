use mlua::{Lua, Result as LuaResult};

/// Time操作のバインディング
pub fn register_time_bindings(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    // Time操作（実際の値はexecute_lua_scriptsで設定される）
    let time_table = lua.create_table()?;
    // delta_timeはグローバル変数として設定されるため、ここでは関数を提供しない
    globals.set("Time", time_table)?;

    Ok(())
}
