//! 従業員ドキュメントのモデル定義

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ドキュメントタイプ（カテゴリ）
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(
    feature = "server",
    sqlx(type_name = "VARCHAR", rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum DocumentCategory {
    /// 顔写真
    #[serde(rename = "FACE_PHOTO")]
    FacePhoto,
    /// 免許証（表）
    #[serde(rename = "LICENSE_FRONT")]
    LicenseFront,
    /// 免許証（裏）
    #[serde(rename = "LICENSE_BACK")]
    LicenseBack,
    /// 資格証明書
    #[serde(rename = "QUALIFICATION_CERT")]
    QualificationCert,
    /// 車検証
    #[serde(rename = "VEHICLE_INSPECTION")]
    VehicleInspection,
    /// 健康診断書
    #[serde(rename = "HEALTH_CHECKUP")]
    HealthCheckup,
    /// 適性診断書
    #[serde(rename = "APTITUDE_CHECKUP")]
    AptitudeCheckup,
    /// その他
    #[serde(rename = "OTHER")]
    Other,
}

impl DocumentCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            DocumentCategory::FacePhoto => "FACE_PHOTO",
            DocumentCategory::LicenseFront => "LICENSE_FRONT",
            DocumentCategory::LicenseBack => "LICENSE_BACK",
            DocumentCategory::QualificationCert => "QUALIFICATION_CERT",
            DocumentCategory::VehicleInspection => "VEHICLE_INSPECTION",
            DocumentCategory::HealthCheckup => "HEALTH_CHECKUP",
            DocumentCategory::AptitudeCheckup => "APTITUDE_CHECKUP",
            DocumentCategory::Other => "OTHER",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            DocumentCategory::FacePhoto => "顔写真",
            DocumentCategory::LicenseFront => "免許証（表）",
            DocumentCategory::LicenseBack => "免許証（裏）",
            DocumentCategory::QualificationCert => "資格証明書",
            DocumentCategory::VehicleInspection => "車検証",
            DocumentCategory::HealthCheckup => "健康診断書",
            DocumentCategory::AptitudeCheckup => "適性診断書",
            DocumentCategory::Other => "その他",
        }
    }
}

impl std::fmt::Display for DocumentCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// 従業員ドキュメント
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct EmployeeDocument {
    /// ドキュメントID
    pub id: i32,
    /// 従業員ID
    pub employee_id: i32,
    /// ドキュメントカテゴリ
    pub category: Option<DocumentCategory>,
    /// ファイル名（元のファイル名）
    pub filename: String,
    /// S3オブジェクトキー（パス）
    pub s3_key: Option<String>,
    /// S3 URL
    pub s3_url: Option<String>,
    /// MIMEタイプ
    pub mime_type: Option<String>,
    /// ファイルサイズ（バイト）
    pub file_size: Option<i64>,
    /// 説明・メモ
    pub description: Option<String>,
    /// 関連ID（資格IDや車両IDなど）
    pub related_id: Option<i32>,
    /// アップロード日時
    pub uploaded_at: DateTime<Utc>,
    /// 作成日時
    pub created_at: DateTime<Utc>,
    /// 更新日時
    pub updated_at: DateTime<Utc>,
}

/// 新規ドキュメント作成用の入力データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmployeeDocument {
    /// 従業員ID
    pub employee_id: i32,
    /// ドキュメントカテゴリ
    pub category: Option<DocumentCategory>,
    /// ファイル名
    pub filename: String,
    /// S3オブジェクトキー
    pub s3_key: Option<String>,
    /// S3 URL
    pub s3_url: Option<String>,
    /// MIMEタイプ
    pub mime_type: Option<String>,
    /// ファイルサイズ
    pub file_size: Option<i64>,
    /// 説明
    pub description: Option<String>,
    /// 関連ID
    pub related_id: Option<i32>,
}

/// ドキュメント更新用の入力データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmployeeDocument {
    /// 説明
    pub description: Option<String>,
    /// 関連ID
    pub related_id: Option<i32>,
}

/// ドキュメント一覧取得用のフィルタ
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentFilter {
    /// 従業員ID
    pub employee_id: Option<i32>,
    /// カテゴリ
    pub category: Option<DocumentCategory>,
    /// 関連ID
    pub related_id: Option<i32>,
}

/// アップロード用のファイルデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUpload {
    /// 従業員ID
    pub employee_id: i32,
    /// カテゴリ
    pub category: DocumentCategory,
    /// ファイル名
    pub filename: String,
    /// Base64エンコードされたファイルデータ
    pub data: String,
    /// 説明
    pub description: Option<String>,
    /// 関連ID
    pub related_id: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_category_display() {
        assert_eq!(DocumentCategory::FacePhoto.display_name(), "顔写真");
        assert_eq!(
            DocumentCategory::LicenseFront.display_name(),
            "免許証（表）"
        );
        assert_eq!(
            DocumentCategory::QualificationCert.display_name(),
            "資格証明書"
        );
    }

    #[test]
    fn test_document_category_as_str() {
        assert_eq!(DocumentCategory::FacePhoto.as_str(), "FACE_PHOTO");
        assert_eq!(DocumentCategory::LicenseFront.as_str(), "LICENSE_FRONT");
    }
}
