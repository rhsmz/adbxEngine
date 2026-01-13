#[cfg(test)]
mod tests {
    use adbx_shared::error::{AdbxError, ErrorContext, ErrorSeverity};
    use std::io;

    #[test]
    fn test_error_severity() {
        let error = AdbxError::Asset {
            message: "Test error".to_string(),
            context: ErrorContext::default(),
            severity: ErrorSeverity::High,
        };

        assert_eq!(error.severity(), ErrorSeverity::High);
    }

    #[test]
    fn test_error_context() {
        let context = ErrorContext {
            file_path: Some(std::path::PathBuf::from("test.rs")),
            line_number: Some(42),
            column_number: Some(10),
            function_name: Some("test".to_string()),
            stack_trace: None,
            additional_info: None,
        };

        let error = AdbxError::Asset {
            message: "Test error".to_string(),
            context,
            severity: ErrorSeverity::Medium,
        };

        assert_eq!(error.context().line_number, Some(42));
    }

    #[test]
    fn test_error_from_io() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let adbx_error: AdbxError = io_error.into();

        assert_eq!(adbx_error.severity(), ErrorSeverity::Medium);
    }
}
