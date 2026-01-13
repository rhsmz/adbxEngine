use std::path::PathBuf;
use thiserror::Error;

/// エラーの優先度
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Low,      // 低優先度（警告レベル）
    Medium,   // 中優先度（通常のエラー）
    High,     // 高優先度（重大なエラー）
    Critical, // 致命的（アプリケーションを停止させる可能性がある）
}

/// エラーコンテキスト
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub file_path: Option<PathBuf>,
    pub line_number: Option<usize>,
    pub column_number: Option<usize>,
    pub function_name: Option<String>,
    pub stack_trace: Option<String>,
    pub additional_info: Option<String>,
}

impl Default for ErrorContext {
    fn default() -> Self {
        Self {
            file_path: None,
            line_number: None,
            column_number: None,
            function_name: None,
            stack_trace: None,
            additional_info: None,
        }
    }
}

/// 共有エラー型
#[derive(Error, Debug)]
pub enum AdbxError {
    #[error("IO error: {source}")]
    Io {
        source: std::io::Error,
        context: ErrorContext,
        severity: ErrorSeverity,
    },

    #[error("Serialization error: {source}")]
    Serialization {
        source: serde_json::Error,
        context: ErrorContext,
        severity: ErrorSeverity,
    },

    #[error("Asset error: {message}")]
    Asset {
        message: String,
        context: ErrorContext,
        severity: ErrorSeverity,
    },

    #[error("Scene error: {message}")]
    Scene {
        message: String,
        context: ErrorContext,
        severity: ErrorSeverity,
    },

    #[error("Lua error: {message}")]
    Lua {
        message: String,
        context: ErrorContext,
        severity: ErrorSeverity,
    },

    #[error("Communication error: {message}")]
    Communication {
        message: String,
        context: ErrorContext,
        severity: ErrorSeverity,
    },

    #[error("Validation error: {message}")]
    Validation {
        message: String,
        context: ErrorContext,
        severity: ErrorSeverity,
    },
}

impl AdbxError {
    /// エラーの優先度を取得
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AdbxError::Io { severity, .. } => *severity,
            AdbxError::Serialization { severity, .. } => *severity,
            AdbxError::Asset { severity, .. } => *severity,
            AdbxError::Scene { severity, .. } => *severity,
            AdbxError::Lua { severity, .. } => *severity,
            AdbxError::Communication { severity, .. } => *severity,
            AdbxError::Validation { severity, .. } => *severity,
        }
    }

    /// エラーコンテキストを取得
    pub fn context(&self) -> &ErrorContext {
        match self {
            AdbxError::Io { context, .. } => context,
            AdbxError::Serialization { context, .. } => context,
            AdbxError::Asset { context, .. } => context,
            AdbxError::Scene { context, .. } => context,
            AdbxError::Lua { context, .. } => context,
            AdbxError::Communication { context, .. } => context,
            AdbxError::Validation { context, .. } => context,
        }
    }

    /// ユーザーフレンドリーなエラーメッセージを生成
    pub fn user_friendly_message(&self) -> String {
        let base_message = format!("{}", self);
        let context = self.context();

        let mut message = base_message;

        if let Some(ref file_path) = context.file_path {
            message.push_str(&format!("\nファイル: {}", file_path.display()));
        }

        if let Some(line) = context.line_number {
            message.push_str(&format!("\n行: {}", line));
            if let Some(col) = context.column_number {
                message.push_str(&format!(", 列: {}", col));
            }
        }

        if let Some(ref func) = context.function_name {
            message.push_str(&format!("\n関数: {}", func));
        }

        if let Some(ref info) = context.additional_info {
            message.push_str(&format!("\n詳細: {}", info));
        }

        // 解決方法を提案
        message.push_str(&self.suggested_solution());

        message
    }

    /// 推奨される解決方法を返す
    fn suggested_solution(&self) -> &str {
        match self {
            AdbxError::Io { .. } => "\n解決方法: ファイルのパスとアクセス権限を確認してください。",
            AdbxError::Serialization { .. } => "\n解決方法: データ形式が正しいか確認してください。",
            AdbxError::Asset { .. } => {
                "\n解決方法: アセットファイルが存在し、正しい形式か確認してください。"
            }
            AdbxError::Scene { .. } => "\n解決方法: シーンファイルの構造を確認してください。",
            AdbxError::Lua { .. } => "\n解決方法: Luaスクリプトの構文を確認してください。",
            AdbxError::Communication { .. } => {
                "\n解決方法: ランタイムプロセスとの接続を確認してください。"
            }
            AdbxError::Validation { .. } => "\n解決方法: 入力データの形式を確認してください。",
        }
    }
}

// 既存のエラー型からの変換を実装
impl From<std::io::Error> for AdbxError {
    fn from(err: std::io::Error) -> Self {
        AdbxError::Io {
            source: err,
            context: ErrorContext::default(),
            severity: ErrorSeverity::Medium,
        }
    }
}

impl From<serde_json::Error> for AdbxError {
    fn from(err: serde_json::Error) -> Self {
        AdbxError::Serialization {
            source: err,
            context: ErrorContext::default(),
            severity: ErrorSeverity::Medium,
        }
    }
}

pub type Result<T> = std::result::Result<T, AdbxError>;
