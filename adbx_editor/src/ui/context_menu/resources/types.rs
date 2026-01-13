use bevy::prelude::*;

/// コンテキストメニューのリソース
#[derive(Resource, Default)]
pub struct ContextMenu {
    pub is_visible: bool,
    pub position: Vec2,
    pub context_type: Option<ContextType>,
    pub menu_entity: Option<Entity>,
}

/// コンテキストメニューの種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextType {
    Hierarchy,
    AssetBrowser,
    CodeEditor,
    ScriptEditor,
    SceneView,
}

/// エンティティ操作の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityOperation {
    Create,
    Delete,
    Duplicate,
}

/// エンティティのクリップボード操作の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityClipboardOperation {
    Cut,
    Copy,
    Paste,
}

/// アセット操作の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetOperation {
    Import,
    Export,
    Delete,
    Rename,
    ShowInExplorer,
}
