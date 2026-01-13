pub mod resource;
pub mod requests;
pub mod processing;

// 公開API
pub use resource::{AiIntegration, AiRequestStatus, AiProvider, AiRequest, AiRequestType, AiResponse};
pub use requests::{request_ai_code_generation, request_ai_code_completion, request_ai_code_refactor};
pub use processing::{process_pending_ai_code_generation_requests, handle_ai_code_generation_responses};
