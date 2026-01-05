use crate::modules::employees::handlers::{
    check_employee_code_available, delete_employee, update_employee,
};
use crate::modules::employees::models::Employee;
use crate::modules::employees::validation::{validate_employee_code, validate_employee_name};
use dioxus::prelude::*;

/// 従業員アイテムコンポーネント（表示・編集切り替え）
#[component]
pub fn EmployeeItem(
    employee: Employee,
    editing_id: Signal<Option<i32>>,
    on_refresh: EventHandler<()>,
) -> Element {
    let mut employee_code = use_signal(|| employee.employee_code.clone().unwrap_or_default());
    let mut first_name = use_signal(|| employee.first_name.clone());
    let mut last_name = use_signal(|| employee.last_name.clone());
    let mut is_active = use_signal(|| employee.is_active);
    let mut success_message = use_signal(|| None::<String>);
    let mut employee_code_error = use_signal(|| None::<String>);
    let mut first_name_error = use_signal(|| None::<String>);
    let mut last_name_error = use_signal(|| None::<String>);
    let is_editing = editing_id() == Some(employee.id);

    // 変更があるかチェック
    let has_changes = employee_code() != employee.employee_code.clone().unwrap_or_default()
        || first_name() != employee.first_name
        || last_name() != employee.last_name
        || is_active() != employee.is_active;

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

    let mut handle_save = move || {
        let employee_id = employee.id;
        let employee_code_val = employee_code();
        let first_name_val = first_name();
        let last_name_val = last_name();
        let is_active_val = is_active();

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
            match update_employee(
                employee_id,
                Some(employee_code_val),
                first_name_val.clone(),
                last_name_val.clone(),
                is_active_val,
            )
            .await
            {
                Ok(_) => {
                    success_message.set(Some(format!(
                        "更新成功: {} {}",
                        last_name_val, first_name_val
                    )));
                    editing_id.set(None);
                    on_refresh.call(());
                }
                Err(e) => {
                    first_name_error.set(Some(format!("エラー: {}", e)));
                }
            }
        });
    };

    let handle_delete = move || {
        let employee_id = employee.id;
        spawn(async move {
            match delete_employee(employee_id).await {
                Ok(_) => {
                    on_refresh.call(());
                }
                Err(e) => {
                    eprintln!("削除エラー: {}", e);
                }
            }
        });
    };

    rsx! {
        div { class: "border p-4 rounded",
            if is_editing {
                form {
                    class: "grid gap-2",
                    onsubmit: move |e: Event<FormData>| {
                        e.prevent_default();
                        handle_save();
                    },

                    div { class: "flex gap-2 items-center",
                        label { class: "font-bold w-24", "ID:" }
                        span { "{employee.id}" }
                    }

                    div { class: "flex gap-2 items-center",
                        label { class: "font-bold w-24",
                            span {
                                "従業員コード:"
                                if let Some(msg) = employee_code_error() {
                                    span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                                }
                            }
                        }
                        input {
                            class: "border border-gray-400 rounded py-1 px-2 flex-1",
                            r#type: "text",
                            value: "{employee_code}",
                            oninput: move |e| {
                                let value = e.value();
                                employee_code.set(value.clone());

                                // リアルタイムバリデーション
                                if let Err(err) = validate_employee_code(&value) {
                                    employee_code_error.set(Some(err));
                                } else {
                                    // 形式OKなら重複チェック（自分自身は除外）
                                    let employee_id = employee.id;
                                    spawn(async move {
                                        match check_employee_code_available(value, Some(employee_id)).await {
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
                            },
                        }
                    }

                    div { class: "flex gap-2 items-center",
                        label { class: "font-bold w-24",
                            span {
                                "姓:"
                                if let Some(msg) = last_name_error() {
                                    span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                                }
                            }
                        }
                        input {
                            class: "border border-gray-400 rounded py-1 px-2 flex-1",
                            r#type: "text",
                            value: "{last_name}",
                            required: true,
                            oninput: move |e| {
                                let value = e.value();
                                last_name.set(value.clone());
                                // リアルタイムバリデーション
                                if let Err(validation_error) = validate_employee_name(&value) {
                                    last_name_error.set(Some(validation_error));
                                } else {
                                    last_name_error.set(None);
                                }
                            },
                        }
                    }

                    div { class: "flex gap-2 items-center",
                        label { class: "font-bold w-24",
                            span {
                                "名:"
                                if let Some(msg) = first_name_error() {
                                    span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                                }
                            }
                        }
                        input {
                            class: "border border-gray-400 rounded py-1 px-2 flex-1",
                            r#type: "text",
                            value: "{first_name}",
                            required: true,
                            oninput: move |e| {
                                let value = e.value();
                                first_name.set(value.clone());
                                // リアルタイムバリデーション
                                if let Err(validation_error) = validate_employee_name(&value) {
                                    first_name_error.set(Some(validation_error));
                                } else {
                                    first_name_error.set(None);
                                }
                            },
                        }
                    }

                    div { class: "flex gap-2 items-center",
                        label { class: "font-bold w-24", "状態:" }
                        input {
                            r#type: "checkbox",
                            checked: is_active(),
                            onchange: move |e| is_active.set(e.checked()),
                        }
                        span { class: "ml-2", "アクティブ" }
                    }

                    div { class: "flex gap-2 mt-2",
                        button {
                            class: if has_changes && !has_error {
                                "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
                            } else {
                                "bg-gray-300 text-gray-500 font-bold py-2 px-4 rounded cursor-not-allowed"
                            },
                            r#type: "submit",
                            disabled: !has_changes || has_error,
                            "保存"
                        }
                        button {
                            class: "bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded",
                            r#type: "button",
                            onclick: move |_| {
                                editing_id.set(None);
                                employee_code.set(employee.employee_code.clone().unwrap_or_default());
                                first_name.set(employee.first_name.clone());
                                last_name.set(employee.last_name.clone());
                                is_active.set(employee.is_active);
                                employee_code_error.set(None);
                                first_name_error.set(None);
                                last_name_error.set(None);
                                success_message.set(None);
                            },
                            "キャンセル"
                        }
                    }

                    if let Some(msg) = success_message() {
                        div { class: "p-2 rounded absolute top-4 right-4 bg-green-100 text-green-800",
                            "{msg}"
                        }
                    }
                }
            } else {
                div { class: "flex justify-between items-center",
                    div {
                        p { "ID: {employee.id}" }
                        if let Some(code) = &employee.employee_code {
                            p { "従業員コード: {code}" }
                        }
                        p { "氏名: {employee.last_name} {employee.first_name}" }
                        p {
                            "状態: "
                            {if employee.is_active { "アクティブ" } else { "非アクティブ" }}
                        }
                    }
                    div { class: "flex gap-2",
                        button {
                            class: "bg-yellow-500 hover:bg-yellow-700 text-white font-bold py-2 px-4 rounded",
                            onclick: move |_| editing_id.set(Some(employee.id)),
                            "編集"
                        }
                        button {
                            class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded",
                            onclick: move |_| handle_delete(),
                            "削除"
                        }
                    }
                }
            }
        }
    }
}
