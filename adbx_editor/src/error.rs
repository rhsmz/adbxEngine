use adbx_shared::{AdbxError, ErrorContext, ErrorSeverity};
use thiserror::Error;

/// エディタエラー型
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum EditorError {
    #[error("Shared error: {0}")]
    Shared(#[from] AdbxError),

    #[error("Project error: {message}")]
    Project {
        message: String,
        context: ErrorContext,
        severity: ErrorSeverity,
    },

    #[error("UI error: {message}")]
    Ui {
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

    #[error("AI integration error: {message}")]
    AiIntegration {
        message: String,
        context: ErrorContext,
        severity: ErrorSeverity,
    },
}

impl EditorError {
    /// エラーの優先度を取得
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            EditorError::Shared(err) => err.severity(),
            EditorError::Project { severity, .. } => *severity,
            EditorError::Ui { severity, .. } => *severity,
            EditorError::Communication { severity, .. } => *severity,
            EditorError::AiIntegration { severity, .. } => *severity,
        }
    }

    /// エラーコンテキストを取得
    pub fn context(&self) -> &ErrorContext {
        match self {
            EditorError::Shared(err) => err.context(),
            EditorError::Project { context, .. } => context,
            EditorError::Ui { context, .. } => context,
            EditorError::Communication { context, .. } => context,
            EditorError::AiIntegration { context, .. } => context,
        }
    }

    /// ユーザーフレンドリーなエラーメッセージを生成
    pub fn user_friendly_message(&self) -> String {
        match self {
            EditorError::Shared(err) => err.user_friendly_message(),
            _ => {
                let base_message = format!("{}", self);
                let context = self.context();

                let mut message = base_message;

                if let Some(ref file_path) = context.file_path {
                    message.push_str(&format!("\nファイル: {}", file_path.display()));
                }

                if let Some(line) = context.line_number {
                    message.push_str(&format!("\n行: {}", line));
                }

                if let Some(ref info) = context.additional_info {
                    message.push_str(&format!("\n詳細: {}", info));
                }

                message
            }
        }
    }
}

// 簡易的なエラー作成ヘルパー
#[allow(dead_code)]
impl EditorError {
    pub fn project(message: impl Into<String>) -> Self {
        EditorError::Project {
            message: message.into(),
            context: ErrorContext::default(),
            severity: ErrorSeverity::Medium,
        }
    }

    pub fn ui(message: impl Into<String>) -> Self {
        EditorError::Ui {
            message: message.into(),
            context: ErrorContext::default(),
            severity: ErrorSeverity::Low,
        }
    }

    pub fn communication(message: impl Into<String>) -> Self {
        EditorError::Communication {
            message: message.into(),
            context: ErrorContext::default(),
            severity: ErrorSeverity::High,
        }
    }
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, EditorError>;
