pub mod asset;
pub mod components;
pub mod error;
pub mod protocol;
pub mod scene;

pub use asset::*;
pub use components::*;
pub use error::*;
pub use protocol::*;
pub use scene::*;

#[cfg(test)]
mod tests {
    use crate::protocol::{EditorMessage, RuntimeMessage};
    use crate::scene::SceneData;
    use serde_json;

    #[test]
    fn test_editor_message_serialization() {
        let message = EditorMessage::LoadScene {
            scene_path: "scenes/main.scene".to_string(),
        };

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: EditorMessage = serde_json::from_str(&json).unwrap();

        match deserialized {
            EditorMessage::LoadScene { scene_path } => {
                assert_eq!(scene_path, "scenes/main.scene");
            }
            _ => panic!("Unexpected message type"),
        }
    }

    #[test]
    fn test_runtime_message_serialization() {
        let message = RuntimeMessage::SceneLoaded {
            scene: SceneData {
                name: "MainScene".to_string(),
                entities: vec![],
            },
        };

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: RuntimeMessage = serde_json::from_str(&json).unwrap();

        match deserialized {
            RuntimeMessage::SceneLoaded { scene } => {
                assert_eq!(scene.name, "MainScene");
            }
            _ => panic!("Unexpected message type"),
        }
    }

    #[test]
    fn test_scene_serialization_json() {
        let scene = SceneData {
            name: "TestScene".to_string(),
            entities: vec![EntityData {
                id: 1,
                name: "Entity1".to_string(),
                parent: None,
                components: vec![],
            }],
        };

        let json = serde_json::to_string(&scene).unwrap();
        let deserialized: SceneData = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "TestScene");
        assert_eq!(deserialized.entities.len(), 1);
    }

    #[test]
    fn test_scene_serialization_msgpack() {
        let scene = SceneData {
            name: "TestScene".to_string(),
            entities: vec![],
        };

        let msgpack = rmp_serde::to_vec(&scene).unwrap();
        let deserialized: SceneData = rmp_serde::from_slice(&msgpack).unwrap();

        assert_eq!(deserialized.name, "TestScene");
    }

    #[test]
    fn test_asset_type() {
        let asset_type = AssetType::Image;
        assert_eq!(asset_type, AssetType::Image);
    }

    #[test]
    fn test_asset_metadata() {
        let metadata = AssetMetadata {
            id: "test_asset".to_string(),
            path: "assets/test.png".to_string(),
            asset_type: AssetType::Image,
            name: "Test Asset".to_string(),
            size: 1024,
            last_modified: 1234567890,
        };

        assert_eq!(metadata.id, "test_asset");
        assert_eq!(metadata.asset_type, AssetType::Image);
    }

    #[test]
    fn test_asset_reference() {
        let reference = AssetReference {
            asset_id: "test_asset".to_string(),
            path: "assets/test.png".to_string(),
        };

        assert_eq!(reference.asset_id, "test_asset");
    }

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
