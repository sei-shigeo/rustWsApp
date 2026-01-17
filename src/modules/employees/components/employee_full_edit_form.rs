use crate::modules::employees::components::department_position_history_section::DepartmentPositionHistorySection;
use crate::modules::employees::components::form_components::FormSection;
use crate::modules::employees::handlers::{check_employee_code_available, update_employee_full};
use crate::modules::employees::models::EmployeeFull;
use crate::modules::employees::validation::{validate_employee_code, validate_employee_name};
use dioxus::prelude::*;

#[component]
pub fn EmployeeFullEditForm(employee: EmployeeFull, on_close: EventHandler<()>) -> Element {
    let mut employee_data = use_signal(|| employee.clone());
    let mut error_message = use_signal(|| String::new());
    let mut success_message = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);

    // 各フィールドのバリデーションエラー
    let mut employee_code_error = use_signal(|| String::new());
    let mut first_name_error = use_signal(|| String::new());
    let mut last_name_error = use_signal(|| String::new());

    let handle_submit = move |_| {
        spawn(async move {
            is_submitting.set(true);
            error_message.set(String::new());
            success_message.set(String::new());

            let emp = employee_data();

            // バリデーション
            let mut has_error = false;

            if let Err(e) = validate_employee_code(&emp.employee_code) {
                employee_code_error.set(e);
                has_error = true;
            } else {
                employee_code_error.set(String::new());
            }

            if let Err(e) = validate_employee_name(&emp.first_name) {
                first_name_error.set(e);
                has_error = true;
            } else {
                first_name_error.set(String::new());
            }

            if let Err(e) = validate_employee_name(&emp.last_name) {
                last_name_error.set(e);
                has_error = true;
            } else {
                last_name_error.set(String::new());
            }

            if has_error {
                is_submitting.set(false);
                return;
            }

            // 従業員コードの重複チェック（自分以外）
            match check_employee_code_available(emp.employee_code.clone(), Some(emp.id)).await {
                Ok(available) => {
                    if !available {
                        employee_code_error
                            .set("この従業員コードは既に使用されています".to_string());
                        is_submitting.set(false);
                        return;
                    }
                }
                Err(e) => {
                    error_message.set(format!("バリデーションエラー: {}", e));
                    is_submitting.set(false);
                    return;
                }
            }

            // 更新処理
            match update_employee_full(emp).await {
                Ok(_) => {
                    success_message.set("従業員情報を更新しました".to_string());
                    is_submitting.set(false);
                    // 更新成功後、親コンポーネントに通知
                    on_close.call(());
                }
                Err(e) => {
                    error_message.set(format!("更新エラー: {}", e));
                    is_submitting.set(false);
                }
            }
        });
    };

    rsx! {
        div { class: "flex flex-col h-full bg-gray-50",
            // ヘッダー
            div { class: "flex items-center justify-between h-14 px-4 border-b border-gray-300 bg-white",
                h3 { class: "text-lg font-semibold", "従業員編集" }
                button {
                    class: "text-gray-500 hover:text-gray-700 transition-colors",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }
            }

            // フォーム
            div { class: "flex-1 overflow-y-auto p-4 space-y-4",
                // エラー・成功メッセージ
                if !error_message().is_empty() {
                    div { class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded",
                        "{error_message}"
                    }
                }
                if !success_message().is_empty() {
                    div { class: "bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded",
                        "{success_message}"
                    }
                }

                // 基本情報
                FormSection {
                    title: "基本情報".to_string(),
                    div { class: "space-y-3",
                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium text-gray-700",
                                "従業員コード"
                                span { class: "text-red-500", " *" }
                            }
                            input {
                                class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                r#type: "text",
                                value: "{employee_data().employee_code}",
                                oninput: move |evt| {
                                    let mut emp = employee_data();
                                    emp.employee_code = evt.value();
                                    employee_data.set(emp);
                                },
                            }
                            if !employee_code_error().is_empty() {
                                p { class: "text-red-500 text-xs mt-1", "{employee_code_error}" }
                            }
                        }

                        div { class: "grid grid-cols-2 gap-3",
                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700",
                                    "姓"
                                    span { class: "text-red-500", " *" }
                                }
                                input {
                                    class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                    r#type: "text",
                                    value: "{employee_data().last_name}",
                                    oninput: move |evt| {
                                        let mut emp = employee_data();
                                        emp.last_name = evt.value();
                                        employee_data.set(emp);
                                    },
                                }
                                if !last_name_error().is_empty() {
                                    p { class: "text-red-500 text-xs mt-1", "{last_name_error}" }
                                }
                            }

                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700",
                                    "名"
                                    span { class: "text-red-500", " *" }
                                }
                                input {
                                    class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                    r#type: "text",
                                    value: "{employee_data().first_name}",
                                    oninput: move |evt| {
                                        let mut emp = employee_data();
                                        emp.first_name = evt.value();
                                        employee_data.set(emp);
                                    },
                                }
                                if !first_name_error().is_empty() {
                                    p { class: "text-red-500 text-xs mt-1", "{first_name_error}" }
                                }
                            }
                        }

                        div { class: "grid grid-cols-2 gap-3",
                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700", "姓（カナ）" }
                                input {
                                    class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                    r#type: "text",
                                    value: "{employee_data().last_name_kana.clone().unwrap_or_default()}",
                                    oninput: move |evt| {
                                        let mut emp = employee_data();
                                        emp.last_name_kana = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        employee_data.set(emp);
                                    },
                                }
                            }

                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700", "名（カナ）" }
                                input {
                                    class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                    r#type: "text",
                                    value: "{employee_data().first_name_kana.clone().unwrap_or_default()}",
                                    oninput: move |evt| {
                                        let mut emp = employee_data();
                                        emp.first_name_kana = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        employee_data.set(emp);
                                    },
                                }
                            }
                        }

                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium text-gray-700", "法的名称" }
                            input {
                                class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                r#type: "text",
                                value: "{employee_data().legal_name.clone().unwrap_or_default()}",
                                oninput: move |evt| {
                                    let mut emp = employee_data();
                                    emp.legal_name = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                    employee_data.set(emp);
                                },
                            }
                        }

                        div { class: "grid grid-cols-2 gap-3",
                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700", "性別" }
                                select {
                                    class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                    value: "{employee_data().gender.clone().unwrap_or_default()}",
                                    onchange: move |evt| {
                                        let mut emp = employee_data();
                                        emp.gender = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                        employee_data.set(emp);
                                    },
                                    option { value: "", "未選択" }
                                    option { value: "男性", "男性" }
                                    option { value: "女性", "女性" }
                                    option { value: "その他", "その他" }
                                }
                            }

                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700", "生年月日" }
                                input {
                                    class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                    r#type: "date",
                                    value: "{employee_data().birth_date.map(|d| d.to_string()).unwrap_or_default()}",
                                    oninput: move |evt| {
                                        let mut emp = employee_data();
                                        emp.birth_date = if evt.value().is_empty() {
                                            None
                                        } else {
                                            chrono::NaiveDate::parse_from_str(&evt.value(), "%Y-%m-%d").ok()
                                        };
                                        employee_data.set(emp);
                                    },
                                }
                            }
                        }

                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium text-gray-700", "ステータス" }
                            select {
                                class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                value: if employee_data().is_active { "true" } else { "false" },
                                onchange: move |evt| {
                                    let mut emp = employee_data();
                                    emp.is_active = evt.value() == "true";
                                    employee_data.set(emp);
                                },
                                option { value: "true", "在職中" }
                                option { value: "false", "退職済み" }
                            }
                        }
                    }
                }

                // 連絡先情報
                FormSection {
                    title: "連絡先情報".to_string(),
                    div { class: "space-y-3",
                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium text-gray-700", "メールアドレス" }
                            input {
                                class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                r#type: "email",
                                value: "{employee_data().email.clone().unwrap_or_default()}",
                                oninput: move |evt| {
                                    let mut emp = employee_data();
                                    emp.email = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                    employee_data.set(emp);
                                },
                            }
                        }

                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium text-gray-700", "電話番号" }
                            input {
                                class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                r#type: "tel",
                                value: "{employee_data().phone.clone().unwrap_or_default()}",
                                oninput: move |evt| {
                                    let mut emp = employee_data();
                                    emp.phone = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                    employee_data.set(emp);
                                },
                            }
                        }

                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium text-gray-700", "携帯電話" }
                            input {
                                class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                r#type: "tel",
                                value: "{employee_data().mobile.clone().unwrap_or_default()}",
                                oninput: move |evt| {
                                    let mut emp = employee_data();
                                    emp.mobile = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                    employee_data.set(emp);
                                },
                            }
                        }
                    }
                }

                // 雇用情報
                FormSection {
                    title: "雇用情報".to_string(),
                    div { class: "grid grid-cols-2 gap-3",
                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium text-gray-700", "雇用開始日" }
                            input {
                                class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                r#type: "date",
                                value: "{employee_data().start_date.map(|d| d.to_string()).unwrap_or_default()}",
                                oninput: move |evt| {
                                    let mut emp = employee_data();
                                    emp.start_date = if evt.value().is_empty() {
                                        None
                                    } else {
                                        chrono::NaiveDate::parse_from_str(&evt.value(), "%Y-%m-%d").ok()
                                    };
                                    employee_data.set(emp);
                                },
                            }
                        }

                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium text-gray-700", "雇用終了日" }
                            input {
                                class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                r#type: "date",
                                value: "{employee_data().end_date.map(|d| d.to_string()).unwrap_or_default()}",
                                oninput: move |evt| {
                                    let mut emp = employee_data();
                                    emp.end_date = if evt.value().is_empty() {
                                        None
                                    } else {
                                        chrono::NaiveDate::parse_from_str(&evt.value(), "%Y-%m-%d").ok()
                                    };
                                    employee_data.set(emp);
                                },
                            }
                        }
                    }
                }

                // ドライバー情報
                FormSection {
                    title: "ドライバー選任情報".to_string(),
                    div { class: "space-y-3",
                        div { class: "grid grid-cols-2 gap-3",
                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700", "選任開始日" }
                                input {
                                    class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                    r#type: "date",
                                    value: "{employee_data().driver_start_date.map(|d| d.to_string()).unwrap_or_default()}",
                                    oninput: move |evt| {
                                        let mut emp = employee_data();
                                        emp.driver_start_date = if evt.value().is_empty() {
                                            None
                                        } else {
                                            chrono::NaiveDate::parse_from_str(&evt.value(), "%Y-%m-%d").ok()
                                        };
                                        employee_data.set(emp);
                                    },
                                }
                            }

                            div { class: "space-y-2",
                                label { class: "block text-sm font-medium text-gray-700", "選任終了日" }
                                input {
                                    class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                    r#type: "date",
                                    value: "{employee_data().driver_end_date.map(|d| d.to_string()).unwrap_or_default()}",
                                    oninput: move |evt| {
                                        let mut emp = employee_data();
                                        emp.driver_end_date = if evt.value().is_empty() {
                                            None
                                        } else {
                                            chrono::NaiveDate::parse_from_str(&evt.value(), "%Y-%m-%d").ok()
                                        };
                                        employee_data.set(emp);
                                    },
                                }
                            }
                        }

                        div { class: "space-y-2",
                            label { class: "block text-sm font-medium text-gray-700", "選任解除理由" }
                            textarea {
                                class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 transition-colors",
                                rows: 3,
                                value: "{employee_data().driver_end_note.clone().unwrap_or_default()}",
                                oninput: move |evt| {
                                    let mut emp = employee_data();
                                    emp.driver_end_note = if evt.value().is_empty() { None } else { Some(evt.value()) };
                                    employee_data.set(emp);
                                },
                            }
                        }
                    }
                }

                // 配属履歴セクション
                DepartmentPositionHistorySection {
                    employee_id: employee.id
                }
            }

            // フッター
            div { class: "flex justify-end gap-2 p-4 border-t border-gray-300 bg-white",
                button {
                    class: "px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors",
                    onclick: move |_| on_close.call(()),
                    disabled: is_submitting(),
                    "キャンセル"
                }
                button {
                    class: "px-4 py-2 bg-amber-400 text-gray-800 rounded-lg hover:bg-amber-500 font-semibold transition-colors disabled:opacity-50",
                    onclick: handle_submit,
                    disabled: is_submitting(),
                    if is_submitting() { "更新中..." } else { "更新" }
                }
            }
        }
    }
}
