use crate::modules::department_position_history::handlers::{
    create_department_position_history, delete_department_position_history,
    get_department_position_history_by_employee, update_department_position_history,
};
use crate::modules::department_position_history::{
    CreateDepartmentPositionHistory, DepartmentPositionHistory, UpdateDepartmentPositionHistory,
};
use dioxus::prelude::*;

#[component]
pub fn DepartmentPositionHistorySection(employee_id: i32) -> Element {
    let mut history_resource: Resource<Result<Vec<DepartmentPositionHistory>, ServerFnError>> =
        use_resource(move || async move {
            get_department_position_history_by_employee(employee_id).await
        });

    let mut show_add_form = use_signal(|| false);
    let mut edit_history_id = use_signal(|| None::<i32>);

    rsx! {
        div { class: "bg-white rounded-xl shadow-sm border border-gray-200 p-6",
            div { class: "flex items-center justify-between mb-6 pb-3 border-b-2 border-amber-400",
                h3 { class: "text-xl font-bold", "部署・役職・営業所履歴" }
                button {
                    class: "px-4 py-2 bg-amber-400 text-gray-800 rounded-lg hover:bg-amber-500 font-semibold transition-colors text-sm",
                    onclick: move |_| show_add_form.set(true),
                    disabled: show_add_form() || edit_history_id().is_some(),
                    "＋ 新規追加"
                }
            }

            // 新規追加フォーム
            if show_add_form() {
                DepartmentPositionHistoryForm {
                    employee_id,
                    history: None,
                    on_save: move |_| {
                        show_add_form.set(false);
                        history_resource.restart();
                    },
                    on_cancel: move |_| show_add_form.set(false),
                }
            }

            // 履歴リスト
            match &*history_resource.read_unchecked() {
                Some(Ok(history_list)) => {
                    if history_list.is_empty() {
                        rsx! {
                            p { class: "text-gray-500 text-center py-4", "配属履歴が登録されていません" }
                        }
                    } else {
                        rsx! {
                            div { class: "space-y-3",
                                for history in history_list.iter() {
                                    {
                                        let history_id = history.id;
                                        let history_clone = history.clone();
                                        let is_editing = edit_history_id().is_some_and(|id| id == history.id);
                                        rsx! {
                                            if is_editing {
                                                DepartmentPositionHistoryForm {
                                                    employee_id,
                                                    history: Some(history_clone),
                                                    on_save: move |_| {
                                                        edit_history_id.set(None);
                                                        history_resource.restart();
                                                    },
                                                    on_cancel: move |_| edit_history_id.set(None),
                                                }
                                            } else {
                                                DepartmentPositionHistoryCard {
                                                    history: history_clone.clone(),
                                                    on_edit: move |_| edit_history_id.set(Some(history_id)),
                                                    on_delete: move |_| {
                                                        spawn(async move {
                                                            if let Ok(_) = delete_department_position_history(history_id).await {
                                                                history_resource.restart();
                                                            }
                                                        });
                                                    },
                                                    disabled: show_add_form() || edit_history_id().is_some(),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    p { class: "text-red-500 text-center py-4", "履歴の読み込みに失敗しました: {e}" }
                },
                None => rsx! {
                    div { class: "flex justify-center py-4",
                        div { class: "inline-block animate-spin rounded-full h-6 w-6 border-b-2 border-amber-500" }
                    }
                },
            }
        }
    }
}

#[component]
fn DepartmentPositionHistoryCard(
    history: DepartmentPositionHistory,
    on_edit: EventHandler<()>,
    on_delete: EventHandler<()>,
    disabled: bool,
) -> Element {
    let mut show_delete_confirm = use_signal(|| false);

    rsx! {
        div {
            class: "border border-gray-200 rounded-lg p-4 transition-colors",
            class: if history.is_current { "bg-amber-50 border-amber-300" } else { "bg-gray-50" },

            div { class: "flex items-start justify-between mb-3",
                div { class: "flex items-center gap-2 flex-wrap",
                    if history.is_current {
                        span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-amber-400 text-gray-800",
                            "現在の配属"
                        }
                    }
                    span { class: "text-sm text-gray-600",
                        "{history.start_date} ～ "
                        if let Some(end_date) = history.end_date {
                            "{end_date}"
                        } else {
                            "現在"
                        }
                    }
                }
                div { class: "flex gap-2",
                    button {
                        class: "text-blue-600 hover:text-blue-800 text-sm font-semibold disabled:opacity-50",
                        onclick: move |_| on_edit.call(()),
                        disabled,
                        "編集"
                    }
                    button {
                        class: "text-red-600 hover:text-red-800 text-sm font-semibold disabled:opacity-50",
                        onclick: move |_| show_delete_confirm.set(true),
                        disabled,
                        "削除"
                    }
                }
            }

            div { class: "grid grid-cols-2 gap-3 text-sm",
                if let Some(office_id) = history.office_id {
                    div { class: "space-y-1",
                        p { class: "text-xs text-gray-500 font-semibold", "営業所ID" }
                        p { class: "text-gray-800", "{office_id}" }
                    }
                }
                if let Some(department_id) = history.department_id {
                    div { class: "space-y-1",
                        p { class: "text-xs text-gray-500 font-semibold", "部署ID" }
                        p { class: "text-gray-800", "{department_id}" }
                    }
                }
                if let Some(position_id) = history.position_id {
                    div { class: "space-y-1",
                        p { class: "text-xs text-gray-500 font-semibold", "役職ID" }
                        p { class: "text-gray-800", "{position_id}" }
                    }
                }
                if let Some(reason) = &history.change_reason {
                    div { class: "col-span-2 space-y-1",
                        p { class: "text-xs text-gray-500 font-semibold", "異動理由" }
                        p { class: "text-gray-800", "{reason}" }
                    }
                }
            }

            // 削除確認ダイアログ
            if show_delete_confirm() {
                div {
                    class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                    onclick: move |_| show_delete_confirm.set(false),
                    div {
                        class: "bg-white rounded-lg shadow-xl p-6 max-w-sm mx-4",
                        onclick: move |e| e.stop_propagation(),
                        h4 { class: "text-lg font-bold mb-3", "配属履歴を削除しますか？" }
                        p { class: "text-sm text-gray-600 mb-6", "この操作は取り消せません。" }
                        div { class: "flex gap-3 justify-end",
                            button {
                                class: "px-4 py-2 bg-gray-200 rounded-lg hover:bg-gray-300 font-semibold",
                                onclick: move |_| show_delete_confirm.set(false),
                                "キャンセル"
                            }
                            button {
                                class: "px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 font-semibold",
                                onclick: move |_| {
                                    show_delete_confirm.set(false);
                                    on_delete.call(());
                                },
                                "削除"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DepartmentPositionHistoryForm(
    employee_id: i32,
    history: Option<DepartmentPositionHistory>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    let is_edit = history.is_some();
    let mut office_id = use_signal(|| {
        history
            .as_ref()
            .and_then(|h| h.office_id)
            .map(|id| id.to_string())
            .unwrap_or_default()
    });
    let mut department_id = use_signal(|| {
        history
            .as_ref()
            .and_then(|h| h.department_id)
            .map(|id| id.to_string())
            .unwrap_or_default()
    });
    let mut position_id = use_signal(|| {
        history
            .as_ref()
            .and_then(|h| h.position_id)
            .map(|id| id.to_string())
            .unwrap_or_default()
    });
    let mut start_date = use_signal(|| {
        history
            .as_ref()
            .map(|h| h.start_date.to_string())
            .unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string())
    });
    let mut end_date = use_signal(|| {
        history
            .as_ref()
            .and_then(|h| h.end_date)
            .map(|d| d.to_string())
            .unwrap_or_default()
    });
    let mut is_current = use_signal(|| history.as_ref().map(|h| h.is_current).unwrap_or(true));
    let mut change_reason = use_signal(|| {
        history
            .as_ref()
            .and_then(|h| h.change_reason.clone())
            .unwrap_or_default()
    });
    let mut error_message = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);

    let handle_submit = move |_| {
        let office_id_val = office_id();
        let department_id_val = department_id();
        let position_id_val = position_id();
        let start_date_val = start_date();
        let end_date_val = end_date();
        let is_current_val = is_current();
        let change_reason_val = change_reason();
        let history_id = history.as_ref().map(|h| h.id);

        spawn(async move {
            is_submitting.set(true);
            error_message.set(String::new());

            let office_id_value = if office_id_val.is_empty() {
                None
            } else {
                office_id_val.parse::<i32>().ok()
            };
            let department_id_value = if department_id_val.is_empty() {
                None
            } else {
                department_id_val.parse::<i32>().ok()
            };
            let position_id_value = if position_id_val.is_empty() {
                None
            } else {
                position_id_val.parse::<i32>().ok()
            };

            // バリデーション
            if office_id_value.is_none()
                && department_id_value.is_none()
                && position_id_value.is_none()
            {
                error_message.set("営業所、部署、役職のいずれかを入力してください".to_string());
                is_submitting.set(false);
                return;
            }

            let start_date_value =
                match chrono::NaiveDate::parse_from_str(&start_date_val, "%Y-%m-%d") {
                    Ok(date) => date,
                    Err(_) => {
                        error_message.set("開始日の形式が正しくありません".to_string());
                        is_submitting.set(false);
                        return;
                    }
                };

            let end_date_value = if end_date_val.is_empty() {
                None
            } else {
                match chrono::NaiveDate::parse_from_str(&end_date_val, "%Y-%m-%d") {
                    Ok(date) => Some(date),
                    Err(_) => {
                        error_message.set("終了日の形式が正しくありません".to_string());
                        is_submitting.set(false);
                        return;
                    }
                }
            };

            let result = if is_edit {
                let data = UpdateDepartmentPositionHistory {
                    id: history_id.unwrap(),
                    employee_id,
                    office_id: office_id_value,
                    department_id: department_id_value,
                    position_id: position_id_value,
                    start_date: start_date_value,
                    end_date: end_date_value,
                    is_current: is_current_val,
                    change_reason: if change_reason_val.is_empty() {
                        None
                    } else {
                        Some(change_reason_val.clone())
                    },
                };
                update_department_position_history(data).await
            } else {
                let data = CreateDepartmentPositionHistory {
                    employee_id,
                    office_id: office_id_value,
                    department_id: department_id_value,
                    position_id: position_id_value,
                    start_date: start_date_value,
                    is_current: is_current_val,
                    change_reason: if change_reason_val.is_empty() {
                        None
                    } else {
                        Some(change_reason_val)
                    },
                };
                create_department_position_history(data).await
            };

            match result {
                Ok(_) => {
                    is_submitting.set(false);
                    on_save.call(());
                }
                Err(e) => {
                    error_message.set(format!("保存エラー: {}", e));
                    is_submitting.set(false);
                }
            }
        });
    };

    rsx! {
        div { class: "border-2 border-amber-400 rounded-lg p-4 bg-white mb-3",
            if !error_message().is_empty() {
                div { class: "bg-red-100 border border-red-400 text-red-700 px-3 py-2 rounded mb-4 text-sm",
                    "{error_message}"
                }
            }

            div { class: "space-y-3",
                div { class: "grid grid-cols-3 gap-3",
                    div { class: "space-y-1",
                        label { class: "block text-xs font-medium text-gray-700", "営業所ID" }
                        input {
                            class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 text-sm",
                            r#type: "number",
                            value: "{office_id()}",
                            oninput: move |evt| office_id.set(evt.value()),
                        }
                    }
                    div { class: "space-y-1",
                        label { class: "block text-xs font-medium text-gray-700", "部署ID" }
                        input {
                            class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 text-sm",
                            r#type: "number",
                            value: "{department_id()}",
                            oninput: move |evt| department_id.set(evt.value()),
                        }
                    }
                    div { class: "space-y-1",
                        label { class: "block text-xs font-medium text-gray-700", "役職ID" }
                        input {
                            class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 text-sm",
                            r#type: "number",
                            value: "{position_id()}",
                            oninput: move |evt| position_id.set(evt.value()),
                        }
                    }
                }

                div { class: "grid grid-cols-2 gap-3",
                    div { class: "space-y-1",
                        label { class: "block text-xs font-medium text-gray-700",
                            "開始日"
                            span { class: "text-red-500", " *" }
                        }
                        input {
                            class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 text-sm",
                            r#type: "date",
                            value: "{start_date()}",
                            oninput: move |evt| start_date.set(evt.value()),
                        }
                    }
                    div { class: "space-y-1",
                        label { class: "block text-xs font-medium text-gray-700", "終了日" }
                        input {
                            class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 text-sm",
                            r#type: "date",
                            value: "{end_date()}",
                            oninput: move |evt| end_date.set(evt.value()),
                        }
                    }
                }

                div { class: "flex items-center gap-2",
                    input {
                        id: "is_current_checkbox",
                        r#type: "checkbox",
                        checked: is_current(),
                        onchange: move |evt| is_current.set(evt.checked()),
                        class: "w-4 h-4 text-amber-600 border-gray-300 rounded focus:ring-amber-500",
                    }
                    label {
                        r#for: "is_current_checkbox",
                        class: "text-sm font-medium text-gray-700 cursor-pointer",
                        "現在の配属"
                    }
                }

                div { class: "space-y-1",
                    label { class: "block text-xs font-medium text-gray-700", "異動理由" }
                    textarea {
                        class: "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 text-sm",
                        rows: 2,
                        value: "{change_reason()}",
                        oninput: move |evt| change_reason.set(evt.value()),
                    }
                }
            }

            div { class: "flex justify-end gap-2 mt-4 pt-3 border-t",
                button {
                    class: "px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors text-sm font-semibold",
                    onclick: move |_| on_cancel.call(()),
                    disabled: is_submitting(),
                    "キャンセル"
                }
                button {
                    class: "px-4 py-2 bg-amber-400 text-gray-800 rounded-lg hover:bg-amber-500 font-semibold transition-colors disabled:opacity-50 text-sm",
                    onclick: handle_submit,
                    disabled: is_submitting(),
                    if is_submitting() { "保存中..." } else if is_edit { "更新" } else { "追加" }
                }
            }
        }
    }
}
