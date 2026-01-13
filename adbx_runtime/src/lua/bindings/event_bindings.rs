use crate::lua::event_bridge::EventBridge;
use mlua::{Lua, Result as LuaResult};
use std::sync::Arc;
use std::sync::Mutex;

/// Event操作のバインディング
pub fn register_event_bindings(lua: &Lua, event_bridge: Arc<Mutex<EventBridge>>) -> LuaResult<()> {
    let globals = lua.globals();

    // Event操作
    let event_table = lua.create_table()?;

    // EventWriter::send
    let event_bridge_clone = event_bridge.clone();
    event_table.set(
        "send",
        lua.create_function(
            move |_lua, (event_name, event_data): (String, mlua::Value)| {
                let mut bridge = event_bridge_clone.lock().unwrap();
                // mlua::Valueをserde_json::Valueに変換
                let json_value = match event_data {
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
                    _ => serde_json::Value::String(format!("{:?}", event_data)),
                };
                bridge
                    .event_send_queue
                    .push(crate::lua::event_bridge::EventMessage {
                        event_name,
                        event_data: json_value,
                    });
                Ok(())
            },
        )?,
    )?;

    // EventReader::read
    let event_bridge_clone2 = event_bridge.clone();
    event_table.set(
        "read",
        lua.create_function(move |_lua, event_name: String| {
            let bridge = event_bridge_clone2.lock().unwrap();
            if let Some(events) = bridge.event_receive_buffer.get(&event_name) {
                // EventのリストをLuaのテーブルに変換
                let lua_table = _lua.create_table()?;
                for (i, event_value) in events.iter().enumerate() {
                    let lua_value = match event_value {
                        serde_json::Value::Object(map) => {
                            let lua_obj = _lua.create_table()?;
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
                                lua_obj.set(k.as_str(), lua_v)?;
                            }
                            mlua::Value::Table(lua_obj)
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
                    lua_table.set(i + 1, lua_value)?;
                }
                Ok(mlua::Value::Table(lua_table))
            } else {
                Ok(mlua::Value::Table(_lua.create_table()?))
            }
        })?,
    )?;

    globals.set("Event", event_table)?;

    Ok(())
}
