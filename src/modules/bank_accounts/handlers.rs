use super::models::{BankAccount, CreateBankAccount, UpdateBankAccount};
#[cfg(feature = "server")]
use super::repository::BankAccountRepository;
use dioxus::prelude::*;

/// 従業員の銀行口座一覧を取得
#[server]
pub async fn get_bank_accounts_by_employee(
    employee_id: i32,
) -> Result<Vec<BankAccount>, ServerFnError> {
    BankAccountRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 有効な銀行口座のみを取得
#[server]
pub async fn get_active_bank_accounts(employee_id: i32) -> Result<Vec<BankAccount>, ServerFnError> {
    BankAccountRepository::get_active_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// プライマリ口座を取得
#[server]
pub async fn get_primary_bank_account(
    employee_id: i32,
) -> Result<Option<BankAccount>, ServerFnError> {
    BankAccountRepository::get_primary_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 銀行口座を作成
#[server]
pub async fn create_bank_account(data: CreateBankAccount) -> Result<BankAccount, ServerFnError> {
    BankAccountRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 銀行口座を更新
#[server]
pub async fn update_bank_account(data: UpdateBankAccount) -> Result<BankAccount, ServerFnError> {
    BankAccountRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 銀行口座を削除
#[server]
pub async fn delete_bank_account(id: i32) -> Result<(), ServerFnError> {
    BankAccountRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
