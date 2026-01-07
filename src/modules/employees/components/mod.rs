/*
 components/mod.rs

 Components ディレクトリ内の各コンポーネントをまとめて公開するためのモジュールです。

 目的:
 - 個々のコンポーネント（name_input, code_input, success_toast）をこのモジュールでまとめて再エクスポートすることで、
   他のコードから `crate::modules::employees::components::NameInput` のように使いやすくします。

 注意（初心者向け）:
 - 各コンポーネント本体は同じディレクトリ内の別ファイルに定義します（例: name_input.rs）。
 - ここでは `pub mod` でサブモジュールを宣言し、`pub use` で名前を公開します。
 - ファイル名とモジュール名は一致させるのが一般的です（`name_input.rs` -> `name_input`）。
*/

pub mod code_input;
pub mod labeled_text_input;
pub mod list_item;
pub mod name_input;

// よく使うコンポーネントをフラットに再エクスポートしておくと、呼び出し元が短く書けて便利です。
// 例: `use crate::modules::employees::components::NameInput;`
//
// 注意: トーストのような汎用コンポーネントはモジュール間で再利用されるため、
//       アプリ共通の `crate::components` （例: `src/components/toast.rs`）へ移行して
//       `crate::components::Toast` を使うことを推奨します。そうすることで page が増えたときの保守性が向上します。
pub use code_input::EmployeeCodeInput;
pub use labeled_text_input::LabeledTextInput;
pub use list_item::EmployeeItem;
pub use name_input::NameInput;
// SuccessToast は汎用トーストへ移行済み／移行予定のため、ローカルで再エクスポートは行いません。
