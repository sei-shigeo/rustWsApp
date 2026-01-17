use super::models::{CreateEducationHistory, EducationHistory, UpdateEducationHistory};
#[cfg(feature = "server")]
use super::repository::EducationHistoryRepository;
use dioxus::prelude::*;

/// 従業員の学歴一覧を取得
#[server]
pub async fn get_education_history_by_employee(
    employee_id: i32,
) -> Result<Vec<EducationHistory>, ServerFnError> {
    EducationHistoryRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 学歴を作成
#[server]
pub async fn create_education_history(
    data: CreateEducationHistory,
) -> Result<EducationHistory, ServerFnError> {
    EducationHistoryRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 学歴を更新
#[server]
pub async fn update_education_history(
    data: UpdateEducationHistory,
) -> Result<EducationHistory, ServerFnError> {
    EducationHistoryRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 学歴を削除
#[server]
pub async fn delete_education_history(id: i32) -> Result<(), ServerFnError> {
    EducationHistoryRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
