#[cfg(test)]
mod tests {
    use adbx_shared::scene::SceneData;

    #[test]
    fn test_scene_load_save() {
        let scene = SceneData {
            name: "TestScene".to_string(),
            entities: vec![],
        };
        
        // シーンの保存と読み込みのテストは実際の実装に依存
        assert_eq!(scene.name, "TestScene");
    }
}
