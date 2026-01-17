use super::models::{CreateEmploymentHistory, EmploymentHistory, UpdateEmploymentHistory};
#[cfg(feature = "server")]
use super::repository::EmploymentHistoryRepository;
use dioxus::prelude::*;

/// 従業員の職歴一覧を取得
#[server]
pub async fn get_employment_history_by_employee(
    employee_id: i32,
) -> Result<Vec<EmploymentHistory>, ServerFnError> {
    EmploymentHistoryRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 現在の職歴を取得
#[server]
pub async fn get_current_employment(
    employee_id: i32,
) -> Result<Option<EmploymentHistory>, ServerFnError> {
    EmploymentHistoryRepository::get_current_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 職歴を作成
#[server]
pub async fn create_employment_history(
    data: CreateEmploymentHistory,
) -> Result<EmploymentHistory, ServerFnError> {
    EmploymentHistoryRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 職歴を更新
#[server]
pub async fn update_employment_history(
    data: UpdateEmploymentHistory,
) -> Result<EmploymentHistory, ServerFnError> {
    EmploymentHistoryRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 職歴を削除
#[server]
pub async fn delete_employment_history(id: i32) -> Result<(), ServerFnError> {
    EmploymentHistoryRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
