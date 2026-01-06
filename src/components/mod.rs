// 共通コンポーネントを配置するモジュール
// 今後、複数のモジュールで使用される共通のUIコンポーネントを追加可能

pub mod search_bar;
pub mod toast;

// 汎用トーストを外部から簡単に使えるよう再エクスポートします。
// これにより `crate::components::Toast` / `crate::components::ToastVariant` としてインポートできます。
pub use toast::{Toast, ToastVariant};
