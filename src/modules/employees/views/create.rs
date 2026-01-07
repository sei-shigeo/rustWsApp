use crate::components::{Toast, ToastVariant};
use crate::modules::employees::actions::create_employee_action;
use crate::modules::employees::components::{EmployeeCodeInput, NameInput};
use dioxus::prelude::*;

/// 従業員作成フォームコンポーネント（リファクタ済み）
///
/// 目的（初心者向けにコメント多め）:
/// - 入力 UI は小さなコンポーネントに分割しました（`EmployeeCodeInput`, `NameInput`, `SuccessToast`）。
/// - このファイルは「Signal を作成して子コンポーネントに渡す」「送信イベントをトリガーする」
///   という責務だけを持ちます（ビジネスロジックは `actions.rs` に委譲）。
/// - `Signal` を子コンポーネントに渡すことで、子が `value.set(...)` すると親の状態が変わり、
///   親と子で状態を共有できます（双方向バインディング的な使い方）。
#[component]
pub fn EmployeeCreate(on_created: EventHandler<()>) -> Element {
    // --- state: 各フィールドの値 ---
    // 親コンポーネントが Signal を作り、子コンポーネントへ渡します
    let employee_code = use_signal(String::new);
    let first_name = use_signal(String::new);
    let last_name = use_signal(String::new);

    // --- state: 各フィールドのエラーメッセージ ---
    // None => エラー無し、Some(msg) => 表示するエラー文字列
    let employee_code_error = use_signal(|| None::<String>);
    let first_name_error = use_signal(|| None::<String>);
    let last_name_error = use_signal(|| None::<String>);

    // --- 成功メッセージ（トースト） ---
    // Some(msg) のとき表示する。トーストの自動消去は SuccessToast に任せるか親で管理可能。
    let success_message = use_signal(|| None::<String>);

    // 入力があるか（登録ボタンの活性判定用）
    let has_input = !first_name().trim().is_empty() && !last_name().trim().is_empty();

    // エラーがあるか（ボタンの活性判定用）
    let has_error = employee_code_error().is_some()
        || first_name_error().is_some()
        || last_name_error().is_some();

    // SuccessToast に自動で閉じるように頼むためのハンドラを作る
    // EventHandler::new により、子コンポーネント（SuccessToast）から呼べるコールバックを渡します。
    // ここでは呼ばれたら success_message を None にしてトーストを消します。
    // note: clone した Signal を .set するため mutable binding が必要になる場合があります
    let on_toast_dismiss = {
        // .set を呼ぶのでミュータブルにします（Signal は Copy のため clone は不要）
        let mut success_message = success_message;
        EventHandler::new(move |()| {
            success_message.set(None);
        })
    };

    // 送信ボタン押下時のハンドラ
    let handle_submit = {
        // クローンした Signal をクロージャに移す（move）
        // ここで .set を呼ぶために mutable なバインディングにする必要があります。
        // Signal や EventHandler は Copy であることが多いため clone は不要です — そのまま使います。
        let mut employee_code = employee_code;
        let mut first_name = first_name;
        let mut last_name = last_name;
        let mut employee_code_error = employee_code_error;
        let mut first_name_error = first_name_error;
        let mut last_name_error = last_name_error;
        let mut success_message = success_message;
        let on_created = on_created;

        move |e: Event<FormData>| {
            // デフォルトのフォーム送信を防ぐ（ページリロード等を防止）
            e.prevent_default();

            // 値を取得（Signal の現在値をクローン）
            let code_opt = if employee_code().trim().is_empty() {
                None
            } else {
                Some(employee_code())
            };
            let f = first_name();
            let l = last_name();

            // 事前に表示中のエラーをクリア
            employee_code_error.set(None);
            first_name_error.set(None);
            last_name_error.set(None);
            success_message.set(None);

            // 非同期で actions の create アクションを呼ぶ（UI をブロックしない）
            spawn(async move {
                // create_employee_action はクライアント側の簡易バリデーションも行い、
                // サーバーの create_employee を呼びます。戻り値は Result<Employee, String>。
                match create_employee_action(code_opt, f.clone(), l.clone()).await {
                    Ok(employee) => {
                        // 成功時の処理:
                        // - 成功メッセージを表示
                        // - 入力フィールドをリセット
                        // - 親に作成完了イベントを通知（リスト更新など）
                        success_message.set(Some(format!(
                            "作成成功: {last} {first}",
                            last = employee.last_name,
                            first = employee.first_name
                        )));
                        employee_code.set(String::new());
                        first_name.set(String::new());
                        last_name.set(String::new());

                        // on_created は親が渡した EventHandler
                        on_created.call(());
                    }
                    Err(err) => {
                        // サーバーエラー等を個別フィールドのエラーにマッピングすることもできます。
                        // ここでは単純に first_name_error に表示していますが、必要に応じてパースして振り分けてください。
                        first_name_error.set(Some(format!("エラー: {err}")));
                    }
                }
            });
        }
    };

    // UI のレンダリング
    rsx! {
        div { class: "p-4",
            h2 { class: "text-xl font-bold mb-4", "新規従業員登録" }

            form { class: "grid gap-4 mb-4", onsubmit: handle_submit,
                div { class: "grid gap-2",
                    // 子コンポーネントに Signal を渡すことで双方向バインディングが可能
                    EmployeeCodeInput { value: employee_code, error: employee_code_error, exclude_id: None }
                    NameInput { value: last_name, error: last_name_error, label: "姓".to_string(), placeholder: "山田".to_string() }
                    NameInput { value: first_name, error: first_name_error, label: "名".to_string(), placeholder: "太郎".to_string() }
                }

                button {
                    class: "font-bold py-2 px-4 rounded",
                    class: if has_input && !has_error {
                        "bg-blue-500 hover:bg-blue-700 text-white "
                    } else {
                        "bg-gray-300 text-gray-500 cursor-not-allowed"
                    },
                    r#type: "submit",
                    disabled: !has_input || has_error,
                    "登録"
                }
            }

            // 成功メッセージは SuccessToast コンポーネントで表示
            // - 第2引数: duration を Some(3) にして 3 秒で自動閉じするように指示します
            // - 第3引数: on_dismiss を渡して閉じられたときに親（このコンポーネント）が state をクリアするようにします
            if let Some(msg) = success_message() {
                Toast { message: msg, duration: Some(3), on_dismiss: Some(on_toast_dismiss), variant: Some(ToastVariant::Success) }
            }
        }
    }
}
