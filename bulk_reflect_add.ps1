$files = @(
    "adbx_editor/src/ui/inspector/inspector_panel_resource.rs",
    "adbx_editor/src/ui/hierarchy/hierarchy_view_resource.rs",
    "adbx_editor/src/ui/scene_view/resource.rs",
    "adbx_editor/src/editor_app.rs",
    "adbx_editor/src/ui/clipboard.rs",
    "adbx_editor/src/ui/text_editor/resource.rs",
    "adbx_editor/src/systems/scene_management.rs",
    "adbx_editor/src/systems/operation_recording/resource.rs",
    "adbx_editor/src/systems/runtime_state.rs",
    "adbx_editor/src/systems/performance.rs",
    "adbx_editor/src/ui/code_editor/ai_integration/resource.rs",
    "adbx_editor/src/ui/search_replace/resource.rs",
    "adbx_editor/src/ui/code_editor/resource.rs",
    "adbx_editor/src/ui/file_dialog/resource.rs",
    "adbx_editor/src/ui/error_dialog/resource.rs",
    "adbx_editor/src/ui/inspector/component_editor_registry.rs",
    "adbx_editor/src/ui/script_editor/resource.rs",
    "adbx_editor/src/ui/realtime_preview/resource.rs",
    "adbx_editor/src/ui/log_panel/resource.rs",
    "adbx_editor/src/ui/rename_dialog/resource.rs",
    "adbx_editor/src/settings/editor_settings/resource.rs",
    "adbx_editor/src/ui/asset_browser/resource.rs",
    "adbx_editor/src/ui/ai_async/resource.rs",
    "adbx_editor/src/systems/selection.rs",
    "adbx_editor/src/systems/menu/menu_click_handler.rs",
    "adbx_editor/src/systems/menu/build_game_menu.rs",
    "adbx_runtime/src/game_config.rs",
    "adbx_runtime/src/lua/component_bridge.rs",
    "adbx_runtime/src/lua/commands_bridge.rs",
    "adbx_runtime/src/lua/resource_bridge.rs",
    "adbx_runtime/src/lua/event_bridge.rs",
    "adbx_runtime/src/lua/transform_bridge.rs",
    "adbx_runtime/src/lua/script_cache.rs",
    "adbx_runtime/src/hot_reload/script_reload.rs",
    "adbx_runtime/src/hot_reload/custom_asset_registry.rs",
    "adbx_editor/src/ui/context_menu/resources/types.rs",
    "adbx_editor/src/systems/build_game/resource.rs",
    "adbx_editor/src/systems/gizmo/resource.rs",
    "adbx_editor/src/ui/editor_layout/resource.rs"
)

foreach ($file in $files) {
    $fullPath = "D:\Develops\Game\adbxEngine\$file"
    if (Test-Path $fullPath) {
        $content = Get-Content $fullPath -Raw

        # #[derive(Resource)] を #[derive(Resource, Reflect)]#[reflect(Resource)] に置換
        $content = $content -replace '#\[derive\(([^)]*?)Resource([^)]*?)\)\]', '#[derive($1Resource$2, Reflect)]`n#[reflect(Resource)]'

        # #[derive(Resource, Default)] を #[derive(Resource, Default, Reflect)]#[reflect(Resource)] に置換
        $content = $content -replace '#\[derive\(([^)]*?)Resource,\s*Default([^)]*?)\)\]', '#[derive($1Resource, Default$2, Reflect)]`n#[reflect(Resource)]'

        # #[derive(Default, Resource)] を #[derive(Default, Resource, Reflect)]#[reflect(Resource)] に置換
        $content = $content -replace '#\[derive\(([^)]*?)Default,\s*Resource([^)]*?)\)\]', '#[derive($1Default, Resource$2, Reflect)]`n#[reflect(Resource)]'

        # 複雑なderiveの場合も対応
        $content = $content -replace '#\[derive\(([^)]*?)Resource([^)]*?)\)\]', '#[derive($1Resource$2, Reflect)]`n#[reflect(Resource)]'

        Set-Content -Path $fullPath -Value $content -Encoding UTF8
        Write-Host "Updated $file"
    } else {
        Write-Host "File not found: $fullPath"
    }
}