use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};

/// テキストエディタの状態管理
#[derive(Resource, Default)]
pub struct TextEditorState {
    pub focused_editor: Option<EditorType>,
    pub clipboard: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorType {
    CodeEditor,
    ScriptEditor,
}
