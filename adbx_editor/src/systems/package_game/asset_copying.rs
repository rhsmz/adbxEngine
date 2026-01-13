use std::path::PathBuf;
use std::fs;

/// アセットをコピー
pub fn copy_assets(project_path: &PathBuf, package_dir: &PathBuf) -> Result<(), String> {
    let assets_dir = project_path.join("assets");
    let package_assets_dir = package_dir.join("assets");
    
    if !assets_dir.exists() {
        // アセットディレクトリが存在しない場合はスキップ
        return Ok(());
    }
    
    fs::create_dir_all(&package_assets_dir)
        .map_err(|e| format!("Failed to create assets directory: {}", e))?;
    
    copy_directory_recursive(&assets_dir, &package_assets_dir)
        .map_err(|e| format!("Failed to copy assets: {}", e))?;
    
    Ok(())
}

/// ディレクトリを再帰的にコピー
pub fn copy_directory_recursive(source: &PathBuf, dest: &PathBuf) -> Result<(), String> {
    if !source.is_dir() {
        return Err(format!("Source is not a directory: {}", source.display()));
    }
    
    fs::create_dir_all(dest)
        .map_err(|e| format!("Failed to create destination directory: {}", e))?;
    
    let entries = fs::read_dir(source)
        .map_err(|e| format!("Failed to read source directory: {}", e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());
        
        if path.is_dir() {
            copy_directory_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)
                .map_err(|e| format!("Failed to copy file {}: {}", path.display(), e))?;
        }
    }
    
    Ok(())
}
