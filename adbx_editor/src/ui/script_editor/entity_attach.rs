use adbx_shared::components::LuaScript;
use bevy::prelude::*;
use std::path::PathBuf;

/// スクリプトをEntityにアタッチ
#[allow(dead_code)]
pub fn attach_script_to_entity(entity: Entity, script_path: PathBuf, mut commands: Commands) {
    commands
        .entity(entity)
        .insert(LuaScript::from_path(script_path));
}
