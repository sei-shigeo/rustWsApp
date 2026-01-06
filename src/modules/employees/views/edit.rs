use crate::modules::employees::handlers::{
    check_employee_code_available, delete_employee, update_employee,
};
use crate::modules::employees::models::Employee;
use crate::modules::employees::validation::{validate_employee_code, validate_employee_name};
use dioxus::prelude::*;

/// 従業員一覧のアイテム（クリックで編集パネルを開く）
#[component]
pub fn EmployeeItem(
    employee: Employee,
    editing_id: Signal<Option<i32>>,
    show_create: Signal<bool>,
    on_refresh: EventHandler<()>,
) -> Element {
    // on_refresh は将来使う可能性があるため受け取っておく（今は未使用）
    let _ = on_refresh;

    rsx! {
        div {
            class: "border p-4 rounded hover:bg-gray-100 cursor-pointer",
            onclick: move |_| {
                editing_id.set(Some(employee.id));
                show_create.set(true);
            },
            div { class: "flex justify-between items-start gap-4",
                div {
                    p { class: "text-sm text-gray-500", "ID: {employee.id}" }
                    if let Some(code) = &employee.employee_code {
                        p { class: "text-sm text-gray-600", "従業員コード: {code}" }
                    }
                    p { class: "font-medium text-gray-900", "{employee.last_name} {employee.first_name}" }
                    p { class: "text-sm text-gray-600",
                        "状態: "
                        { if employee.is_active { "アクティブ" } else { "非アクティブ" } }
                    }
                }
                div { class: "text-sm text-gray-400", "クリックで編集" }
            }
        }
    }
}

/// 右パネルで使う編集フォームコンポーネント
#[component]
pub fn EmployeeEdit(
    employee: Employee,
    on_close: EventHandler<()>,
    on_refresh: EventHandler<()>,
) -> Element {
    // ローカル state (mut にして set() を呼べるようにする)
    let mut employee_code = use_signal(|| employee.employee_code.clone().unwrap_or_default());
    let mut first_name = use_signal(|| employee.first_name.clone());
    let mut last_name = use_signal(|| employee.last_name.clone());
    let mut is_active = use_signal(|| employee.is_active);

    // エラー・トースト
    let mut employee_code_error = use_signal(|| None::<String>);
    let mut first_name_error = use_signal(|| None::<String>);
    let mut last_name_error = use_signal(|| None::<String>);
    let success_message = use_signal(|| None::<String>);

    // 変更 / エラー判定（bool）
    let has_changes = employee_code() != employee.employee_code.clone().unwrap_or_default()
        || first_name() != employee.first_name
        || last_name() != employee.last_name
        || is_active() != employee.is_active;

    let has_error = employee_code_error().is_some()
        || first_name_error().is_some()
        || last_name_error().is_some();

    // 成功メッセージを3秒後に消す
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

    // 保存処理
    let mut handle_save = {
        let emp_id = employee.id;
        let employee_code = employee_code;
        let first_name = first_name;
        let last_name = last_name;
        let is_active = is_active;
        let mut employee_code_error = employee_code_error;
        let mut first_name_error = first_name_error;
        let mut last_name_error = last_name_error;
        let mut success_message = success_message;
        let on_refresh = on_refresh;
        let on_close = on_close;

        move || {
            // 現在の値を取得
            let code_val = employee_code();
            let first_val = first_name();
            let last_val = last_name();
            let active_val = is_active();

            // エラークリア
            employee_code_error.set(None);
            first_name_error.set(None);
            last_name_error.set(None);
            success_message.set(None);

            // バリデーション
            let mut has_err = false;

            if let Err(msg) = validate_employee_code(&code_val) {
                employee_code_error.set(Some(msg));
                has_err = true;
            }
            if let Err(msg) = validate_employee_name(&first_val) {
                first_name_error.set(Some(msg));
                has_err = true;
            }
            if let Err(msg) = validate_employee_name(&last_val) {
                last_name_error.set(Some(msg));
                has_err = true;
            }

            if has_err {
                return;
            }

            // 非同期更新
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
                        success_message.set(Some(format!("更新成功: {} {}", last_val, first_val)));
                        // 編集パネルを閉じてリストを更新
                        on_close.call(());
                        on_refresh.call(());
                    }
                    Err(e) => {
                        first_name_error.set(Some(format!("エラー: {}", e)));
                    }
                }
            });
        }
    };

    // 削除処理
    let handle_delete = {
        let emp_id = employee.id;
        let on_close = on_close;
        let on_refresh = on_refresh;

        move || {
            spawn(async move {
                match delete_employee(emp_id).await {
                    Ok(_) => {
                        on_close.call(());
                        on_refresh.call(());
                    }
                    Err(e) => {
                        eprintln!("削除エラー: {}", e);
                    }
                }
            });
        }
    };

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
                    // ID
                    div { class: "flex gap-2 items-center",
                        label { class: "font-bold w-24", "ID:" }
                        span { "{employee.id}" }
                    }

                    // 従業員コード
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
                                let v = e.value();
                                employee_code.set(v.clone());
                                // 簡易バリデーションと重複チェック（非同期）
                                if let Err(err) = validate_employee_code(&v) {
                                    employee_code_error.set(Some(err));
                                } else {
                                    // 非同期で重複チェック（結果は最終的なsubmit時に二重チェック可能）
                                    let v2 = v.clone();
                                    let emp_id = employee.id;
                                    let mut employee_code_error = employee_code_error;
                                    spawn(async move {
                                        if let Ok(available) = check_employee_code_available(v2, Some(emp_id)).await {
                                            if !available {
                                                employee_code_error.set(Some("この従業員コードは既に使用されています".to_string()));
                                            }
                                        }
                                    });
                                    // ここでは先にエラークリア
                                    employee_code_error.set(None);
                                }
                            },
                        }
                    }

                    // 姓
                    div { class: "flex gap-2 items-center",
                        label { class: "font-bold w-24", "姓:" }
                        input {
                            class: "border border-gray-400 rounded py-1 px-2 flex-1",
                            r#type: "text",
                            value: "{last_name}",
                            required: true,
                            oninput: move |e| {
                                let v = e.value();
                                last_name.set(v.clone());
                                if let Err(msg) = validate_employee_name(&v) {
                                    last_name_error.set(Some(msg));
                                } else {
                                    last_name_error.set(None);
                                }
                            },
                        }
                    }

                    // 名
                    div { class: "flex gap-2 items-center",
                        label { class: "font-bold w-24", "名:" }
                        input {
                            class: "border border-gray-400 rounded py-1 px-2 flex-1",
                            r#type: "text",
                            value: "{first_name}",
                            required: true,
                            oninput: move |e| {
                                let v = e.value();
                                first_name.set(v.clone());
                                if let Err(msg) = validate_employee_name(&v) {
                                    first_name_error.set(Some(msg));
                                } else {
                                    first_name_error.set(None);
                                }
                            },
                        }
                    }

                    // 状態
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

                // 操作ボタン群（保存・削除・キャンセル）
                div { class: "flex gap-2 mt-2",
                    button {
                        r#type: "submit",
                        disabled: !has_changes || has_error,
                        class: if !has_changes || has_error {
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

            // 成功メッセージのトースト表示
            if let Some(msg) = success_message() {
                div { class: "p-2 rounded absolute top-4 right-4 bg-green-100 text-green-800", "{msg}" }
            }
        }
    }
}
