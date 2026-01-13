use bevy::prelude::*;

/// ランタイムの実行状態
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeState {
    #[default]
    Stopped,    // 停止中
    Playing,    // 実行中
    Paused,     // 一時停止中
}

/// ランタイム状態管理リソース
#[derive(Resource, Default)]
pub struct RuntimeStateManager {
    pub state: RuntimeState,
    pub scene_path: Option<String>, // 実行中のシーンのパス
}

impl RuntimeStateManager {
    pub fn new() -> Self {
        Self {
            state: RuntimeState::Stopped,
            scene_path: None,
        }
    }
    
    pub fn start(&mut self, scene_path: Option<String>) {
        self.state = RuntimeState::Playing;
        self.scene_path = scene_path;
    }
    
    pub fn stop(&mut self) {
        self.state = RuntimeState::Stopped;
        self.scene_path = None;
    }
    
    pub fn pause(&mut self) {
        if self.state == RuntimeState::Playing {
            self.state = RuntimeState::Paused;
        }
    }
    
    pub fn resume(&mut self) {
        if self.state == RuntimeState::Paused {
            self.state = RuntimeState::Playing;
        }
    }
    
    pub fn is_playing(&self) -> bool {
        self.state == RuntimeState::Playing
    }
    
    pub fn is_paused(&self) -> bool {
        self.state == RuntimeState::Paused
    }
    
    pub fn is_stopped(&self) -> bool {
        self.state == RuntimeState::Stopped
    }
}
