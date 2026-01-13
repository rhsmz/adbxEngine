use super::types::ContextType;
use bevy::prelude::*;

/// コンテキストタイプに応じたメニュー項目を生成
pub fn spawn_context_menu_items(parent: &mut ChildSpawnerCommands, context_type: ContextType) {
    match context_type {
        ContextType::Hierarchy => {
            spawn_hierarchy_menu_items(parent);
        }
        ContextType::AssetBrowser => {
            spawn_asset_browser_menu_items(parent);
        }
        ContextType::CodeEditor => {
            spawn_code_editor_menu_items(parent);
        }
        ContextType::ScriptEditor => {
            spawn_script_editor_menu_items(parent);
        }
        ContextType::SceneView => {
            spawn_scene_view_menu_items(parent);
        }
    }
}

/// ヒエラルキーメニュー項目
fn spawn_hierarchy_menu_items(parent: &mut ChildSpawnerCommands) {
    crate::spawn_menu_item_inline!(parent, "Create Entity", "ContextMenuCreateEntity");
    crate::spawn_menu_item_inline!(parent, "Delete Entity", "ContextMenuDeleteEntity");
    crate::spawn_menu_item_inline!(parent, "Duplicate Entity", "ContextMenuDuplicateEntity");
    crate::spawn_menu_separator_inline!(parent);
    crate::spawn_menu_item_inline!(parent, "Copy", "ContextMenuCopy");
    crate::spawn_menu_item_inline!(parent, "Paste", "ContextMenuPaste");
}

/// アセットブラウザーメニュー項目
fn spawn_asset_browser_menu_items(parent: &mut ChildSpawnerCommands) {
    crate::spawn_menu_item_inline!(parent, "Import Asset", "ContextMenuImportAsset");
    crate::spawn_menu_item_inline!(parent, "Export Asset", "ContextMenuExportAsset");
    crate::spawn_menu_separator_inline!(parent);
    crate::spawn_menu_item_inline!(parent, "Delete", "ContextMenuDeleteAsset");
    crate::spawn_menu_item_inline!(parent, "Rename", "ContextMenuRenameAsset");
    crate::spawn_menu_separator_inline!(parent);
    crate::spawn_menu_item_inline!(parent, "Show in Explorer", "ContextMenuShowInExplorer");
}

/// コードエディタメニュー項目
fn spawn_code_editor_menu_items(parent: &mut ChildSpawnerCommands) {
    crate::spawn_menu_item_inline!(parent, "Cut", "ContextMenuCut");
    crate::spawn_menu_item_inline!(parent, "Copy", "ContextMenuCopy");
    crate::spawn_menu_item_inline!(parent, "Paste", "ContextMenuPaste");
    crate::spawn_menu_separator_inline!(parent);
    crate::spawn_menu_item_inline!(parent, "Select All", "ContextMenuSelectAll");
    crate::spawn_menu_separator_inline!(parent);
    crate::spawn_menu_item_inline!(parent, "Find", "ContextMenuFind");
    crate::spawn_menu_item_inline!(parent, "Replace", "ContextMenuReplace");
}

/// スクリプトエディタメニュー項目
fn spawn_script_editor_menu_items(parent: &mut ChildSpawnerCommands) {
    crate::spawn_menu_item_inline!(parent, "Cut", "ContextMenuCut");
    crate::spawn_menu_item_inline!(parent, "Copy", "ContextMenuCopy");
    crate::spawn_menu_item_inline!(parent, "Paste", "ContextMenuPaste");
    crate::spawn_menu_separator_inline!(parent);
    crate::spawn_menu_item_inline!(parent, "Select All", "ContextMenuSelectAll");
    crate::spawn_menu_separator_inline!(parent);
    crate::spawn_menu_item_inline!(parent, "Run Script", "ContextMenuRunScript");
    crate::spawn_menu_item_inline!(parent, "Validate Script", "ContextMenuValidateScript");
}

/// シーンビューメニュー項目
fn spawn_scene_view_menu_items(parent: &mut ChildSpawnerCommands) {
    crate::spawn_menu_item_inline!(parent, "Create Entity", "ContextMenuCreateEntity");
    crate::spawn_menu_item_inline!(parent, "Paste", "ContextMenuPaste");
    crate::spawn_menu_separator_inline!(parent);
    crate::spawn_menu_item_inline!(parent, "Focus Selection", "ContextMenuFocusSelection");
}
