use crate::components::{Icon, IconType};
use crate::modules::employees::models::EmployeeWithAddress;
use dioxus::prelude::*;

#[component]
pub fn EmployeeCard(
    employee: EmployeeWithAddress,
    is_selected: bool,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            key: "{employee.id}",
            class: "bg-white rounded-lg border border-gray-200 shadow-sm hover:shadow-md transition-all cursor-pointer overflow-hidden",
            class: if is_selected { "ring-2 ring-amber-400" } else { "" },
            onclick: move |evt| on_click.call(evt),

            div { class: "h-25 flex gap-4 items-center",
                // プロフィール画像
                img {
                    class: "w-25 h-full object-cover",
                    src: "https://placehold.jp/150x150.png",
                    alt: "{employee.first_name} {employee.last_name}"
                }

                // 従業員情報
                ul { class: "flex-1 grid gap-0.5 text-xs",
                    li { class: "flex items-center gap-2",
                        Icon {
                            icon_type: IconType::IdCard,
                            class: Some("size-4 text-gray-500".to_string())
                        }
                        span { class: "text-gray-700", "{employee.employee_code}" }
                    }
                    li { class: "flex items-center gap-2",
                        Icon {
                            icon_type: IconType::Person,
                            class: Some("size-4 text-gray-500".to_string())
                        }
                        span { class: "text-gray-700 font-medium",
                            "{employee.last_name} {employee.first_name}"
                        }
                    }
                    li { class: "flex items-center gap-2",
                        Icon {
                            icon_type: IconType::Mobile,
                            class: Some("size-4 text-gray-500".to_string())
                        }
                        span { class: "text-gray-600",
                            if let Some(mobile) = &employee.mobile {
                                "{mobile}"
                            } else {
                                "未設定"
                            }
                        }
                    }
                    li { class: "flex items-center gap-2",
                        Icon {
                            icon_type: IconType::Map,
                            class: Some("size-4 text-gray-500".to_string())
                        }
                        span { class: "text-gray-600",
                            if let (Some(prefecture), Some(city)) = (&employee.current_prefecture, &employee.current_city) {
                                "{prefecture} {city}"
                            } else {
                                "未設定"
                            }
                        }
                    }
                }

                // ソートボタン（将来の機能用）
                div { class: "grid w-10 h-full",
                    button {
                        class: "hover:bg-gray-100 transition-colors",
                        onclick: move |evt| {
                            evt.stop_propagation();
                        },
                        Icon {
                            icon_type: IconType::ArrowUp,
                            class: Some("size-4 text-gray-400".to_string())
                        }
                    }
                    button {
                        class: "hover:bg-gray-100 transition-colors",
                        onclick: move |evt| {
                            evt.stop_propagation();
                        },
                        Icon {
                            icon_type: IconType::ArrowDown,
                            class: Some("size-4 text-gray-400".to_string())
                        }
                    }
                }
            }
        }
    }
}
