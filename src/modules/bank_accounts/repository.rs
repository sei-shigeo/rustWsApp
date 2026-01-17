#[cfg(feature = "server")]
use super::models::{BankAccount, CreateBankAccount, UpdateBankAccount};
#[cfg(feature = "server")]
use crate::db;

#[cfg(feature = "server")]
pub struct BankAccountRepository;

#[cfg(feature = "server")]
impl BankAccountRepository {
    /// 従業員の銀行口座一覧を取得
    pub async fn get_by_employee_id(employee_id: i32) -> Result<Vec<BankAccount>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            BankAccount,
            r#"SELECT id, employee_id, bank_code, bank_name, branch_code, branch_name,
                      account_type, account_number, account_holder_name, is_primary, is_active,
                      created_at, updated_at
               FROM employee_bank_accounts
               WHERE employee_id = $1
               ORDER BY is_primary DESC, is_active DESC, created_at DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// 有効な銀行口座のみを取得
    pub async fn get_active_by_employee_id(
        employee_id: i32,
    ) -> Result<Vec<BankAccount>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            BankAccount,
            r#"SELECT id, employee_id, bank_code, bank_name, branch_code, branch_name,
                      account_type, account_number, account_holder_name, is_primary, is_active,
                      created_at, updated_at
               FROM employee_bank_accounts
               WHERE employee_id = $1 AND is_active = TRUE
               ORDER BY is_primary DESC, created_at DESC"#,
            employee_id
        )
        .fetch_all(pool)
        .await
    }

    /// プライマリ口座を取得
    pub async fn get_primary_by_employee_id(
        employee_id: i32,
    ) -> Result<Option<BankAccount>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            BankAccount,
            r#"SELECT id, employee_id, bank_code, bank_name, branch_code, branch_name,
                      account_type, account_number, account_holder_name, is_primary, is_active,
                      created_at, updated_at
               FROM employee_bank_accounts
               WHERE employee_id = $1 AND is_primary = TRUE AND is_active = TRUE
               LIMIT 1"#,
            employee_id
        )
        .fetch_optional(pool)
        .await
    }

    /// IDで銀行口座を取得
    pub async fn get_by_id(id: i32) -> Result<Option<BankAccount>, sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query_as!(
            BankAccount,
            r#"SELECT id, employee_id, bank_code, bank_name, branch_code, branch_name,
                      account_type, account_number, account_holder_name, is_primary, is_active,
                      created_at, updated_at
               FROM employee_bank_accounts
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    /// 銀行口座を作成
    pub async fn create(data: CreateBankAccount) -> Result<BankAccount, sqlx::Error> {
        let pool = db::get_pool_async().await?;

        // プライマリ口座として作成する場合、既存のプライマリ口座を解除
        if data.is_primary {
            sqlx::query!(
                r#"UPDATE employee_bank_accounts
                   SET is_primary = FALSE, updated_at = CURRENT_TIMESTAMP
                   WHERE employee_id = $1 AND is_primary = TRUE AND is_active = TRUE"#,
                data.employee_id
            )
            .execute(pool)
            .await?;
        }

        sqlx::query_as!(
            BankAccount,
            r#"INSERT INTO employee_bank_accounts (employee_id, bank_code, bank_name, branch_code, branch_name, account_type, account_number, account_holder_name, is_primary, is_active, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, TRUE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
               RETURNING id, employee_id, bank_code, bank_name, branch_code, branch_name, account_type, account_number, account_holder_name, is_primary, is_active, created_at, updated_at"#,
            data.employee_id,
            data.bank_code,
            data.bank_name,
            data.branch_code,
            data.branch_name,
            data.account_type,
            data.account_number,
            data.account_holder_name,
            data.is_primary
        )
        .fetch_one(pool)
        .await
    }

    /// 銀行口座を更新
    pub async fn update(data: UpdateBankAccount) -> Result<BankAccount, sqlx::Error> {
        let pool = db::get_pool_async().await?;

        // プライマリ口座として更新する場合、他のプライマリ口座を解除
        if data.is_primary && data.is_active {
            sqlx::query!(
                r#"UPDATE employee_bank_accounts
                   SET is_primary = FALSE, updated_at = CURRENT_TIMESTAMP
                   WHERE employee_id = (SELECT employee_id FROM employee_bank_accounts WHERE id = $1)
                   AND is_primary = TRUE
                   AND is_active = TRUE
                   AND id != $1"#,
                data.id
            )
            .execute(pool)
            .await?;
        }

        sqlx::query_as!(
            BankAccount,
            r#"UPDATE employee_bank_accounts
               SET bank_code = $2, bank_name = $3, branch_code = $4, branch_name = $5,
                   account_type = $6, account_number = $7, account_holder_name = $8,
                   is_primary = $9, is_active = $10, updated_at = CURRENT_TIMESTAMP
               WHERE id = $1
               RETURNING id, employee_id, bank_code, bank_name, branch_code, branch_name, account_type, account_number, account_holder_name, is_primary, is_active, created_at, updated_at"#,
            data.id,
            data.bank_code,
            data.bank_name,
            data.branch_code,
            data.branch_name,
            data.account_type,
            data.account_number,
            data.account_holder_name,
            data.is_primary,
            data.is_active
        )
        .fetch_one(pool)
        .await
    }

    /// 銀行口座を削除
    pub async fn delete(id: i32) -> Result<(), sqlx::Error> {
        let pool = db::get_pool_async().await?;
        sqlx::query!(r#"DELETE FROM employee_bank_accounts WHERE id = $1"#, id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
