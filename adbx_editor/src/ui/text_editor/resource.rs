use bevy::prelude::*;

/// テキストエディタの状態管理
#[derive(Resource, Default)]
pub struct TextEditorState {
    #[allow(dead_code)]
    pub focused_editor: Option<EditorType>,
    pub clipboard: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorType {
    #[allow(dead_code)]
    CodeEditor,
    #[allow(dead_code)]
    ScriptEditor,
}
