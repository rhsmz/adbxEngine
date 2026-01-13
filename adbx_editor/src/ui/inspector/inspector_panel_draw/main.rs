use super::super::component_editor_registry::ComponentEditorRegistry;
use super::super::inspector_panel_resource::{
    InspectorContent, InspectorPanel, TransformInputValues,
};
use super::component_editor::draw_component_editors;
use super::header::{draw_entity_header, draw_no_selection};
use super::transform_editor::draw_transform_editor;
use bevy::prelude::*;

/// インスペクターパネルのUI描画（編集可能版）
pub fn draw_inspector_panel_ui(
    mut commands: Commands,
    mut inspector: ResMut<InspectorPanel>,
    selection: Res<crate::systems::selection::Selection>,
    editor_registry: Res<ComponentEditorRegistry>,
    transform_query: Query<&Transform, Changed<Transform>>,
    camera_3d_query: Query<&bevy::camera::Camera3d>,
    camera_2d_query: Query<&bevy::camera::Camera2d>,
    sprite_query: Query<&Sprite>,
    mesh_3d_query: Query<&bevy::prelude::Mesh3d>,
    mesh_material_3d_query: Query<&bevy::prelude::MeshMaterial3d<bevy::pbr::StandardMaterial>>,
    lua_script_query: Query<&adbx_runtime::lua::component::LuaScript>,
    lua_script_state_query: Query<&adbx_runtime::lua::component::LuaScriptState>,
    name_query: Query<&Name>,
    inspector_panel_query: Query<Entity, (With<Name>, With<Node>)>,
) {
    // インスペクターパネルを検索
    let panel_opt = inspector_panel_query.iter().find(|&e| {
        if let Ok(name) = name_query.get(e) {
            name.as_str() == "InspectorPanel"
        } else {
            false
        }
    });

    if let Some(panel) = panel_opt {
        // 選択されたエンティティを取得
        let selected_entity = selection.selected_entities.first().copied();

        // 選択が変更された場合、またはTransformが変更された場合のみ更新
        let needs_update = inspector.selected_entity != selected_entity
            || (selected_entity.is_some() && transform_query.get(selected_entity.unwrap()).is_ok());

        if needs_update {
            // 既存のコンテンツを削除
            if let Some(content_entity) = inspector.content_entity {
                if let Ok(mut entity_commands) = commands.get_entity(content_entity) {
                    entity_commands.despawn();
                }
            }

            if let Some(entity) = selected_entity {
                // エンティティ名を表示
                let entity_name = name_query
                    .get(entity)
                    .map(|n| n.as_str().to_string())
                    .unwrap_or_else(|_| format!("Entity {}", entity.index()));

                // Transformの現在の値を取得
                let current_transform = transform_query.get(entity).copied().unwrap_or_default();

                // 入力値を初期化または更新
                if !inspector.transform_input_values.contains_key(&entity) {
                    inspector.transform_input_values.insert(
                        entity,
                        TransformInputValues {
                            translation: current_transform.translation,
                            rotation: current_transform.rotation,
                            scale: current_transform.scale,
                        },
                    );
                } else if transform_query.get(entity).is_ok() {
                    // Transformが変更された場合、入力値を更新
                    if let Some(input_values) = inspector.transform_input_values.get_mut(&entity) {
                        input_values.translation = current_transform.translation;
                        input_values.rotation = current_transform.rotation;
                        input_values.scale = current_transform.scale;
                    }
                }

                let content_entity = commands
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(5.0)),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        InspectorContent,
                        Name::new("InspectorContent"),
                    ))
                    .with_children(|content| {
                        // エンティティ名ヘッダー
                        draw_entity_header(content, &entity_name);

                        // Transformコンポーネントの表示と編集
                        if let Ok(transform) = transform_query.get(entity) {
                            draw_transform_editor(content, entity, transform);
                        }
                    })
                    .id();

                // エディタレジストリを使用してコンポーネントを表示
                draw_component_editors(
                    &mut commands,
                    entity,
                    content_entity,
                    &editor_registry,
                    &camera_3d_query,
                    &camera_2d_query,
                    &sprite_query,
                    &mesh_3d_query,
                    &mesh_material_3d_query,
                    &lua_script_query,
                    &lua_script_state_query,
                    &transform_query,
                );

                commands.entity(panel).add_child(content_entity);
                inspector.content_entity = Some(content_entity);
            } else {
                // 何も選択されていない場合
                let content_entity = commands
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        InspectorContent,
                        Name::new("InspectorContent"),
                    ))
                    .with_children(|content| {
                        draw_no_selection(content);
                    })
                    .id();

                commands.entity(panel).add_child(content_entity);
                inspector.content_entity = Some(content_entity);
            }

            inspector.selected_entity = selected_entity;
        }
    }
}
