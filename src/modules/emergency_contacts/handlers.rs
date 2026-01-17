use super::models::{CreateEmergencyContact, EmergencyContact, UpdateEmergencyContact};
#[cfg(feature = "server")]
use super::repository::EmergencyContactRepository;
use dioxus::prelude::*;

/// 従業員の緊急連絡先一覧を取得
#[server]
pub async fn get_emergency_contacts_by_employee(
    employee_id: i32,
) -> Result<Vec<EmergencyContact>, ServerFnError> {
    EmergencyContactRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 有効な緊急連絡先のみを取得
#[server]
pub async fn get_active_emergency_contacts(
    employee_id: i32,
) -> Result<Vec<EmergencyContact>, ServerFnError> {
    EmergencyContactRepository::get_active_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 緊急連絡先を作成
#[server]
pub async fn create_emergency_contact(
    data: CreateEmergencyContact,
) -> Result<EmergencyContact, ServerFnError> {
    EmergencyContactRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 緊急連絡先を更新
#[server]
pub async fn update_emergency_contact(
    data: UpdateEmergencyContact,
) -> Result<EmergencyContact, ServerFnError> {
    EmergencyContactRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 緊急連絡先を削除
#[server]
pub async fn delete_emergency_contact(id: i32) -> Result<(), ServerFnError> {
    EmergencyContactRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
