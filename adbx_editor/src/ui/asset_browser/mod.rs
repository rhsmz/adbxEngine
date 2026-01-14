pub mod click_handler;
pub mod directory_scan;
pub mod image_preview;
pub mod import_export;
pub mod resource;
pub mod ui_draw;

// 公開API
pub use click_handler::handle_asset_browser_click;
pub use directory_scan::AssetBrowserExt;
pub use image_preview::update_image_previews;
pub use resource::{AssetBrowser, AssetFileInfo};
pub use ui_draw::draw_asset_browser;

// 未使用だが将来使用予定のAPI
#[allow(unused_imports)]
pub use import_export::{export_asset, import_asset};
#[allow(unused_imports)]
pub use resource::AssetType;

// AssetBrowserにscan_directoryメソッドを追加
impl AssetBrowser {
    pub fn scan_directory(&mut self) {
        AssetBrowserExt::scan_directory(self);
    }
}
