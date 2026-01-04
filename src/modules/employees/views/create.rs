use crate::modules::employees::handlers::create_employee;
use crate::modules::employees::validation::validate_employee_name;
use dioxus::prelude::*;

/// 従業員作成フォームコンポーネント
#[component]
pub fn EmployeeCreate(on_created: EventHandler<()>) -> Element {
    let mut first_name = use_signal(String::new);
    let mut last_name = use_signal(String::new);
    let mut success_message = use_signal(|| None::<String>);
    let mut first_name_error = use_signal(|| None::<String>);
    let mut last_name_error = use_signal(|| None::<String>);

    // 入力があるかチェック
    let has_input = !first_name().trim().is_empty() && !last_name().trim().is_empty();

    // エラーがあるかチェック
    let has_error = first_name_error().is_some() || last_name_error().is_some();

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

    let handle_submit = move |e: Event<FormData>| {
        e.prevent_default();
        let first_name_val = first_name();
        let last_name_val = last_name();

        // エラーメッセージをクリア
        first_name_error.set(None);
        last_name_error.set(None);
        success_message.set(None);

        // バリデーション
        let mut has_error = false;

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
            match create_employee(first_name_val.clone(), last_name_val.clone()).await {
                Ok(employee) => {
                    success_message.set(Some(format!(
                        "作成成功: {} {}",
                        employee.last_name, employee.first_name
                    )));
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

    rsx! {
        div { class: "p-4",
            h3 { class: "text-xl font-bold mb-4", "新規従業員登録" }

            form { class: "grid gap-4 mb-4", onsubmit: handle_submit,

                div { class: "grid gap-2",
                    label { class: "font-bold",
                        span {
                            "姓（Last Name）"
                            if let Some(msg) = last_name_error() {
                                span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                            }
                        }
                        input {
                            class: "border border-gray-400 rounded py-2 px-4 w-full",
                            r#type: "text",
                            placeholder: "山田",
                            value: "{last_name}",
                            required: true,
                            oninput: move |e| {
                                let value = e.value();
                                last_name.set(value.clone());
                                // 入力が空の場合はエラーメッセージをクリア
                                if value.trim().is_empty() {
                                    last_name_error.set(None);
                                } else {
                                    // リアルタイムバリデーション
                                    if let Err(validation_error) = validate_employee_name(&value) {
                                        last_name_error.set(Some(validation_error));
                                    } else {
                                        last_name_error.set(None);
                                    }
                                }
                            },
                        }
                    }

                    label { class: "font-bold",
                        span {
                            "名（First Name）"
                            if let Some(msg) = first_name_error() {
                                span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                            }
                        }
                        input {
                            class: "border border-gray-400 rounded py-2 px-4 w-full",
                            r#type: "text",
                            placeholder: "太郎",
                            value: "{first_name}",
                            required: true,
                            oninput: move |e| {
                                let value = e.value();
                                first_name.set(value.clone());
                                // 入力が空の場合はエラーメッセージをクリア
                                if value.trim().is_empty() {
                                    first_name_error.set(None);
                                } else {
                                    // リアルタイムバリデーション
                                    if let Err(validation_error) = validate_employee_name(&value) {
                                        first_name_error.set(Some(validation_error));
                                    } else {
                                        first_name_error.set(None);
                                    }
                                }
                            },
                        }
                    }
                }

                button {
                    class: if has_input && !has_error {
                        "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                    } else {
                        "bg-gray-300 text-gray-500 font-bold py-2 px-4 rounded cursor-not-allowed"
                    },
                    r#type: "submit",
                    disabled: !has_input || has_error,
                    "登録"
                }
            }

            if let Some(msg) = success_message() {
                div { class: "p-2 rounded absolute top-4 right-4 bg-green-100 text-green-800",
                    "{msg}"
                }
            }
        }
    }
}
