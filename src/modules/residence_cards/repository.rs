#[cfg(feature = "server")]
use super::models::{CreateResidenceCard, ResidenceCard, UpdateResidenceCard};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct ResidenceCardRepository;

#[cfg(feature = "server")]
impl ResidenceCardRepository {
    /// 従業員の在留カード一覧を取得
    pub async fn get_by_employee_id(employee_id: i32) -> Result<Vec<ResidenceCard>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            ResidenceCard,
            r#"SELECT id, employee_id, residence_card_type_id, card_number, issue_date,
                      expiration_date, work_restrictions, is_active, created_at, updated_at
               FROM residence_cards
               WHERE employee_id = $1
               ORDER BY expiration_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 有効な在留カードのみを取得
    pub async fn get_active_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<ResidenceCard>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            ResidenceCard,
            r#"SELECT id, employee_id, residence_card_type_id, card_number, issue_date,
                      expiration_date, work_restrictions, is_active, created_at, updated_at
               FROM residence_cards
               WHERE employee_id = $1 AND is_active = TRUE
               ORDER BY expiration_date DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 現在有効な在留カードを取得
    pub async fn get_current_by_employee_id(
        employee_id: i32,
    ) -> Result<Option<ResidenceCard>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            ResidenceCard,
            r#"SELECT id, employee_id, residence_card_type_id, card_number, issue_date,
                      expiration_date, work_restrictions, is_active, created_at, updated_at
               FROM residence_cards
               WHERE employee_id = $1
                 AND is_active = TRUE
                 AND expiration_date >= CURRENT_DATE
               ORDER BY expiration_date DESC
               LIMIT 1"#,
            employee_id
        )
        .fetch_optional(pool)
        .await
    }

    /// IDで在留カードを取得
    pub async fn get_by_id(id: i32) -> Result<Option<ResidenceCard>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            ResidenceCard,
            r#"SELECT id, employee_id, residence_card_type_id, card_number, issue_date,
                      expiration_date, work_restrictions, is_active, created_at, updated_at
               FROM residence_cards
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 在留カードを作成
    pub async fn create(data: CreateResidenceCard) -> Result<ResidenceCard, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            ResidenceCard,
            r#"INSERT INTO residence_cards (employee_id, residence_card_type_id, card_number, issue_date, expiration_date, work_restrictions, is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, TRUE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, residence_card_type_id, card_number, issue_date, expiration_date, work_restrictions, is_active, created_at, updated_at"#,
            data.employee_id,
            data.residence_card_type_id,
            data.card_number,
            data.issue_date,
            data.expiration_date,
            data.work_restrictions
        )
        .fetch_one(pool)
        .await
    }

    /// 在留カードを更新
    pub async fn update(data: UpdateResidenceCard) -> Result<ResidenceCard, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            ResidenceCard,
            r#"UPDATE residence_cards
               SET residence_card_type_id = $2, card_number = $3, issue_date = $4,
                   expiration_date = $5, work_restrictions = $6, is_active = $7,
                   updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, residence_card_type_id, card_number, issue_date, expiration_date, work_restrictions, is_active, created_at, updated_at"#,
            data.id,
            data.residence_card_type_id,
            data.card_number,
            data.issue_date,
            data.expiration_date,
            data.work_restrictions,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 在留カードを削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query!(r#"DELETE FROM residence_cards WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 期限切れ間近の在留カードを取得（指定日数以内に期限が切れるもの）
    pub async fn get_expiring_soon(days: i32) -> Result<Vec<ResidenceCard>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        let query = format!(
            r#"SELECT id, employee_id, residence_card_type_id, card_number, issue_date,
                      expiration_date, work_restrictions, is_active, created_at, updated_at
               FROM residence_cards
               WHERE is_active = TRUE
                 AND expiration_date BETWEEN CURRENT_DATE AND CURRENT_DATE + INTERVAL '{} days'
               ORDER BY expiration_date ASC"#,
            days
        );
        sqlx::query_as::<_, ResidenceCard>(&query)
            .fetch_all(pool)
            .await
    }
}
