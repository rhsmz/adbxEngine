use crate::lua::bindings::lua_data_types::Vec3Data;
use mlua::{Lua, Result as LuaResult};

/// Vec3操作のバインディング
pub fn register_vec3_bindings(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    // Vec3操作
    let vec3_table = lua.create_table()?;
    vec3_table.set(
        "new",
        lua.create_function(|_lua, (x, y, z): (f32, f32, f32)| Ok(Vec3Data { x, y, z }))?,
    )?;
    vec3_table.set(
        "zero",
        lua.create_function(|_lua, _: ()| {
            Ok(Vec3Data {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            })
        })?,
    )?;
    globals.set("Vec3", vec3_table)?;

    Ok(())
}
