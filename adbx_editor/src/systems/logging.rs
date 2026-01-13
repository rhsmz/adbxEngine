use bevy::prelude::*;
use crate::ui::log_panel::{LogPanel, LogLevel};

/// BevyのログをLogPanelに転送するシステム
/// 注意: これは簡易実装です。実際の実装では、Bevyのカスタムログプラグインを使用する必要があります
pub fn capture_bevy_logs(
    _log_panel: ResMut<LogPanel>,
) {
    // Bevyのログシステムから直接ログを取得することは難しいため、
    // このシステムは他のシステムから呼び出されるヘルパー関数として使用されます
    // 実際の実装では、BevyのLogPluginを拡張してカスタムロガーを実装する必要があります
}

/// ログを追加するヘルパー関数
pub fn add_log(log_panel: &mut LogPanel, level: LogLevel, message: String) {
    log_panel.add_log(level, message);
}

/// エラーログを追加
pub fn log_error(log_panel: &mut LogPanel, message: String) {
    add_log(log_panel, LogLevel::Error, message);
}

/// 警告ログを追加
pub fn log_warn(log_panel: &mut LogPanel, message: String) {
    add_log(log_panel, LogLevel::Warn, message);
}

/// 情報ログを追加
pub fn log_info(log_panel: &mut LogPanel, message: String) {
    add_log(log_panel, LogLevel::Info, message);
}

/// デバッグログを追加
pub fn log_debug(log_panel: &mut LogPanel, message: String) {
    add_log(log_panel, LogLevel::Debug, message);
}
