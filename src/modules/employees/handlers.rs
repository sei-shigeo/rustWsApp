use crate::modules::employees::validation::validate_employee_code;

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
    employee_code: Option<String>,
    first_name: String,
    last_name: String,
) -> Result<Employee, ServerFnError> {
    // サーバー側でもバリデーション
    // employee_codeがある場合のみバリデーション
    if let Some(ref code) = employee_code {
        validate_employee_code(code).map_err(ServerFnError::new)?;
    }
    validate_employee_name(&first_name).map_err(ServerFnError::new)?;
    validate_employee_name(&last_name).map_err(ServerFnError::new)?;

    EmployeeRepository::create(employee_code, first_name, last_name)
        .await
        .map_err(|e| {
            // 重複エラーの場合、わかりやすいメッセージに変換
            let error_msg = e.to_string();
            if error_msg.contains("uq_employees_employee_code")
                || error_msg.contains("duplicate key")
            {
                ServerFnError::new("この従業員コードは既に使用されています".to_string())
            } else {
                ServerFnError::new(error_msg)
            }
        })
}

/// 従業員の更新
#[server]
pub async fn update_employee(
    id: i32,
    employee_code: Option<String>,
    first_name: String,
    last_name: String,
    is_active: bool,
) -> Result<Employee, ServerFnError> {
    // サーバー側でもバリデーション
    // employee_codeがある場合のみバリデーション
    if let Some(ref code) = employee_code {
        validate_employee_code(code).map_err(ServerFnError::new)?;
    }
    validate_employee_name(&first_name).map_err(ServerFnError::new)?;
    validate_employee_name(&last_name).map_err(ServerFnError::new)?;

    EmployeeRepository::update(id, employee_code, first_name, last_name, is_active)
        .await
        .map_err(|e| {
            // 重複エラーの場合、わかりやすいメッセージに変換
            let error_msg = e.to_string();
            if error_msg.contains("uq_employees_employee_code")
                || error_msg.contains("duplicate key")
            {
                ServerFnError::new("この従業員コードは既に使用されています".to_string())
            } else {
                ServerFnError::new(error_msg)
            }
        })
}

/// 従業員の削除
#[server]
pub async fn delete_employee(id: i32) -> Result<(), ServerFnError> {
    EmployeeRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}
