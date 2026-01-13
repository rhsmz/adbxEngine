use std::path::PathBuf;

/// リネームの検証
pub fn validate_rename_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("名前が空です".to_string());
    }
    
    if name.contains('/') || name.contains('\\') || name.contains(':') {
        return Err("名前に無効な文字が含まれています".to_string());
    }
    
    Ok(())
}

/// リネームパスの生成
pub fn generate_rename_path(target_path: &PathBuf, new_name: &str) -> PathBuf {
    if let Some(parent) = target_path.parent() {
        parent.join(new_name)
    } else {
        PathBuf::from(new_name)
    }
}
