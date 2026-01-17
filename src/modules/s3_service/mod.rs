//! S3サービスモジュール
//! AWS S3へのファイル操作を提供

#[cfg(feature = "server")]
pub mod service;

#[cfg(feature = "server")]
pub use service::*;
