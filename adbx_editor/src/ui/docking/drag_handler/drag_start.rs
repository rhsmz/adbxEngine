use super::super::resource::{DockingSystem, DragState};
use bevy::prelude::*;

/// パネルのドラッグ開始処理
#[allow(dead_code)]
pub fn handle_panel_drag(
    mut docking: ResMut<DockingSystem>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    panel_header_query: Query<(
        &super::super::resource::PanelHeader,
        &bevy::ui::Interaction,
        &Node,
        &GlobalTransform,
    )>,
) {
    if docking.drag_state.is_none() && mouse_input.just_pressed(MouseButton::Left) {
        if let Some(window) = windows.iter().next() {
            if let Some(cursor_pos) = window.cursor_position() {
                // パネルのヘッダー部分をクリックしたかチェック
                for (header, interaction, node, transform) in panel_header_query.iter() {
                    if *interaction == bevy::ui::Interaction::Pressed {
                        let panel_name = header.panel_name.clone();
                        if docking.panels.contains_key(&panel_name) {
                            // リサイズエッジを検出（パネルヘッダーの端をクリックした場合）
                            let (is_resizing, resize_edge) =
                                crate::ui::docking::ui_draw::detect_resize_edge(
                                    cursor_pos, node, transform, window,
                                );

                            docking.drag_state = Some(DragState {
                                panel_name,
                                start_position: cursor_pos,
                                current_position: cursor_pos,
                                is_resizing,
                                resize_edge,
                            });
                        }
                    }
                }
            }
        }
    }
}
