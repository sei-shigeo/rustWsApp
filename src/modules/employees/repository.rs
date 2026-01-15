#[cfg(feature = "server")]
use super::models::Employee;
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct EmployeeRepository;

#[cfg(feature = "server")]
impl EmployeeRepository {
    // 従業員コードの重複チェック
    // 使用可能ならtrue、既に使われていたらfalse
    pub async fn is_employee_code_available(
        employee_code: &str,
        exclude_id: Option<i32>,
    ) -> Result<bool, sqlx::Error> {
        let pool = db::get_pool_async().await?;

        let count = if let Some(id) = exclude_id {
            // 編集時: 自分以外で同じコードがあるかチェック
            sqlx::query_scalar!(
                "SELECT COUNT(*) as count FROM employees WHERE employee_code = $1 AND id != $2",
                employee_code,
                id
            )
            .fetch_one(pool)
            .await?
        } else {
            // 新規作成時: 同じコードがあるかチェック
            sqlx::query_scalar!(
                "SELECT COUNT(*) as count FROM employees WHERE employee_code = $1",
                employee_code
            )
            .fetch_one(pool)
            .await?
        };

        Ok(count.unwrap_or(0) == 0)
    }

    // 全従業員の取得（基本情報のみ）
    pub async fn get_all() -> Result<Vec<Employee>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            Employee,
            r#" SELECT id, employee_code, first_name, last_name, is_active, created_at, updated_at FROM employees ORDER BY id"#
        )
        .fetch_all(pool)
        .await
    }

    // IDで従業員を取得（基本情報のみ）
    #[allow(dead_code)]
    pub async fn get_by_id(id: i32) -> Result<Option<Employee>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            Employee,
            r#"SELECT id, employee_code, first_name, last_name, is_active, created_at, updated_at
            FROM employees WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    // 従業員の作成
    pub async fn create(
        employee_code: String,
        first_name: String,
        last_name: String,
    ) -> Result<Employee, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            Employee,
            r#"INSERT INTO employees (employee_code, first_name, last_name, is_active, created_at, updated_at)
             VALUES ($1, $2, $3, true, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
             RETURNING id, employee_code, first_name, last_name, is_active, created_at, updated_at"#,
            employee_code,
            first_name,
            last_name
        )
        .fetch_one(pool)
        .await
    }

    // 従業員の更新
    pub async fn update(
        id: i32,
        employee_code: String,
        first_name: String,
        last_name: String,
        is_active: bool,
    ) -> Result<Employee, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            Employee,
            r#"UPDATE employees SET employee_code = $1, first_name = $2, last_name = $3, is_active = $4, updated_at = CURRENT_TIMESTAMP
             WHERE id = $5
             RETURNING id, employee_code, first_name, last_name, is_active, created_at, updated_at"#,
            employee_code,
            first_name,
            last_name,
            is_active,
            id
        )
        .fetch_one(pool)
        .await
    }

    // 従業員の削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query!(r#"DELETE FROM employees WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
