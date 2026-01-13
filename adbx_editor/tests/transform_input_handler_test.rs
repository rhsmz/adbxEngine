#[cfg(test)]
mod tests {
    use adbx_editor::ui::inspector::transform_input_handler::input_handling::*;

    #[test]
    fn test_validate_transform_value() {
        use adbx_editor::ui::inspector::inspector_panel_resource::TransformFieldType;

        // Scale値は0より大きい必要がある
        assert!(validate_transform_value(TransformFieldType::ScaleX, 1.0));
        assert!(!validate_transform_value(TransformFieldType::ScaleX, 0.0));
        assert!(!validate_transform_value(TransformFieldType::ScaleX, -1.0));

        // その他の値は常に有効
        assert!(validate_transform_value(
            TransformFieldType::TranslationX,
            0.0
        ));
        assert!(validate_transform_value(
            TransformFieldType::TranslationX,
            -100.0
        ));
    }

    #[test]
    fn test_normalize_transform_value() {
        use adbx_editor::ui::inspector::inspector_panel_resource::TransformFieldType;

        // Scale値は0.01以上に正規化される
        assert_eq!(
            normalize_transform_value(TransformFieldType::ScaleX, 0.0),
            0.01
        );
        assert_eq!(
            normalize_transform_value(TransformFieldType::ScaleX, -1.0),
            0.01
        );
        assert_eq!(
            normalize_transform_value(TransformFieldType::ScaleX, 1.0),
            1.0
        );

        // その他の値はそのまま
        assert_eq!(
            normalize_transform_value(TransformFieldType::TranslationX, 0.0),
            0.0
        );
        assert_eq!(
            normalize_transform_value(TransformFieldType::TranslationX, -100.0),
            -100.0
        );
    }
}
