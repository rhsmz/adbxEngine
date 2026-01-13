use super::menu_display::show_context_menu;
use super::types::ContextType;
use bevy::prelude::*;
use bevy::ui::Interaction;

/// 右クリック検出とメニュー表示
pub fn detect_right_click_for_context_menu(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    interaction_query: Query<(&Interaction, &Name), Changed<Interaction>>,
    commands: Commands,
    context_menu: ResMut<super::types::ContextMenu>,
) {
    // 右クリックでメニューを表示
    if mouse_input.just_pressed(MouseButton::Right) {
        // クリック位置を取得
        let cursor_pos = if let Some(window) = windows.iter().next() {
            window.cursor_position()
        } else {
            None
        };

        if let Some(_pos) = cursor_pos {
            // クリックされたUI要素を確認
            for (interaction, name) in interaction_query.iter() {
                if *interaction == Interaction::Pressed {
                    let name_str = name.as_str();

                    // ヒエラルキー関連
                    if name_str.starts_with("HierarchyItem_") || name_str == "HierarchyPanel" {
                        show_context_menu(commands, context_menu, windows, ContextType::Hierarchy);
                        return;
                    }

                    // アセットブラウザー関連
                    if name_str.starts_with("AssetItem_") || name_str == "AssetBrowserPanel" {
                        show_context_menu(
                            commands,
                            context_menu,
                            windows,
                            ContextType::AssetBrowser,
                        );
                        return;
                    }

                    // コードエディタ関連
                    if name_str.starts_with("CodeEditor") || name_str == "CodeEditorPanel" {
                        show_context_menu(commands, context_menu, windows, ContextType::CodeEditor);
                        return;
                    }

                    // スクリプトエディタ関連
                    if name_str.starts_with("ScriptEditor") || name_str == "ScriptEditorPanel" {
                        show_context_menu(
                            commands,
                            context_menu,
                            windows,
                            ContextType::ScriptEditor,
                        );
                        return;
                    }

                    // シーンビュー関連
                    if name_str == "SceneViewArea" {
                        show_context_menu(commands, context_menu, windows, ContextType::SceneView);
                        return;
                    }
                }
            }
        }
    }
}
