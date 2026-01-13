pub mod component_bindings;
pub mod entity_bindings;
pub mod event_bindings;
pub mod log_bindings;
pub mod lua_data_types;
pub mod resource_bindings;
pub mod time_bindings;
pub mod transform_bindings;
pub mod vec3_bindings;

pub use lua_data_types::{TransformData, Vec3Data};

use crate::lua::commands_bridge::LuaCommandsBridge;
use crate::lua::component_bridge::ComponentBridge;
use crate::lua::event_bridge::EventBridge;
use crate::lua::resource_bridge::ResourceBridge;
use crate::lua::transform_bridge::TransformBridge;
use mlua::{Lua, Result as LuaResult};
use std::sync::Arc;
use std::sync::Mutex;

/// Bevy APIをLuaにバインディング
pub fn register_bevy_bindings(
    lua: &Lua,
    commands_bridge: Arc<Mutex<LuaCommandsBridge>>,
    transform_bridge: Arc<Mutex<TransformBridge>>,
    resource_bridge: Arc<Mutex<ResourceBridge>>,
    event_bridge: Arc<Mutex<EventBridge>>,
    component_bridge: Arc<Mutex<ComponentBridge>>,
) -> LuaResult<()> {
    entity_bindings::register_entity_bindings(lua, commands_bridge)?;
    transform_bindings::register_transform_bindings(lua, transform_bridge)?;
    vec3_bindings::register_vec3_bindings(lua)?;
    time_bindings::register_time_bindings(lua)?;
    log_bindings::register_log_bindings(lua)?;
    resource_bindings::register_resource_bindings(lua, resource_bridge)?;
    event_bindings::register_event_bindings(lua, event_bridge)?;
    component_bindings::register_component_bindings(lua, component_bridge)?;

    Ok(())
}
