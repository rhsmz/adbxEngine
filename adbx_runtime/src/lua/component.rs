use bevy::prelude::*;
use std::path::PathBuf;

/// EntityにアタッチされるLuaスクリプトコンポーネント
#[derive(Component, Debug, Clone)]
pub struct LuaScript {
    pub script_path: PathBuf,
    pub script_content: String,
}

impl LuaScript {
    pub fn new(script_path: PathBuf, script_content: String) -> Self {
        Self {
            script_path,
            script_content,
        }
    }

    pub fn from_path(script_path: PathBuf) -> Self {
        let script_content =
            std::fs::read_to_string(&script_path).unwrap_or_else(|_| String::new());
        Self::new(script_path, script_content)
    }
}

/// Luaスクリプトの実行状態
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LuaScriptState {
    /// スクリプトが読み込まれ、実行準備ができている
    Loaded,
    /// スクリプトが実行中
    Running,
    /// スクリプトでエラーが発生
    Error,
}
