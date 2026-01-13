use thiserror::Error;
use adbx_shared::{AdbxError, ErrorContext, ErrorSeverity};

/// ランタイムエラー型
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Shared error: {0}")]
    Shared(#[from] AdbxError),
    
    #[error("Lua VM error: {message}")]
    LuaVm {
        message: String,
        context: ErrorContext,
        severity: ErrorSeverity,
    },
    
    #[error("Script load error: {message}")]
    ScriptLoad {
        message: String,
        context: ErrorContext,
        severity: ErrorSeverity,
    },
    
    #[error("Hot reload error: {message}")]
    HotReload {
        message: String,
        context: ErrorContext,
        severity: ErrorSeverity,
    },
}

impl RuntimeError {
    /// エラーの優先度を取得
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            RuntimeError::Shared(err) => err.severity(),
            RuntimeError::LuaVm { severity, .. } => *severity,
            RuntimeError::ScriptLoad { severity, .. } => *severity,
            RuntimeError::HotReload { severity, .. } => *severity,
        }
    }
    
    /// エラーコンテキストを取得
    pub fn context(&self) -> &ErrorContext {
        match self {
            RuntimeError::Shared(err) => err.context(),
            RuntimeError::LuaVm { context, .. } => context,
            RuntimeError::ScriptLoad { context, .. } => context,
            RuntimeError::HotReload { context, .. } => context,
        }
    }
    
    /// ユーザーフレンドリーなエラーメッセージを生成
    pub fn user_friendly_message(&self) -> String {
        match self {
            RuntimeError::Shared(err) => err.user_friendly_message(),
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
impl RuntimeError {
    pub fn lua_vm(message: impl Into<String>) -> Self {
        RuntimeError::LuaVm {
            message: message.into(),
            context: ErrorContext::default(),
            severity: ErrorSeverity::Medium,
        }
    }
    
    pub fn script_load(message: impl Into<String>) -> Self {
        RuntimeError::ScriptLoad {
            message: message.into(),
            context: ErrorContext::default(),
            severity: ErrorSeverity::Medium,
        }
    }
    
    pub fn hot_reload(message: impl Into<String>) -> Self {
        RuntimeError::HotReload {
            message: message.into(),
            context: ErrorContext::default(),
            severity: ErrorSeverity::Low,
        }
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;
