pub mod candidate_generation;
pub mod rust_keywords;
pub mod lua_keywords;
pub mod python_keywords;
pub mod javascript_keywords;
pub mod common_keywords;
pub mod identifier_extraction;

// 公開API
pub use candidate_generation::get_completion_candidates;
