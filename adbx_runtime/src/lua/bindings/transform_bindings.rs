use crate::lua::bindings::lua_data_types::TransformData;
use crate::lua::transform_bridge::TransformBridge;
use mlua::{Lua, Result as LuaResult};
use std::sync::Arc;
use std::sync::Mutex;

/// Transform操作のバインディング
pub fn register_transform_bindings(
    lua: &Lua,
    transform_bridge: Arc<Mutex<TransformBridge>>,
) -> LuaResult<()> {
    let globals = lua.globals();

    // Transform操作
    let transform_table = lua.create_table()?;
    transform_table.set(
        "new",
        lua.create_function(|_lua, (x, y, z): (f32, f32, f32)| Ok(TransformData { x, y, z }))?,
    )?;

    // Transformの位置を設定
    let transform_bridge_clone = transform_bridge.clone();
    transform_table.set(
        "set_translation",
        lua.create_function(move |_lua, (entity_id, x, y, z): (u32, f32, f32, f32)| {
            let mut bridge = transform_bridge_clone.lock().unwrap();
            bridge.transform_updates.insert(
                entity_id,
                crate::lua::transform_bridge::TransformUpdate::SetTranslation { x, y, z },
            );
            Ok(())
        })?,
    )?;

    // Transformの位置を移動
    let transform_bridge_clone2 = transform_bridge.clone();
    transform_table.set(
        "translate",
        lua.create_function(move |_lua, (entity_id, x, y, z): (u32, f32, f32, f32)| {
            let mut bridge = transform_bridge_clone2.lock().unwrap();
            bridge.transform_updates.insert(
                entity_id,
                crate::lua::transform_bridge::TransformUpdate::Translate { x, y, z },
            );
            Ok(())
        })?,
    )?;

    // Transformのスケールを設定
    let transform_bridge_clone3 = transform_bridge.clone();
    transform_table.set(
        "set_scale",
        lua.create_function(move |_lua, (entity_id, x, y, z): (u32, f32, f32, f32)| {
            let mut bridge = transform_bridge_clone3.lock().unwrap();
            bridge.transform_updates.insert(
                entity_id,
                crate::lua::transform_bridge::TransformUpdate::SetScale { x, y, z },
            );
            Ok(())
        })?,
    )?;

    globals.set("Transform", transform_table)?;

    Ok(())
}
