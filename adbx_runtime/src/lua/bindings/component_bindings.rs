use crate::lua::component_bridge::ComponentBridge;
use mlua::{Lua, Result as LuaResult};
use std::sync::Arc;
use std::sync::Mutex;

/// Component操作のバインディング
pub fn register_component_bindings(
    lua: &Lua,
    component_bridge: Arc<Mutex<ComponentBridge>>,
) -> LuaResult<()> {
    let globals = lua.globals();

    // Component操作
    let component_table = lua.create_table()?;

    // Component::get
    let component_bridge_clone = component_bridge.clone();
    component_table.set(
        "get",
        lua.create_function(move |_lua, (entity_id, component_name): (u32, String)| {
            let mut bridge = component_bridge_clone.lock().unwrap();
            bridge
                .component_get_requests
                .push(crate::lua::component_bridge::ComponentRequest {
                    entity_id,
                    component_name,
                });
            Ok(())
        })?,
    )?;

    // Component::get_value（取得した値を返す）
    let component_bridge_clone2 = component_bridge.clone();
    component_table.set(
        "get_value",
        lua.create_function(move |_lua, (entity_id, component_name): (u32, String)| {
            let bridge = component_bridge_clone2.lock().unwrap();
            let cache_key = format!("{}_{}", entity_id, component_name);
            if let Some(value) = bridge.component_values.get(&cache_key) {
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
                                serde_json::Value::Object(obj) => {
                                    let lua_obj = _lua.create_table()?;
                                    for (k2, v2) in obj {
                                        let lua_v2 = match v2 {
                                            serde_json::Value::Number(n) => {
                                                if let Some(f) = n.as_f64() {
                                                    mlua::Value::Number(f)
                                                } else {
                                                    mlua::Value::String(
                                                        _lua.create_string(&n.to_string())?,
                                                    )
                                                }
                                            }
                                            serde_json::Value::String(s) => {
                                                mlua::Value::String(_lua.create_string(s)?)
                                            }
                                            serde_json::Value::Bool(b) => mlua::Value::Boolean(*b),
                                            _ => mlua::Value::String(
                                                _lua.create_string(&v2.to_string())?,
                                            ),
                                        };
                                        lua_obj.set(k2.as_str(), lua_v2)?;
                                    }
                                    mlua::Value::Table(lua_obj)
                                }
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

    // Component::set
    let component_bridge_clone3 = component_bridge.clone();
    component_table.set(
        "set",
        lua.create_function(
            move |_lua, (entity_id, component_name, component_data): (u32, String, mlua::Value)| {
                let mut bridge = component_bridge_clone3.lock().unwrap();
                // mlua::Valueをserde_json::Valueに変換
                let json_value = match component_data {
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
                    _ => serde_json::Value::String(format!("{:?}", component_data)),
                };
                bridge
                    .component_set_requests
                    .push(crate::lua::component_bridge::ComponentUpdate {
                        entity_id,
                        component_name,
                        component_data: json_value,
                    });
                Ok(())
            },
        )?,
    )?;

    globals.set("Component", component_table)?;

    Ok(())
}
