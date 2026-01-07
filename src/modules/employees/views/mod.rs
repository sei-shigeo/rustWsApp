mod create;
mod edit;
mod page;
mod right_panel;

// Re-export commonly used view components from this module.
// `EmployeeEdit` is defined in `edit.rs`, while `EmployeeItem` was
// moved into the employees components submodule; re-export it here
// so callers can continue to access `crate::modules::employees::views::EmployeeItem`.

pub use page::EmployeePage;
