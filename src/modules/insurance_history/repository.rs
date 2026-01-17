#[cfg(feature = "server")]
use super::models::{
    CreateInsuranceHistory, InsuranceHistory, InsuranceType, UpdateInsuranceHistory,
};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct InsuranceHistoryRepository;

#[cfg(feature = "server")]
impl InsuranceHistoryRepository {
    /// データベースプールを取得するヘルパーメソッド
    async fn pool() -> Result<&'static sqlx::PgPool, sqlx::Error> {
        db::get_pool_async().await
    }

    /// 従業員の保険証履歴を取得（新しい順）
    pub async fn get_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<InsuranceHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            InsuranceHistory,
            r#"SELECT id, employee_id, insurance_type_id, insurance_number,
                      start_date, end_date, insurer_name,
                      is_active, created_at, updated_at
               FROM insurance_history
               WHERE employee_id = $1
               ORDER BY start_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 有効な保険証履歴を取得
    pub async fn get_active_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<InsuranceHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            InsuranceHistory,
            r#"SELECT id, employee_id, insurance_type_id, insurance_number,
                      start_date, end_date, insurer_name,
                      is_active, created_at, updated_at
               FROM insurance_history
               WHERE employee_id = $1 AND is_active = TRUE
               ORDER BY start_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// IDで保険証履歴を取得
    pub async fn get_by_id(id: i32) -> Result<Option<InsuranceHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            InsuranceHistory,
            r#"SELECT id, employee_id, insurance_type_id, insurance_number,
                      start_date, end_date, insurer_name,
                      is_active, created_at, updated_at
               FROM insurance_history
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 保険証履歴を作成
    pub async fn create(data: CreateInsuranceHistory) -> Result<InsuranceHistory, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            InsuranceHistory,
            r#"INSERT INTO insurance_history
               (employee_id, insurance_type_id, insurance_number, start_date,
                end_date, insurer_name, is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, insurance_type_id, insurance_number,
                         start_date, end_date, insurer_name,
                         is_active, created_at, updated_at"#,
            data.employee_id,
            data.insurance_type_id,
            data.insurance_number,
            data.start_date,
            data.end_date,
            data.insurer_name,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 保険証履歴を更新
    pub async fn update(data: UpdateInsuranceHistory) -> Result<InsuranceHistory, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            InsuranceHistory,
            r#"UPDATE insurance_history
               SET insurance_type_id = $2, insurance_number = $3, start_date = $4,
                   end_date = $5, insurer_name = $6,
                   is_active = $7, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, insurance_type_id, insurance_number,
                         start_date, end_date, insurer_name,
                         is_active, created_at, updated_at"#,
            data.id,
            data.insurance_type_id,
            data.insurance_number,
            data.start_date,
            data.end_date,
            data.insurer_name,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 保険証履歴を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query!(r#"DELETE FROM insurance_history WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 全保険種別を取得
    pub async fn get_all_insurance_types() -> Result<Vec<InsuranceType>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            InsuranceType,
            r#"SELECT id, name, description, is_mandatory, created_at, updated_at
               FROM insurance_types
               ORDER BY id"#
        )
        .fetch_all(pool)
        .await
    }
}
