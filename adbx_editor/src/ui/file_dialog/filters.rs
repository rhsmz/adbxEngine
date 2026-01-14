use super::resource::FileFilter;

/// よく使うファイルフィルターの定義
pub fn all_files() -> Vec<FileFilter> {
    vec![FileFilter {
        name: "All Files".to_string(),
        extensions: vec!["*".to_string()],
    }]
}

pub fn code_files() -> Vec<FileFilter> {
    vec![
        FileFilter {
            name: "Rust Files".to_string(),
            extensions: vec!["rs".to_string()],
        },
        FileFilter {
            name: "Lua Files".to_string(),
            extensions: vec!["lua".to_string()],
        },
        FileFilter {
            name: "All Code Files".to_string(),
            extensions: vec![
                "rs".to_string(),
                "lua".to_string(),
                "py".to_string(),
                "js".to_string(),
                "ts".to_string(),
            ],
        },
        FileFilter {
            name: "All Files".to_string(),
            extensions: vec!["*".to_string()],
        },
    ]
}

#[allow(dead_code)]
pub fn image_files() -> Vec<FileFilter> {
    vec![
        FileFilter {
            name: "Image Files".to_string(),
            extensions: vec![
                "png".to_string(),
                "jpg".to_string(),
                "jpeg".to_string(),
                "bmp".to_string(),
                "tga".to_string(),
                "dds".to_string(),
            ],
        },
        FileFilter {
            name: "All Files".to_string(),
            extensions: vec!["*".to_string()],
        },
    ]
}

#[allow(dead_code)]
pub fn scene_files() -> Vec<FileFilter> {
    vec![
        FileFilter {
            name: "Scene Files".to_string(),
            extensions: vec!["scene".to_string(), "scn".to_string()],
        },
        FileFilter {
            name: "All Files".to_string(),
            extensions: vec!["*".to_string()],
        },
    ]
}
