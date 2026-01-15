use dioxus::prelude::*;

// use crate::modules::employees;
use crate::modules::employees::handlers::{
    check_employee_code_available, create_employee, get_employees, update_employee,
};
use crate::modules::employees::models::Employee;
use crate::modules::employees::validation::{validate_employee_code, validate_employee_name};

#[component]
pub fn EmployeesPage() -> Element {
    let employees_list = use_resource(|| async move { get_employees().await });
    let mut selectd_employee = use_signal(|| None::<Employee>);
    let mut create_panel = use_signal(|| false);
    let mut edit_panel = use_signal(|| false);

    rsx! {
        div { class: "flex h-full",
            div { class: "bg-red-100 px-4 flex-1 flex flex-col gap-2",
                match &*employees_list.read_unchecked() {
                    Some(Ok(list)) => rsx! {
                        div { class: "flex justify-between items-center h-14 py-2",
                            h2 { class: "text-xl font-bold", "従業員一覧" }
                            button {
                                class: "bg-green-500 text-white font-bold py-2 px-4 rounded  hover:bg-green-700 transition-opacity",
                                class: if create_panel() { "opacity-0" } else { "opacity-100" },
                                disabled: edit_panel(),
                                onclick: move |_| create_panel.set(true),
                                "新規登録"
                            }
                        }
                        p { class: "text-gray-600", "全 {list.len()} 件" }
                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                            for emp in list.iter() {
                                {
                                    let emp = emp.clone();
                                    let is_selected = selectd_employee()
                                        .is_some_and(|selected| selected.id == emp.id);
                                    rsx! {
                                        EmployeeCard {
                                            employee: emp.clone(),
                                            is_selected,
                                            on_click: move |_| {
                                                if !create_panel() {
                                                    if is_selected {
                                                        // 既に選択されている場合は閉じる
                                                        selectd_employee.set(None);
                                                        edit_panel.set(false);
                                                    } else {
                                                        // 選択されていない場合は開く
                                                        selectd_employee.set(Some(emp.clone()));
                                                        edit_panel.set(true);
                                                    }
                                                }
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(_)) => rsx! {
                        p { "従業員データの取得に失敗しました。" }
                    },
                    None => rsx! {
                        p { "読み込み中..." }
                    },
                }
            }

            // CreateForm 開閉可能なパネル
            div {
                class: "bg-blue-100 transition-all duration-500 ease-in-out",
                class: if create_panel() { "w-96" } else { "w-0 overflow-hidden" },
                if create_panel() {
                    EmployeeCreateForm {
                        employees_list: employees_list,
                        on_close: move |_| {
                            create_panel.set(false);
                        },
                    }
                }
            }
            // EditForm 開閉可能なパネル
            div {
                class: "bg-green-100 transition-all duration-500 ease-in-out",
                class: if edit_panel() { "w-96" } else { "w-0 overflow-hidden" },
                if let Some(emp) = selectd_employee() {
                    EmployeeEditForm {
                        key: "{emp.id}",
                        employees_list: employees_list,
                        employee: emp,
                        on_close: move |_| {
                            selectd_employee.set(None);
                            edit_panel.set(false);
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn EmployeeCard(
    employee: Employee,
    is_selected: bool,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            key: "{employee.id}",
            class: "bg-white rounded-lg shadow-md p-4 hover:shadow-lg transition-all cursor-pointer",
            class: if is_selected { "ring-2 ring-blue-200" } else { "" },
            onclick: move |evt| on_click.call(evt),
            // header
            div { class: "flex justify-between items-center mb-3",
                div { class: "text-sm text-gray-600",
                    "従業員コード: "
                    span { class: "font-mono", { employee.employee_code} }
                }
                div {
                    class: "text-xs px-2 py-1 rounded",
                    class: if employee.is_active { "bg-green-100 text-green-800" } else { "bg-gray-100 text-gray-800" },
                    if employee.is_active {
                        "在籍中"
                    } else {
                        "退職済"
                    }
                }
            }
            div { class: "mb-2",
                div { class: "text-lg font-bold text-gray-800",
                    "{employee.first_name} {employee.last_name}"
                }
            }
        }
    }
}

// カスタムフック
pub fn use_field_validation(
    value: Signal<String>,
    validator: fn(&str) -> Result<(), String>,
) -> Memo<Option<String>> {
    use_memo(move || {
        let v = value();
        if v.trim().is_empty() {
            return None;
        }
        validator(&v).err()
    })
}

#[component]
pub fn InputLabel(
    value: Signal<String>,
    label: String,
    placeholder: String,
    error: ReadSignal<Option<String>>,
) -> Element {
    rsx!(
        div { class: "space-y-2",
            label { class: "block text-sm font-medium",
                span { "{label}" }
                if let Some(err) = error() {
                    span { class: "text-red-500 ml-2", "{err}" }
                }
            }
            input {
                r#type: "text",
                class: "w-full px-3 py-2 border rounded",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |evt| value.set(evt.value()),
            }
        }
    )
}

#[component]
pub fn EmployeeCreateForm(
    mut employees_list: Resource<Result<Vec<Employee>, ServerFnError>>,
    on_close: EventHandler<MouseEvent>,
) -> Element {
    let mut employee_code = use_signal(String::new);
    let first_name = use_signal(String::new);
    let last_name = use_signal(String::new);
    let mut err_msg = use_signal(String::new);
    let mut is_code_duplicate = use_signal(|| false);

    // use_memo でバリデーション結果
    let err_first_name = use_field_validation(first_name, validate_employee_name);
    let err_last_name = use_field_validation(last_name, validate_employee_name);
    let err_code = use_field_validation(employee_code, validate_employee_code);

    // フォームリセット関数
    let reset_form = move || {
        for mut field in [employee_code, first_name, last_name, err_msg] {
            field.set(String::new());
        }
    };

    // 全体のバリデーション状態 (formの有効/無効を制御)
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

    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        spawn(async move {
            let code = employee_code();
            let first = first_name();
            let last = last_name();
            match create_employee(code, first, last).await {
                Ok(_) => {
                    employees_list.restart();
                    reset_form();
                }
                Err(e) => {
                    err_msg.set(format!("従業員の作成に失敗しました: {}", e));
                }
            }
        });
    };

    use_effect(move || {
        let code = employee_code();

        // バリデーションエラーがある場合はチェックをしない
        if code.is_empty() || validate_employee_code(&code).is_err() {
            is_code_duplicate.set(false);
            return;
        }

        // 重複チェック
        spawn(async move {
            match check_employee_code_available(code, None).await {
                Ok(available) => {
                    is_code_duplicate.set(!available);
                }
                Err(_) => {
                    is_code_duplicate.set(false);
                }
            }
        });
    });

    rsx! {
        form { class: "p-4 space-y-4", onsubmit: handle_submit,

            div { class: "flex justify-between items-center mb-4",
                h2 { class: "text-lg font-bold text-blue-800", "従業員登録" }
                button {
                    r#type: "button",
                    class: "text-gray-500 hover:text-gray-700",
                    onclick: move |evt| on_close.call(evt),
                    "✕"
                }

            }

            div { class: "space-y-2",
                label { class: "block text-sm font-medium",
                    span { "従業員コード" }
                    if let Some(err) = err_code() {
                        span { class: "text-red-500 ml-2", "{err}" }
                    } else if is_code_duplicate() {
                        span { class: "text-red-500 ml-2", "この従業員コードは既に使用されています" }
                    }
                }
                input {
                    r#type: "text",
                    class: "w-full px-3 py-2 border rounded",
                    placeholder: "EMP001",
                    value: "{employee_code}",
                    oninput: move |evt| employee_code.set(evt.value())
                }
            }


            // InputLabelコンポーネントを使うと
            InputLabel {
                value: first_name,
                label: "名".to_string(),
                placeholder: "太郎".to_string(),
                error: err_first_name,
            }
            InputLabel {
                value: last_name,
                label: "姓".to_string(),
                placeholder: "山田".to_string(),
                error: err_last_name,
            }
            button {
                r#type: "submit",
                class: "w-full bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700",
                class: if !is_valid() { "opacity-50 cursor-not-allowed" },
                disabled: !is_valid(),
                "登録"
            }

            if !err_msg().is_empty() {
                div { class: "text-red-600 text-sm mt-2", "{err_msg}" }
            }
        }
    }
}

#[component]
pub fn EmployeeEditForm(
    mut employees_list: Resource<Result<Vec<Employee>, ServerFnError>>,
    employee: Employee,
    on_close: EventHandler<MouseEvent>,
) -> Element {
    let mut employee_code = use_signal(|| employee.employee_code.clone());
    let first_name = use_signal(|| employee.first_name.clone());
    let last_name = use_signal(|| employee.last_name.clone());
    let mut is_active = use_signal(|| employee.is_active);
    let mut err_msg = use_signal(String::new);
    let mut is_code_duplicate = use_signal(|| false);

    // use_memo でバリデーション結果
    let err_first_name = use_field_validation(first_name, validate_employee_name);
    let err_last_name = use_field_validation(last_name, validate_employee_name);
    let err_code = use_field_validation(employee_code, validate_employee_code);

    let employee_id = employee.id;
    use_effect(move || {
        let code = employee_code();
        // バリデーションエラーがある場合はチェックしない
        if code.is_empty() || validate_employee_code(&code).is_err() {
            is_code_duplicate.set(false);
            return;
        }
        // 重複チェック
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

    // 全体のバリデーション状態
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

    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        // フォームの値を取得
        spawn(async move {
            let id = employee.id;
            let code = employee_code();
            let first = first_name();
            let last = last_name();
            let active = is_active();

            // 更新処理を呼び出し
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
        form { class: "p-4 space-y-4", onsubmit: handle_submit,

            div { class: "flex justify-between items-center mb-4",
                h2 { class: "text-lg font-bold text-green-800", "従業員編集" }
                button {
                    r#type: "button",
                    class: "text-gray-500 hover:text-gray-700",
                    onclick: move |evt| on_close.call(evt),
                    "✕"
                }
            }

            div { class: "space-y-2",
                label { class: "block text-sm font-medium",
                    span { "従業員コード" }
                    if let Some(err) = err_code() {
                        span { class: "text-red-500 ml-2", "{err}" }
                    } else if is_code_duplicate() {
                        span { class: "text-red-500 ml-2", "この従業員コードは既に使用されています" }
                    }
                }
                input {
                    r#type: "text",
                    class: "w-full px-3 py-2 border rounded",
                    placeholder: "EMP001",
                    value: "{employee_code}",
                    oninput: move |evt| employee_code.set(evt.value())
                }
            }

            InputLabel {
                value: first_name,
                label: "名".to_string(),
                placeholder: "太郎".to_string(),
                error: err_first_name,
            }
            InputLabel {
                value: last_name,
                label: "姓".to_string(),
                placeholder: "山田".to_string(),
                error: err_last_name,
            }

            div { class: "space-y-2",
                label { class: "flex items-center space-x-2",
                    input {
                        r#type: "checkbox",
                        checked: is_active(),
                        oninput: move |evt| is_active.set(evt.checked()),
                    }
                    span { class: "text-sm font-medium", "在籍中" }
                }
            }

            button {
                r#type: "submit",
                class: "w-full bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700",
                class: if !is_valid() { "opacity-50 cursor-not-allowed" },
                disabled: !is_valid(),
                "更新"
            }

            if !err_msg().is_empty() {
                div { class: "text-red-600 text-sm mt-2", "{err_msg}" }
            }
        }
    }
}
