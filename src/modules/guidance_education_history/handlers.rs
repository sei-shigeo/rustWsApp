use super::models::{
    CreateGuidanceEducationHistory, GuidanceEducationHistory, GuidanceEducationType,
    UpdateGuidanceEducationHistory,
};
#[cfg(feature = "server")]
use super::repository::GuidanceEducationHistoryRepository;
use dioxus::prelude::*;

/// 従業員の指導教育履歴を取得
#[server]
pub async fn get_guidance_education_history_by_employee(
    employee_id: i32,
) -> Result<Vec<GuidanceEducationHistory>, ServerFnError> {
    GuidanceEducationHistoryRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 指導教育履歴を作成
#[server]
pub async fn create_guidance_education_history(
    data: CreateGuidanceEducationHistory,
) -> Result<GuidanceEducationHistory, ServerFnError> {
    // バリデーション
    if let Some(expiration_date) = data.expiration_date {
        if data.education_date >= expiration_date {
            return Err(ServerFnError::new(
                "実施日は有効期限より前の日付を指定してください".to_string(),
            ));
        }
    }

    GuidanceEducationHistoryRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 指導教育履歴を更新
#[server]
pub async fn update_guidance_education_history(
    data: UpdateGuidanceEducationHistory,
) -> Result<GuidanceEducationHistory, ServerFnError> {
    // バリデーション
    if let Some(expiration_date) = data.expiration_date {
        if data.education_date >= expiration_date {
            return Err(ServerFnError::new(
                "実施日は有効期限より前の日付を指定してください".to_string(),
            ));
        }
    }

    GuidanceEducationHistoryRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 指導教育履歴を削除
#[server]
pub async fn delete_guidance_education_history(id: i32) -> Result<(), ServerFnError> {
    GuidanceEducationHistoryRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 全指導教育種別を取得
#[server]
pub async fn get_all_guidance_education_types() -> Result<Vec<GuidanceEducationType>, ServerFnError>
{
    GuidanceEducationHistoryRepository::get_all_guidance_education_types()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
