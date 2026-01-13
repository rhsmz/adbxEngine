use bevy::prelude::*;
use super::super::component_editor_registry::ComponentEditorRegistry;
use super::super::lua_script_display::draw_lua_script_component;

/// 個別コンポーネントエディタの描画（エディタレジストリを使用）
pub fn draw_component_editors(
    commands: &mut Commands,
    entity: bevy::ecs::entity::Entity,
    content_entity: bevy::ecs::entity::Entity,
    editor_registry: &ComponentEditorRegistry,
    camera_3d_query: &Query<&bevy::camera::Camera3d>,
    camera_2d_query: &Query<&bevy::camera::Camera2d>,
    sprite_query: &Query<&Sprite>,
    mesh_3d_query: &Query<&bevy::prelude::Mesh3d>,
    mesh_material_3d_query: &Query<&bevy::prelude::MeshMaterial3d<bevy::pbr::StandardMaterial>>,
    lua_script_query: &Query<&adbx_runtime::lua::component::LuaScript>,
    lua_script_state_query: &Query<&adbx_runtime::lua::component::LuaScriptState>,
    transform_query: &Query<&Transform>,
) {
    // Camera3dコンポーネントの表示
    if camera_3d_query.get(entity).is_ok() {
        if let Some(editor) = editor_registry.get::<bevy::camera::Camera3d>() {
            editor.draw_ui(commands, entity, content_entity);
        }
    }
    
    // Camera2dコンポーネントの表示
    if camera_2d_query.get(entity).is_ok() {
        if let Some(editor) = editor_registry.get::<bevy::camera::Camera2d>() {
            editor.draw_ui(commands, entity, content_entity);
        }
    }
    
    // Transformエディタもレジストリから取得して表示（既にインライン実装されているが、レジストリ版も使用可能）
    if transform_query.get(entity).is_ok() {
        if let Some(_editor) = editor_registry.get::<Transform>() {
            // Transformは既にインライン実装されているため、レジストリ版はオプション
        }
    }
    
    // Spriteコンポーネントの表示
    if sprite_query.get(entity).is_ok() {
        if let Some(editor) = editor_registry.get::<Sprite>() {
            editor.draw_ui(commands, entity, content_entity);
        }
    }
    
    // LuaScriptコンポーネントの表示
    if let Ok(lua_script) = lua_script_query.get(entity) {
        let lua_script_state = lua_script_state_query.get(entity).ok();
        draw_lua_script_component(
            &mut commands,
            entity,
            content_entity,
            lua_script,
            lua_script_state.as_ref(),
        );
    }
    
    // Mesh3dコンポーネントの表示
    if mesh_3d_query.get(entity).is_ok() {
        if let Some(editor) = editor_registry.get::<bevy::prelude::Mesh3d>() {
            editor.draw_ui(commands, entity, content_entity);
        }
    }
    
    // MeshMaterial3dコンポーネントの表示
    if mesh_material_3d_query.get(entity).is_ok() {
        if let Some(editor) = editor_registry.get::<bevy::prelude::MeshMaterial3d<bevy::pbr::StandardMaterial>>() {
            editor.draw_ui(commands, entity, content_entity);
        }
    }
}
