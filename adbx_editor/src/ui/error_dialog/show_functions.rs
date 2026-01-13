use bevy::prelude::*;
use crate::error::EditorError;
use super::{ErrorDialog, ErrorType};

/// エラーダイアログを表示（EditorErrorから）
pub fn show_error_dialog_from_error(
    error_dialog: &mut ResMut<ErrorDialog>,
    error: EditorError,
) {
    let severity = error.severity();
    let user_message = error.user_friendly_message();
    let context = error.context();
    
    error_dialog.is_visible = true;
    error_dialog.title = match severity {
        adbx_shared::ErrorSeverity::Critical => "致命的エラー".to_string(),
        adbx_shared::ErrorSeverity::High => "エラー".to_string(),
        adbx_shared::ErrorSeverity::Medium => "エラー".to_string(),
        adbx_shared::ErrorSeverity::Low => "警告".to_string(),
    };
    error_dialog.message = user_message;
    error_dialog.details = context.stack_trace.clone();
    error_dialog.error_type = match severity {
        adbx_shared::ErrorSeverity::Critical | adbx_shared::ErrorSeverity::High => ErrorType::Error,
        adbx_shared::ErrorSeverity::Medium => ErrorType::Warning,
        adbx_shared::ErrorSeverity::Low => ErrorType::Info,
    };
    error_dialog.error = Some(error);
    error_dialog.show_details = false;
}

/// エラーダイアログを表示（従来の方法）
pub fn show_error_dialog(
    error_dialog: &mut ResMut<ErrorDialog>,
    title: String,
    message: String,
    details: Option<String>,
) {
    error_dialog.is_visible = true;
    error_dialog.title = title;
    error_dialog.message = message;
    error_dialog.details = details;
    error_dialog.error_type = ErrorType::Error;
    error_dialog.error = None;
    error_dialog.show_details = false;
}

/// 警告ダイアログを表示
pub fn show_warning_dialog(
    error_dialog: &mut ErrorDialog,
    title: String,
    message: String,
    details: Option<String>,
) {
    error_dialog.is_visible = true;
    error_dialog.title = title;
    error_dialog.message = message;
    error_dialog.details = details;
    error_dialog.error_type = ErrorType::Warning;
}

/// 情報ダイアログを表示
pub fn show_info_dialog(
    error_dialog: &mut ErrorDialog,
    title: String,
    message: String,
    details: Option<String>,
) {
    error_dialog.is_visible = true;
    error_dialog.title = title;
    error_dialog.message = message;
    error_dialog.details = details;
    error_dialog.error_type = ErrorType::Info;
}

/// エラーダイアログを閉じる
pub fn close_error_dialog(error_dialog: &mut ErrorDialog) {
    error_dialog.is_visible = false;
    error_dialog.title.clear();
    error_dialog.message.clear();
    error_dialog.details = None;
    error_dialog.error = None;
    error_dialog.show_details = false;
}
