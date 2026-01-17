use super::models::{
    AptitudeCheckupHistory, AptitudeCheckupType, CreateAptitudeCheckupHistory,
    UpdateAptitudeCheckupHistory,
};
#[cfg(feature = "server")]
use super::repository::AptitudeCheckupHistoryRepository;
use dioxus::prelude::*;

/// 従業員の適性診断履歴を取得
#[server]
pub async fn get_aptitude_checkup_history_by_employee(
    employee_id: i32,
) -> Result<Vec<AptitudeCheckupHistory>, ServerFnError> {
    AptitudeCheckupHistoryRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 有効な適性診断履歴を取得
#[server]
pub async fn get_active_aptitude_checkup_history(
    employee_id: i32,
) -> Result<Vec<AptitudeCheckupHistory>, ServerFnError> {
    AptitudeCheckupHistoryRepository::get_active_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 適性診断履歴を作成
#[server]
pub async fn create_aptitude_checkup_history(
    data: CreateAptitudeCheckupHistory,
) -> Result<AptitudeCheckupHistory, ServerFnError> {
    // バリデーション
    if let Some(expiration_date) = data.expiration_date {
        if data.checkup_date >= expiration_date {
            return Err(ServerFnError::new(
                "受診日は有効期限より前の日付を指定してください".to_string(),
            ));
        }
    }

    AptitudeCheckupHistoryRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 適性診断履歴を更新
#[server]
pub async fn update_aptitude_checkup_history(
    data: UpdateAptitudeCheckupHistory,
) -> Result<AptitudeCheckupHistory, ServerFnError> {
    // バリデーション
    if let Some(expiration_date) = data.expiration_date {
        if data.checkup_date >= expiration_date {
            return Err(ServerFnError::new(
                "受診日は有効期限より前の日付を指定してください".to_string(),
            ));
        }
    }

    AptitudeCheckupHistoryRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 適性診断履歴を削除
#[server]
pub async fn delete_aptitude_checkup_history(id: i32) -> Result<(), ServerFnError> {
    AptitudeCheckupHistoryRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 全適性診断種別を取得
#[server]
pub async fn get_all_aptitude_checkup_types() -> Result<Vec<AptitudeCheckupType>, ServerFnError> {
    AptitudeCheckupHistoryRepository::get_all_aptitude_checkup_types()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
