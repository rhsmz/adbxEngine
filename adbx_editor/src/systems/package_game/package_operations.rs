use std::path::PathBuf;
use std::fs;
use super::asset_copying::copy_assets;
use super::config_copying::copy_config_files;

/// ゲームをパッケージ化する
pub fn package_game(
    project_path: &PathBuf,
    build_output_path: &PathBuf,
) -> Result<(), String> {
    // パッケージディレクトリを作成
    let package_dir = build_output_path.join("game_package");
    fs::create_dir_all(&package_dir)
        .map_err(|e| format!("Failed to create package directory: {}", e))?;
    
    // 実行可能ファイルをコピー
    let exe_name = if cfg!(target_os = "windows") {
        "adbx_runtime.exe"
    } else {
        "adbx_runtime"
    };
    
    let exe_source = build_output_path.join(exe_name);
    let exe_dest = package_dir.join(exe_name);
    
    if !exe_source.exists() {
        return Err(format!("Executable not found: {}", exe_source.display()));
    }
    
    fs::copy(&exe_source, &exe_dest)
        .map_err(|e| format!("Failed to copy executable: {}", e))?;
    
    // アセットをコピー
    copy_assets(project_path, &package_dir)?;
    
    // 設定ファイルをコピー
    copy_config_files(project_path, &package_dir)?;
    
    bevy::log::info!("Game packaged successfully: {}", package_dir.display());
    
    Ok(())
}
