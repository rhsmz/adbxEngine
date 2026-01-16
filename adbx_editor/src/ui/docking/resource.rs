// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
use crate::systems::operation_recording::{
    record_panel_moved, record_panel_resized, OperationRecorder,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ドッキング可能なパネルの状態
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct DockingSystem {
    pub panels: HashMap<String, PanelState>,
    pub drag_state: Option<DragState>,
}

/// パネルの状態
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct PanelState {
    pub name: String,
    pub position: PanelPosition,
    pub size: (f32, f32), // 幅、高さ（パーセントまたはピクセル）
    pub is_docked: bool,
    pub is_visible: bool,
    pub floating_position: Option<(f32, f32)>, // フローティングパネルの絶対位置（x, y）
}

/// パネルの位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum PanelPosition {
    Left,
    Right,
    Top,
    Bottom,
    Center,
    Floating,
}

/// ドラッグ状態
#[derive(Debug, Clone, Reflect)]
pub struct DragState {
    pub panel_name: String,
    pub start_position: Vec2,
    pub current_position: Vec2,
    pub is_resizing: bool,               // リサイズ中かどうか
    pub resize_edge: Option<ResizeEdge>, // リサイズするエッジ
}

/// リサイズエッジ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum ResizeEdge {
    Left,
    Right,
    Top,
    Bottom,
}

impl Default for PanelState {
    fn default() -> Self {
        Self {
            name: String::new(),
            position: PanelPosition::Right,
            size: (300.0, 100.0),
            is_docked: true,
            is_visible: true,
            floating_position: None,
        }
    }
}

/// ドッキングシステムの初期化
pub fn initialize_docking_system(mut docking: ResMut<DockingSystem>) {
    // デフォルトのパネル状態を設定
    docking.panels.insert(
        "HierarchyPanel".to_string(),
        PanelState {
            name: "HierarchyPanel".to_string(),
            position: PanelPosition::Left,
            size: (250.0, 100.0),
            is_docked: true,
            is_visible: true,
            floating_position: None,
        },
    );

    docking.panels.insert(
        "InspectorPanel".to_string(),
        PanelState {
            name: "InspectorPanel".to_string(),
            position: PanelPosition::Right,
            size: (300.0, 100.0),
            is_docked: true,
            is_visible: true,
            floating_position: None,
        },
    );

    docking.panels.insert(
        "AssetBrowserPanel".to_string(),
        PanelState {
            name: "AssetBrowserPanel".to_string(),
            position: PanelPosition::Bottom,
            size: (100.0, 200.0),
            is_docked: true,
            is_visible: true,
            floating_position: None,
        },
    );

    docking.panels.insert(
        "ScriptEditorPanel".to_string(),
        PanelState {
            name: "ScriptEditorPanel".to_string(),
            position: PanelPosition::Bottom,
            size: (100.0, 200.0),
            is_docked: true,
            is_visible: true,
            floating_position: None,
        },
    );

    docking.panels.insert(
        "CodeEditorPanel".to_string(),
        PanelState {
            name: "CodeEditorPanel".to_string(),
            position: PanelPosition::Bottom,
            size: (100.0, 200.0),
            is_docked: true,
            is_visible: true,
            floating_position: None,
        },
    );
}

/// パネルヘッダーのマーカーコンポーネント
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PanelHeader {
    pub panel_name: String,
}

/// パネルのドラッグ開始
pub fn start_panel_drag(
    mut docking: ResMut<DockingSystem>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    panel_header_query: Query<
        (&PanelHeader, &Interaction, &Node, &GlobalTransform),
        Changed<Interaction>,
    >,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
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
