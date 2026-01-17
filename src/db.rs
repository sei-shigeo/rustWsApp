use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::OnceCell;

// グローバルなデータベース接続プール（起動時に一度だけ初期化）
static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

/// データベース接続プールを取得（遅延初期化対応）
pub async fn get_pool_async() -> Result<&'static PgPool, sqlx::Error> {
    DB_POOL
        .get_or_try_init(|| async {
            // 環境変数からデータベースURLを取得
            let database_url = std::env::var("DATABASE_URL")
                .map_err(|_| sqlx::Error::Configuration("DATABASE_URL not set".into()))?;

            println!("🔌 Connecting to database...");
            let pool = PgPoolOptions::new()
                .max_connections(10)
                .connect(&database_url)
                .await?;

            println!("✅ Database connection established");
            Ok(pool)
        })
        .await
}
