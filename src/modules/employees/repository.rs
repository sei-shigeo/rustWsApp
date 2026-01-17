#[cfg(feature = "server")]
use super::models::{
    Address, CreateAddress, Employee, EmployeeFull, EmployeeWithAddress, UpdateAddress,
};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct EmployeeRepository;

#[cfg(feature = "server")]
impl EmployeeRepository {
    /// データベースプールを取得するヘルパーメソッド
    async fn pool() -> Result<&'static sqlx::PgPool, sqlx::Error> {
        db::get_pool_async().await
    }

    /// 従業員コードの重複チェック
    /// 使用可能ならtrue、既に使われていたらfalse
    pub async fn is_employee_code_available(
        employee_code: &str,
        exclude_id: Option<i32>,
    ) -> Result<bool, sqlx::Error> {
        let pool = Self::pool().await?;

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

    /// 全従業員の取得（基本情報のみ）
    pub async fn get_all() -> Result<Vec<Employee>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            Employee,
            r#"SELECT id, employee_code, first_name, last_name, mobile, nationality_id, birth_date, gender, is_active, created_at, updated_at FROM employees ORDER BY id"#
        )
        .fetch_all(pool)
        .await
    }

    /// IDで従業員を取得（全詳細情報）
    #[allow(dead_code)]
    pub async fn get_by_id(id: i32) -> Result<Option<EmployeeFull>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            EmployeeFull,
            r#"SELECT
                id,
                company_id,
                first_name,
                last_name,
                first_name_kana,
                last_name_kana,
                legal_name,
                nationality_id,
                birth_date,
                gender,
                email,
                phone,
                mobile,
                employee_code,
                start_date,
                end_date,
                office_id,
                department_id,
                position_id,
                driver_start_date,
                driver_end_date,
                driver_end_note,
                is_active,
                created_at,
                updated_at
            FROM employees WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 従業員の作成
    pub async fn create(
        employee_code: String,
        first_name: String,
        last_name: String,
    ) -> Result<Employee, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            Employee,
            r#"INSERT INTO employees (employee_code, first_name, last_name, is_active, created_at, updated_at)
             VALUES ($1, $2, $3, true, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
             RETURNING id, employee_code, first_name, last_name, mobile, nationality_id, birth_date, gender, is_active, created_at, updated_at"#,
            employee_code,
            first_name,
            last_name
        )
        .fetch_one(pool)
        .await
    }

    /// 従業員の更新
    pub async fn update(
        id: i32,
        employee_code: String,
        first_name: String,
        last_name: String,
        is_active: bool,
    ) -> Result<Employee, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            Employee,
            r#"UPDATE employees SET employee_code = $1, first_name = $2, last_name = $3, is_active = $4, updated_at = CURRENT_TIMESTAMP
             WHERE id = $5
             RETURNING id, employee_code, first_name, last_name, mobile, nationality_id, birth_date, gender, is_active, created_at, updated_at"#,
            employee_code,
            first_name,
            last_name,
            is_active,
            id
        )
        .fetch_one(pool)
        .await
    }

    /// 従業員の詳細情報を更新（全フィールド対応）
    #[allow(dead_code)]
    pub async fn update_full(employee: EmployeeFull) -> Result<EmployeeFull, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            EmployeeFull,
            r#"UPDATE employees SET
                company_id = $2,
                first_name = $3,
                last_name = $4,
                first_name_kana = $5,
                last_name_kana = $6,
                legal_name = $7,
                nationality_id = $8,
                birth_date = $9,
                gender = $10,
                email = $11,
                phone = $12,
                mobile = $13,
                employee_code = $14,
                start_date = $15,
                end_date = $16,
                office_id = $17,
                department_id = $18,
                position_id = $19,
                driver_start_date = $20,
                driver_end_date = $21,
                driver_end_note = $22,
                is_active = $23,
                updated_at = CURRENT_TIMESTAMP
             WHERE id = $1
             RETURNING
                id,
                company_id,
                first_name,
                last_name,
                first_name_kana,
                last_name_kana,
                legal_name,
                nationality_id,
                birth_date,
                gender,
                email,
                phone,
                mobile,
                employee_code,
                start_date,
                end_date,
                office_id,
                department_id,
                position_id,
                driver_start_date,
                driver_end_date,
                driver_end_note,
                is_active,
                created_at,
                updated_at"#,
            employee.id,
            employee.company_id,
            employee.first_name,
            employee.last_name,
            employee.first_name_kana,
            employee.last_name_kana,
            employee.legal_name,
            employee.nationality_id,
            employee.birth_date,
            employee.gender,
            employee.email,
            employee.phone,
            employee.mobile,
            employee.employee_code,
            employee.start_date,
            employee.end_date,
            employee.office_id,
            employee.department_id,
            employee.position_id,
            employee.driver_start_date,
            employee.driver_end_date,
            employee.driver_end_note,
            employee.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 従業員の削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query!(r#"DELETE FROM employees WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 住所情報を含む全従業員の取得
    pub async fn get_all_with_address() -> Result<Vec<EmployeeWithAddress>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            EmployeeWithAddress,
            r#"
            SELECT
                e.id,
                e.employee_code,
                e.first_name,
                e.last_name,
                e.mobile,
                e.nationality_id,
                e.birth_date,
                e.gender,
                e.is_active,
                e.created_at,
                e.updated_at,
                a.postal_code as "current_postal_code?",
                a.prefecture as "current_prefecture?",
                a.city as "current_city?",
                a.street as "current_street?",
                a.building as "current_building?"
            FROM employees e
            LEFT JOIN addresses a ON e.id = a.employee_id AND a.is_current = true
            ORDER BY e.id
            "#
        )
        .fetch_all(pool)
        .await
    }

    /// 従業員の現住所を取得
    pub async fn get_current_address(employee_id: i32) -> Result<Option<Address>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            Address,
            r#"
            SELECT
                id,
                employee_id,
                postal_code,
                prefecture,
                city,
                street,
                building,
                start_date,
                end_date,
                is_current,
                created_at,
                updated_at
            FROM addresses
            WHERE employee_id = $1 AND is_current = true
            ORDER BY start_date DESC
            LIMIT 1
            "#,
            employee_id
        )
        .fetch_optional(pool)
        .await
    }

    /// 従業員の全住所履歴を取得
    pub async fn get_all_addresses(employee_id: i32) -> Result<Vec<Address>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            Address,
            r#"
            SELECT
                id,
                employee_id,
                postal_code,
                prefecture,
                city,
                street,
                building,
                start_date,
                end_date,
                is_current,
                created_at,
                updated_at
            FROM addresses
            WHERE employee_id = $1
            ORDER BY start_date DESC
            "#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 住所の作成
    pub async fn create_address(address: CreateAddress) -> Result<Address, sqlx::Error> {
        let pool = Self::pool().await?;

        // 新しい住所が現住所の場合、既存の現住所フラグを解除
        if address.is_current {
            sqlx::query!(
                r#"UPDATE addresses SET is_current = false WHERE employee_id = $1 AND is_current = true"#,
                address.employee_id
            )
            .execute(pool)
            .await?;
        }

        sqlx::query_as!(
            Address,
            r#"
            INSERT INTO addresses (
                employee_id,
                postal_code,
                prefecture,
                city,
                street,
                building,
                start_date,
                is_current,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            RETURNING
                id,
                employee_id,
                postal_code,
                prefecture,
                city,
                street,
                building,
                start_date,
                end_date,
                is_current,
                created_at,
                updated_at
            "#,
            address.employee_id,
            address.postal_code,
            address.prefecture,
            address.city,
            address.street,
            address.building,
            address.start_date,
            address.is_current
        )
        .fetch_one(pool)
        .await
    }

    /// 住所の更新
    pub async fn update_address(address: UpdateAddress) -> Result<Address, sqlx::Error> {
        let pool = Self::pool().await?;

        // 新しい住所が現住所の場合、既存の現住所フラグを解除（自分以外）
        if address.is_current {
            sqlx::query!(
                r#"UPDATE addresses SET is_current = false WHERE employee_id = $1 AND is_current = true AND id != $2"#,
                address.employee_id,
                address.id
            )
            .execute(pool)
            .await?;
        }

        sqlx::query_as!(
            Address,
            r#"
            UPDATE addresses SET
                postal_code = $2,
                prefecture = $3,
                city = $4,
                street = $5,
                building = $6,
                start_date = $7,
                end_date = $8,
                is_current = $9,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING
                id,
                employee_id,
                postal_code,
                prefecture,
                city,
                street,
                building,
                start_date,
                end_date,
                is_current,
                created_at,
                updated_at
            "#,
            address.id,
            address.postal_code,
            address.prefecture,
            address.city,
            address.street,
            address.building,
            address.start_date,
            address.end_date,
            address.is_current
        )
        .fetch_one(pool)
        .await
    }

    /// 住所の削除
    pub async fn delete_address(id: i32) -> Result<(), sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query!(r#"DELETE FROM addresses WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
