use bevy::prelude::*;
use bevy::reflect::Reflect;
use std::path::PathBuf;

/// シーンに属するエンティティをマークするコンポーネント
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct SceneEntity {
    pub scene_name: String,
}

/// EntityにアタッチされるLuaスクリプトコンポーネント
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
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
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum LuaScriptState {
    /// スクリプトが読み込まれ、実行準備ができている
    Loaded,
    /// スクリプトが実行中
    Running,
    /// スクリプトでエラーが発生
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::reflect::Reflect;

    #[test]
    fn test_scene_entity_creation() {
        let scene_entity = SceneEntity {
            scene_name: "TestScene".to_string(),
        };
        assert_eq!(scene_entity.scene_name, "TestScene");
    }

    #[test]
    fn test_lua_script_creation() {
        let script_path = PathBuf::from("test.lua");
        let script_content = "print('Hello World')".to_string();
        let lua_script = LuaScript::new(script_path.clone(), script_content.clone());

        assert_eq!(lua_script.script_path, script_path);
        assert_eq!(lua_script.script_content, script_content);
    }

    #[test]
    fn test_lua_script_from_path() {
        let script_path = PathBuf::from("test.lua");
        let lua_script = LuaScript::from_path(script_path.clone());

        assert_eq!(lua_script.script_path, script_path);
        // ファイルが存在しないので空文字列になるはず
        assert_eq!(lua_script.script_content, "");
    }

    #[test]
    fn test_lua_script_state_equality() {
        assert_eq!(LuaScriptState::Loaded, LuaScriptState::Loaded);
        assert_eq!(LuaScriptState::Running, LuaScriptState::Running);
        assert_eq!(LuaScriptState::Error, LuaScriptState::Error);

        assert_ne!(LuaScriptState::Loaded, LuaScriptState::Running);
        assert_ne!(LuaScriptState::Running, LuaScriptState::Error);
        assert_ne!(LuaScriptState::Error, LuaScriptState::Loaded);
    }

    #[test]
    fn test_scene_entity_reflect() {
        let scene_entity = SceneEntity {
            scene_name: "TestScene".to_string(),
        };

        // Reflectが正しく機能するかテスト
        let reflect_value = &scene_entity as &dyn Reflect;
        assert!(reflect_value.is::<SceneEntity>());
    }

    #[test]
    fn test_lua_script_reflect() {
        let lua_script = LuaScript::new(
            PathBuf::from("test.lua"),
            "print('test')".to_string(),
        );

        let reflect_value = &lua_script as &dyn Reflect;
        assert!(reflect_value.is::<LuaScript>());
    }

    #[test]
    fn test_lua_script_state_reflect() {
        let state = LuaScriptState::Loaded;

        let reflect_value = &state as &dyn Reflect;
        assert!(reflect_value.is::<LuaScriptState>());
    }
}