#[cfg(feature = "server")]
use super::models::{Address, CreateAddress, UpdateAddress};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct AddressRepository;

#[cfg(feature = "server")]
impl AddressRepository {
    /// 従業員の住所一覧を取得
    pub async fn get_by_employee_id(employee_id: i32) -> Result<Vec<Address>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            Address,
            r#"SELECT id, employee_id, postal_code, prefecture, city, street, building,
                      start_date, end_date, is_current, created_at, updated_at
               FROM addresses
               WHERE employee_id = $1
               ORDER BY start_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// IDで住所を取得
    pub async fn get_by_id(id: i32) -> Result<Option<Address>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            Address,
            r#"SELECT id, employee_id, postal_code, prefecture, city, street, building,
                      start_date, end_date, is_current, created_at, updated_at
               FROM addresses
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 従業員の現住所を取得
    pub async fn get_current_by_employee_id(
        employee_id: i32,
    ) -> Result<Option<Address>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            Address,
            r#"SELECT id, employee_id, postal_code, prefecture, city, street, building,
                      start_date, end_date, is_current, created_at, updated_at
               FROM addresses
               WHERE employee_id = $1 AND is_current = TRUE
               LIMIT 1"#,
            employee_id
        )
        .fetch_optional(pool)
        .await
    }

    /// 住所を作成
    pub async fn create(data: CreateAddress) -> Result<Address, sqlx::Error> {
        let pool = db::get_pool_async().await?;

        // 現住所として作成する場合、既存の現住所を解除
        if data.is_current {
            sqlx::query!(
                r#"UPDATE addresses
                   SET is_current = FALSE, updated_at = CURRENT_TIMESTAMP
                   WHERE employee_id = $1 AND is_current = TRUE"#,
                data.employee_id
            )
            .execute(pool)
            .await?;
        }

        sqlx::query_as!(
            Address,
            r#"INSERT INTO addresses (employee_id, postal_code, prefecture, city, street, building, start_date, is_current, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, postal_code, prefecture, city, street, building, start_date, end_date, is_current, created_at, updated_at"#,
            data.employee_id,
            data.postal_code,
            data.prefecture,
            data.city,
            data.street,
            data.building,
            data.start_date,
            data.is_current
        )
        .fetch_one(pool)
        .await
    }

    /// 住所を更新
    pub async fn update(data: UpdateAddress) -> Result<Address, sqlx::Error> {
        let pool = db::get_pool_async().await?;

        // 現住所として更新する場合、他の現住所を解除
        if data.is_current {
            sqlx::query!(
                r#"UPDATE addresses
                   SET is_current = FALSE, updated_at = CURRENT_TIMESTAMP
                   WHERE employee_id = (SELECT employee_id FROM addresses WHERE id = $1)
                   AND is_current = TRUE
                   AND id != $1"#,
                data.id
            )
            .execute(pool)
            .await?;
        }

        sqlx::query_as!(
            Address,
            r#"UPDATE addresses
               SET postal_code = $2, prefecture = $3, city = $4, street = $5, building = $6,
                   start_date = $7, end_date = $8, is_current = $9, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, postal_code, prefecture, city, street, building, start_date, end_date, is_current, created_at, updated_at"#,
            data.id,
            data.postal_code,
            data.prefecture,
            data.city,
            data.street,
            data.building,
            data.start_date,
            data.end_date,
            data.is_current
        )
        .fetch_one(pool)
        .await
    }

    /// 住所を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query!(r#"DELETE FROM addresses WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
