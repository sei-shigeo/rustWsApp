use dioxus::prelude::*;

/// `RightPanel` is a small wrapper component that encapsulates the right-side panel's
/// layout and open/close animation. It keeps responsibilities minimal:
/// - controls the panel width/visibility based on a `show` signal
/// - renders any children placed inside it (the page passes `EmployeeCreate` or `EmployeeEdit`)
///
/// Usage (in `page.rs`):
/// rsx! {
///     RightPanel {
///         show: show_create,
///         (right_panel) // pass the previously constructed RSX fragment as children
///     }
/// }
///
/// Note:
/// - The component only controls layout/visibility. The parent decides whether to render
///   Create or Edit and passes that fragment as children.
#[component]
pub fn RightPanel(show: Signal<bool>, children: Element) -> Element {
    rsx! {
        // container for the right-side panel
        div {
            class: "flex-none bg-gray-50 transition-all duration-300",
            // conditionally apply width / visibility classes based on `show`
            class: if show() { "w-1/2" } else { "w-0 opacity-0" },

            // render the provided content (Create / Edit) inside the panel
            { children }
        }
    }
}
