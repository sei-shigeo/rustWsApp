use super::models::Employee;
use crate::db;

pub struct EmployeeRepository;

impl EmployeeRepository {
    // 全従業員の取得
    pub async fn get_all() -> Result<Vec<Employee>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            Employee,
            "SELECT id, first_name, last_name, is_active, created_at, updated_at FROM employees ORDER BY id"
        )
        .fetch_all(pool)
        .await
    }

    // IDで従業員を取得
    #[allow(dead_code)]
    pub async fn get_by_id(id: i32) -> Result<Option<Employee>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            Employee,
            "SELECT id, first_name, last_name, is_active, created_at, updated_at FROM employees WHERE id = $1",
            id
        )
        .fetch_optional(pool)
        .await
    }

    // 従業員の作成
    pub async fn create(first_name: String, last_name: String) -> Result<Employee, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            Employee,
            "INSERT INTO employees (first_name, last_name, is_active, created_at, updated_at)
             VALUES ($1, $2, true, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
             RETURNING id, first_name, last_name, is_active, created_at, updated_at",
            first_name,
            last_name
        )
        .fetch_one(pool)
        .await
    }

    // 従業員の更新
    pub async fn update(
        id: i32,
        first_name: String,
        last_name: String,
        is_active: bool,
    ) -> Result<Employee, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            Employee,
            "UPDATE employees SET first_name = $1, last_name = $2, is_active = $3, updated_at = CURRENT_TIMESTAMP
             WHERE id = $4
             RETURNING id, first_name, last_name, is_active, created_at, updated_at",
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
        sqlx::query!("DELETE FROM employees WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
