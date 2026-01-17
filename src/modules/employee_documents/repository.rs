//! 従業員ドキュメントのリポジトリ層

use super::models::{
    CreateEmployeeDocument, DocumentFilter, EmployeeDocument, UpdateEmployeeDocument,
};
use crate::db;

/// 従業員ドキュメントリポジトリ
pub struct EmployeeDocumentRepository;

impl EmployeeDocumentRepository {
    /// データベースプールを取得するヘルパーメソッド
    async fn pool() -> Result<&'static sqlx::PgPool, sqlx::Error> {
        db::get_pool_async().await
    }

    /// ドキュメントを作成
    pub async fn create(doc: CreateEmployeeDocument) -> Result<EmployeeDocument, sqlx::Error> {
        let pool = Self::pool().await?;

        let document = sqlx::query_as!(
            EmployeeDocument,
            r#"
            INSERT INTO employee_documents (
                employee_id, category, file_name, s3_key, s3_url,
                mime_type, file_size, notes, related_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                id,
                employee_id,
                category as "category?: _",
                file_name as filename,
                s3_key as "s3_key?",
                s3_url as "s3_url?",
                mime_type as "mime_type?",
                file_size as "file_size?",
                notes as description,
                related_id,
                uploaded_at,
                created_at,
                updated_at
            "#,
            doc.employee_id,
            doc.category.as_ref().map(|c| c.as_str()),
            doc.filename,
            doc.s3_key,
            doc.s3_url,
            doc.mime_type,
            doc.file_size,
            doc.description,
            doc.related_id
        )
        .fetch_one(pool)
        .await?;

        Ok(document)
    }

    /// IDでドキュメントを取得
    pub async fn get_by_id(id: i32) -> Result<Option<EmployeeDocument>, sqlx::Error> {
        let pool = Self::pool().await?;

        let document = sqlx::query_as!(
            EmployeeDocument,
            r#"
            SELECT
                id,
                employee_id,
                category as "category?: _",
                file_name as filename,
                s3_key as "s3_key?",
                s3_url as "s3_url?",
                mime_type as "mime_type?",
                file_size as "file_size?",
                notes as description,
                related_id,
                uploaded_at,
                created_at,
                updated_at
            FROM employee_documents
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(document)
    }

    /// 従業員IDでドキュメント一覧を取得
    pub async fn get_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<EmployeeDocument>, sqlx::Error> {
        let pool = Self::pool().await?;

        let documents = sqlx::query_as!(
            EmployeeDocument,
            r#"
            SELECT
                id,
                employee_id,
                category as "category?: _",
                file_name as filename,
                s3_key as "s3_key?",
                s3_url as "s3_url?",
                mime_type as "mime_type?",
                file_size as "file_size?",
                notes as description,
                related_id,
                uploaded_at,
                created_at,
                updated_at
            FROM employee_documents
            WHERE employee_id = $1
            ORDER BY uploaded_at DESC
            "#,
            employee_id
        )
        .fetch_all(pool)
        .await?;

        Ok(documents)
    }

    /// 従業員の特定カテゴリのドキュメントを取得
    pub async fn get_by_category(
        employee_id: i32,
        category: &str,
    ) -> Result<Vec<EmployeeDocument>, sqlx::Error> {
        let pool = Self::pool().await?;

        let documents = sqlx::query_as!(
            EmployeeDocument,
            r#"
            SELECT
                id,
                employee_id,
                category as "category?: _",
                file_name as filename,
                s3_key as "s3_key?",
                s3_url as "s3_url?",
                mime_type as "mime_type?",
                file_size as "file_size?",
                notes as description,
                related_id,
                uploaded_at,
                created_at,
                updated_at
            FROM employee_documents
            WHERE employee_id = $1 AND category = $2
            ORDER BY uploaded_at DESC
            "#,
            employee_id,
            category
        )
        .fetch_all(pool)
        .await?;

        Ok(documents)
    }

    /// 関連IDでドキュメントを取得
    pub async fn get_by_related_id(
        employee_id: i32,
        related_id: i32,
    ) -> Result<Vec<EmployeeDocument>, sqlx::Error> {
        let pool = Self::pool().await?;

        let documents = sqlx::query_as!(
            EmployeeDocument,
            r#"
            SELECT
                id,
                employee_id,
                category as "category?: _",
                file_name as filename,
                s3_key as "s3_key?",
                s3_url as "s3_url?",
                mime_type as "mime_type?",
                file_size as "file_size?",
                notes as description,
                related_id,
                uploaded_at,
                created_at,
                updated_at
            FROM employee_documents
            WHERE employee_id = $1 AND related_id = $2
            ORDER BY uploaded_at DESC
            "#,
            employee_id,
            related_id
        )
        .fetch_all(pool)
        .await?;

        Ok(documents)
    }

    /// ドキュメントを更新
    pub async fn update(
        id: i32,
        update: UpdateEmployeeDocument,
    ) -> Result<EmployeeDocument, sqlx::Error> {
        let pool = Self::pool().await?;

        let document = sqlx::query_as!(
            EmployeeDocument,
            r#"
            UPDATE employee_documents
            SET
                notes = $2,
                related_id = $3,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            RETURNING
                id,
                employee_id,
                category as "category?: _",
                file_name as filename,
                s3_key as "s3_key?",
                s3_url as "s3_url?",
                mime_type as "mime_type?",
                file_size as "file_size?",
                notes as description,
                related_id,
                uploaded_at,
                created_at,
                updated_at
            "#,
            id,
            update.description,
            update.related_id
        )
        .fetch_one(pool)
        .await?;

        Ok(document)
    }

    /// ドキュメントを削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = Self::pool().await?;

        sqlx::query!("DELETE FROM employee_documents WHERE id = $1", id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// 従業員のドキュメント数を取得
    pub async fn count_by_employee_id(employee_id: i32) -> Result<i64, sqlx::Error> {
        let pool = Self::pool().await?;

        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM employee_documents WHERE employee_id = $1",
            employee_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result.count.unwrap_or(0))
    }

    /// 従業員の特定カテゴリのドキュメント数を取得
    pub async fn count_by_category(employee_id: i32, category: &str) -> Result<i64, sqlx::Error> {
        let pool = Self::pool().await?;

        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM employee_documents WHERE employee_id = $1 AND category = $2",
            employee_id,
            category
        )
        .fetch_one(pool)
        .await?;

        Ok(result.count.unwrap_or(0))
    }
}
