use super::create::EmployeeCreate;
use super::edit::EmployeeItem;
use crate::modules::employees::handlers::get_employees;
use dioxus::prelude::*;

/// 従業員ページのメインコンポーネント
#[component]
pub fn EmployeePage() -> Element {
    let editing_id = use_signal(|| None::<i32>);
    let mut show_create = use_signal(|| true);
    let mut employees = use_resource(|| async move { get_employees().await });

    rsx! {
        div { class: "h-full",
            div { class: "p-2 text-right",
                button {
                    class: "font-bold py-2 px-4 rounded",
                    class: if show_create() { "bg-blue-300 hover:bg-blue-400 text-white" } else { "bg-amber-300 hover:bg-amber-400 text-black" },
                    onclick: move |_| show_create.toggle(),
                    span { if show_create() { "閉じる" } else { "新規登録" } },
                }
            }
            div { class: "flex h-full",
                // show list
                div { class: "bg-green-50 flex-1 p-4",
                    h2 { class: "text-2xl  font-bold mb-4", "従業員一覧" }

                    match &*employees.read_unchecked() {
                        Some(Ok(list)) => {
                            let employees_list = list.clone();
                            rsx! {
                                if employees_list.is_empty() {
                                    div { class: "p-8 text-center text-gray-500 border border-dashed rounded",
                                        "データがありません"
                                        p { class: "text-sm mt-2", "上のフォームから従業員を登録してください" }
                                    }
                                } else {
                                    div { class: "grid gap-2",
                                        for employee in employees_list {
                                            EmployeeItem {
                                                key: "{employee.id}",
                                                employee,
                                                editing_id,
                                                on_refresh: move |_| employees.restart(),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(Err(e)) => rsx! {
                            div { class: "p-4 bg-red-100 border border-red-400 text-red-700 rounded",
                                strong { "エラーが発生しました: " }
                                p { class: "mt-2", "{e}" }
                            }
                        },
                        None => rsx! {
                            div { class: "p-4 text-center text-gray-600",
                                "読み込み中..."
                            }
                        },
                    }
                }
                div {
                    class: "flex-none bg-gray-50 transition-all duration-300",
                    class: if show_create() { "w-1/2" } else { "w-0 opacity-0" },
                    EmployeeCreate { on_created: move |_| employees.restart() }
                }
            }
        }
    }
}
