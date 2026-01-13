#[cfg(test)]
mod tests {
    use adbx_shared::protocol::{EditorMessage, RuntimeMessage};

    #[test]
    fn test_communication_serialization() {
        let message = EditorMessage::LoadScene {
            scene_path: "scenes/test.scene".to_string(),
        };
        
        let json = serde_json::to_string(&message).unwrap();
        let deserialized: EditorMessage = serde_json::from_str(&json).unwrap();
        
        match deserialized {
            EditorMessage::LoadScene { scene_path } => {
                assert_eq!(scene_path, "scenes/test.scene");
            }
            _ => panic!("Unexpected message type"),
        }
    }
}
