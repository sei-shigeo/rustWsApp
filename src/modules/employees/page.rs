use dioxus::prelude::*;

use crate::components::SearchBar;
use crate::modules::employees::components::{EmployeeCard, EmployeeCreateForm, EmployeeEditForm};
use crate::modules::employees::handlers::get_employees_with_address;
use crate::modules::employees::models::EmployeeWithAddress;
use crate::Route;

// 共通のCSSクラス定数
const HEADER_CLASS: &str =
    "flex items-center justify-between h-14 px-6 border-b border-gray-200 bg-white";
const CONTENT_CLASS: &str = "flex-1 overflow-auto p-6";
const BUTTON_PRIMARY_CLASS: &str =
    "bg-amber-400 font-semibold py-2 px-6 rounded-lg hover:bg-amber-500 transition-all shadow-sm";
const BUTTON_TOGGLE_ACTIVE_CLASS: &str = "bg-amber-400 text-gray-800 hover:bg-amber-500";
const BUTTON_TOGGLE_INACTIVE_CLASS: &str = "bg-gray-200 text-gray-700 hover:bg-gray-300";
const BUTTON_TOGGLE_BASE_CLASS: &str =
    "px-4 py-2 rounded-lg text-sm font-semibold transition-all shadow-sm";
const PANEL_CLASS: &str =
    "border-l border-gray-200 bg-white transition-all duration-300 ease-in-out shadow-xl";
const GRID_CLASS: &str = "grid grid-cols-[repeat(auto-fill,minmax(360px,1fr))] gap-5";

#[component]
pub fn EmployeesPage() -> Element {
    let employees_list = use_resource(|| async move { get_employees_with_address().await });
    let mut selected_employee = use_signal(|| None::<EmployeeWithAddress>);
    let mut create_panel = use_signal(|| false);
    let mut edit_panel = use_signal(|| false);
    let mut show_active_only = use_signal(|| true); // true: 在職中, false: 退職済み

    rsx! {
        document::Title { "従業員一覧" }
        div { class: "flex h-full",
            div { class: "flex-1 flex flex-col overflow-hidden",
                // ヘッダー
                div { class: HEADER_CLASS,
                    div { class: "flex items-center gap-4",
                        h1 { class: "text-lg font-semibold text-gray-800", "従業員一覧" }
                    }
                    div { class: "flex items-center gap-3",
                        SearchBar {}
                        button {
                            class: format!("{} {}", BUTTON_PRIMARY_CLASS, if create_panel() { "opacity-0 pointer-events-none" } else { "opacity-100" }),
                            disabled: edit_panel(),
                            onclick: move |_| create_panel.set(true),
                            "＋ 新規登録"
                        }
                    }
                }

                // コンテンツ
                match &*employees_list.read_unchecked() {
                    Some(Ok(list)) => rsx! {
                        div { class: CONTENT_CLASS,
                            div { class: "mb-6 flex items-center justify-between",
                                div { class: "flex items-center gap-3",
                                    p { class: "text-gray-700 font-semibold text-lg",
                                        "全 {list.iter().filter(|e| e.is_active == show_active_only()).count()} 件"
                                    }
                                    div { class: "h-6 w-px bg-gray-300" }
                                    button {
                                        class: format!("{} {}", BUTTON_TOGGLE_BASE_CLASS, if show_active_only() {
                                            BUTTON_TOGGLE_ACTIVE_CLASS
                                        } else {
                                            BUTTON_TOGGLE_INACTIVE_CLASS
                                        }),
                                        onclick: move |_| show_active_only.set(!show_active_only()),
                                        if show_active_only() { "在職中のみ" } else { "退職済みのみ" }
                                    }
                                }
                            }
                            div { class: GRID_CLASS,
                                for emp in list.iter().filter(|e| e.is_active == show_active_only()) {
                                    {
                                        let emp = emp.clone();
                                        let is_selected = selected_employee()
                                            .is_some_and(|selected| selected.id == emp.id);
                                        let nav = navigator();
                                        rsx! {
                                            EmployeeCard {
                                                employee: emp.clone(),
                                                is_selected,
                                                on_click: move |_| {
                                                    if !create_panel() {
                                                        nav.push(Route::EmployeeDetailPage { id: emp.id });
                                                    }
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! {
                        div { class: "flex-1 flex items-center justify-center p-6",
                            div { class: "text-center",
                                p { class: "text-red-500 font-semibold text-lg mb-2", "エラーが発生しました" }
                                p { class: "text-gray-600 text-sm", "{e}" }
                            }
                        }
                    },
                    None => rsx! {
                        div { class: "flex-1 flex items-center justify-center p-6",
                            div { class: "text-center",
                                div { class: "inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-amber-500 mb-4" }
                                p { class: "text-gray-600", "読み込み中..." }
                            }
                        }
                    },
                }
            }

            // CreateForm 開閉可能なパネル
            div {
                class: format!("{} {}", PANEL_CLASS, if create_panel() { "w-[480px]" } else { "w-0 overflow-hidden" }),
                if create_panel() {
                    EmployeeCreateForm {
                        employees_list,
                        on_close: move |_| {
                            create_panel.set(false);
                        },
                    }
                }
            }

            // EditForm 開閉可能なパネル
            div {
                class: format!("{} {}", PANEL_CLASS, if edit_panel() { "w-[480px]" } else { "w-0 overflow-hidden" }),
                if let Some(emp) = selected_employee() {
                    EmployeeEditForm {
                        key: "{emp.id}",
                        employee: emp,
                        employees_list,
                        on_close: move |_| {
                            edit_panel.set(false);
                            selected_employee.set(None);
                        },
                    }
                }
            }
        }
    }
}
