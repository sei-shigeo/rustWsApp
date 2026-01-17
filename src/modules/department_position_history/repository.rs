#[cfg(feature = "server")]
use super::models::{
    CreateDepartmentPositionHistory, DepartmentPositionHistory, UpdateDepartmentPositionHistory,
};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct DepartmentPositionHistoryRepository;

#[cfg(feature = "server")]
impl DepartmentPositionHistoryRepository {
    /// データベースプールを取得するヘルパーメソッド
    async fn pool() -> Result<&'static sqlx::PgPool, sqlx::Error> {
        db::get_pool_async().await
    }

    /// 従業員の部署・役職履歴を取得（新しい順）
    pub async fn get_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<DepartmentPositionHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            DepartmentPositionHistory,
            r#"SELECT id, employee_id, office_id, department_id, position_id,
                      start_date, end_date, is_current, change_reason,
                      created_at, updated_at
               FROM department_position_history
               WHERE employee_id = $1
               ORDER BY start_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 現在の配属情報を取得
    pub async fn get_current_by_employee_id(
        employee_id: i32,
    ) -> Result<Option<DepartmentPositionHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            DepartmentPositionHistory,
            r#"SELECT id, employee_id, office_id, department_id, position_id,
                      start_date, end_date, is_current, change_reason,
                      created_at, updated_at
               FROM department_position_history
               WHERE employee_id = $1 AND is_current = TRUE
               ORDER BY start_date DESC
               LIMIT 1"#,
            employee_id
        )
        .fetch_optional(pool)
        .await
    }

    /// IDで部署・役職履歴を取得
    pub async fn get_by_id(id: i32) -> Result<Option<DepartmentPositionHistory>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            DepartmentPositionHistory,
            r#"SELECT id, employee_id, office_id, department_id, position_id,
                      start_date, end_date, is_current, change_reason,
                      created_at, updated_at
               FROM department_position_history
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 部署・役職履歴を作成
    pub async fn create(
        data: CreateDepartmentPositionHistory,
    ) -> Result<DepartmentPositionHistory, sqlx::Error> {
        let pool = Self::pool().await?;

        // 新しい履歴が現在の配属の場合、既存の現在配属フラグを解除
        if data.is_current {
            sqlx::query!(
                r#"UPDATE department_position_history
                   SET is_current = FALSE, updated_at = CURRENT_TIMESTAMP
                   WHERE employee_id = $1 AND is_current = TRUE"#,
                data.employee_id
            )
            .execute(pool)
            .await?;
        }

        sqlx::query_as!(
            DepartmentPositionHistory,
            r#"INSERT INTO department_position_history
               (employee_id, office_id, department_id, position_id, start_date,
                is_current, change_reason, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, office_id, department_id, position_id,
                         start_date, end_date, is_current, change_reason,
                         created_at, updated_at"#,
            data.employee_id,
            data.office_id,
            data.department_id,
            data.position_id,
            data.start_date,
            data.is_current,
            data.change_reason
        )
        .fetch_one(pool)
        .await
    }

    /// 部署・役職履歴を更新
    pub async fn update(
        data: UpdateDepartmentPositionHistory,
    ) -> Result<DepartmentPositionHistory, sqlx::Error> {
        let pool = Self::pool().await?;

        // 新しい履歴が現在の配属の場合、既存の現在配属フラグを解除（自分以外）
        if data.is_current {
            sqlx::query!(
                r#"UPDATE department_position_history
                   SET is_current = FALSE, updated_at = CURRENT_TIMESTAMP
                   WHERE employee_id = $1 AND is_current = TRUE AND id != $2"#,
                data.employee_id,
                data.id
            )
            .execute(pool)
            .await?;
        }

        sqlx::query_as!(
            DepartmentPositionHistory,
            r#"UPDATE department_position_history
               SET office_id = $2, department_id = $3, position_id = $4,
                   start_date = $5, end_date = $6, is_current = $7,
                   change_reason = $8, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, office_id, department_id, position_id,
                         start_date, end_date, is_current, change_reason,
                         created_at, updated_at"#,
            data.id,
            data.office_id,
            data.department_id,
            data.position_id,
            data.start_date,
            data.end_date,
            data.is_current,
            data.change_reason
        )
        .fetch_one(pool)
        .await
    }

    /// 部署・役職履歴を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query!(
            r#"DELETE FROM department_position_history WHERE id = $1"#,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
