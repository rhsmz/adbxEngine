use bevy::prelude::*;
use std::path::PathBuf;

/// スクリプトをEntityにアタッチ
pub fn attach_script_to_entity(entity: Entity, script_path: PathBuf, mut commands: Commands) {
    use adbx_runtime::lua::component::LuaScript;
    commands
        .entity(entity)
        .insert(LuaScript::from_path(script_path));
}
