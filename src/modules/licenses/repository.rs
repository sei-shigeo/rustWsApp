#[cfg(feature = "server")]
use super::models::{CreateLicense, License, LicenseType, UpdateLicense};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct LicenseRepository;

#[cfg(feature = "server")]
impl LicenseRepository {
    /// データベースプールを取得するヘルパーメソッド
    async fn pool() -> Result<&'static sqlx::PgPool, sqlx::Error> {
        db::get_pool_async().await
    }

    /// 従業員の運転免許証を取得（新しい順）
    pub async fn get_by_employee_id(employee_id: i32) -> Result<Vec<License>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            License,
            r#"SELECT id, employee_id, license_type_id, license_number,
                      issue_date, expiration_date, issuing_authority, conditions,
                      is_active, created_at, updated_at
               FROM licenses
               WHERE employee_id = $1
               ORDER BY expiration_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 有効な運転免許証を取得
    pub async fn get_active_by_employee_id(employee_id: i32) -> Result<Vec<License>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            License,
            r#"SELECT id, employee_id, license_type_id, license_number,
                      issue_date, expiration_date, issuing_authority, conditions,
                      is_active, created_at, updated_at
               FROM licenses
               WHERE employee_id = $1 AND is_active = TRUE
               ORDER BY expiration_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// IDで運転免許証を取得
    pub async fn get_by_id(id: i32) -> Result<Option<License>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            License,
            r#"SELECT id, employee_id, license_type_id, license_number,
                      issue_date, expiration_date, issuing_authority, conditions,
                      is_active, created_at, updated_at
               FROM licenses
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 運転免許証を作成
    pub async fn create(data: CreateLicense) -> Result<License, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            License,
            r#"INSERT INTO licenses
               (employee_id, license_type_id, license_number, issue_date,
                expiration_date, issuing_authority, conditions, is_active,
                created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, license_type_id, license_number,
                         issue_date, expiration_date, issuing_authority, conditions,
                         is_active, created_at, updated_at"#,
            data.employee_id,
            data.license_type_id,
            data.license_number,
            data.issue_date,
            data.expiration_date,
            data.issuing_authority,
            data.conditions,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 運転免許証を更新
    pub async fn update(data: UpdateLicense) -> Result<License, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            License,
            r#"UPDATE licenses
               SET license_type_id = $2, license_number = $3, issue_date = $4,
                   expiration_date = $5, issuing_authority = $6, conditions = $7,
                   is_active = $8, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, license_type_id, license_number,
                         issue_date, expiration_date, issuing_authority, conditions,
                         is_active, created_at, updated_at"#,
            data.id,
            data.license_type_id,
            data.license_number,
            data.issue_date,
            data.expiration_date,
            data.issuing_authority,
            data.conditions,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 運転免許証を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query!(r#"DELETE FROM licenses WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 全免許種別を取得
    pub async fn get_all_license_types() -> Result<Vec<LicenseType>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            LicenseType,
            r#"SELECT id, name, description, display_order, created_at, updated_at
               FROM license_types
               ORDER BY display_order NULLS LAST, id"#
        )
        .fetch_all(pool)
        .await
    }
}
