use crate::components::Navbar;
use crate::{components::search_bar::SearchBar, modules::employees::EmployeesPage};

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Base)]
    #[route("/")]
    Homepage {},
    #[route("/employees")]
    EmployeesPage {},
}

#[component]
pub fn Homepage() -> Element {
    rsx! {
        div { class: "p-4", "Welcome to the Homepage!" }
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
                div { class: "p-2 border-b border-gray-300 grid items-center grid-cols-3 gap-3",
                    SearchBar {}
                }
                Outlet::<Route> {}

            }
        }
    }
}
