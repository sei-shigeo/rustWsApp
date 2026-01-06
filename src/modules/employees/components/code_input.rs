use crate::modules::employees::handlers::check_employee_code_available;
use crate::modules::employees::validation::validate_employee_code;
use dioxus::prelude::*;

/// EmployeeCodeInput コンポーネント
///
/// 説明（初心者向けコメント多め）:
/// - 親コンポーネントが管理する `Signal<String>`（`value`）を受け取り、入力変化時に直接更新します。
/// - バリデーション結果は `Signal<Option<String>>`（`error`）に格納します。
///   - `None` ならエラー無し、`Some(msg)` ならそのエラーメッセージを表示します。
/// - 形式チェックが通れば非同期でサーバー側に重複チェックを行い、使用不可ならエラーをセットします。
/// - `exclude_id` は編集時に「自分自身を除外」して重複チェックしたい場合に利用します。
///
/// 使い方（親側の例）:
/// ```ignore
/// let mut code = use_signal(String::new);
/// let mut code_error = use_signal(|| None::<String>);
/// rsx! {
///   EmployeeCodeInput { value: code, error: code_error, exclude_id: None }
/// }
/// ```
#[component]
pub fn EmployeeCodeInput(
    value: Signal<String>,
    error: Signal<Option<String>>,
    exclude_id: Option<i32>,
) -> Element {
    rsx! {
        label { class: "grid gap-0.5",
            // ラベルと、もしあればエラーメッセージを表示
            span { class: "ml-2 font-bold", "従業員コード:"
                if let Some(msg) = error() {
                    span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                }
            }
            input {
                class: "border border-gray-400 rounded py-2 px-4 w-full outline-amber-300",
                r#type: "text",
                placeholder: "EMP001",
                // value を表示（Signal を展開）
                value: "{value}",
                // oninput ハンドラ: 入力が変わるたびに呼ばれます
                oninput: move |e: Event<FormData>| {
                    // 入力値を取得して親の Signal を更新
                    let v = e.value();
                    value.set(v.clone());

                    // まずは形式バリデーション（同期）
                    if let Err(err) = validate_employee_code(&v) {
                        // 形式エラーがあれば即座にエラーメッセージをセット
                        error.set(Some(err));
                    } else {
                        // 形式が問題なければ、非同期で重複チェックを実行する
                        // 注意: 非同期クロージャへ移動するので必要な値は clone しておきます
                        let v_clone = v.clone();
                        // Prepare Signal for spawn without calling clone (Signal implements Copy)
                        let mut error_for_spawn = error;
                        // exclude_id は Copy 可能な Option<i32> なのでそのままコピーして move できます
                        let exclude = exclude_id;
                        spawn(async move {
                            // サーバー呼び出し。Ok(true) => 利用可能、Ok(false) => 重複あり
                            match check_employee_code_available(v_clone, exclude).await {
                                Ok(available) => {
                                    if !available {
                                        // 使用できない場合はエラーメッセージをセット
                                        error_for_spawn.set(Some("この従業員コードは既に使用されています".to_string()));
                                    } else {
                                        // 利用可能な場合はエラーをクリア
                                        error_for_spawn.set(None);
                                    }
                                }
                                Err(_) => {
                                    // 通信エラーなどで確認できない場合、ここではエラーをクリアしておく方針
                                    // （必要ならログ出力や再試行の実装を検討してください）
                                    error_for_spawn.set(None);
                                }
                            }
                        });
                    }
                }
            }
        }
    }
}
