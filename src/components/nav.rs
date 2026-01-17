use crate::components::{Icon, IconType};
use dioxus::prelude::*;

#[component]
pub fn Navbar(show_menu: Signal<bool>, on_toggle: EventHandler<()>) -> Element {
    rsx! {
        nav { class: "flex flex-col h-full",
            // ヘッダー部分（トグルボタン）- メインヘッダーと高さを統一（h-14）
            div { class: "flex items-center h-14 px-3 border-b border-gray-200",
                button {
                    class: "flex items-center gap-2 transition-colors hover:bg-gray-100 rounded-lg px-2 py-1 w-full",
                    onclick: move |_| on_toggle.call(()),
                    title: if show_menu() { "メニューを閉じる" } else { "メニューを開く" },
                    Icon {
                        icon_type: IconType::Menu,
                        class: Some("size-5 text-gray-700".to_string())
                    }
                    if show_menu() {
                        span { class: "text-sm font-medium text-gray-800 whitespace-nowrap", "和清商事" }
                    }
                }
            }

            // ナビゲーションリンク
            div { class: "flex-1 px-3 py-4 space-y-1",
                NavLink {
                    to: crate::routes::Route::Homepage {},
                    icon_type: IconType::Home,
                    label: "ホーム",
                    show_label: show_menu()
                }
                NavLink {
                    to: crate::routes::Route::EmployeesPage {},
                    icon_type: IconType::Person,
                    label: "従業員記録簿",
                    show_label: show_menu()
                }
            }

            // フッター（オプション）
            if show_menu() {
                div { class: "px-6 py-3 border-t border-gray-200",
                    p { class: "text-xs text-gray-500 text-center",
                        "© 2025 和清商事"
                    }
                }
            }
        }
    }
}

#[component]
fn NavLink(
    to: crate::routes::Route,
    icon_type: IconType,
    label: &'static str,
    show_label: bool,
) -> Element {
    rsx! {
        Link {
            to: to,
            class: if !show_label {
                "group flex items-center justify-center px-3 py-2 rounded-lg transition-colors hover:bg-amber-200"
            } else {
                "group flex items-center px-3 py-2 rounded-lg transition-colors hover:bg-amber-200"
            },
            active_class: "bg-amber-300 text-gray-700",

            Icon {
                icon_type: icon_type,
                class: Some("size-5 shrink-0 text-gray-600 group-hover:text-amber-700".to_string())
            }

            if show_label {
                span {
                    class: "ml-3 text-sm font-medium text-gray-700 group-hover:text-amber-900 whitespace-nowrap",
                    "{label}"
                }
            }
        }
    }
}
