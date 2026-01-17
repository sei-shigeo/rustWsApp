#[cfg(feature = "server")]
use super::models::{CreateEducationHistory, EducationHistory, UpdateEducationHistory};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct EducationHistoryRepository;

#[cfg(feature = "server")]
impl EducationHistoryRepository {
    /// 従業員の学歴一覧を取得
    pub async fn get_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<EducationHistory>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EducationHistory,
            r#"SELECT id, employee_id, school_name, degree, major, start_date, end_date,
                      graduation_status, created_at, updated_at
               FROM education_history
               WHERE employee_id = $1
               ORDER BY end_date DESC NULLS FIRST, start_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// IDで学歴を取得
    pub async fn get_by_id(id: i32) -> Result<Option<EducationHistory>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EducationHistory,
            r#"SELECT id, employee_id, school_name, degree, major, start_date, end_date,
                      graduation_status, created_at, updated_at
               FROM education_history
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 学歴を作成
    pub async fn create(data: CreateEducationHistory) -> Result<EducationHistory, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EducationHistory,
            r#"INSERT INTO education_history (employee_id, school_name, degree, major, start_date, end_date, graduation_status, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, school_name, degree, major, start_date, end_date, graduation_status, created_at, updated_at"#,
            data.employee_id,
            data.school_name,
            data.degree,
            data.major,
            data.start_date,
            data.end_date,
            data.graduation_status
        )
        .fetch_one(pool)
        .await
    }

    /// 学歴を更新
    pub async fn update(data: UpdateEducationHistory) -> Result<EducationHistory, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EducationHistory,
            r#"UPDATE education_history
               SET school_name = $2, degree = $3, major = $4, start_date = $5,
                   end_date = $6, graduation_status = $7, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, school_name, degree, major, start_date, end_date, graduation_status, created_at, updated_at"#,
            data.id,
            data.school_name,
            data.degree,
            data.major,
            data.start_date,
            data.end_date,
            data.graduation_status
        )
        .fetch_one(pool)
        .await
    }

    /// 学歴を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query!(r#"DELETE FROM education_history WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
