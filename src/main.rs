#![allow(non_snake_case)]

use dioxus::prelude::*;

// モジュール定義
mod components;
#[cfg(feature = "server")]
mod db;
mod modules;
mod routes;

use routes::Route;

// アセット定義
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

/// サーバー側のエントリーポイント
///
/// データベース接続は遅延初期化され、最初のリクエスト時に確立されます
#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::extract::DefaultBodyLimit;
    use dioxus_server::DioxusRouterExt;
    use tower::ServiceBuilder;

    // 環境変数を読み込み
    dotenvy::dotenv().ok();
    println!("🚀 Starting server with lazy database initialization...");

    // S3クライアントを初期化
    let s3_bucket =
        std::env::var("S3_BUCKET_NAME").unwrap_or_else(|_| "rust-ws-app-documents".to_string());

    modules::s3_service::init_s3_client(s3_bucket).await;
    println!("📦 S3 client initialized");

    // サーバーアドレスを決定
    let address = dioxus_cli_config::fullstack_address_or_localhost();

    // Axumルーターをセットアップ（ボディサイズ制限を20MBに設定）
    let router = axum::Router::new()
        .serve_dioxus_application(dioxus_server::ServeConfig::new(), App)
        .layer(
            ServiceBuilder::new().layer(DefaultBodyLimit::max(20 * 1024 * 1024)), // 20MB
        );

    // サーバーを起動
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("🎉 Server listening on: http://{}", address);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

/// クライアント側（Web/デスクトップ）のエントリーポイント
#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

/// アプリケーションのルートコンポーネント
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
