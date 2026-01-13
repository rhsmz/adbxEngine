#[cfg(test)]
mod tests {
    use adbx_editor::ui::log_panel::*;

    #[test]
    fn test_log_panel_new() {
        let panel = LogPanel::new();
        assert_eq!(panel.logs.len(), 0);
        assert_eq!(panel.max_logs, 1000);
        assert!(panel.content_entity.is_none());
        assert!(!panel.is_visible);
    }

    #[test]
    fn test_log_panel_add_log() {
        let mut panel = LogPanel::new();
        panel.add_log(LogLevel::Info, "Test message".to_string());
        assert_eq!(panel.logs.len(), 1);
        assert_eq!(panel.logs[0].level, LogLevel::Info);
        assert_eq!(panel.logs[0].message, "Test message");
    }

    #[test]
    fn test_log_panel_max_logs() {
        let mut panel = LogPanel::new();
        panel.max_logs = 5;

        for i in 0..10 {
            panel.add_log(LogLevel::Info, format!("Message {}", i));
        }

        assert_eq!(panel.logs.len(), 5);
    }

    #[test]
    fn test_log_level_color() {
        assert_eq!(LogLevel::Info.color().r(), 0.8);
        assert_eq!(LogLevel::Warn.color().r(), 1.0);
        assert_eq!(LogLevel::Error.color().r(), 1.0);
        assert_eq!(LogLevel::Debug.color().r(), 0.5);
    }

    #[test]
    fn test_log_level_prefix() {
        assert_eq!(LogLevel::Info.prefix(), "[INFO]");
        assert_eq!(LogLevel::Warn.prefix(), "[WARN]");
        assert_eq!(LogLevel::Error.prefix(), "[ERROR]");
        assert_eq!(LogLevel::Debug.prefix(), "[DEBUG]");
    }
}
