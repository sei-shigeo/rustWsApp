use dioxus::prelude::*;

mod components;
#[cfg(feature = "server")]
mod db;
mod modules;
mod routes;

use routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // サーバー側で環境変数を読み込み
    #[cfg(feature = "server")]
    {
        dotenvy::dotenv().ok();
        println!("🚀 Starting server with lazy database initialization...");
    }

    // アプリケーションを起動（DB接続は遅延初期化される）
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
