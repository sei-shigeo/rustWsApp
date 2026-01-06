use dioxus::prelude::*;

/// 汎用トースト（通知）コンポーネント（簡易版、常に自動閉じ）
///
/// Props:
/// - `message`: 表示メッセージ
/// - `duration`: 自動で閉じる秒数（None = 自動で閉じない）
/// - `on_dismiss`: 閉じたときに親に通知（親は state をクリアする）
/// - `variant`: 見た目のバリエーション（成功・エラー・情報）
/// - `position`: 画面内の表示位置（右上/右下/左上/左下）
#[allow(dead_code)]
#[derive(Clone, PartialEq, Default)]
pub enum ToastVariant {
    #[default]
    Success,
    Error,
    Info,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Default)]
pub enum ToastPosition {
    #[default]
    TopRight,
    BottomRight,
    TopLeft,
    BottomLeft,
}

/// バリアントに応じたクラス（背景色/テキスト色/枠色/アイコン）を返す
fn variant_classes(
    variant: &ToastVariant,
) -> (&'static str, &'static str, &'static str, &'static str) {
    match variant {
        ToastVariant::Success => ("bg-green-100", "text-green-800", "border-green-200", "✓"),
        ToastVariant::Error => ("bg-red-100", "text-red-800", "border-red-200", "!"),
        ToastVariant::Info => ("bg-blue-100", "text-blue-800", "border-blue-200", "i"),
    }
}

/// 位置に応じたクラス（fixed の座標）を返す
fn position_classes(position: &ToastPosition) -> &'static str {
    match position {
        ToastPosition::TopRight => "fixed top-4 right-4",
        ToastPosition::BottomRight => "fixed bottom-4 right-4",
        ToastPosition::TopLeft => "fixed top-4 left-4",
        ToastPosition::BottomLeft => "fixed bottom-4 left-4",
    }
}

#[component]
pub fn Toast(
    message: String,
    duration: Option<u64>,
    on_dismiss: Option<EventHandler<()>>,
    variant: Option<ToastVariant>,
    position: Option<ToastPosition>,
) -> Element {
    // デフォルト設定
    let variant = variant.unwrap_or_default();
    let position = position.unwrap_or_default();

    // 自動閉じの副作用（duration が Some のときのみ）
    {
        let on_dismiss_for_effect = on_dismiss;
        use_effect(move || {
            if let Some(secs) = duration {
                spawn(async move {
                    #[cfg(target_family = "wasm")]
                    gloo_timers::future::sleep(std::time::Duration::from_secs(secs)).await;
                    #[cfg(not(target_family = "wasm"))]
                    tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
                    if let Some(cb) = on_dismiss_for_effect {
                        cb.call(());
                    }
                });
            }
        });
    }

    // クラス算出
    let (bg_class, text_class, border_class, icon_text) = variant_classes(&variant);
    let pos_class = position_classes(&position);

    rsx! {
        // アクセシビリティ: role="status" と aria-live を指定（読み上げなど）
        div {
            role: "status",
            aria_live: "polite",
            class: "{pos_class} {bg_class} {border_class} border p-2 rounded shadow-md flex items-center gap-3 max-w-md",
            // アイコン
            div { class: "shrink-0",
                span { class: "font-bold text-lg {text_class}", "{icon_text}" }
            }
            // メッセージ
            div { class: "flex-1",
                p { class: "m-0 {text_class} text-sm", "{message}" }
            }
        }
    }
}
