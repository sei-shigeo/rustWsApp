use crate::components::{Icon, IconType};
use crate::modules::department_position_history::handlers::get_department_position_history_by_employee;
use crate::modules::department_position_history::DepartmentPositionHistory;
use crate::modules::employee_documents::components::{DocumentList, DocumentUploadForm};
use crate::modules::employees::components::employee_full_edit_form::EmployeeFullEditForm;
use crate::modules::employees::handlers::{delete_employee, get_all_addresses, get_employee_full};
use crate::modules::employees::models::{Address, EmployeeFull};
use crate::modules::licenses::handlers::get_licenses_by_employee;
use crate::modules::licenses::License;
use crate::Route;

use dioxus::prelude::*;

#[component]
pub fn EmployeeDetailPage(id: i32) -> Element {
    let mut employee_resource = use_resource(move || async move { get_employee_full(id).await });
    let mut edit_mode = use_signal(|| false);
    let mut show_delete_confirm = use_signal(|| false);
    let mut is_deleting = use_signal(|| false);
    let nav = navigator();

    rsx! {
        document::Title { "従業員詳細" }
        div { class: "flex h-full bg-gray-50",
            div { class: "flex-1 flex flex-col overflow-hidden",
                // ヘッダー
                div { class: "flex items-center justify-between h-14 px-6 border-b border-gray-200 bg-white shadow-sm",
                    div { class: "flex items-center gap-3",
                        button {
                            class: "p-2 hover:bg-gray-100 rounded-lg transition-colors",
                            onclick: move |_| nav.go_back(),
                            title: "一覧に戻る",
                            Icon {
                                icon_type: IconType::ArrowLeft,
                                class: Some("size-5".to_string())
                            }
                        }
                        h1 { class: "text-lg font-semibold text-gray-800", "従業員詳細" }
                    }
                    div { class: "flex items-center gap-3",
                        if !edit_mode() {
                            button {
                                class: "font-semibold py-2 px-6 rounded-lg transition-all shadow-sm bg-red-500 text-white hover:bg-red-600",
                                onclick: move |_| show_delete_confirm.set(true),
                                "🗑 削除"
                            }
                        }
                        button {
                            class: "font-semibold py-2 px-6 rounded-lg transition-all shadow-sm",
                            class: if edit_mode() {
                                "bg-gray-200 text-gray-700 hover:bg-gray-300"
                            } else {
                                "bg-amber-400 text-gray-800 hover:bg-amber-500"
                            },
                            onclick: move |_| edit_mode.set(!edit_mode()),
                            if edit_mode() { "✕ キャンセル" } else { "✎ 編集" }
                        }
                    }
                }

                // コンテンツ
                div { class: "flex-1 overflow-auto",
                    match &*employee_resource.read_unchecked() {
                        Some(Ok(Some(employee))) => {
                            if edit_mode() {
                                rsx! {
                                    EmployeeFullEditForm {
                                        employee: employee.clone(),
                                        on_close: move |_| {
                                            edit_mode.set(false);
                                            employee_resource.restart();
                                        },
                                    }
                                }
                            } else {
                                rsx! {
                                    EmployeeDetailView { employee: employee.clone() }
                                }
                            }
                        },
                        Some(Ok(None)) => rsx! {
                            div { class: "flex items-center justify-center p-12",
                                div { class: "text-center",
                                    p { class: "text-red-500 font-semibold text-lg mb-2", "従業員が見つかりませんでした" }
                                    p { class: "text-gray-600 text-sm", "指定されたIDの従業員は存在しません。" }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! {
                            div { class: "flex items-center justify-center p-12",
                                div { class: "text-center",
                                    p { class: "text-red-500 font-semibold text-lg mb-2", "エラーが発生しました" }
                                    p { class: "text-gray-600 text-sm", "{e}" }
                                }
                            }
                        },
                        None => rsx! {
                            div { class: "flex items-center justify-center p-12",
                                div { class: "text-center",
                                    div { class: "inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-amber-500 mb-4" }
                                    p { class: "text-gray-600", "読み込み中..." }
                                }
                            }
                        },
                    }
                }

                // 削除確認ダイアログ
                if show_delete_confirm() {
                    div {
                        class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                        onclick: move |_| {
                            if !is_deleting() {
                                show_delete_confirm.set(false);
                            }
                        },
                        div {
                            class: "bg-white rounded-xl shadow-2xl p-6 max-w-md w-full mx-4",
                            onclick: move |e| e.stop_propagation(),
                            div { class: "flex items-start gap-4 mb-6",
                                div { class: "shrink-0 w-12 h-12 bg-red-100 rounded-full flex items-center justify-center",
                                    svg {
                                        class: "w-6 h-6 text-red-600",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        path { d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" }
                                    }
                                }
                                div { class: "flex-1",
                                    h3 { class: "text-lg font-bold text-gray-900 mb-2", "従業員を削除しますか？" }
                                    p { class: "text-sm text-gray-600",
                                        "この操作は取り消せません。従業員データが完全に削除されます。"
                                    }
                                }
                            }
                            div { class: "flex gap-3 justify-end",
                                button {
                                    class: "px-4 py-2 text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 transition-colors font-semibold",
                                    disabled: is_deleting(),
                                    onclick: move |_| show_delete_confirm.set(false),
                                    "キャンセル"
                                }
                                button {
                                    class: "px-4 py-2 text-white bg-red-600 rounded-lg hover:bg-red-700 transition-colors font-semibold disabled:opacity-50 disabled:cursor-not-allowed",
                                    disabled: is_deleting(),
                                    onclick: move |_| {
                                        spawn(async move {
                                            is_deleting.set(true);
                                            match delete_employee(id).await {
                                                Ok(_) => {
                                                    nav.push(Route::EmployeesPage {});
                                                }
                                                Err(_e) => {
                                                    // エラーハンドリング（必要に応じてトースト通知等を追加）
                                                    is_deleting.set(false);
                                                    show_delete_confirm.set(false);
                                                }
                                            }
                                        });
                                    },
                                    if is_deleting() {
                                        span { class: "flex items-center gap-2",
                                            div { class: "inline-block animate-spin rounded-full h-4 w-4 border-b-2 border-white" }
                                            "削除中..."
                                        }
                                    } else {
                                        "削除する"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EmployeeDetailView(employee: EmployeeFull) -> Element {
    let addresses_resource =
        use_resource(move || async move { get_all_addresses(employee.id).await });
    let dept_history_resource: Resource<Result<Vec<DepartmentPositionHistory>, ServerFnError>> =
        use_resource(move || async move {
            get_department_position_history_by_employee(employee.id).await
        });
    let licenses_resource: Resource<Result<Vec<License>, ServerFnError>> =
        use_resource(move || async move { get_licenses_by_employee(employee.id).await });

    rsx! {
        div { class: "max-w-5xl mx-auto p-6 space-y-6",
            // 基本情報セクション
            div { class: "bg-white rounded-xl shadow-sm border border-gray-200 p-6",
                h3 { class: "text-xl font-bold mb-6 pb-3 border-b-2 border-amber-400",
                    "基本情報"
                }
                div { class: "grid grid-cols-2 gap-4",
                    DetailFieldView { label: "従業員コード".to_string(), value: employee.employee_code.clone() }
                    DetailFieldView {
                        label: "ステータス".to_string(),
                        value: if employee.is_active { "在職中".to_string() } else { "退職済み".to_string() }
                    }
                    DetailFieldView { label: "姓".to_string(), value: employee.last_name.clone() }
                    DetailFieldView { label: "名".to_string(), value: employee.first_name.clone() }
                    DetailFieldView {
                        label: "姓（カナ）".to_string(),
                        value: employee.last_name_kana.clone().unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "名（カナ）".to_string(),
                        value: employee.first_name_kana.clone().unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "法的名称".to_string(),
                        value: employee.legal_name.clone().unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "性別".to_string(),
                        value: employee.gender.clone().unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "生年月日".to_string(),
                        value: employee.birth_date.map(|d| d.to_string()).unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "国籍ID".to_string(),
                        value: employee.nationality_id.map(|n| n.to_string()).unwrap_or_default()
                    }
                }
            }

            // 連絡先情報セクション
            div { class: "bg-white rounded-xl shadow-sm border border-gray-200 p-6",
                h3 { class: "text-xl font-bold mb-6 pb-3 border-b-2 border-amber-400",
                    "連絡先情報"
                }
                div { class: "grid grid-cols-2 gap-4",
                    DetailFieldView {
                        label: "メールアドレス".to_string(),
                        value: employee.email.clone().unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "電話番号".to_string(),
                        value: employee.phone.clone().unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "携帯電話".to_string(),
                        value: employee.mobile.clone().unwrap_or_default()
                    }
                }
            }

            // 雇用情報セクション
            div { class: "bg-white rounded-xl shadow-sm border border-gray-200 p-6",
                h3 { class: "text-xl font-bold mb-6 pb-3 border-b-2 border-amber-400",
                    "雇用情報"
                }
                div { class: "grid grid-cols-2 gap-4",
                    DetailFieldView {
                        label: "会社ID".to_string(),
                        value: employee.company_id.map(|n| n.to_string()).unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "営業所ID".to_string(),
                        value: employee.office_id.map(|n| n.to_string()).unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "部署ID".to_string(),
                        value: employee.department_id.map(|n| n.to_string()).unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "役職ID".to_string(),
                        value: employee.position_id.map(|n| n.to_string()).unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "雇用開始日".to_string(),
                        value: employee.start_date.map(|d| d.to_string()).unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "雇用終了日".to_string(),
                        value: employee.end_date.map(|d| d.to_string()).unwrap_or_default()
                    }
                }
            }

            // ドライバー情報セクション
            div { class: "bg-white rounded-xl shadow-sm border border-gray-200 p-6",
                h3 { class: "text-xl font-bold mb-6 pb-3 border-b-2 border-amber-400",
                    "ドライバー選任情報"
                }
                div { class: "grid grid-cols-2 gap-4",
                    DetailFieldView {
                        label: "選任開始日".to_string(),
                        value: employee.driver_start_date.map(|d| d.to_string()).unwrap_or_default()
                    }
                    DetailFieldView {
                        label: "選任終了日".to_string(),
                        value: employee.driver_end_date.map(|d| d.to_string()).unwrap_or_default()
                    }
                    div { class: "col-span-2",
                        DetailFieldView {
                            label: "選任解除理由".to_string(),
                            value: employee.driver_end_note.clone().unwrap_or_default()
                        }
                    }
                }
            }

            // 運転免許証セクション
            div { class: "bg-white rounded-xl shadow-sm border border-gray-200 p-6",
                h3 { class: "text-xl font-bold mb-6 pb-3 border-b-2 border-amber-400",
                    "運転免許証"
                }
                match &*licenses_resource.read_unchecked() {
                    Some(Ok(licenses_list)) => {
                        if licenses_list.is_empty() {
                            rsx! {
                                p { class: "text-gray-500 text-center py-4", "運転免許証が登録されていません" }
                            }
                        } else {
                            rsx! {
                                div { class: "space-y-4",
                                    for license in licenses_list.iter() {
                                        LicenseView { license: license.clone() }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! {
                        p { class: "text-red-500 text-center py-4", "運転免許証の読み込みに失敗しました: {e}" }
                    },
                    None => rsx! {
                        div { class: "flex justify-center py-4",
                            div { class: "inline-block animate-spin rounded-full h-6 w-6 border-b-2 border-amber-500" }
                        }
                    },
                }
            }

            // ドキュメント管理セクション
            DocumentManagementSection { employee_id: employee.id }

            // 配属履歴セクション
            div { class: "bg-white rounded-xl shadow-sm border border-gray-200 p-6",
                h3 { class: "text-xl font-bold mb-6 pb-3 border-b-2 border-amber-400",
                    "部署・役職・営業所履歴"
                }
                match &*dept_history_resource.read_unchecked() {
                    Some(Ok(history_list)) => {
                        if history_list.is_empty() {
                            rsx! {
                                p { class: "text-gray-500 text-center py-4", "配属履歴が登録されていません" }
                            }
                        } else {
                            rsx! {
                                div { class: "space-y-4",
                                    for history in history_list.iter() {
                                        DepartmentPositionHistoryView { history: history.clone() }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! {
                        p { class: "text-red-500 text-center py-4", "配属履歴の読み込みに失敗しました: {e}" }
                    },
                    None => rsx! {
                        div { class: "flex justify-center py-4",
                            div { class: "inline-block animate-spin rounded-full h-6 w-6 border-b-2 border-amber-500" }
                        }
                    },
                }
            }

            // 住所情報セクション
            div { class: "bg-white rounded-xl shadow-sm border border-gray-200 p-6",
                h3 { class: "text-xl font-bold mb-6 pb-3 border-b-2 border-amber-400",
                    "住所履歴"
                }
                match &*addresses_resource.read_unchecked() {
                    Some(Ok(addresses)) => {
                        if addresses.is_empty() {
                            rsx! {
                                p { class: "text-gray-500 text-center py-4", "住所情報が登録されていません" }
                            }
                        } else {
                            rsx! {
                                div { class: "space-y-4",
                                    for address in addresses {
                                        AddressView { address: address.clone() }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! {
                        p { class: "text-red-500 text-center py-4", "住所情報の読み込みに失敗しました: {e}" }
                    },
                    None => rsx! {
                        div { class: "flex justify-center py-4",
                            div { class: "inline-block animate-spin rounded-full h-6 w-6 border-b-2 border-amber-500" }
                        }
                    },
                }
            }

            // メタ情報セクション
            div { class: "bg-white rounded-xl shadow-sm border border-gray-200 p-6",
                h3 { class: "text-xl font-bold mb-6 pb-3 border-b-2 border-amber-400",
                    "メタ情報"
                }
                div { class: "grid grid-cols-2 gap-4",
                    DetailFieldView {
                        label: "作成日時".to_string(),
                        value: employee.created_at.format("%Y-%m-%d %H:%M:%S").to_string()
                    }
                    DetailFieldView {
                        label: "更新日時".to_string(),
                        value: employee.updated_at.format("%Y-%m-%d %H:%M:%S").to_string()
                    }
                }
            }
        }
    }
}

#[component]
fn DepartmentPositionHistoryView(history: DepartmentPositionHistory) -> Element {
    rsx! {
        div {
            class: "border border-gray-200 rounded-lg p-4",
            class: if history.is_current { "bg-amber-50 border-amber-300" } else { "bg-gray-50" },

            div { class: "flex items-start justify-between mb-3",
                div { class: "flex items-center gap-2",
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
        }
    }
}

#[component]
fn AddressView(address: Address) -> Element {
    rsx! {
        div {
            class: "border border-gray-200 rounded-lg p-4",
            class: if address.is_current { "bg-amber-50 border-amber-300" } else { "bg-gray-50" },

            div { class: "flex items-start justify-between mb-3",
                div { class: "flex items-center gap-2",
                    if address.is_current {
                        span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-amber-400 text-gray-800",
                            "現住所"
                        }
                    }
                    span { class: "text-sm text-gray-600",
                        "{address.start_date} ～ "
                        if let Some(end_date) = address.end_date {
                            "{end_date}"
                        } else {
                            "現在"
                        }
                    }
                }
            }

            div { class: "grid grid-cols-2 gap-3 text-sm",
                div { class: "space-y-1",
                    p { class: "text-xs text-gray-500 font-semibold", "郵便番号" }
                    p { class: "text-gray-800", "〒{address.postal_code}" }
                }
                div { class: "space-y-1",
                    p { class: "text-xs text-gray-500 font-semibold", "都道府県" }
                    p { class: "text-gray-800", "{address.prefecture}" }
                }
                div { class: "col-span-2 space-y-1",
                    p { class: "text-xs text-gray-500 font-semibold", "住所" }
                    p { class: "text-gray-800",
                        "{address.city} {address.street}"
                        if let Some(building) = &address.building {
                            " {building}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LicenseView(license: License) -> Element {
    rsx! {
        div {
            class: "border border-gray-200 rounded-lg p-4",
            class: if license.is_active { "bg-green-50 border-green-300" } else { "bg-gray-50" },

            div { class: "flex items-start justify-between mb-3",
                div { class: "flex items-center gap-2",
                    if license.is_active {
                        span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-400 text-gray-800",
                            "有効"
                        }
                    }
                    span { class: "text-sm text-gray-600 font-semibold",
                        "免許種別ID: {license.license_type_id}"
                    }
                }
            }

            div { class: "grid grid-cols-2 gap-3 text-sm",
                if let Some(license_number) = &license.license_number {
                    div { class: "space-y-1",
                        p { class: "text-xs text-gray-500 font-semibold", "免許証番号" }
                        p { class: "text-gray-800", "{license_number}" }
                    }
                }
                if let Some(issue_date) = license.issue_date {
                    div { class: "space-y-1",
                        p { class: "text-xs text-gray-500 font-semibold", "交付日" }
                        p { class: "text-gray-800", "{issue_date}" }
                    }
                }
                div { class: "space-y-1",
                    p { class: "text-xs text-gray-500 font-semibold", "有効期限" }
                    p { class: "text-gray-800", "{license.expiration_date}" }
                }
                if let Some(authority) = &license.issuing_authority {
                    div { class: "space-y-1",
                        p { class: "text-xs text-gray-500 font-semibold", "交付機関" }
                        p { class: "text-gray-800", "{authority}" }
                    }
                }
                if let Some(conditions) = &license.conditions {
                    div { class: "col-span-2 space-y-1",
                        p { class: "text-xs text-gray-500 font-semibold", "条件等" }
                        p { class: "text-gray-800", "{conditions}" }
                    }
                }
            }
        }
    }
}

#[component]
fn DetailFieldView(label: String, value: String) -> Element {
    rsx! {
        div { class: "space-y-1.5",
            p { class: "text-sm text-gray-500 font-semibold uppercase tracking-wide", "{label}" }
            p { class: "text-base text-gray-800",
                if value.is_empty() {
                    span { class: "text-gray-400", "未設定" }
                } else {
                    "{value}"
                }
            }
        }
    }
}

/// ドキュメント管理セクション
#[component]
fn DocumentManagementSection(employee_id: i32) -> Element {
    let mut refresh_trigger = use_signal(|| 0);

    rsx! {
        div { class: "bg-white rounded-xl shadow-sm border border-gray-200 p-6",
            h3 { class: "text-xl font-bold mb-6 pb-3 border-b-2 border-amber-400",
                "ドキュメント管理"
            }

            div { class: "space-y-6",
                // アップロードフォーム
                DocumentUploadForm {
                    employee_id,
                    on_upload_success: move |_| {
                        // アップロード成功時にリストを更新
                        refresh_trigger.set(refresh_trigger() + 1);
                    }
                }

                // ドキュメント一覧
                DocumentList {
                    employee_id,
                    refresh_trigger: refresh_trigger()
                }
            }
        }
    }
}
