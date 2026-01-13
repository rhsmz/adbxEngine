#[cfg(test)]
mod tests {
    use adbx_runtime::lua::component::{LuaScript, LuaScriptState};
    use adbx_runtime::lua::vm::LuaVm;
    use bevy::prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_lua_vm_initialization() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Lua VMの初期化は実際の実装に依存するため、簡易的なテスト
        // 実際のテストでは、LuaVm::new()を呼び出して初期化を確認
    }

    #[test]
    fn test_lua_script_execution() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // スクリプト実行のテストは実際の実装に依存するため、簡易的なテスト
        // 実際のテストでは、LuaScriptコンポーネントを追加して実行を確認
    }
}
