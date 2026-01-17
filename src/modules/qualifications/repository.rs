#[cfg(feature = "server")]
use super::models::{CreateQualification, Qualification, QualificationType, UpdateQualification};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct QualificationRepository;

#[cfg(feature = "server")]
impl QualificationRepository {
    /// データベースプールを取得するヘルパーメソッド
    async fn pool() -> Result<&'static sqlx::PgPool, sqlx::Error> {
        db::get_pool_async().await
    }

    /// 従業員の資格証を取得（新しい順）
    pub async fn get_by_employee_id(employee_id: i32) -> Result<Vec<Qualification>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            Qualification,
            r#"SELECT id, employee_id, qualification_type_id, qualification_number,
                      issue_date, expiration_date, issuing_authority,
                      is_active, created_at, updated_at
               FROM qualifications
               WHERE employee_id = $1
               ORDER BY issue_date DESC NULLS LAST"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 有効な資格証を取得
    pub async fn get_active_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<Qualification>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            Qualification,
            r#"SELECT id, employee_id, qualification_type_id, qualification_number,
                      issue_date, expiration_date, issuing_authority,
                      is_active, created_at, updated_at
               FROM qualifications
               WHERE employee_id = $1 AND is_active = TRUE
               ORDER BY issue_date DESC NULLS LAST"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// IDで資格証を取得
    pub async fn get_by_id(id: i32) -> Result<Option<Qualification>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            Qualification,
            r#"SELECT id, employee_id, qualification_type_id, qualification_number,
                      issue_date, expiration_date, issuing_authority,
                      is_active, created_at, updated_at
               FROM qualifications
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 資格証を作成
    pub async fn create(data: CreateQualification) -> Result<Qualification, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            Qualification,
            r#"INSERT INTO qualifications
               (employee_id, qualification_type_id, qualification_number, issue_date,
                expiration_date, issuing_authority, is_active,
                created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, qualification_type_id, qualification_number,
                         issue_date, expiration_date, issuing_authority,
                         is_active, created_at, updated_at"#,
            data.employee_id,
            data.qualification_type_id,
            data.qualification_number,
            data.issue_date,
            data.expiration_date,
            data.issuing_authority,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 資格証を更新
    pub async fn update(data: UpdateQualification) -> Result<Qualification, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            Qualification,
            r#"UPDATE qualifications
               SET qualification_type_id = $2, qualification_number = $3, issue_date = $4,
                   expiration_date = $5, issuing_authority = $6,
                   is_active = $7, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, qualification_type_id, qualification_number,
                         issue_date, expiration_date, issuing_authority,
                         is_active, created_at, updated_at"#,
            data.id,
            data.qualification_type_id,
            data.qualification_number,
            data.issue_date,
            data.expiration_date,
            data.issuing_authority,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 資格証を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query!(r#"DELETE FROM qualifications WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 全資格種別を取得
    pub async fn get_all_qualification_types() -> Result<Vec<QualificationType>, sqlx::Error> {
        let pool = Self::pool().await?;
        sqlx::query_as!(
            QualificationType,
            r#"SELECT id, name, description, created_at, updated_at
               FROM qualification_types
               ORDER BY id"#
        )
        .fetch_all(pool)
        .await
    }
}
