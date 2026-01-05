use crate::modules::employees::handlers::{check_employee_code_available, create_employee};
use crate::modules::employees::validation::{validate_employee_code, validate_employee_name};
use dioxus::prelude::*;

/// 従業員作成フォームコンポーネント
#[component]
pub fn EmployeeCreate(on_created: EventHandler<()>) -> Element {
    let mut employee_code = use_signal(String::new);
    let mut first_name = use_signal(String::new);
    let mut last_name = use_signal(String::new);
    let mut success_message = use_signal(|| None::<String>);
    let mut employee_code_error = use_signal(|| None::<String>);
    let mut first_name_error = use_signal(|| None::<String>);
    let mut last_name_error = use_signal(|| None::<String>);

    // 入力があるかチェック
    let has_input = !first_name().trim().is_empty() && !last_name().trim().is_empty();

    // エラーがあるかチェック
    let has_error = employee_code_error().is_some()
        || first_name_error().is_some()
        || last_name_error().is_some();

    // 成功メッセージを3秒後に消す
    use_effect(move || {
        if success_message().is_some() {
            spawn(async move {
                #[cfg(target_family = "wasm")]
                gloo_timers::future::sleep(std::time::Duration::from_secs(3)).await;

                #[cfg(not(target_family = "wasm"))]
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;

                success_message.set(None);
            });
        }
    });

    // 送信処理
    let handle_submit = move |e: Event<FormData>| {
        e.prevent_default();
        let first_name_val = first_name();
        let last_name_val = last_name();

        let employee_code_val = employee_code();

        // エラーメッセージをクリア
        employee_code_error.set(None);
        first_name_error.set(None);
        last_name_error.set(None);
        success_message.set(None);

        // バリデーション
        let mut has_error = false;

        if let Err(validation_error) = validate_employee_code(&employee_code_val) {
            employee_code_error.set(Some(validation_error));
            has_error = true;
        }

        if let Err(validation_error) = validate_employee_name(&first_name_val) {
            first_name_error.set(Some(validation_error));
            has_error = true;
        }

        if let Err(validation_error) = validate_employee_name(&last_name_val) {
            last_name_error.set(Some(validation_error));
            has_error = true;
        }

        if has_error {
            return;
        }

        spawn(async move {
            match create_employee(
                Some(employee_code_val),
                first_name_val.clone(),
                last_name_val.clone(),
            )
            .await
            {
                Ok(employee) => {
                    success_message.set(Some(format!(
                        "作成成功: {} {}",
                        employee.last_name, employee.first_name
                    )));
                    employee_code.set(String::new());
                    first_name.set(String::new());
                    last_name.set(String::new());
                    on_created.call(());
                }
                Err(e) => {
                    first_name_error.set(Some(format!("エラー: {}", e)));
                }
            }
        });
    };

    // 各フィールド用のハンドラーを生成
    let create_name_handler = |mut name: Signal<String>, mut error: Signal<Option<String>>| {
        move |e: Event<FormData>| {
            let value = e.value();
            name.set(value.clone());
            error.set(if value.trim().is_empty() {
                None
            } else {
                validate_employee_name(&value).err()
            });
        }
    };

    let handle_first_name = create_name_handler(first_name, first_name_error);
    let handle_last_name = create_name_handler(last_name, last_name_error);
    rsx! {
        div { class: "p-4",
            h3 { class: "text-xl font-bold mb-4", "新規従業員登録" }

            form { class: "grid gap-4 mb-4", onsubmit: handle_submit,

                div { class: "grid gap-2",

                    label { class: "grid gap-0.5",
                        span { class: "ml-2 font-bold", "従業員コード:"
                            if let Some(msg) = employee_code_error() {
                                span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                            }
                        }
                        input {
                            class: "border border-gray-400 rounded py-2 px-4 w-full outline-amber-300",
                            r#type: "text",
                            name: "employee_code",
                            placeholder: "EMP001",
                            value: "{employee_code}",
                            oninput: move |e: Event<FormData>| {
                                let value = e.value();
                                employee_code.set(value.clone());

                                // リアルタイムバリデーション
                                if let Err(err) = validate_employee_code(&value) {
                                    employee_code_error.set(Some(err));
                                } else {
                                    // 形式OKなら重複チェック
                                    spawn(async move {
                                        match check_employee_code_available(value, None).await {
                                            Ok(available) => {
                                                if !available {
                                                    employee_code_error.set(Some(
                                                        "この従業員コードは既に使用されています".to_string()
                                                    ));
                                                } else {
                                                    employee_code_error.set(None);
                                                }
                                            }
                                            Err(_) => {
                                                // ネットワークエラーは無視
                                                employee_code_error.set(None);
                                            }
                                        }
                                    });
                                }
                            }
                        }
                    }

                    label { class: "grid gap-0.5",
                        span { class: "ml-2 font-bold", "姓:"
                            if let Some(msg) = last_name_error() {
                                span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                            }
                        }
                        input {
                            class: "border border-gray-400 rounded py-2 px-4 w-full outline-amber-300",
                            r#type: "text",
                            name: "last_name",
                            placeholder: "山田",
                            value: "{last_name}",
                            required: true,
                            oninput: handle_last_name
                        }
                    }

                    label { class: "grid gap-0.5",
                        span { class: "ml-2 font-bold", "名:",
                            if let Some(msg) = first_name_error() {
                                span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                            }
                        }
                        input {
                            class: "border border-gray-400 rounded py-2 px-4 w-full outline-amber-300",
                            r#type: "text",
                            name: "first_name",
                            placeholder: "太郎",
                            value: "{first_name}",
                            required: true,
                            oninput: handle_first_name
                        }
                    }
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

            // 成功時に画面に表示される
            if let Some(msg) = success_message() {
                div { class: "p-2 rounded absolute top-4 right-4 bg-green-100 text-green-800",
                    "{msg}"
                }
            }
        }
    }
}
