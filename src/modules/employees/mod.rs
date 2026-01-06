pub mod actions; // 追加: クライアント側のアクション (create_employee_action など) を公開
pub mod components;
pub mod handlers;
pub mod models;
pub mod validation;
pub mod views;

#[cfg(feature = "server")]
pub mod repository;

// 公開するもの
pub use views::EmployeePage;
