use super::super::resource::PanelPosition;
use bevy::prelude::*;

/// ドロップゾーン情報
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DockZone {
    pub position: PanelPosition,
    pub bounds: bevy::math::Rect,
}

/// ドロップゾーンを検出
pub fn detect_drop_zone(position: Vec2, window_size: Vec2) -> Option<PanelPosition> {
    // 画面の領域に基づいてドロップゾーンを決定
    let left_threshold = window_size.x * 0.15;
    let right_threshold = window_size.x * 0.85;
    let top_threshold = window_size.y * 0.15;
    let bottom_threshold = window_size.y * 0.85;

    if position.x < left_threshold {
        Some(PanelPosition::Left)
    } else if position.x > right_threshold {
        Some(PanelPosition::Right)
    } else if position.y < top_threshold {
        Some(PanelPosition::Top)
    } else if position.y > bottom_threshold {
        Some(PanelPosition::Bottom)
    } else {
        Some(PanelPosition::Center)
    }
}

/// ドロップゾーンの視覚的表示用コンポーネント
#[derive(Component, Reflect)]
#[reflect(Component)]
#[allow(dead_code)]
pub struct DropZoneIndicator {
    pub position: PanelPosition,
}
