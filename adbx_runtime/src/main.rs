use adbx_runtime::game_config::{load_build_config, BuildConfig, GameConfig};
use adbx_runtime::scene_loader::load_initial_scene;
use adbx_runtime::AdbxRuntimePlugin;
use bevy::prelude::*;
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    // コマンドライン引数の解析
    let (scene_path, project_path, window_title, window_width, window_height) = parse_args(&args);

    // プロジェクトパスが指定されている場合、ビルド設定を読み込む
    let mut game_config = if let Some(ref proj_path) = project_path {
        match load_build_config(proj_path) {
            Ok(build_config) => {
                // ビルド設定からGameConfigを作成
                let mut config = GameConfig {
                    scene_path: scene_path.clone(),
                    project_path: project_path.clone(),
                    window_title: build_config.window_title.clone(),
                    window_width: build_config.window_width,
                    window_height: build_config.window_height,
                };

                // シーンパスが指定されていない場合、ビルド設定のメインシーンを使用
                if config.scene_path.is_none() {
                    let main_scene_path = proj_path
                        .join("scenes")
                        .join(format!("{}.json", build_config.main_scene));
                    if main_scene_path.exists() {
                        config.scene_path = Some(main_scene_path);
                    }
                }

                config
            }
            Err(e) => {
                bevy::log::warn!("Failed to load build config: {}, using defaults", e);
                GameConfig {
                    scene_path: scene_path.clone(),
                    project_path: project_path.clone(),
                    window_title: window_title.unwrap_or_else(|| "Adbx Game".to_string()),
                    window_width: window_width.unwrap_or(1920),
                    window_height: window_height.unwrap_or(1080),
                }
            }
        }
    } else {
        GameConfig {
            scene_path: scene_path.clone(),
            project_path: None,
            window_title: window_title.unwrap_or_else(|| "Adbx Game".to_string()),
            window_width: window_width.unwrap_or(1920),
            window_height: window_height.unwrap_or(1080),
        }
    };

    // アセットパスを設定（プロジェクトパスがある場合）
    if let Some(ref proj_path) = game_config.project_path {
        // Bevyのアセットパスをプロジェクトディレクトリに設定
        // 注意: Bevy 0.17では、アセットパスは環境変数や設定で制御される
        // ここでは、プロジェクトパスを基準にアセットを読み込む
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: game_config.window_title.clone(),
                resolution: bevy::window::WindowResolution::new(
                    game_config.window_width as u32,
                    game_config.window_height as u32,
                ),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AdbxRuntimePlugin)
        .insert_resource(game_config)
        .add_systems(Startup, load_initial_scene)
        .run();
}

/// コマンドライン引数を解析
fn parse_args(
    args: &[String],
) -> (
    Option<PathBuf>, // scene_path
    Option<PathBuf>, // project_path
    Option<String>,  // window_title
    Option<u32>,     // window_width
    Option<u32>,     // window_height
) {
    let mut scene_path = None;
    let mut project_path = None;
    let mut window_title = None;
    let mut window_width = None;
    let mut window_height = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--scene" | "-s" => {
                if i + 1 < args.len() {
                    scene_path = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--project" | "-p" => {
                if i + 1 < args.len() {
                    project_path = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--title" | "-t" => {
                if i + 1 < args.len() {
                    window_title = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--width" | "-w" => {
                if i + 1 < args.len() {
                    if let Ok(width) = args[i + 1].parse::<u32>() {
                        window_width = Some(width);
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--height" | "-h" => {
                if i + 1 < args.len() {
                    if let Ok(height) = args[i + 1].parse::<u32>() {
                        window_height = Some(height);
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            _ => {
                // 位置引数としてシーンパスとして扱う
                if scene_path.is_none() && args[i].ends_with(".json")
                    || args[i].ends_with(".msgpack")
                    || args[i].ends_with(".mp")
                {
                    scene_path = Some(PathBuf::from(&args[i]));
                }
                i += 1;
            }
        }
    }

    (
        scene_path,
        project_path,
        window_title,
        window_width,
        window_height,
    )
}

/// 使用方法を表示
fn print_usage() {
    println!("Adbx Runtime - Standalone Game Runner");
    println!();
    println!("Usage: adbx_runtime [OPTIONS] [SCENE_FILE]");
    println!();
    println!("Options:");
    println!("  -s, --scene <PATH>     Scene file path");
    println!("  -p, --project <PATH>   Project directory path");
    println!("  -t, --title <TITLE>    Window title");
    println!("  -w, --width <WIDTH>   Window width (default: 1920)");
    println!("  -h, --height <HEIGHT>  Window height (default: 1080)");
    println!("  --help                 Show this help message");
    println!();
    println!("Examples:");
    println!("  adbx_runtime --project ./my_project");
    println!("  adbx_runtime --scene ./scenes/MainScene.json");
    println!("  adbx_runtime --project ./my_project --title \"My Game\" --width 1280 --height 720");
}
