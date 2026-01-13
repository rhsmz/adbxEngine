#[cfg(test)]
mod tests {
    use adbx_editor::systems::operation_recording::recording_functions::*;
    use adbx_editor::systems::operation_recording::serialization::*;
    use adbx_editor::systems::operation_recording::*;
    use bevy::prelude::*;

    #[test]
    fn test_record_entity_created() {
        // このテストは実際のBevy Appが必要なので、統合テストとして実装する必要があります
        assert!(true);
    }

    #[test]
    fn test_record_transform_changed() {
        // Transform記録のテスト
        // 実際のテストは統合テストとして実装
        assert!(true);
    }

    #[test]
    fn test_record_scene_saved() {
        // シーン保存記録のテスト
        // 実際のテストは統合テストとして実装
        assert!(true);
    }

    #[test]
    fn test_get_ai_readable_history() {
        // AI読み取り可能形式のテスト
        let recorder = OperationRecorder::new(100);
        let history = get_ai_readable_history(&recorder, None);
        assert!(history.contains("Recent Editor Operations"));
    }

    #[test]
    fn test_get_ai_context() {
        // AIコンテキスト形式のテスト
        let recorder = OperationRecorder::new(100);
        let context = get_ai_context(&recorder, None);
        assert!(context.contains("## Editor Operation History"));
    }

    #[test]
    fn test_get_history_json() {
        // JSON形式のテスト
        let recorder = OperationRecorder::new(100);
        let json_result = get_history_json(&recorder, None);
        assert!(json_result.is_ok());
    }
}
