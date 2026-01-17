use crate::modules::employees::components::form_components::{use_field_validation, InputLabel};
use crate::modules::employees::handlers::{check_employee_code_available, update_employee};
use crate::modules::employees::models::EmployeeWithAddress;
use crate::modules::employees::validation::{validate_employee_code, validate_employee_name};
use dioxus::prelude::*;

#[component]
pub fn EmployeeEditForm(
    employee: EmployeeWithAddress,
    mut employees_list: Resource<Result<Vec<EmployeeWithAddress>, ServerFnError>>,
    on_close: EventHandler<MouseEvent>,
) -> Element {
    let employee_code = use_signal(|| employee.employee_code.clone());
    let first_name = use_signal(|| employee.first_name.clone());
    let last_name = use_signal(|| employee.last_name.clone());
    let mut is_active = use_signal(|| employee.is_active);
    let mut err_msg = use_signal(String::new);
    let mut is_code_duplicate = use_signal(|| false);

    // バリデーション結果
    let err_first_name = use_field_validation(first_name, validate_employee_name);
    let err_last_name = use_field_validation(last_name, validate_employee_name);
    let err_code = use_field_validation(employee_code, validate_employee_code);

    // 従業員コードの重複チェック
    let employee_id = employee.id;
    use_effect(move || {
        let code = employee_code();
        if code.is_empty() || validate_employee_code(&code).is_err() {
            is_code_duplicate.set(false);
            return;
        }
        spawn(async move {
            match check_employee_code_available(code, Some(employee_id)).await {
                Ok(available) => {
                    is_code_duplicate.set(!available);
                }
                Err(_) => {
                    is_code_duplicate.set(false);
                }
            }
        });
    });

    // フォームの有効性チェック
    let is_valid = use_memo(move || {
        let validations = [
            (employee_code(), err_code()),
            (first_name(), err_first_name()),
            (last_name(), err_last_name()),
        ];
        validations
            .iter()
            .all(|(value, error)| !value.trim().is_empty() && error.is_none())
            && !is_code_duplicate()
    });

    // フォーム送信
    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        spawn(async move {
            let id = employee.id;
            let code = employee_code();
            let first = first_name();
            let last = last_name();
            let active = is_active();

            match update_employee(id, code, first, last, active).await {
                Ok(_) => {
                    employees_list.restart();
                    err_msg.set(String::new());
                }
                Err(e) => {
                    err_msg.set(format!("従業員の更新に失敗しました: {}", e));
                }
            }
        });
    };

    rsx! {
        div { class: "flex flex-col h-full bg-white",
            // ヘッダー
            div { class: "flex items-center justify-between h-14 px-4 border-b border-gray-200",
                h3 { class: "text-lg font-semibold text-gray-800", "従業員編集" }
                button {
                    class: "text-gray-500 hover:text-gray-700 transition-colors",
                    onclick: move |evt| on_close.call(evt),
                    "✕"
                }
            }

            // フォーム
            form {
                class: "flex-1 overflow-y-auto p-4 space-y-4",
                onsubmit: handle_submit,

                InputLabel {
                    value: employee_code,
                    label: "従業員コード".to_string(),
                    placeholder: "例: EMP001".to_string(),
                    error: err_code
                }

                if is_code_duplicate() {
                    p { class: "text-red-500 text-sm -mt-2",
                        "この従業員コードは既に使用されています"
                    }
                }

                InputLabel {
                    value: first_name,
                    label: "名".to_string(),
                    placeholder: "太郎".to_string(),
                    error: err_first_name
                }

                InputLabel {
                    value: last_name,
                    label: "姓".to_string(),
                    placeholder: "山田".to_string(),
                    error: err_last_name
                }

                div { class: "space-y-2",
                    label { class: "flex items-center gap-2",
                        input {
                            r#type: "checkbox",
                            class: "w-4 h-4 text-amber-500 border-gray-300 rounded focus:ring-amber-400",
                            checked: is_active(),
                            oninput: move |evt| is_active.set(evt.checked()),
                        }
                        span { class: "text-sm font-medium text-gray-700", "在籍中" }
                    }
                }

                if !err_msg().is_empty() {
                    div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded",
                        "{err_msg}"
                    }
                }
            }

            // フッター
            div { class: "flex justify-end gap-2 p-4 border-t border-gray-200",
                button {
                    class: "px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors",
                    onclick: move |evt| on_close.call(evt),
                    "キャンセル"
                }
                button {
                    class: "px-4 py-2 bg-amber-400 text-gray-800 rounded-lg hover:bg-amber-500 font-semibold transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                    r#type: "submit",
                    disabled: !is_valid(),
                    "更新"
                }
            }
        }
    }
}
