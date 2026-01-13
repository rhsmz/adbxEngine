use super::super::resource::{DockingSystem, PanelPosition};
use bevy::prelude::*;

/// パネルの位置とサイズを動的に更新するシステム
pub fn update_panel_layout(
    docking: ResMut<DockingSystem>,
    mut panel_query: Query<
        (&Name, &mut Node),
        (With<Node>, Without<super::super::resource::PanelHeader>),
    >,
    windows: Query<&Window>,
) {
    if let Some(window) = windows.iter().next() {
        let _window_width = window.width();
        let _window_height = window.height();

        // 各パネルのUIノードを更新
        for (name, mut node) in panel_query.iter_mut() {
            let panel_name = name.as_str();

            if let Some(panel_state) = docking.panels.get(panel_name) {
                if !panel_state.is_visible {
                    // 非表示のパネルはスキップ
                    continue;
                }

                // パネルの位置に応じてサイズを更新
                match panel_state.position {
                    PanelPosition::Left | PanelPosition::Right => {
                        // 左右のパネルは幅を設定
                        node.width = Val::Px(panel_state.size.0);
                        node.height = Val::Percent(100.0);
                    }
                    PanelPosition::Top | PanelPosition::Bottom => {
                        // 上下のパネルは高さを設定
                        node.width = Val::Percent(100.0);
                        node.height = Val::Px(panel_state.size.1);
                    }
                    PanelPosition::Center => {
                        // 中央のパネルは両方を設定
                        node.width = Val::Percent(panel_state.size.0);
                        node.height = Val::Percent(panel_state.size.1);
                    }
                    PanelPosition::Floating => {
                        // フローティングパネルは固定サイズ
                        if let Some((_x, _y)) = panel_state.floating_position {
                            node.width = Val::Px(panel_state.size.0);
                            node.height = Val::Px(panel_state.size.1);
                            // 位置も設定する必要がある場合は、PositionType::Absoluteを使用
                        }
                    }
                }
            }
        }
    }
}
