#[cfg(test)]
mod tests {
    use adbx_shared::asset::{AssetMetadata, AssetReference, AssetType};

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
}
