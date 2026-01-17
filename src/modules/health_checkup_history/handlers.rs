use super::models::{
    CreateHealthCheckupHistory, HealthCheckupHistory, HealthCheckupType, UpdateHealthCheckupHistory,
};
#[cfg(feature = "server")]
use super::repository::HealthCheckupHistoryRepository;
use dioxus::prelude::*;

/// 従業員の健康診断履歴を取得
#[server]
pub async fn get_health_checkup_history_by_employee(
    employee_id: i32,
) -> Result<Vec<HealthCheckupHistory>, ServerFnError> {
    HealthCheckupHistoryRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 有効な健康診断履歴を取得
#[server]
pub async fn get_active_health_checkup_history(
    employee_id: i32,
) -> Result<Vec<HealthCheckupHistory>, ServerFnError> {
    HealthCheckupHistoryRepository::get_active_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 健康診断履歴を作成
#[server]
pub async fn create_health_checkup_history(
    data: CreateHealthCheckupHistory,
) -> Result<HealthCheckupHistory, ServerFnError> {
    // バリデーション
    if let Some(expiration_date) = data.expiration_date {
        if data.checkup_date >= expiration_date {
            return Err(ServerFnError::new(
                "受診日は有効期限より前の日付を指定してください".to_string(),
            ));
        }
    }

    HealthCheckupHistoryRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 健康診断履歴を更新
#[server]
pub async fn update_health_checkup_history(
    data: UpdateHealthCheckupHistory,
) -> Result<HealthCheckupHistory, ServerFnError> {
    // バリデーション
    if let Some(expiration_date) = data.expiration_date {
        if data.checkup_date >= expiration_date {
            return Err(ServerFnError::new(
                "受診日は有効期限より前の日付を指定してください".to_string(),
            ));
        }
    }

    HealthCheckupHistoryRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 健康診断履歴を削除
#[server]
pub async fn delete_health_checkup_history(id: i32) -> Result<(), ServerFnError> {
    HealthCheckupHistoryRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 全健康診断種別を取得
#[server]
pub async fn get_all_health_checkup_types() -> Result<Vec<HealthCheckupType>, ServerFnError> {
    HealthCheckupHistoryRepository::get_all_health_checkup_types()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
