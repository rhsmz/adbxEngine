use bevy::prelude::*;
use super::super::resource::{DockingSystem, PanelPosition, ResizeEdge};
use super::super::drag_handler::{detect_drop_zone, DropZoneIndicator, save_layout};
use crate::systems::operation_recording::{OperationRecorder, record_panel_moved, record_panel_resized};
use super::drop_zone::show_drop_zone;

/// パネルのドラッグ更新
pub fn update_panel_drag(
    mut commands: Commands,
    mut docking: ResMut<DockingSystem>,
    operation_recorder: ResMut<OperationRecorder>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    _mouse_motion_events: bevy::prelude::MessageReader<bevy::input::mouse::MouseMotion>,
    drop_zone_query: Query<Entity, With<DropZoneIndicator>>,
) {
    if let Some(ref mut drag_state) = docking.drag_state {
        if mouse_input.pressed(MouseButton::Left) {
            if let Some(window) = windows.iter().next() {
                if let Some(cursor_pos) = window.cursor_position() {
                    drag_state.current_position = cursor_pos;
                    
                    // ドロップゾーンを更新
                    let window_size = Vec2::new(window.width(), window.height());
                    if let Some(drop_position) = detect_drop_zone(cursor_pos, window_size) {
                        // 既存のドロップゾーンインジケーターを削除
                        for entity in drop_zone_query.iter() {
                            commands.entity(entity).despawn();
                        }
                        
                        // 新しいドロップゾーンインジケーターを表示
                        show_drop_zone(&mut commands, drop_position, window_size);
                    }
                }
            }
        } else {
            // ドラッグ終了：ドロップゾーンを検出してドッキング、またはリサイズを適用
            let panel_name = drag_state.panel_name.clone();
            let is_resizing = drag_state.is_resizing;
            let resize_edge = drag_state.resize_edge;
            let current_position = drag_state.current_position;
            let start_position = drag_state.start_position;
            drop(drag_state); // drag_stateの借用を終了
            if let Some(panel_state) = docking.panels.get_mut(&panel_name) {
                let _old_position = format!("{:?}", panel_state.position);
                
                if is_resizing {
                    // リサイズ処理
                    if let Some(edge) = resize_edge {
                        let old_size = panel_state.size;
                        let delta = current_position - start_position;
                        match edge {
                            ResizeEdge::Left | ResizeEdge::Right => {
                                panel_state.size.0 = (panel_state.size.0 + delta.x).max(100.0).min(800.0);
                            }
                            ResizeEdge::Top | ResizeEdge::Bottom => {
                                panel_state.size.1 = (panel_state.size.1 + delta.y).max(100.0).min(600.0);
                            }
                        }
                        
                        // パネルリサイズを記録
                        if old_size != panel_state.size {
                            record_panel_resized(
                                operation_recorder,
                                panel_name.clone(),
                                old_size,
                                panel_state.size,
                            );
                        }
                    }
                } else {
                    // ドッキング処理
                    if let Some(window) = windows.iter().next() {
                        let window_size = Vec2::new(window.width(), window.height());
                        if let Some(drop_zone) = detect_drop_zone(current_position, window_size) {
                            let old_position = format!("{:?}", panel_state.position);
                            panel_state.position = drop_zone;
                            
                            // フローティングパネルの場合、位置を保存
                            if drop_zone == PanelPosition::Floating {
                                panel_state.floating_position = Some((
                                    current_position.x,
                                    current_position.y,
                                ));
                                panel_state.is_docked = false;
                            } else {
                                panel_state.is_docked = true;
                                panel_state.floating_position = None;
                            }
                            
                            // パネル移動を記録
                            let new_position = format!("{:?}", drop_zone).to_string();
                            record_panel_moved(
                                operation_recorder,
                                panel_name.clone(),
                                old_position,
                                new_position,
                            );
                        }
                    }
                }
            }
            
            // ドロップゾーンインジケーターを削除
            for entity in drop_zone_query.iter() {
                commands.entity(entity).despawn();
            }
            
            // レイアウトを保存
            if let Err(e) = save_layout(&*docking) {
                bevy::log::warn!("Failed to save layout: {}", e);
            }
            
            docking.drag_state = None;
        }
    }
}
