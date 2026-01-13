#[cfg(test)]
mod tests {
    use adbx_shared::scene::{ComponentData, EntityData, SceneData};
    use rmp_serde;
    use serde_json;

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
}
