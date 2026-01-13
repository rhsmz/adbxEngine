use std::path::PathBuf;
use std::fs;
use super::package_operations::package_game;
use super::asset_copying::copy_directory_recursive;

/// プラットフォーム
#[derive(Debug, Clone, Copy)]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
}

/// プラットフォーム別のパッケージ化
pub fn package_game_platform(
    project_path: &PathBuf,
    build_output_path: &PathBuf,
    platform: Platform,
) -> Result<PathBuf, String> {
    // 基本的なパッケージ化を実行
    package_game(project_path, build_output_path)?;
    
    let package_dir = build_output_path.join("game_package");
    
    match platform {
        Platform::Windows => {
            // Windows: フォルダのまま（必要に応じてZIP化）
            Ok(package_dir)
        }
        Platform::Linux => {
            // Linux: tar.gzを作成
            create_tar_gz(&package_dir)
        }
        Platform::MacOS => {
            // macOS: .appバンドルを作成（簡易実装）
            create_app_bundle(&package_dir)
        }
    }
}

/// tar.gzを作成（Linux用）
fn create_tar_gz(package_dir: &PathBuf) -> Result<PathBuf, String> {
    // 簡易実装: tarコマンドを使用（実際の実装では、tarライブラリを使用することを推奨）
    let tar_path = package_dir.parent()
        .ok_or("Cannot get parent directory")?
        .join(format!("{}.tar.gz", package_dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("game")));
    
    // tarコマンドが利用可能な場合のみ実行
    #[cfg(unix)]
    {
        use std::process::Command;
        
        let output = Command::new("tar")
            .arg("-czf")
            .arg(&tar_path)
            .arg("-C")
            .arg(package_dir.parent().unwrap())
            .arg(package_dir.file_name().unwrap())
            .output();
        
        if let Ok(output) = output {
            if !output.status.success() {
                return Err(format!("Failed to create tar.gz: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }
    }
    
    Ok(tar_path)
}

/// .appバンドルを作成（macOS用）
fn create_app_bundle(package_dir: &PathBuf) -> Result<PathBuf, String> {
    // 簡易実装: .appディレクトリ構造を作成
    let app_name = package_dir.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("game");
    let app_path = package_dir.parent()
        .ok_or("Cannot get parent directory")?
        .join(format!("{}.app", app_name));
    
    let contents_path = app_path.join("Contents");
    let macos_path = contents_path.join("MacOS");
    
    fs::create_dir_all(&macos_path)
        .map_err(|e| format!("Failed to create .app bundle: {}", e))?;
    
    // 実行可能ファイルを移動
    let exe_name = "adbx_runtime";
    let exe_source = package_dir.join(exe_name);
    let exe_dest = macos_path.join(exe_name);
    
    if exe_source.exists() {
        fs::copy(&exe_source, &exe_dest)
            .map_err(|e| format!("Failed to copy executable to .app bundle: {}", e))?;
    }
    
    // Resourcesディレクトリを作成してアセットをコピー
    let resources_path = contents_path.join("Resources");
    if package_dir.join("assets").exists() {
        fs::create_dir_all(&resources_path)
            .map_err(|e| format!("Failed to create Resources directory: {}", e))?;
        
        copy_directory_recursive(&package_dir.join("assets"), &resources_path.join("assets"))?;
    }
    
    Ok(app_path)
}
