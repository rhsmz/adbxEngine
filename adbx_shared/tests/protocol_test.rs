#[cfg(test)]
mod tests {
    use adbx_shared::protocol::{EditorMessage, RuntimeMessage};
    use adbx_shared::scene::SceneData;
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
}
