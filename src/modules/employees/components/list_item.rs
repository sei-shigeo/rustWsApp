use crate::components::{
    Icon, ARROW_DOWN_LIGHT_ICON, ARROW_UP_LIGHT_ICON, IDCARD_LIGHT_ICON, MAP_LIGHT_ICON,
    MOBILE_LIGHT_ICON, PRSON_LIGHT_ICON,
};
use crate::modules::employees::models::Employee;
use dioxus::prelude::*;

/// Employee list item - clicking opens the edit panel.
/// This component is intentionally lightweight and uses the shared `Icon` component.
#[component]
pub fn EmployeeItem(
    employee: Employee,
    editing_id: Signal<Option<i32>>,
    show_create: Signal<bool>,
    on_refresh: EventHandler<()>,
) -> Element {
    // Keep the param to avoid unused warning (might be used later)
    let _ = on_refresh;

    rsx! {
        div { class: "relative bg-gray-50 border border-gray-400 rounded shadow-md flex justify-between cursor-pointer overflow-hidden hover:shadow-lg",
            div {
                class: "flex grow gap-2 h-20",
                onclick: move |_| {
                    editing_id.set(Some(employee.id));
                    show_create.set(true);
                },
                // avatar / placeholder
                div { class: "flex-none size-20 bg-[url(https://placehold.jp/80x80.png)] bg-no-repeat bg-center bg-cover" }
                div { class: "flex justify-between items-start gap-4",
                    ul { class: "truncate",
                        if let Some(code) = &employee.employee_code {
                            li { class: "text-xs text-gray-600 flex items-center gap-1",
                                Icon { path_d: IDCARD_LIGHT_ICON.to_string(), class: Some("inline-block text-gray-500 size-5".to_string()) }
                                span { "{code}" }
                            }
                        }
                        li { class: "text-xs text-gray-600 flex items-center gap-1",
                            Icon { path_d: PRSON_LIGHT_ICON.to_string(), class: Some("inline-block text-gray-500 size-5".to_string()) }
                            span { "{employee.last_name} {employee.first_name}" }
                        }
                        li { class: "text-xs text-gray-600 flex items-center gap-1",
                            Icon { path_d: MOBILE_LIGHT_ICON.to_string(), class: Some("inline-block text-gray-500 size-5".to_string()) }
                            span { "未設定" }
                        }
                        li { class: "text-xs text-gray-600 flex items-center gap-1",
                            Icon { path_d: MAP_LIGHT_ICON.to_string(), class: Some("inline-block text-gray-500 size-5".to_string()) }
                            span { "未設定" }
                        }
                    }
                }
            }
            div { class: "grid flex-none w-10",
                button { class: "hover:bg-gray-100 rounded-bl",
                    Icon { path_d: ARROW_UP_LIGHT_ICON.to_string(), class: Some("inline-block text-gray-400 size-8".to_string()) }
                }
                button { class: "hover:bg-gray-100 rounded-tl",
                    Icon { path_d: ARROW_DOWN_LIGHT_ICON.to_string(), class: Some("inline-block text-gray-400 size-8".to_string()) }
                }
            }
        }
    }
}
