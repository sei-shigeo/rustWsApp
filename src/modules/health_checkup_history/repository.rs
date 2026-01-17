#[cfg(feature = "server")]
use super::models::{
    CreateHealthCheckupHistory, HealthCheckupHistory, HealthCheckupType, UpdateHealthCheckupHistory,
};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct HealthCheckupHistoryRepository;

#[cfg(feature = "server")]
impl HealthCheckupHistoryRepository {
    /// データベースプールを取得するヘルパーメソッド
    async fn pool() -> Result<&'static sqlx::PgPool, sqlx::Error> {
        db::get_pool_async().await
    }

    /// 従業員の健康診断履歴を取得（新しい順）
    pub async fn get_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<HealthCheckupHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            HealthCheckupHistory,
            r#"SELECT id, employee_id, health_checkup_type_id, checkup_date,
                      expiration_date, medical_institution, result, notes,
                      is_active, created_at, updated_at
               FROM health_checkup_history
               WHERE employee_id = $1
               ORDER BY checkup_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 有効な健康診断履歴を取得
    pub async fn get_active_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<HealthCheckupHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            HealthCheckupHistory,
            r#"SELECT id, employee_id, health_checkup_type_id, checkup_date,
                      expiration_date, medical_institution, result, notes,
                      is_active, created_at, updated_at
               FROM health_checkup_history
               WHERE employee_id = $1 AND is_active = TRUE
               ORDER BY checkup_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// IDで健康診断履歴を取得
    pub async fn get_by_id(id: i32) -> Result<Option<HealthCheckupHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            HealthCheckupHistory,
            r#"SELECT id, employee_id, health_checkup_type_id, checkup_date,
                      expiration_date, medical_institution, result, notes,
                      is_active, created_at, updated_at
               FROM health_checkup_history
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 健康診断履歴を作成
    pub async fn create(
        data: CreateHealthCheckupHistory,
    ) -> Result<HealthCheckupHistory, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            HealthCheckupHistory,
            r#"INSERT INTO health_checkup_history
               (employee_id, health_checkup_type_id, checkup_date, expiration_date,
                medical_institution, result, notes, is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, health_checkup_type_id, checkup_date,
                         expiration_date, medical_institution, result, notes,
                         is_active, created_at, updated_at"#,
            data.employee_id,
            data.health_checkup_type_id,
            data.checkup_date,
            data.expiration_date,
            data.medical_institution,
            data.result,
            data.notes,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 健康診断履歴を更新
    pub async fn update(
        data: UpdateHealthCheckupHistory,
    ) -> Result<HealthCheckupHistory, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            HealthCheckupHistory,
            r#"UPDATE health_checkup_history
               SET health_checkup_type_id = $2, checkup_date = $3, expiration_date = $4,
                   medical_institution = $5, result = $6, notes = $7,
                   is_active = $8, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, health_checkup_type_id, checkup_date,
                         expiration_date, medical_institution, result, notes,
                         is_active, created_at, updated_at"#,
            data.id,
            data.health_checkup_type_id,
            data.checkup_date,
            data.expiration_date,
            data.medical_institution,
            data.result,
            data.notes,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 健康診断履歴を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query!(r#"DELETE FROM health_checkup_history WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 全健康診断種別を取得
    pub async fn get_all_health_checkup_types() -> Result<Vec<HealthCheckupType>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            HealthCheckupType,
            r#"SELECT id, name, description, required_frequency_months, is_mandatory, created_at, updated_at
               FROM health_checkup_types
               ORDER BY id"#
        )
        .fetch_all(pool)
        .await
    }
}
