use super::create::EmployeeCreate;
use super::edit::{EmployeeEdit, EmployeeItem};
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
                    class: "font-bold py-2 px-4 rounded bg-amber-300 hover:bg-amber-400 text-black",
                    onclick: move |_| show_create.toggle(),
                    span { if show_create() { "閉じる" } else { "新規登録" } },
                }
            }
            div { class: "flex h-full",
                // Main Content: 従業員一覧
                div { class: "bg-green-50 flex-1 p-4",
                    h2 { class: "text-xl  font-bold mb-4", "従業員一覧" }

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

                // Right panel: Create または Edit を切り替える
                div {
                    class: "flex-none bg-gray-50 transition-all duration-300",
                    class: if show_create() { "w-1/2" } else { "w-0 opacity-0" },

                    // editing_id が設定されているときは編集フォームを表示する
                    if editing_id().is_some() {
                        // editing_id() は Option<i32> を返すので unwrap() を使って中身を取得します。
                        // rsx! マクロのパースを壊さないように、内部で明示的に rsx! を使って子要素を返しています。
                        let id = editing_id();
                        match &*employees.read_unchecked() {
                            Some(Ok(list)) => {
                                // リストから該当従業員を探して Edit コンポーネントへ渡す
                                let emp_opt = list.clone().into_iter().find(|e| e.id == id.unwrap());
                                if let Some(emp) = emp_opt {
                                    EmployeeEdit {
                                        employee: emp,
                                        on_close: move |_| editing_id.set(None),
                                        on_refresh: move |_| employees.restart(),
                                    }
                                } else {
                                    rsx! { div { class: "p-4 text-center text-gray-600", "従業員が見つかりません" } }
                                }
                            }
                            Some(Err(_)) => rsx! { div { class: "p-4 text-center text-gray-600", "データ読み取りエラー" } },
                            None => rsx! { div { class: "p-4 text-center text-gray-600", "読み込み中..." } },
                        }
                    } else {
                        // editing_id が None のときは新規作成フォームを表示
                        EmployeeCreate { on_created: move |_| employees.restart() }
                    }
                }
            }
        }
    }
}
