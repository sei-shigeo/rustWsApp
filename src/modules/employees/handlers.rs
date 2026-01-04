use super::models::Employee;
#[cfg(feature = "server")]
use super::repository::EmployeeRepository;
use super::validation::validate_employee_name;
use dioxus::prelude::*;

/// 全従業員の取得
#[server]
pub async fn get_employees() -> Result<Vec<Employee>, ServerFnError> {
    EmployeeRepository::get_all()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 従業員の作成
#[server]
pub async fn create_employee(
    first_name: String,
    last_name: String,
) -> Result<Employee, ServerFnError> {
    // サーバー側でもバリデーション
    validate_employee_name(&first_name).map_err(ServerFnError::new)?;
    validate_employee_name(&last_name).map_err(ServerFnError::new)?;

    EmployeeRepository::create(first_name, last_name)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 従業員の更新
#[server]
pub async fn update_employee(
    id: i32,
    first_name: String,
    last_name: String,
    is_active: bool,
) -> Result<Employee, ServerFnError> {
    // サーバー側でもバリデーション
    validate_employee_name(&first_name).map_err(ServerFnError::new)?;
    validate_employee_name(&last_name).map_err(ServerFnError::new)?;

    EmployeeRepository::update(id, first_name, last_name, is_active)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

/// 従業員の削除
#[server]
pub async fn delete_employee(id: i32) -> Result<(), ServerFnError> {
    EmployeeRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
