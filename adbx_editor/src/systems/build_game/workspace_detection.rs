use std::path::PathBuf;

/// ワークスペースルートを見つける
pub fn find_workspace_root(project_path: &PathBuf) -> Result<PathBuf, String> {
    // プロジェクトパスからCargo.tomlを探す
    let mut current = project_path.clone();
    
    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            // Cargo.tomlを読み込んで、workspaceかどうか確認
            if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
                if content.contains("[workspace]") {
                    return Ok(current);
                }
            }
        }
        
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            break;
        }
    }
    
    // 見つからない場合、プロジェクトパスの親ディレクトリを返す
    project_path.parent()
        .ok_or_else(|| "Cannot find workspace root".to_string())
        .map(|p| p.to_path_buf())
}
