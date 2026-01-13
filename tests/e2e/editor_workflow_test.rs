#[cfg(test)]
mod tests {
    use adbx_shared::scene::SceneData;

    #[test]
    fn test_scene_creation() {
        let scene = SceneData {
            name: "TestScene".to_string(),
            entities: vec![],
        };
        
        assert_eq!(scene.name, "TestScene");
    }
}
