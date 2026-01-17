#[cfg(feature = "server")]
use super::models::{CreateEmergencyContact, EmergencyContact, UpdateEmergencyContact};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct EmergencyContactRepository;

#[cfg(feature = "server")]
impl EmergencyContactRepository {
    /// 従業員の緊急連絡先一覧を取得
    pub async fn get_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<EmergencyContact>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EmergencyContact,
            r#"SELECT id, employee_id, name, relationship, phone, mobile, postal_code, address,
                      priority_order as "priority_order!", is_active, created_at, updated_at
               FROM emergency_contacts
               WHERE employee_id = $1
               ORDER BY priority_order ASC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 有効な緊急連絡先のみを取得
    pub async fn get_active_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<EmergencyContact>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EmergencyContact,
            r#"SELECT id, employee_id, name, relationship, phone, mobile, postal_code, address,
                      priority_order as "priority_order!", is_active, created_at, updated_at
               FROM emergency_contacts
               WHERE employee_id = $1 AND is_active = TRUE
               ORDER BY priority_order ASC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// IDで緊急連絡先を取得
    pub async fn get_by_id(id: i32) -> Result<Option<EmergencyContact>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EmergencyContact,
            r#"SELECT id, employee_id, name, relationship, phone, mobile, postal_code, address,
                      priority_order as "priority_order!", is_active, created_at, updated_at
               FROM emergency_contacts
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 緊急連絡先を作成
    pub async fn create(data: CreateEmergencyContact) -> Result<EmergencyContact, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EmergencyContact,
            r#"INSERT INTO emergency_contacts (employee_id, name, relationship, phone, mobile, postal_code, address, priority_order, is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, TRUE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, name, relationship, phone, mobile, postal_code, address, priority_order as "priority_order!", is_active, created_at, updated_at"#,
            data.employee_id,
            data.name,
            data.relationship,
            data.phone,
            data.mobile,
            data.postal_code,
            data.address,
            data.priority_order
        )
        .fetch_one(pool)
        .await
    }

    /// 緊急連絡先を更新
    pub async fn update(data: UpdateEmergencyContact) -> Result<EmergencyContact, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            EmergencyContact,
            r#"UPDATE emergency_contacts
               SET name = $2, relationship = $3, phone = $4, mobile = $5, postal_code = $6,
                   address = $7, priority_order = $8, is_active = $9, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, name, relationship, phone, mobile, postal_code, address, priority_order as "priority_order!", is_active, created_at, updated_at"#,
            data.id,
            data.name,
            data.relationship,
            data.phone,
            data.mobile,
            data.postal_code,
            data.address,
            data.priority_order,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 緊急連絡先を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query!(r#"DELETE FROM emergency_contacts WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
