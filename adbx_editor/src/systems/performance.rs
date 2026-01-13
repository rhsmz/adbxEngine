use bevy::prelude::*;

/// パフォーマンス設定リソース
#[derive(Resource)]
pub struct PerformanceSettings {
    /// UI更新の間隔（フレーム数）
    pub ui_update_interval: u32,
    /// 大規模シーンでのレイキャスト最適化を有効化
    pub enable_spatial_optimization: bool,
    /// Gizmo描画の最適化を有効化
    pub optimize_gizmo_drawing: bool,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            ui_update_interval: 1, // 毎フレーム更新（必要に応じて調整可能）
            enable_spatial_optimization: true,
            optimize_gizmo_drawing: true,
        }
    }
}

/// フレームカウンター（UI更新の間隔制御用）
#[derive(Resource, Default)]
pub struct FrameCounter {
    pub frame: u32,
}

impl FrameCounter {
    pub fn should_update_ui(&self, interval: u32) -> bool {
        self.frame % interval == 0
    }
    
    pub fn increment(&mut self) {
        self.frame += 1;
    }
}

/// フレームカウンターを更新するシステム
pub fn update_frame_counter(mut frame_counter: ResMut<FrameCounter>) {
    frame_counter.increment();
}
