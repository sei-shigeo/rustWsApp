use crate::modules::employees::components::LabeledTextInput;
use crate::modules::employees::handlers::{
    check_employee_code_available, delete_employee, update_employee,
};
use crate::modules::employees::models::Employee;
use crate::modules::employees::validation::{validate_employee_code, validate_employee_name};
use dioxus::prelude::*;

// EmployeeItem moved to `list_item.rs`. See `crate::modules::employees::views::list_item::EmployeeItem`.

/// Employee edit form on the right panel.
/// Refactored to use small components, memos for derived state, and clearer handlers.
#[component]
pub fn EmployeeEdit(
    employee: Employee,
    on_close: EventHandler<()>,
    on_refresh: EventHandler<()>,
) -> Element {
    // Local editable state
    let mut employee_code = use_signal(|| employee.employee_code.clone().unwrap_or_default());
    let first_name = use_signal(|| employee.first_name.clone());
    let last_name = use_signal(|| employee.last_name.clone());
    let mut is_active = use_signal(|| employee.is_active);

    // Errors and success toast
    let employee_code_error = use_signal(|| None::<String>);
    let first_name_error = use_signal(|| None::<String>);
    let last_name_error = use_signal(|| None::<String>);
    let success_message = use_signal(|| None::<String>);

    // Derived state as memos - recomputed when signals they reference change
    let has_changes = use_memo({
        move || {
            employee_code() != employee.employee_code.clone().unwrap_or_default()
                || first_name() != employee.first_name
                || last_name() != employee.last_name
                || is_active() != employee.is_active
        }
    });

    let has_error = use_memo({
        move || {
            employee_code_error().is_some()
                || first_name_error().is_some()
                || last_name_error().is_some()
        }
    });

    // success_message auto-clear after 3 seconds (uses spawn inside effect)
    {
        let mut success_message = success_message;
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
    }

    // Save handler: validate, then call async update_employee and notify parent
    let mut handle_save = {
        let emp_id = employee.id;
        let employee_code = employee_code;
        let is_active = is_active;
        let mut employee_code_error = employee_code_error;
        let mut first_name_error = first_name_error;
        let mut last_name_error = last_name_error;
        let mut success_message = success_message;
        let on_refresh = on_refresh;
        let on_close = on_close;

        move || {
            // Clear previous errors
            employee_code_error.set(None);
            first_name_error.set(None);
            last_name_error.set(None);
            success_message.set(None);

            // Read current values
            let code_val = employee_code();
            let first_val = first_name();
            let last_val = last_name();
            let active_val = is_active();

            // Local validation
            let mut had_error = false;
            if let Err(msg) = validate_employee_code(&code_val) {
                employee_code_error.set(Some(msg));
                had_error = true;
            }
            if let Err(msg) = validate_employee_name(&first_val) {
                first_name_error.set(Some(msg));
                had_error = true;
            }
            if let Err(msg) = validate_employee_name(&last_val) {
                last_name_error.set(Some(msg));
                had_error = true;
            }
            if had_error {
                return;
            }

            // Async update
            spawn(async move {
                match update_employee(
                    emp_id,
                    Some(code_val),
                    first_val.clone(),
                    last_val.clone(),
                    active_val,
                )
                .await
                {
                    Ok(_) => {
                        success_message.set(Some(format!("更新成功: {last_val} {first_val}")));
                        // Close and refresh list
                        on_close.call(());
                        on_refresh.call(());
                    }
                    Err(e) => {
                        // Put server error into first name field error for visibility
                        first_name_error.set(Some(format!("エラー: {e}")));
                    }
                }
            });
        }
    };

    // Delete handler: call async delete_employee and notify parent
    let handle_delete = {
        let emp_id = employee.id;
        let on_close = on_close;
        let on_refresh = on_refresh;

        move || {
            spawn(async move {
                match delete_employee(emp_id).await {
                    Ok(()) => {
                        on_close.call(());
                        on_refresh.call(());
                    }
                    Err(e) => {
                        // Log the delete error - keep UI simple
                        eprintln!("削除エラー: {e}");
                    }
                }
            });
        }
    };

    // Validation: perform automatic validation when `employee_code` changes,
    // and run an async uniqueness check when local validation passes.
    {
        let employee_code = employee_code;
        let mut employee_code_error = employee_code_error;
        let emp_id = employee.id;
        use_effect(move || {
            let v = employee_code();
            if v.is_empty() {
                employee_code_error.set(None);
            } else if let Err(err) = validate_employee_code(&v) {
                // Local format validation failed
                employee_code_error.set(Some(err));
            } else {
                // Clear any previous format error and check uniqueness asynchronously.
                employee_code_error.set(None);
                let v2 = v.clone();
                let mut employee_code_error = employee_code_error;
                spawn(async move {
                    if let Ok(available) = check_employee_code_available(v2, Some(emp_id)).await {
                        if !available {
                            employee_code_error
                                .set(Some("この従業員コードは既に使用されています".to_string()));
                        }
                    }
                });
            }
        });
    }

    rsx! {
        div { class: "p-4",
            h2 { class: "text-xl font-bold mb-4", "従業員編集" }
            form {
                class: "grid gap-4 mb-4",
                onsubmit: move |e: Event<FormData>| {
                    e.prevent_default();
                    handle_save();
                },

                div { class: "grid gap-2",
                    // Employee code
                    div { class: "grid gap-2 items-center",
                        label { class: "font-bold",
                            span { class: "flex items-center",
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
                            oninput: move |e| employee_code.set(e.value()),
                        }
                    }

                    // Name fields
                    div { class: "flex flex-wrap gap-2",
                        // Last name
                        LabeledTextInput {
                            value: last_name,
                            label: "姓".to_string(),
                            error: last_name_error,
                            required: true,
                            input_type: Some("text".to_string()),
                        }

                        // First name
                        LabeledTextInput {
                            value: first_name,
                            label: "名".to_string(),
                            error: first_name_error,
                            required: true,
                            input_type: Some("text".to_string()),
                        }
                    }

                    // Status checkbox
                    div { class: "flex gap-2 items-center",
                        label { class: "font-bold w-24", "状態:" }
                        input {
                            r#type: "checkbox",
                            checked: is_active(),
                            onchange: move |e| is_active.set(e.checked()),
                        }
                        span { class: "ml-2", "アクティブ" }
                    }
                }

                // Buttons
                div { class: "flex gap-2 mt-2",
                    button {
                        r#type: "submit",
                        disabled: !has_changes() || has_error(),
                        class: if !has_changes() || has_error() {
                            "bg-gray-300 text-gray-500 font-bold py-2 px-4 rounded cursor-not-allowed"
                        } else {
                            "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
                        },
                        "保存"
                    }
                    button {
                        r#type: "button",
                        onclick: move |_| handle_delete(),
                        class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded",
                        "削除"
                    }
                    button {
                        r#type: "button",
                        onclick: move |_| on_close.call(()),
                        class: "bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded",
                        "キャンセル"
                    }
                }
            }

            // Success toast
            if let Some(msg) = success_message() {
                div { class: "p-2 rounded absolute top-4 right-4 bg-green-100 text-green-800", "{msg}" }
            }
        }
    }
}
