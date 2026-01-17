#[cfg(feature = "server")]
use super::models::{
    AptitudeCheckupHistory, AptitudeCheckupType, CreateAptitudeCheckupHistory,
    UpdateAptitudeCheckupHistory,
};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct AptitudeCheckupHistoryRepository;

#[cfg(feature = "server")]
impl AptitudeCheckupHistoryRepository {
    /// データベースプールを取得するヘルパーメソッド
    async fn pool() -> Result<&'static sqlx::PgPool, sqlx::Error> {
        db::get_pool_async().await
    }

    /// 従業員の適性診断履歴を取得（新しい順）
    pub async fn get_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<AptitudeCheckupHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            AptitudeCheckupHistory,
            r#"SELECT id, employee_id, aptitude_checkup_type_id, checkup_date,
                      expiration_date, testing_organization, result, notes,
                      is_active, created_at, updated_at
               FROM aptitude_checkup_history
               WHERE employee_id = $1
               ORDER BY checkup_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 有効な適性診断履歴を取得
    pub async fn get_active_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<AptitudeCheckupHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            AptitudeCheckupHistory,
            r#"SELECT id, employee_id, aptitude_checkup_type_id, checkup_date,
                      expiration_date, testing_organization, result, notes,
                      is_active, created_at, updated_at
               FROM aptitude_checkup_history
               WHERE employee_id = $1 AND is_active = TRUE
               ORDER BY checkup_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// IDで適性診断履歴を取得
    pub async fn get_by_id(id: i32) -> Result<Option<AptitudeCheckupHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            AptitudeCheckupHistory,
            r#"SELECT id, employee_id, aptitude_checkup_type_id, checkup_date,
                      expiration_date, testing_organization, result, notes,
                      is_active, created_at, updated_at
               FROM aptitude_checkup_history
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 適性診断履歴を作成
    pub async fn create(
        data: CreateAptitudeCheckupHistory,
    ) -> Result<AptitudeCheckupHistory, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            AptitudeCheckupHistory,
            r#"INSERT INTO aptitude_checkup_history
               (employee_id, aptitude_checkup_type_id, checkup_date, expiration_date,
                testing_organization, result, notes, is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, aptitude_checkup_type_id, checkup_date,
                         expiration_date, testing_organization, result, notes,
                         is_active, created_at, updated_at"#,
            data.employee_id,
            data.aptitude_checkup_type_id,
            data.checkup_date,
            data.expiration_date,
            data.testing_organization,
            data.result,
            data.notes,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 適性診断履歴を更新
    pub async fn update(
        data: UpdateAptitudeCheckupHistory,
    ) -> Result<AptitudeCheckupHistory, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            AptitudeCheckupHistory,
            r#"UPDATE aptitude_checkup_history
               SET aptitude_checkup_type_id = $2, checkup_date = $3, expiration_date = $4,
                   testing_organization = $5, result = $6, notes = $7,
                   is_active = $8, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, aptitude_checkup_type_id, checkup_date,
                         expiration_date, testing_organization, result, notes,
                         is_active, created_at, updated_at"#,
            data.id,
            data.aptitude_checkup_type_id,
            data.checkup_date,
            data.expiration_date,
            data.testing_organization,
            data.result,
            data.notes,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 適性診断履歴を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query!(r#"DELETE FROM aptitude_checkup_history WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 全適性診断種別を取得
    pub async fn get_all_aptitude_checkup_types() -> Result<Vec<AptitudeCheckupType>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            AptitudeCheckupType,
            r#"SELECT id, name, description, target_age_min, target_age_max,
                      required_frequency_years, created_at, updated_at
               FROM aptitude_checkup_types
               ORDER BY id"#
        )
        .fetch_all(pool)
        .await
    }
}
