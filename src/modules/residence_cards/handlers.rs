use super::models::{CreateResidenceCard, ResidenceCard, UpdateResidenceCard};
#[cfg(feature = "server")]
use super::repository::ResidenceCardRepository;
use dioxus::prelude::*;

/// 従業員の在留カード一覧を取得
#[server]
pub async fn get_residence_cards_by_employee(
    employee_id: i32,
) -> Result<Vec<ResidenceCard>, ServerFnError> {
    ResidenceCardRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 有効な在留カードのみを取得
#[server]
pub async fn get_active_residence_cards(
    employee_id: i32,
) -> Result<Vec<ResidenceCard>, ServerFnError> {
    ResidenceCardRepository::get_active_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 現在有効な在留カードを取得
#[server]
pub async fn get_current_residence_card(
    employee_id: i32,
) -> Result<Option<ResidenceCard>, ServerFnError> {
    ResidenceCardRepository::get_current_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 在留カードを作成
#[server]
pub async fn create_residence_card(
    data: CreateResidenceCard,
) -> Result<ResidenceCard, ServerFnError> {
    ResidenceCardRepository::create(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 在留カードを更新
#[server]
pub async fn update_residence_card(
    data: UpdateResidenceCard,
) -> Result<ResidenceCard, ServerFnError> {
    ResidenceCardRepository::update(data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 在留カードを削除
#[server]
pub async fn delete_residence_card(id: i32) -> Result<(), ServerFnError> {
    ResidenceCardRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 期限切れ間近の在留カードを取得
#[server]
pub async fn get_expiring_residence_cards(days: i32) -> Result<Vec<ResidenceCard>, ServerFnError> {
    ResidenceCardRepository::get_expiring_soon(days)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
