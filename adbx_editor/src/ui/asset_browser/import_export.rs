use std::path::{Path, PathBuf};

/// アセットをインポート
pub fn import_asset(source_path: PathBuf, target_directory: &Path) -> Result<PathBuf, String> {
    if !source_path.exists() {
        return Err(format!("ソースファイルが存在しません: {:?}", source_path));
    }

    if !source_path.is_file() {
        return Err(format!(
            "ソースパスはファイルではありません: {:?}",
            source_path
        ));
    }

    if !target_directory.exists() {
        std::fs::create_dir_all(target_directory)
            .map_err(|e| format!("ターゲットディレクトリの作成に失敗しました: {}", e))?;
    }

    let file_name = source_path
        .file_name()
        .ok_or_else(|| "無効なファイル名です".to_string())?;

    let target_path = target_directory.join(file_name);

    // 既に同じファイルが存在する場合は、ファイル名を変更
    let final_target_path = if target_path.exists() {
        // ファイル名に番号を追加（例: file.txt -> file (1).txt）
        let stem = target_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("file");
        let extension = target_path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| format!(".{}", s))
            .unwrap_or_else(|| "".to_string());

        let mut counter = 1;
        let mut new_path = target_directory.join(format!("{} ({}){}", stem, counter, extension));
        while new_path.exists() {
            counter += 1;
            new_path = target_directory.join(format!("{} ({}){}", stem, counter, extension));
        }
        new_path
    } else {
        target_path
    };

    // ファイルをコピー
    std::fs::copy(&source_path, &final_target_path)
        .map_err(|e| format!("ファイルのコピーに失敗しました: {}", e))?;

    Ok(final_target_path)
}

/// アセットをエクスポート
pub fn export_asset(source_path: &Path, target_path: PathBuf) -> Result<(), String> {
    if !source_path.exists() {
        return Err(format!("ソースファイルが存在しません: {:?}", source_path));
    }

    if !source_path.is_file() {
        return Err(format!(
            "ソースパスはファイルではありません: {:?}",
            source_path
        ));
    }

    // ターゲットディレクトリを作成
    if let Some(parent) = target_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("ターゲットディレクトリの作成に失敗しました: {}", e))?;
        }
    }

    // 既に同じファイルが存在する場合は警告（上書き）
    if target_path.exists() {
        bevy::log::warn!(
            "エクスポート先のファイルが既に存在します。上書きします: {:?}",
            target_path
        );
    }

    // ファイルをコピー
    std::fs::copy(source_path, &target_path)
        .map_err(|e| format!("ファイルのコピーに失敗しました: {}", e))?;

    Ok(())
}
