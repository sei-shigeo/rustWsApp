use dioxus::prelude::*;

use crate::components::Navbar;
use crate::modules::employees::{EmployeeDetailPage, EmployeesPage};

// 共通のCSSクラス定数
const HEADER_CLASS: &str = "flex items-center h-14 px-6 border-b border-gray-200 bg-white";
const CONTENT_CLASS: &str = "flex-1 overflow-auto p-6";
const CARD_CLASS: &str = "bg-white rounded-lg shadow p-6";
const LINK_CARD_CLASS: &str =
    "p-4 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors";

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Base)]
    #[route("/")]
    Homepage {},
    #[route("/employees")]
    EmployeesPage {},
    #[route("/employees/:id")]
    EmployeeDetailPage { id: i32 },
}

#[component]
pub fn Homepage() -> Element {
    rsx! {
        div { class: "flex flex-col h-full overflow-hidden",
            // ヘッダー
            div { class: HEADER_CLASS,
                h1 { class: "text-lg font-semibold text-gray-800", "ホーム" }
            }

            // コンテンツ
            div { class: CONTENT_CLASS,
                div { class: "max-w-4xl mx-auto",
                    h2 { class: "text-xl font-bold mb-4", "ようこそ" }
                    p { class: "text-gray-600 mb-6", "和清商事 従業員管理システム" }
                    div { class: CARD_CLASS,
                        h2 { class: "text-xl font-semibold mb-4", "クイックアクセス" }
                        div { class: "grid gap-4 md:grid-cols-2",
                            Link {
                                to: Route::EmployeesPage {},
                                class: LINK_CARD_CLASS,
                                div { class: "flex items-center gap-3",
                                    svg {
                                        class: "size-8 text-amber-500",
                                        view_box: "0 0 24 24",
                                        fill: "currentColor",
                                        path { d: "M12 5.9a2.1 2.1 0 1 1 0 4.2a2.1 2.1 0 0 1 0-4.2m0 9c2.97 0 6.1 1.46 6.1 2.1v1.1H5.9V17c0-.64 3.13-2.1 6.1-2.1M12 4C9.79 4 8 5.79 8 8s1.79 4 4 4s4-1.79 4-4s-1.79-4-4-4m0 9c-2.67 0-8 1.34-8 4v3h16v-3c0-2.66-5.33-4-8-4" }
                                    }
                                    div {
                                        h3 { class: "font-semibold text-lg", "従業員記録簿" }
                                        p { class: "text-sm text-gray-600", "従業員情報の管理" }
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
pub fn Base() -> Element {
    let mut show_menu = use_signal(|| true);

    rsx! {
        div { class: "flex h-screen overflow-hidden bg-gray-50",
            // サイドバー
            Sidebar {
                show_menu,
                on_toggle: move |_| show_menu.toggle()
            }

            // メインコンテンツ
            MainContent {}
        }
    }
}

#[component]
fn Sidebar(show_menu: Signal<bool>, on_toggle: EventHandler<()>) -> Element {
    rsx! {
        aside {
            class: "shrink-0 border-r border-gray-300 bg-white transition-all duration-300 ease-in-out",
            class: if show_menu() { "w-64" } else { "w-16" },

            Navbar {
                show_menu,
                on_toggle
            }
        }
    }
}

#[component]
fn MainContent() -> Element {
    rsx! {
        main {
            class: "flex-1 overflow-auto",
            Outlet::<Route> {}
        }
    }
}
