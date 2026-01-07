use super::create::EmployeeCreate;
use super::edit::EmployeeEdit;
use super::right_panel::RightPanel;
use crate::modules::employees::handlers::get_employees;
use dioxus::prelude::*;

/// 従業員ページのメインコンポーネント
#[component]
pub fn EmployeePage() -> Element {
    let mut editing_id = use_signal(|| None::<i32>);
    let mut show_create = use_signal(|| false);
    let mut employees = use_resource(|| async move { get_employees().await });

    // 現在の編集中 ID を外で取得しておく（rsx! 内で let を使わないため）
    let current_editing = editing_id();

    // 右パネル表示内容を rsx! の外で構築しておく（rsx! のパーサ問題回避）
    let right_panel = {
        // editing_id が設定されている場合は Edit、そうでなければ Create を返す
        if let Some(id) = current_editing {
            match &*employees.read_unchecked() {
                Some(Ok(list)) => {
                    let emp_opt = list.clone().into_iter().find(|e| e.id == id);
                    if let Some(emp) = emp_opt {
                        rsx! {
                            EmployeeEdit {
                                key: "{emp.id}",
                                employee: emp,
                                on_close: move |()| { editing_id.set(None); show_create.set(false); },
                                on_refresh: move |()| employees.restart(),
                            }
                        }
                    } else {
                        rsx! { div { class: "p-4 text-center text-gray-600", "従業員が見つかりません" } }
                    }
                }
                Some(Err(_)) => {
                    rsx! { div { class: "p-4 text-center text-gray-600", "データ読み取りエラー" } }
                }
                None => rsx! { div { class: "p-4 text-center text-gray-600", "読み込み中..." } },
            }
        } else {
            rsx! { EmployeeCreate { on_created: move |()| employees.restart() } }
        }
    };

    rsx! {
        div { class: "h-full",
            div { class: "p-2 text-right",
                button {
                    class: "font-bold py-2 px-4 rounded bg-amber-300 hover:bg-amber-400 text-black",
                    onclick: move |_| {
                        // トグルで右パネルを開く／閉じる。
                        // 「開く」側になったときは編集中の ID をクリアして
                        // edit フォームが表示されないようにする。
                        show_create.set(!show_create());
                        if show_create() {
                            editing_id.set(None);
                        }
                    },
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
                                    div { class: "grid gap-2 grid-cols-[repeat(auto-fill,minmax(300px,1fr))]",
                                        for employee in employees_list {
                                            crate::modules::employees::components::EmployeeItem {
                                                key: "{employee.id}",
                                                employee,
                                                editing_id,
                                                show_create,
                                                on_refresh: move |()| employees.restart(),
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
                RightPanel {
                    show: show_create,
                    { right_panel }
                }
            }
        }
    }
}
