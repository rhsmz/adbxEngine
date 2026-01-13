/// ファイル名のバリデーション
pub fn validate_filename(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("ファイル名が空です".to_string());
    }
    
    // Windowsで無効な文字をチェック
    let invalid_chars = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    for ch in invalid_chars {
        if name.contains(ch) {
            return Err(format!("ファイル名に無効な文字 '{}' が含まれています", ch));
        }
    }
    
    // 予約名をチェック（Windows）
    let reserved_names = ["CON", "PRN", "AUX", "NUL", 
                          "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
                          "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"];
    if let Some(name_upper) = name.split('.').next() {
        if reserved_names.contains(&name_upper.to_uppercase().as_str()) {
            return Err(format!("'{}' は予約されたファイル名です", name_upper));
        }
    }
    
    // 末尾のドットやスペースをチェック（Windows）
    if name.ends_with('.') || name.ends_with(' ') {
        return Err("ファイル名はドットやスペースで終了できません".to_string());
    }
    
    Ok(())
}
