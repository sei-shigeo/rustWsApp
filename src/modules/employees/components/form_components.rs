use dioxus::prelude::*;

/// フィールドのバリデーションフック
pub fn use_field_validation(
    value: Signal<String>,
    validator: fn(&str) -> Result<(), String>,
) -> Memo<Option<String>> {
    use_memo(move || {
        let v = value();
        if v.trim().is_empty() {
            return None;
        }
        validator(&v).err()
    })
}

/// ラベル付き入力フィールド
#[component]
pub fn InputLabel(
    value: Signal<String>,
    label: String,
    placeholder: String,
    error: Memo<Option<String>>,
) -> Element {
    rsx! {
        div { class: "space-y-2",
            label { class: "block text-sm font-medium text-gray-700",
                span { "{label}" }
                if let Some(err) = error() {
                    span { class: "text-red-500 text-xs ml-2", "{err}" }
                }
            }
            input {
                r#type: "text",
                class: "w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-amber-400 focus:border-transparent transition-colors",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |evt| value.set(evt.value()),
            }
        }
    }
}

/// フィールド表示用ラベル（編集不可）
#[component]
pub fn InputLabelField(label: String, required: bool, children: Element) -> Element {
    rsx! {
        div { class: "space-y-2",
            label { class: "block text-sm font-medium text-gray-700",
                "{label}"
                if required {
                    span { class: "text-red-500 ml-1", "*" }
                }
            }
            {children}
        }
    }
}

/// 詳細表示用フィールド
#[component]
pub fn DetailField(label: String, value: String) -> Element {
    rsx! {
        div { class: "space-y-1.5",
            p { class: "text-sm text-gray-500 font-semibold uppercase tracking-wide", "{label}" }
            p { class: "text-base text-gray-800",
                if value.is_empty() {
                    span { class: "text-gray-400", "未設定" }
                } else {
                    "{value}"
                }
            }
        }
    }
}

/// セクションヘッダー
#[component]
pub fn SectionHeader(title: String) -> Element {
    rsx! {
        h3 { class: "text-xl font-bold mb-6 pb-3 border-b-2 border-amber-400",
            "{title}"
        }
    }
}

/// セクションカード
#[component]
pub fn SectionCard(title: String, children: Element) -> Element {
    rsx! {
        div { class: "bg-white rounded-xl shadow-sm border border-gray-200 p-6",
            SectionHeader { title: title }
            {children}
        }
    }
}

/// フォームセクション
#[component]
pub fn FormSection(title: String, children: Element) -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow p-4 space-y-3",
            h4 { class: "font-bold text-md border-b pb-2", "{title}" }
            {children}
        }
    }
}
