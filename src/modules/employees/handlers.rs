use super::models::{
    Address, CreateAddress, Employee, EmployeeFull, EmployeeWithAddress, UpdateAddress,
};
#[cfg(feature = "server")]
use super::repository::EmployeeRepository;
use super::validation::{
    validate_address_field, validate_employee_code, validate_employee_name, validate_postal_code,
};
use dioxus::prelude::*;

/// データベースエラーを`ServerFnError`に変換するヘルパー関数
#[cfg(feature = "server")]
fn db_error_to_server_error(error: sqlx::Error) -> ServerFnError {
    let error_msg = error.to_string();

    // 重複エラーの場合、わかりやすいメッセージに変換
    if error_msg.contains("uq_employees_employee_code") || error_msg.contains("duplicate key") {
        ServerFnError::new("この従業員コードは既に使用されています".to_string())
    } else if error_msg.contains("employees_email_key") {
        ServerFnError::new("このメールアドレスは既に使用されています".to_string())
    } else {
        ServerFnError::new(error_msg)
    }
}

/// 従業員の基本情報をバリデーション
fn validate_employee_basic(
    employee_code: &str,
    first_name: &str,
    last_name: &str,
) -> Result<(), ServerFnError> {
    validate_employee_code(employee_code).map_err(ServerFnError::new)?;
    validate_employee_name(first_name).map_err(ServerFnError::new)?;
    validate_employee_name(last_name).map_err(ServerFnError::new)?;
    Ok(())
}

/// 全従業員の取得
#[server]
pub async fn get_employees() -> Result<Vec<Employee>, ServerFnError> {
    EmployeeRepository::get_all()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 従業員コードの重複チェック
#[server]
pub async fn check_employee_code_available(
    employee_code: String,
    exclude_id: Option<i32>, // 編集時に自分自身を除外
) -> Result<bool, ServerFnError> {
    EmployeeRepository::is_employee_code_available(&employee_code, exclude_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 従業員の作成
#[server]
pub async fn create_employee(
    employee_code: String,
    first_name: String,
    last_name: String,
) -> Result<Employee, ServerFnError> {
    // バリデーション
    validate_employee_basic(&employee_code, &first_name, &last_name)?;

    EmployeeRepository::create(employee_code, first_name, last_name)
        .await
        .map_err(db_error_to_server_error)
}

/// 従業員の更新
#[server]
pub async fn update_employee(
    id: i32,
    employee_code: String,
    first_name: String,
    last_name: String,
    is_active: bool,
) -> Result<Employee, ServerFnError> {
    // バリデーション
    validate_employee_basic(&employee_code, &first_name, &last_name)?;

    EmployeeRepository::update(id, employee_code, first_name, last_name, is_active)
        .await
        .map_err(db_error_to_server_error)
}

/// 従業員の削除
#[server]
pub async fn delete_employee(id: i32) -> Result<(), ServerFnError> {
    EmployeeRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// IDで従業員の詳細情報を取得
#[server]
pub async fn get_employee_full(id: i32) -> Result<Option<EmployeeFull>, ServerFnError> {
    EmployeeRepository::get_by_id(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 従業員の詳細情報を更新
#[server]
pub async fn update_employee_full(employee: EmployeeFull) -> Result<EmployeeFull, ServerFnError> {
    // バリデーション
    validate_employee_basic(
        &employee.employee_code,
        &employee.first_name,
        &employee.last_name,
    )?;

    EmployeeRepository::update_full(employee)
        .await
        .map_err(db_error_to_server_error)
}

/// 住所情報を含む全従業員の取得
#[server]
pub async fn get_employees_with_address() -> Result<Vec<EmployeeWithAddress>, ServerFnError> {
    EmployeeRepository::get_all_with_address()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 従業員の現住所を取得
#[server]
pub async fn get_current_address(employee_id: i32) -> Result<Option<Address>, ServerFnError> {
    EmployeeRepository::get_current_address(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 従業員の全住所履歴を取得
#[server]
pub async fn get_all_addresses(employee_id: i32) -> Result<Vec<Address>, ServerFnError> {
    EmployeeRepository::get_all_addresses(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 住所の作成
#[server]
pub async fn create_address(address: CreateAddress) -> Result<Address, ServerFnError> {
    // バリデーション
    validate_postal_code(&address.postal_code).map_err(ServerFnError::new)?;
    validate_address_field(&address.prefecture, "都道府県").map_err(ServerFnError::new)?;
    validate_address_field(&address.city, "市区町村").map_err(ServerFnError::new)?;
    validate_address_field(&address.street, "番地").map_err(ServerFnError::new)?;

    EmployeeRepository::create_address(address)
        .await
        .map_err(db_error_to_server_error)
}

/// 住所の更新
#[server]
pub async fn update_address(address: UpdateAddress) -> Result<Address, ServerFnError> {
    // バリデーション
    validate_postal_code(&address.postal_code).map_err(ServerFnError::new)?;
    validate_address_field(&address.prefecture, "都道府県").map_err(ServerFnError::new)?;
    validate_address_field(&address.city, "市区町村").map_err(ServerFnError::new)?;
    validate_address_field(&address.street, "番地").map_err(ServerFnError::new)?;

    EmployeeRepository::update_address(address)
        .await
        .map_err(db_error_to_server_error)
}

/// 住所の削除
#[server]
pub async fn delete_address(id: i32) -> Result<(), ServerFnError> {
    EmployeeRepository::delete_address(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
