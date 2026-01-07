use dioxus::prelude::*;

const OPEN_ICON: &str = "M17.51 3.87L15.73 2.1L5.84 12l9.9 9.9l1.77-1.77L9.38 12z";
const CLOSE_ICON: &str = "M6.23 20.23L8 22l10-10L8 2L6.23 3.77L14.46 12z";
const HOME_ICON: &str = "M12 5.69l5 4.5V18h-2v-6H9v6H7v-7.81zM12 3L2 12h3v8h6v-6h2v6h6v-8h3z";
const PRSON_ICON: &str = "M12 5.9a2.1 2.1 0 1 1 0 4.2a2.1 2.1 0 0 1 0-4.2m0 9c2.97 0 6.1 1.46 6.1 2.1v1.1H5.9V17c0-.64 3.13-2.1 6.1-2.1M12 4C9.79 4 8 5.79 8 8s1.79 4 4 4s4-1.79 4-4s-1.79-4-4-4m0 9c-2.67 0-8 1.34-8 4v3h16v-3c0-2.66-5.33-4-8-4";

#[component]
pub fn Navbar(show_menu: Signal<bool>) -> Element {
    rsx! {
        nav { class: "flex flex-col p-4 gap-4",
            button {
                class: "bg-amber-300 hover:bg-amber-400 rounded-md flex items-center p-2",
                onclick: move |_| show_menu.toggle(),
                svg {
                    view_box: "0 0 24 24",
                    fill: "currentColor",
                    class: "shrink-0 size-6 mr-4",
                    path { d: if show_menu() { OPEN_ICON } else { CLOSE_ICON } }
                }
                span { class: "truncate", "Wasei App" }
            }
            Link {
                class: "bg-amber-300 hover:bg-amber-400 rounded-md flex items-center p-2",
                to: crate::routes::Route::Home {},
                svg {
                    view_box: "0 0 24 24",
                    fill: "currentColor",
                    class: "shrink-0 size-6 mr-4",
                    path { d: HOME_ICON }
                }
                span { class: "truncate", "Home" }
            }
            Link {
                class: "bg-amber-300 hover:bg-amber-400 rounded-md flex items-center p-2",
                to: crate::routes::Route::EmployeePage {},
                svg {
                    view_box: "0 0 24 24",
                    fill: "currentColor",
                    class: "shrink-0 size-6 mr-4",
                    path { d: PRSON_ICON }
                }
                span { class: "truncate", "従業員記録簿" }
            }
        }
    }
}
