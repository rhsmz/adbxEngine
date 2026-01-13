pub mod lua_tokenizer;
pub mod syntect_highlight;

// 公開API
pub use syntect_highlight::highlight_lua_text;
