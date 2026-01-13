pub mod asset_copying;
pub mod config_copying;
pub mod package_operations;
pub mod platform_specific;

// 公開API
pub use package_operations::package_game;

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use asset_copying::copy_assets;
#[allow(unused_imports)]
pub use config_copying::copy_config_files;
#[allow(unused_imports)]
pub use platform_specific::{package_game_platform, Platform};
