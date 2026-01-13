use std::path::PathBuf;
use std::fs;
use super::asset_copying::copy_directory_recursive;

/// 設定ファイルをコピー
pub fn copy_config_files(project_path: &PathBuf, package_dir: &PathBuf) -> Result<(), String> {
    // build_config.jsonをコピー
    let build_config_source = project_path.join("build_config.json");
    if build_config_source.exists() {
        let build_config_dest = package_dir.join("build_config.json");
        fs::copy(&build_config_source, &build_config_dest)
            .map_err(|e| format!("Failed to copy build config: {}", e))?;
    }
    
    // scenesディレクトリをコピー
    let scenes_source = project_path.join("scenes");
    if scenes_source.exists() {
        let scenes_dest = package_dir.join("scenes");
        fs::create_dir_all(&scenes_dest)
            .map_err(|e| format!("Failed to create scenes directory: {}", e))?;
        
        copy_directory_recursive(&scenes_source, &scenes_dest)
            .map_err(|e| format!("Failed to copy scenes: {}", e))?;
    }
    
    Ok(())
}
