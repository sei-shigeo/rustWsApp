use super::models::{
    CreateInsuranceHistory, InsuranceHistory, InsuranceType, UpdateInsuranceHistory,
};
#[cfg(feature = "server")]
use super::repository::InsuranceHistoryRepository;
use dioxus::prelude::*;

/// 従業員の保険証履歴を取得
#[server]
pub async fn get_insurance_history_by_employee(
    employee_id: i32,
) -> Result<Vec<InsuranceHistory>, ServerFnError> {
    InsuranceHistoryRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 有効な保険証履歴を取得
#[server]
pub async fn get_active_insurance_history(
    employee_id: i32,
) -> Result<Vec<InsuranceHistory>, ServerFnError> {
    InsuranceHistoryRepository::get_active_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 保険証履歴を作成
#[server]
pub async fn create_insurance_history(
    data: CreateInsuranceHistory,
) -> Result<InsuranceHistory, ServerFnError> {
    // バリデーション
    if let Some(end_date) = data.end_date {
        if data.start_date >= end_date {
            return Err(ServerFnError::new(
                "加入日は終了日より前の日付を指定してください".to_string(),
            ));
        }
    }

    InsuranceHistoryRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 保険証履歴を更新
#[server]
pub async fn update_insurance_history(
    data: UpdateInsuranceHistory,
) -> Result<InsuranceHistory, ServerFnError> {
    // バリデーション
    if let Some(end_date) = data.end_date {
        if data.start_date >= end_date {
            return Err(ServerFnError::new(
                "加入日は終了日より前の日付を指定してください".to_string(),
            ));
        }
    }

    InsuranceHistoryRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 保険証履歴を削除
#[server]
pub async fn delete_insurance_history(id: i32) -> Result<(), ServerFnError> {
    InsuranceHistoryRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 全保険種別を取得
#[server]
pub async fn get_all_insurance_types() -> Result<Vec<InsuranceType>, ServerFnError> {
    InsuranceHistoryRepository::get_all_insurance_types()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
