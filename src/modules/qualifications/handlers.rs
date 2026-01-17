use super::models::{CreateQualification, Qualification, QualificationType, UpdateQualification};
#[cfg(feature = "server")]
use super::repository::QualificationRepository;
use dioxus::prelude::*;

/// 従業員の資格証を取得
#[server]
pub async fn get_qualifications_by_employee(
    employee_id: i32,
) -> Result<Vec<Qualification>, ServerFnError> {
    QualificationRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 有効な資格証を取得
#[server]
pub async fn get_active_qualifications(
    employee_id: i32,
) -> Result<Vec<Qualification>, ServerFnError> {
    QualificationRepository::get_active_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 資格証を作成
#[server]
pub async fn create_qualification(
    data: CreateQualification,
) -> Result<Qualification, ServerFnError> {
    // バリデーション
    if let (Some(issue_date), Some(expiration_date)) = (data.issue_date, data.expiration_date) {
        if issue_date >= expiration_date {
            return Err(ServerFnError::new(
                "取得日は有効期限より前の日付を指定してください".to_string(),
            ));
        }
    }

    QualificationRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 資格証を更新
#[server]
pub async fn update_qualification(
    data: UpdateQualification,
) -> Result<Qualification, ServerFnError> {
    // バリデーション
    if let (Some(issue_date), Some(expiration_date)) = (data.issue_date, data.expiration_date) {
        if issue_date >= expiration_date {
            return Err(ServerFnError::new(
                "取得日は有効期限より前の日付を指定してください".to_string(),
            ));
        }
    }

    QualificationRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 資格証を削除
#[server]
pub async fn delete_qualification(id: i32) -> Result<(), ServerFnError> {
    QualificationRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 全資格種別を取得
#[server]
pub async fn get_all_qualification_types() -> Result<Vec<QualificationType>, ServerFnError> {
    QualificationRepository::get_all_qualification_types()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
