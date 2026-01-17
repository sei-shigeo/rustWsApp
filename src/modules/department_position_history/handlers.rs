use super::models::{
    CreateDepartmentPositionHistory, DepartmentPositionHistory, UpdateDepartmentPositionHistory,
};
#[cfg(feature = "server")]
use super::repository::DepartmentPositionHistoryRepository;
use dioxus::prelude::*;

/// 従業員の部署・役職履歴を取得
#[server]
pub async fn get_department_position_history_by_employee(
    employee_id: i32,
) -> Result<Vec<DepartmentPositionHistory>, ServerFnError> {
    DepartmentPositionHistoryRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 現在の配属情報を取得
#[server]
pub async fn get_current_department_position(
    employee_id: i32,
) -> Result<Option<DepartmentPositionHistory>, ServerFnError> {
    DepartmentPositionHistoryRepository::get_current_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 部署・役職履歴を作成
#[server]
pub async fn create_department_position_history(
    data: CreateDepartmentPositionHistory,
) -> Result<DepartmentPositionHistory, ServerFnError> {
    // バリデーション
    if data.office_id.is_none() && data.department_id.is_none() && data.position_id.is_none() {
        return Err(ServerFnError::new(
            "営業所、部署、役職のいずれかを指定してください".to_string(),
        ));
    }

    DepartmentPositionHistoryRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 部署・役職履歴を更新
#[server]
pub async fn update_department_position_history(
    data: UpdateDepartmentPositionHistory,
) -> Result<DepartmentPositionHistory, ServerFnError> {
    // バリデーション
    if data.office_id.is_none() && data.department_id.is_none() && data.position_id.is_none() {
        return Err(ServerFnError::new(
            "営業所、部署、役職のいずれかを指定してください".to_string(),
        ));
    }

    DepartmentPositionHistoryRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 部署・役職履歴を削除
#[server]
pub async fn delete_department_position_history(id: i32) -> Result<(), ServerFnError> {
    DepartmentPositionHistoryRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
