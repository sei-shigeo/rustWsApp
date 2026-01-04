use crate::modules::employees::handlers::{delete_employee, update_employee};
use crate::modules::employees::models::Employee;
use crate::modules::employees::validation::validate_employee_name;
use dioxus::prelude::*;

/// 従業員アイテムコンポーネント（表示・編集切り替え）
#[component]
pub fn EmployeeItem(
    employee: Employee,
    editing_id: Signal<Option<i32>>,
    on_refresh: EventHandler<()>,
) -> Element {
    let mut first_name = use_signal(|| employee.first_name.clone());
    let mut last_name = use_signal(|| employee.last_name.clone());
    let mut is_active = use_signal(|| employee.is_active);
    let mut success_message = use_signal(|| None::<String>);
    let mut error_message = use_signal(|| None::<String>);
    let is_editing = editing_id() == Some(employee.id);

    // 変更があるかチェック
    let has_changes = first_name() != employee.first_name
        || last_name() != employee.last_name
        || is_active() != employee.is_active;

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
        let first_name_val = first_name();
        let last_name_val = last_name();
        let is_active_val = is_active();

        // エラーメッセージをクリア
        error_message.set(None);
        success_message.set(None);

        // バリデーション
        if let Err(validation_error) = validate_employee_name(&first_name_val) {
            error_message.set(Some(format!("名: {}", validation_error)));
            return;
        }

        if let Err(validation_error) = validate_employee_name(&last_name_val) {
            error_message.set(Some(format!("姓: {}", validation_error)));
            return;
        }

        spawn(async move {
            match update_employee(
                employee_id,
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
                    error_message.set(Some(format!("エラー: {}", e)));
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
                                "姓:"
                                if let Some(msg) = error_message() {
                                    if msg.starts_with("姓:") {
                                        span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                                    }
                                }
                            }
                        }
                        input {
                            class: "border border-gray-400 rounded py-1 px-2 flex-1",
                            r#type: "text",
                            value: "{last_name}",
                            required: true,
                            oninput: move |e| last_name.set(e.value()),
                        }
                    }

                    div { class: "flex gap-2 items-center",
                        label { class: "font-bold w-24",
                            span {
                                "名:"
                                if let Some(msg) = error_message() {
                                    if msg.starts_with("名:") {
                                        span { class: "ml-2 text-red-600 text-sm font-normal", "{msg}" }
                                    }
                                }
                            }
                        }
                        input {
                            class: "border border-gray-400 rounded py-1 px-2 flex-1",
                            r#type: "text",
                            value: "{first_name}",
                            required: true,
                            oninput: move |e| first_name.set(e.value()),
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
                            class: if has_changes {
                                "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded"
                            } else {
                                "bg-gray-300 text-gray-500 font-bold py-2 px-4 rounded cursor-not-allowed"
                            },
                            r#type: "submit",
                            disabled: !has_changes,
                            "保存"
                        }
                        button {
                            class: "bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded",
                            r#type: "button",
                            onclick: move |_| {
                                editing_id.set(None);
                                first_name.set(employee.first_name.clone());
                                last_name.set(employee.last_name.clone());
                                is_active.set(employee.is_active);
                                error_message.set(None);
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
