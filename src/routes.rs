use crate::components::Navbar;
use crate::{components::search_bar::SearchBar, modules::employees::EmployeePage};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Base)]
    #[route("/")]
    Home {},
    #[route("/employees_page")]
    EmployeePage {},
}

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "welcome wsapp" }
    }
}

#[component]
pub fn Base() -> Element {
    let show_menu = use_signal(|| false);
    rsx! {
        div { class: "flex h-screen",
            div {
                class: if !show_menu() { "w-18" } else { "w-60" },
                class: "border-r  border-gray-300 transition-all duration-300",

                Navbar { show_menu }
            }
            div { class: "flex-1",

                // Main Header
                div { class: "p-4 border-b border-gray-300 flex items-center",
                    SearchBar {}
                }
                Outlet::<Route> {}

            }
        }
    }
}

/* Navbar moved to shared component:
Use `crate::components::Navbar` (defined in `src/components/nav.rs`) instead of the local implementation. */
