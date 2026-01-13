pub mod build_execution;
pub mod config_generation;
pub mod resource;
pub mod workspace_detection;

pub use build_execution::build_game;
pub use resource::{BuildGameRequest, BuildProgress};

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use config_generation::{generate_build_config, BuildConfig};
#[allow(unused_imports)]
pub use workspace_detection::find_workspace_root;
