#[cfg(feature = "server")]
use super::models::{
    CreateGuidanceEducationHistory, GuidanceEducationHistory, GuidanceEducationType,
    UpdateGuidanceEducationHistory,
};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct GuidanceEducationHistoryRepository;

#[cfg(feature = "server")]
impl GuidanceEducationHistoryRepository {
    /// データベースプールを取得するヘルパーメソッド
    async fn pool() -> Result<&'static sqlx::PgPool, sqlx::Error> {
        db::get_pool_async().await
    }

    /// 従業員の指導教育履歴を取得（新しい順）
    pub async fn get_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<GuidanceEducationHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            GuidanceEducationHistory,
            r#"SELECT id, employee_id, guidance_education_type_id, education_date,
                      expiration_date, instructor_name, location, duration_hours,
                      content, notes, created_at, updated_at
               FROM guidance_education_history
               WHERE employee_id = $1
               ORDER BY education_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// IDで指導教育履歴を取得
    pub async fn get_by_id(id: i32) -> Result<Option<GuidanceEducationHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            GuidanceEducationHistory,
            r#"SELECT id, employee_id, guidance_education_type_id, education_date,
                      expiration_date, instructor_name, location, duration_hours,
                      content, notes, created_at, updated_at
               FROM guidance_education_history
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 指導教育履歴を作成
    pub async fn create(
        data: CreateGuidanceEducationHistory,
    ) -> Result<GuidanceEducationHistory, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            GuidanceEducationHistory,
            r#"INSERT INTO guidance_education_history
               (employee_id, guidance_education_type_id, education_date, expiration_date,
                instructor_name, location, duration_hours, content, notes,
                created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, guidance_education_type_id, education_date,
                         expiration_date, instructor_name, location, duration_hours,
                         content, notes, created_at, updated_at"#,
            data.employee_id,
            data.guidance_education_type_id,
            data.education_date,
            data.expiration_date,
            data.instructor_name,
            data.location,
            data.duration_hours,
            data.content,
            data.notes
        )
        .fetch_one(pool)
        .await
    }

    /// 指導教育履歴を更新
    pub async fn update(
        data: UpdateGuidanceEducationHistory,
    ) -> Result<GuidanceEducationHistory, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            GuidanceEducationHistory,
            r#"UPDATE guidance_education_history
               SET guidance_education_type_id = $2, education_date = $3, expiration_date = $4,
                   instructor_name = $5, location = $6, duration_hours = $7,
                   content = $8, notes = $9, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, guidance_education_type_id, education_date,
                         expiration_date, instructor_name, location, duration_hours,
                         content, notes, created_at, updated_at"#,
            data.id,
            data.guidance_education_type_id,
            data.education_date,
            data.expiration_date,
            data.instructor_name,
            data.location,
            data.duration_hours,
            data.content,
            data.notes
        )
        .fetch_one(pool)
        .await
    }

    /// 指導教育履歴を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query!(
            r#"DELETE FROM guidance_education_history WHERE id = $1"#,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /// 全指導教育種別を取得
    pub async fn get_all_guidance_education_types(
    ) -> Result<Vec<GuidanceEducationType>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            GuidanceEducationType,
            r#"SELECT id, name, description, required_frequency_months, is_mandatory,
                      created_at, updated_at
               FROM guidance_education_types
               ORDER BY id"#
        )
        .fetch_all(pool)
        .await
    }
}
