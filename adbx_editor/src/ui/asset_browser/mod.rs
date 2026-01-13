pub mod resource;
pub mod directory_scan;
pub mod ui_draw;
pub mod click_handler;
pub mod import_export;
pub mod image_preview;

// 公開API
pub use resource::{AssetBrowser, AssetFileInfo, AssetType};
pub use directory_scan::AssetBrowserExt;
pub use ui_draw::draw_asset_browser;
pub use click_handler::handle_asset_browser_click;
pub use import_export::{import_asset, export_asset};
pub use image_preview::update_image_previews;

// AssetBrowserにscan_directoryメソッドを追加
impl AssetBrowser {
    pub fn scan_directory(&mut self) {
        AssetBrowserExt::scan_directory(self);
    }
}
