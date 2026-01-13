#[cfg(test)]
mod tests {
    use adbx_editor::ui::file_dialog::*;

    #[test]
    fn test_file_dialog_request_default() {
        let request = FileDialogRequest::default();
        assert!(request.dialog_type.is_none());
        assert!(request.result.is_none());
    }

    #[test]
    fn test_file_filter_structure() {
        let filter = FileFilter {
            name: "Test Files".to_string(),
            extensions: vec!["test".to_string(), "txt".to_string()],
        };
        assert_eq!(filter.name, "Test Files");
        assert_eq!(filter.extensions.len(), 2);
    }

    #[test]
    fn test_filters_all_files() {
        let filters = filters::all_files();
        assert_eq!(filters.len(), 1);
        assert_eq!(filters[0].name, "All Files");
    }

    #[test]
    fn test_filters_code_files() {
        let filters = filters::code_files();
        assert!(filters.len() > 0);
        assert!(filters.iter().any(|f| f.name == "Rust Files"));
        assert!(filters.iter().any(|f| f.name == "Lua Files"));
    }

    #[test]
    fn test_filters_image_files() {
        let filters = filters::image_files();
        assert!(filters.len() > 0);
        assert!(filters.iter().any(|f| f.name == "Image Files"));
    }

    #[test]
    fn test_filters_scene_files() {
        let filters = filters::scene_files();
        assert!(filters.len() > 0);
        assert!(filters.iter().any(|f| f.name == "Scene Files"));
    }
}
