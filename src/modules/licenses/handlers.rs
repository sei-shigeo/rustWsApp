use super::models::{CreateLicense, License, LicenseType, UpdateLicense};
#[cfg(feature = "server")]
use super::repository::LicenseRepository;
use dioxus::prelude::*;

/// 従業員の運転免許証を取得
#[server]
pub async fn get_licenses_by_employee(employee_id: i32) -> Result<Vec<License>, ServerFnError> {
    LicenseRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 有効な運転免許証を取得
#[server]
pub async fn get_active_licenses(employee_id: i32) -> Result<Vec<License>, ServerFnError> {
    LicenseRepository::get_active_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 運転免許証を作成
#[server]
pub async fn create_license(data: CreateLicense) -> Result<License, ServerFnError> {
    // バリデーション
    if data.issue_date.is_some() && data.issue_date >= Some(data.expiration_date) {
        return Err(ServerFnError::new(
            "交付日は有効期限より前の日付を指定してください".to_string(),
        ));
    }

    LicenseRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 運転免許証を更新
#[server]
pub async fn update_license(data: UpdateLicense) -> Result<License, ServerFnError> {
    // バリデーション
    if data.issue_date.is_some() && data.issue_date >= Some(data.expiration_date) {
        return Err(ServerFnError::new(
            "交付日は有効期限より前の日付を指定してください".to_string(),
        ));
    }

    LicenseRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 運転免許証を削除
#[server]
pub async fn delete_license(id: i32) -> Result<(), ServerFnError> {
    LicenseRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 全免許種別を取得
#[server]
pub async fn get_all_license_types() -> Result<Vec<LicenseType>, ServerFnError> {
    LicenseRepository::get_all_license_types()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
