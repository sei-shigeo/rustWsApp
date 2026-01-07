// 共通コンポーネントを配置するモジュール
// 今後、複数のモジュールで使用される共通のUIコンポーネントを追加可能

pub mod icon;
pub mod nav;
pub mod search_bar;
pub mod toast;

// 汎用トーストを外部から簡単に使えるよう再エクスポートします。
// これにより `crate::components::Toast` / `crate::components::ToastVariant` としてインポートできます。
pub use toast::{Toast, ToastVariant};

// Shared icon component and common icon path constants.
// Re-export here so other modules can import via `crate::components::Icon`
// or `crate::components::IDCARD_LIGHT_ICON`, etc.
pub use icon::{
    Icon, ARROW_DOWN_LIGHT_ICON, ARROW_UP_LIGHT_ICON, IDCARD_LIGHT_ICON, MAP_LIGHT_ICON,
    MOBILE_LIGHT_ICON, PRSON_LIGHT_ICON,
};

// Shared navigation component (sidebar navbar)
pub use nav::Navbar;
