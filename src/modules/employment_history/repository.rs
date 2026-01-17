#[cfg(feature = "server")]
use super::models::{CreateEmploymentHistory, EmploymentHistory, UpdateEmploymentHistory};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct EmploymentHistoryRepository;

#[cfg(feature = "server")]
impl EmploymentHistoryRepository {
    /// 従業員の職歴一覧を取得
    pub async fn get_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<EmploymentHistory>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EmploymentHistory,
            r#"SELECT id, employee_id, company_name, department, position, job_description,
                      start_date, end_date, is_current, created_at, updated_at
               FROM employment_history
               WHERE employee_id = $1
               ORDER BY start_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 現在の職歴を取得
    pub async fn get_current_by_employee_id(
        employee_id: i32,
    ) -> Result<Option<EmploymentHistory>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EmploymentHistory,
            r#"SELECT id, employee_id, company_name, department, position, job_description,
                      start_date, end_date, is_current, created_at, updated_at
               FROM employment_history
               WHERE employee_id = $1 AND is_current = TRUE
               LIMIT 1"#,
            employee_id
        )
        .fetch_optional(pool)
        .await
    }

    /// IDで職歴を取得
    pub async fn get_by_id(id: i32) -> Result<Option<EmploymentHistory>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EmploymentHistory,
            r#"SELECT id, employee_id, company_name, department, position, job_description,
                      start_date, end_date, is_current, created_at, updated_at
               FROM employment_history
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 職歴を作成
    pub async fn create(data: CreateEmploymentHistory) -> Result<EmploymentHistory, sqlx::Error> {
        let pool = db::get_pool_async().await?;

        // 現在の職場として作成する場合、既存の現在の職場を解除
        if data.is_current {
            sqlx::query!(
                r#"UPDATE employment_history
                   SET is_current = FALSE, updated_at = CURRENT_TIMESTAMP
                   WHERE employee_id = $1 AND is_current = TRUE"#,
                data.employee_id
            )
            .execute(pool)
            .await?;
        }

        sqlx::query_as!(
            EmploymentHistory,
            r#"INSERT INTO employment_history (employee_id, company_name, department, position, job_description, start_date, end_date, is_current, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, company_name, department, position, job_description, start_date, end_date, is_current, created_at, updated_at"#,
            data.employee_id,
            data.company_name,
            data.department,
            data.position,
            data.job_description,
            data.start_date,
            data.end_date,
            data.is_current
        )
        .fetch_one(pool)
        .await
    }

    /// 職歴を更新
    pub async fn update(data: UpdateEmploymentHistory) -> Result<EmploymentHistory, sqlx::Error> {
        let pool = db::get_pool_async().await?;

        // 現在の職場として更新する場合、他の現在の職場を解除
        if data.is_current {
            sqlx::query!(
                r#"UPDATE employment_history
                   SET is_current = FALSE, updated_at = CURRENT_TIMESTAMP
                   WHERE employee_id = (SELECT employee_id FROM employment_history WHERE id = $1)
                   AND is_current = TRUE
                   AND id != $1"#,
                data.id
            )
            .execute(pool)
            .await?;
        }

        sqlx::query_as!(
            EmploymentHistory,
            r#"UPDATE employment_history
               SET company_name = $2, department = $3, position = $4, job_description = $5,
                   start_date = $6, end_date = $7, is_current = $8, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, company_name, department, position, job_description, start_date, end_date, is_current, created_at, updated_at"#,
            data.id,
            data.company_name,
            data.department,
            data.position,
            data.job_description,
            data.start_date,
            data.end_date,
            data.is_current
        )
        .fetch_one(pool)
        .await
    }

    /// 職歴を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query!(r#"DELETE FROM employment_history WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
