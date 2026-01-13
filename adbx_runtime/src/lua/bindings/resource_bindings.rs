use crate::lua::resource_bridge::ResourceBridge;
use mlua::{Lua, Result as LuaResult};
use std::sync::Arc;
use std::sync::Mutex;

/// Resource操作のバインディング
pub fn register_resource_bindings(
    lua: &Lua,
    resource_bridge: Arc<Mutex<ResourceBridge>>,
) -> LuaResult<()> {
    let globals = lua.globals();

    // Resource操作
    let resource_table = lua.create_table()?;

    // Resource::get
    let resource_bridge_clone = resource_bridge.clone();
    resource_table.set(
        "get",
        lua.create_function(move |_lua, resource_name: String| {
            let mut bridge = resource_bridge_clone.lock().unwrap();
            bridge.resource_get_requests.push(resource_name.clone());
            Ok(())
        })?,
    )?;

    // Resource::get_value（取得した値を返す）
    let resource_bridge_clone2 = resource_bridge.clone();
    resource_table.set(
        "get_value",
        lua.create_function(move |_lua, resource_name: String| {
            let bridge = resource_bridge_clone2.lock().unwrap();
            if let Some(value) = bridge.resource_values.get(&resource_name) {
                // serde_json::ValueをLuaのテーブルに変換
                let lua_value = match value {
                    serde_json::Value::Object(map) => {
                        let lua_table = _lua.create_table()?;
                        for (k, v) in map {
                            let lua_v = match v {
                                serde_json::Value::Number(n) => {
                                    if let Some(f) = n.as_f64() {
                                        mlua::Value::Number(f)
                                    } else {
                                        mlua::Value::String(_lua.create_string(&n.to_string())?)
                                    }
                                }
                                serde_json::Value::String(s) => {
                                    mlua::Value::String(_lua.create_string(s)?)
                                }
                                serde_json::Value::Bool(b) => mlua::Value::Boolean(*b),
                                _ => mlua::Value::String(_lua.create_string(&v.to_string())?),
                            };
                            lua_table.set(k.as_str(), lua_v)?;
                        }
                        mlua::Value::Table(lua_table)
                    }
                    serde_json::Value::Number(n) => {
                        if let Some(f) = n.as_f64() {
                            mlua::Value::Number(f)
                        } else {
                            mlua::Value::Nil
                        }
                    }
                    serde_json::Value::String(s) => mlua::Value::String(_lua.create_string(s)?),
                    serde_json::Value::Bool(b) => mlua::Value::Boolean(*b),
                    _ => mlua::Value::Nil,
                };
                Ok(lua_value)
            } else {
                Ok(mlua::Value::Nil)
            }
        })?,
    )?;

    // Resource::set
    let resource_bridge_clone3 = resource_bridge.clone();
    resource_table.set(
        "set",
        lua.create_function(move |_lua, (resource_name, value): (String, mlua::Value)| {
            let mut bridge = resource_bridge_clone3.lock().unwrap();
            // mlua::Valueをserde_json::Valueに変換
            let json_value = match value {
                mlua::Value::Nil => serde_json::Value::Null,
                mlua::Value::Boolean(b) => serde_json::Value::Bool(b),
                mlua::Value::Integer(i) => serde_json::Value::Number(i.into()),
                mlua::Value::Number(n) => serde_json::Value::Number(
                    serde_json::Number::from_f64(n).unwrap_or(serde_json::Number::from(0)),
                ),
                mlua::Value::String(s) => serde_json::Value::String(s.to_str()?.to_string()),
                mlua::Value::Table(t) => {
                    let mut map = serde_json::Map::new();
                    for pair in t.pairs::<mlua::Value, mlua::Value>() {
                        let (k, v) = pair?;
                        let key = match k {
                            mlua::Value::String(s) => s.to_str()?.to_string(),
                            _ => format!("{:?}", k),
                        };
                        let value = match v {
                            mlua::Value::Nil => serde_json::Value::Null,
                            mlua::Value::Boolean(b) => serde_json::Value::Bool(b),
                            mlua::Value::Integer(i) => serde_json::Value::Number(i.into()),
                            mlua::Value::Number(n) => serde_json::Value::Number(
                                serde_json::Number::from_f64(n)
                                    .unwrap_or(serde_json::Number::from(0)),
                            ),
                            mlua::Value::String(s) => {
                                serde_json::Value::String(s.to_str()?.to_string())
                            }
                            _ => serde_json::Value::String(format!("{:?}", v)),
                        };
                        map.insert(key, value);
                    }
                    serde_json::Value::Object(map)
                }
                _ => serde_json::Value::String(format!("{:?}", value)),
            };
            bridge
                .resource_set_requests
                .insert(resource_name, json_value);
            Ok(())
        })?,
    )?;

    globals.set("Resource", resource_table)?;

    Ok(())
}
