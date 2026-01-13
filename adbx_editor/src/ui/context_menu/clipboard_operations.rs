use crate::systems::selection::Selection;
use crate::ui::clipboard::{Clipboard, EntityClipboardData, SerializedEntity, SerializedTransform};
use crate::ui::context_menu::EntityClipboardOperation;
use bevy::prelude::*;

/// エンティティのクリップボード操作（カット、コピー、ペースト）
pub fn handle_entity_clipboard_operations(
    commands: &mut Commands,
    selection: &mut ResMut<Selection>,
    clipboard: &mut ResMut<Clipboard>,
    transform_query: &Query<&Transform>,
    name_query: &Query<&Name>,
    operation: EntityClipboardOperation,
) {
    match operation {
        EntityClipboardOperation::Cut => {
            if !selection.selected_entities.is_empty() {
                // Entityをコピー
                let mut entity_data = EntityClipboardData {
                    entities: Vec::new(),
                };

                for entity in selection.selected_entities.iter() {
                    if let (Ok(name), Ok(transform)) =
                        (name_query.get(*entity), transform_query.get(*entity))
                    {
                        entity_data.entities.push(SerializedEntity {
                            name: name.as_str().to_string(),
                            transform: SerializedTransform {
                                translation: [
                                    transform.translation.x,
                                    transform.translation.y,
                                    transform.translation.z,
                                ],
                                rotation: [
                                    transform.rotation.x,
                                    transform.rotation.y,
                                    transform.rotation.z,
                                    transform.rotation.w,
                                ],
                                scale: [transform.scale.x, transform.scale.y, transform.scale.z],
                            },
                            components: Vec::new(), // 簡易実装
                        });
                    }
                }

                clipboard.set_entity_data(entity_data);

                // Entityを削除
                let entities_to_delete: Vec<Entity> =
                    selection.selected_entities.iter().copied().collect();
                for entity in entities_to_delete {
                    if let Ok(mut entity_commands) = commands.get_entity(entity) {
                        entity_commands.despawn();
                    }
                }
                selection.selected_entities.clear();
            }
        }
        EntityClipboardOperation::Copy => {
            if !selection.selected_entities.is_empty() {
                let mut entity_data = EntityClipboardData {
                    entities: Vec::new(),
                };

                for entity in selection.selected_entities.iter() {
                    if let (Ok(name), Ok(transform)) =
                        (name_query.get(*entity), transform_query.get(*entity))
                    {
                        entity_data.entities.push(SerializedEntity {
                            name: name.as_str().to_string(),
                            transform: SerializedTransform {
                                translation: [
                                    transform.translation.x,
                                    transform.translation.y,
                                    transform.translation.z,
                                ],
                                rotation: [
                                    transform.rotation.x,
                                    transform.rotation.y,
                                    transform.rotation.z,
                                    transform.rotation.w,
                                ],
                                scale: [transform.scale.x, transform.scale.y, transform.scale.z],
                            },
                            components: Vec::new(), // 簡易実装
                        });
                    }
                }

                let entity_count = entity_data.entities.len();
                clipboard.set_entity_data(entity_data);
                bevy::log::info!("Copied {} entities", entity_count);
            }
        }
        EntityClipboardOperation::Paste => {
            if let Some(entity_data) = clipboard.get_entity_data() {
                let mut new_entities = Vec::new();

                for serialized_entity in &entity_data.entities {
                    let transform = Transform::from_translation(Vec3::new(
                        serialized_entity.transform.translation[0],
                        serialized_entity.transform.translation[1],
                        serialized_entity.transform.translation[2],
                    ))
                    .with_rotation(Quat::from_xyzw(
                        serialized_entity.transform.rotation[0],
                        serialized_entity.transform.rotation[1],
                        serialized_entity.transform.rotation[2],
                        serialized_entity.transform.rotation[3],
                    ))
                    .with_scale(Vec3::new(
                        serialized_entity.transform.scale[0],
                        serialized_entity.transform.scale[1],
                        serialized_entity.transform.scale[2],
                    ));

                    let new_name = format!("{}_Copy", serialized_entity.name);
                    let new_entity = commands
                        .spawn((Name::new(new_name.clone()), transform))
                        .id();

                    new_entities.push(new_entity);
                    bevy::log::info!("Pasted entity: {}", new_name);
                }

                if !new_entities.is_empty() {
                    selection.selected_entities = new_entities;
                }
            }
        }
    }
}

/// テキストエディタのクリップボード操作の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEditorClipboardOperation {
    Cut,
    Copy,
    Paste,
}

/// 検索・置換ダイアログの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchReplaceDialogType {
    Search,
    Replace,
}

/// テキストエディタのクリップボード操作を処理
pub fn handle_text_editor_clipboard_operations(
    _code_editor: &mut crate::ui::code_editor::CodeEditor,
    _script_editor: &mut crate::ui::script_editor::ScriptEditor,
    _text_editor_state: &mut crate::ui::text_editor::TextEditorState,
    _operation: TextEditorClipboardOperation,
) {
    // TODO: 実装
}

/// 検索・置換ダイアログを処理
pub fn handle_search_replace_dialog(
    _search_replace: &mut crate::ui::search_replace::SearchReplace,
    _dialog_type: SearchReplaceDialogType,
) {
    // TODO: 実装
}
