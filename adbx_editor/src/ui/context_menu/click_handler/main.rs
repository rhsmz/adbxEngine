use bevy::prelude::*;
use super::super::resources::types::ContextMenu;
use super::super::EntityOperation;
use super::super::hide_context_menu;
use super::entity_operations::handle_entity_operation_menu_item;
use super::clipboard_operations::handle_clipboard_operation_menu_item;
use super::text_editor_operations::handle_text_editor_operation_menu_item;
use super::search_replace_operations::handle_search_replace_operation_menu_item;
use super::script_operations::handle_script_operation_menu_item;
use super::asset_operations::handle_asset_operation_menu_item;
use super::scene_operations::handle_scene_operation_menu_item;
use crate::ui::code_editor::CodeEditor;
use crate::ui::script_editor::ScriptEditor;
use crate::ui::search_replace::SearchReplace;
use crate::systems::selection::Selection;
use crate::ui::clipboard::Clipboard;
use crate::ui::text_editor::TextEditorState;
use crate::ui::asset_browser::AssetBrowser;
use crate::ui::file_dialog::FileDialogRequest;
use crate::ui::rename_dialog::RenameDialogRequest;
use crate::ui::context_menu::TextEditorClipboardOperation;

/// コンテキストメニューのクリック処理
pub fn handle_context_menu_item_click(
    mut commands: Commands,
    mut context_menu: ResMut<ContextMenu>,
    mut code_editor: ResMut<CodeEditor>,
    mut script_editor: ResMut<ScriptEditor>,
    mut search_replace: ResMut<SearchReplace>,
    mut selection: ResMut<Selection>,
    mut clipboard: ResMut<Clipboard>,
    mut text_editor_state: ResMut<TextEditorState>,
    mut asset_browser: ResMut<AssetBrowser>,
    mut file_dialog_request: ResMut<FileDialogRequest>,
    mut rename_dialog: ResMut<RenameDialogRequest>,
    transform_query: Query<&Transform>,
    name_query: Query<&Name>,
    children_query: Query<&Children>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    interaction_query: Query<(&Interaction, &Name), Changed<Interaction>>,
) {
    // 左クリックでメニューを閉じる（メニュー外をクリックした場合）
    if mouse_input.just_pressed(MouseButton::Left) {
        if context_menu.is_visible {
            // メニュー項目がクリックされたかチェック
            let mut menu_item_clicked = false;
            for (interaction, name) in interaction_query.iter() {
                if *interaction == Interaction::Pressed {
                    let name_str = name.as_str();
                    if name_str.starts_with("ContextMenu") {
                        menu_item_clicked = true;
                        
                        // メニュー項目の処理
                        match name_str {
                            "ContextMenuCreateEntity" => {
                                handle_entity_operation_menu_item(
                                    &mut commands,
                                    &mut selection,
                                    &transform_query,
                                    &name_query,
                                    &children_query,
                                    EntityOperation::Create,
                                );
                            }
                            "ContextMenuDeleteEntity" => {
                                handle_entity_operation_menu_item(
                                    &mut commands,
                                    &mut selection,
                                    &transform_query,
                                    &name_query,
                                    &children_query,
                                    EntityOperation::Delete,
                                );
                            }
                            "ContextMenuDuplicateEntity" => {
                                handle_entity_operation_menu_item(
                                    &mut commands,
                                    &mut selection,
                                    &transform_query,
                                    &name_query,
                                    &children_query,
                                    EntityOperation::Duplicate,
                                );
                            }
                            "ContextMenuCut" => {
                                handle_clipboard_operation_menu_item(
                                    &mut code_editor,
                                    &mut script_editor,
                                    &mut text_editor_state,
                                    &mut commands,
                                    &mut selection,
                                    &mut clipboard,
                                    &transform_query,
                                    &name_query,
                                    TextEditorClipboardOperation::Cut,
                                );
                            }
                            "ContextMenuCopy" => {
                                handle_clipboard_operation_menu_item(
                                    &mut code_editor,
                                    &mut script_editor,
                                    &mut text_editor_state,
                                    &mut commands,
                                    &mut selection,
                                    &mut clipboard,
                                    &transform_query,
                                    &name_query,
                                    TextEditorClipboardOperation::Copy,
                                );
                            }
                            "ContextMenuPaste" => {
                                handle_clipboard_operation_menu_item(
                                    &mut code_editor,
                                    &mut script_editor,
                                    &mut text_editor_state,
                                    &mut commands,
                                    &mut selection,
                                    &mut clipboard,
                                    &transform_query,
                                    &name_query,
                                    TextEditorClipboardOperation::Paste,
                                );
                            }
                            "ContextMenuSelectAll" => {
                                handle_text_editor_operation_menu_item(&mut code_editor, name_str);
                            }
                            "ContextMenuFind" | "ContextMenuReplace" => {
                                handle_search_replace_operation_menu_item(&mut search_replace, name_str);
                            }
                            "ContextMenuRunScript" | "ContextMenuValidateScript" => {
                                handle_script_operation_menu_item(&mut script_editor, name_str);
                            }
                            "ContextMenuImportAsset" | "ContextMenuExportAsset" | "ContextMenuDeleteAsset" | "ContextMenuRenameAsset" | "ContextMenuShowInExplorer" => {
                                handle_asset_operation_menu_item(
                                    &mut asset_browser,
                                    &mut file_dialog_request,
                                    &mut rename_dialog,
                                    name_str,
                                );
                            }
                            "ContextMenuFocusSelection" => {
                                handle_scene_operation_menu_item(&selection, &transform_query, name_str);
                            }
                            _ => {}
                        }
                    }
                }
            }
            
            // メニュー外をクリックした場合はメニューを閉じる
            if !menu_item_clicked {
                hide_context_menu(commands, context_menu);
            } else {
                // メニュー項目をクリックした場合もメニューを閉じる
                hide_context_menu(commands, context_menu);
            }
        }
    }
}
