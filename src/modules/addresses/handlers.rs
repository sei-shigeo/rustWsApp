use super::models::{Address, CreateAddress, UpdateAddress};
#[cfg(feature = "server")]
use super::repository::AddressRepository;
use dioxus::prelude::*;

/// 従業員の住所一覧を取得
#[server]
pub async fn get_addresses_by_employee(employee_id: i32) -> Result<Vec<Address>, ServerFnError> {
    AddressRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 従業員の現住所を取得
#[server]
pub async fn get_current_address(employee_id: i32) -> Result<Option<Address>, ServerFnError> {
    AddressRepository::get_current_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 住所を作成
#[server]
pub async fn create_address(data: CreateAddress) -> Result<Address, ServerFnError> {
    AddressRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 住所を更新
#[server]
pub async fn update_address(data: UpdateAddress) -> Result<Address, ServerFnError> {
    AddressRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 住所を削除
#[server]
pub async fn delete_address(id: i32) -> Result<(), ServerFnError> {
    AddressRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
