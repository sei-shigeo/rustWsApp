use crate::modules::employees::handlers::create_employee;
use crate::modules::employees::models::Employee;
use crate::modules::employees::validation::{validate_employee_code, validate_employee_name};
use dioxus::prelude::*;

/// Client-side action helper for creating an employee.
///
/// Responsibilities:
/// - Run quick client-side validation to provide immediate feedback to the user.
/// - Delegate the actual creation to the server via the generated `create_employee` server function.
/// - Normalize errors into a simple `Result<Employee, String>` so UI code can handle success/failure easily.
///
/// Notes for beginners:
/// - We still run the same validations on the server side (see `handlers::create_employee`).
///   Client-side validation is for UX only and does not replace server validation.
/// - `create_employee` is a server function (annotated with `#[server]` in `handlers.rs`).
///   When compiled for the client, the macro generates an async function you can `.await`.
pub async fn create_employee_action(
    employee_code: Option<String>,
    first_name: String,
    last_name: String,
) -> Result<Employee, String> {
    // === Client-side quick validation ===
    // Validate employee code format if provided.
    if let Some(ref code) = employee_code {
        // propagate validation error directly without extra to_string clone
        validate_employee_code(code)?;
    }

    // Validate names (first and last).
    // propagate errors directly (no implicit clone via to_string)
    validate_employee_name(&first_name)?;
    validate_employee_name(&last_name)?;

    // === Call server function ===
    // The server function will perform the authoritative validation too and persist the data.
    match create_employee(employee_code, first_name, last_name).await {
        Ok(employee) => Ok(employee),
        Err(e) => {
            // ServerFnError -> Render a friendly string for UI consumption.
            // Handlers may already convert some DB errors into user-friendly messages.
            Err(e.to_string())
        }
    }
}
