use bevy::prelude::*;
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// ログエントリ
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

impl LogLevel {
    pub fn color(&self) -> Color {
        match self {
            LogLevel::Info => Color::srgb(0.8, 0.8, 0.8),
            LogLevel::Warn => Color::srgb(1.0, 0.8, 0.0),
            LogLevel::Error => Color::srgb(1.0, 0.2, 0.2),
            LogLevel::Debug => Color::srgb(0.5, 0.5, 0.5),
        }
    }
    
    pub fn prefix(&self) -> &'static str {
        match self {
            LogLevel::Info => "[INFO]",
            LogLevel::Warn => "[WARN]",
            LogLevel::Error => "[ERROR]",
            LogLevel::Debug => "[DEBUG]",
        }
    }
}

/// ログパネルのリソース
#[derive(Resource, Default)]
pub struct LogPanel {
    pub logs: VecDeque<LogEntry>,
    pub max_logs: usize,
    pub content_entity: Option<Entity>,
    pub is_visible: bool,
}

impl LogPanel {
    pub fn new() -> Self {
        Self {
            logs: VecDeque::new(),
            max_logs: 1000,
            content_entity: None,
            is_visible: false,
        }
    }
    
    pub fn add_log(&mut self, level: LogLevel, message: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.logs.push_back(LogEntry {
            level,
            message,
            timestamp,
        });
        
        // 最大ログ数を超えた場合、古いログを削除
        while self.logs.len() > self.max_logs {
            self.logs.pop_front();
        }
    }
}
