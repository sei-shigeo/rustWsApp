use crate::modules::employees::validation::validate_employee_name;
use dioxus::prelude::*;

/// `NameInput` コンポーネント
///
/// 説明（初心者向けコメントを多めに記載）:
/// - このコンポーネントは「姓」や「名」のような名前入力フィールドを共通化します。
/// - 親コンポーネントが管理する `Signal<String>` を直接受け取り（`value`）、
///   入力が変わったらその Signal を更新します。これにより親と子で状態を共有できます。
/// - `error` は `Signal<Option<String>>` で、検証エラーがあれば Some(String) をセットし、
///   なければ None をセットします（エラーメッセージの表示に使います）。
/// - `label` と `placeholder` は表示用の文字列を所有して受け取ります（`String`）。
///
/// 使い方（親側の例）:
/// ```ignore
/// let mut last_name = use_signal(String::new);
/// let mut last_name_error = use_signal(|| None::<String>);
/// rsx! {
///   NameInput { value: last_name, error: last_name_error, label: "姓".to_string(), placeholder: "山田".to_string() }
/// }
/// ```
///
/// 注意:
/// - バリデーションはここでは簡易なリアルタイム検証のみ行い、最終的な検証は送信時にも行うべきです。
#[component]
pub fn NameInput(
    // 親が持つ Signal をそのまま受け取り、ここで value.set(...) すると親の state が更新されます
    mut value: Signal<String>,
    // エラーメッセージを格納する Signal。None => エラー無し、Some(msg) => エラー表示
    mut error: Signal<Option<String>>,
    // ラベルとプレースホルダーは文字列を受け取る
    label: String,
    placeholder: String,
) -> Element {
    rsx! {
        label { class: "grid gap-0.5",
            // ラベルを表示し、もしエラーがあればその横にエラーメッセージを表示します
            span { class: "ml-2 font-bold", "{label}"
                if let Some(msg) = error() {
                    span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                }
            }
            input {
                // デフォルトのスタイル。プロジェクトの Tailwind 等に合わせて調整して下さい。
                class: "border border-gray-400 rounded py-2 px-4 w-full outline-amber-300",
                r#type: "text",
                placeholder: "{placeholder}",
                // input の表示値として Signal を展開します（Signal をクローンして文字列として表示）
                value: "{value}",
                // oninput は入力が変わるたびに呼ばれます。
                // 引数の型は Dioxus の Event<FormData> を使って値を取り出します。
                oninput: move |e: Event<FormData>| {
                    // 入力値を取得して親の Signal を更新
                    let v = e.value();
                    value.set(v.clone());

                    // 簡易リアルタイムバリデーション:
                    // - 空文字列なら一旦エラーを表示しない（エラーは送信時に出す方が UX が良い場合もある）
                    // - 空でなければ validate_employee_name を呼んでエラーメッセージを設定
                    error.set(if v.trim().is_empty() {
                        None
                    } else {
                        validate_employee_name(&v).err()
                    });
                }
            }
        }
    }
}
