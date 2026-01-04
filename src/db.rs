use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::OnceLock;
use tokio::sync::OnceCell;

// グローバルなデータベース接続プール（起動時に一度だけ初期化）
static DB_POOL: OnceLock<PgPool> = OnceLock::new();
static ASYNC_INIT: OnceCell<()> = OnceCell::const_new();

/// データベース接続プールを取得（遅延初期化対応）
pub async fn get_pool_async() -> Result<&'static PgPool, sqlx::Error> {
    ASYNC_INIT
        .get_or_try_init(|| async {
            // 環境変数からデータベースURLを取得
            let database_url = std::env::var("DATABASE_URL")
                .map_err(|_| sqlx::Error::Configuration("DATABASE_URL not set".into()))?;

            let pool = PgPoolOptions::new()
                .max_connections(10)
                .connect(&database_url)
                .await?;

            DB_POOL.set(pool).map_err(|_| {
                sqlx::Error::Configuration("Database pool already initialized".into())
            })?;

            Ok::<(), sqlx::Error>(())
        })
        .await?;

    DB_POOL
        .get()
        .ok_or_else(|| sqlx::Error::Configuration("Database pool not initialized".into()))
}
