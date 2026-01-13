#[cfg(feature = "ai")]
pub mod resource;
#[cfg(feature = "ai")]
pub mod processor;
#[cfg(feature = "ai")]
pub mod openai_client;
#[cfg(feature = "ai")]
pub mod claude_client;

// 公開API
#[cfg(feature = "ai")]
pub use resource::AiAsyncProcessor;
#[cfg(feature = "ai")]
pub use processor::process_ai_request_async;
#[cfg(feature = "ai")]
pub use openai_client::generate_with_openai_async;
#[cfg(feature = "ai")]
pub use claude_client::generate_with_claude_async;
