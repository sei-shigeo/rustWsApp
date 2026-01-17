//! 従業員ドキュメントのハンドラー層（サーバー関数）

use super::models::{
    CreateEmployeeDocument, DocumentCategory, DocumentFilter, EmployeeDocument, FileUpload,
    UpdateEmployeeDocument,
};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::repository;
#[cfg(feature = "server")]
use crate::modules::s3_service;

/// ファイルをアップロードしてドキュメントを作成
#[server(UploadDocument)]
pub async fn upload_document(upload: FileUpload) -> Result<EmployeeDocument, ServerFnError> {
    // Base64デコード
    let data = base64_decode(&upload.data)?;

    // MIMEタイプの推測
    let mime_type = s3_service::guess_mime_type(&upload.filename);

    // S3キーの生成
    let category_str = match upload.category {
        DocumentCategory::FacePhoto => "face",
        DocumentCategory::LicenseFront => "licenses",
        DocumentCategory::LicenseBack => "licenses",
        DocumentCategory::QualificationCert => "qualifications",
        DocumentCategory::VehicleInspection => "vehicles",
        DocumentCategory::HealthCheckup => "health",
        DocumentCategory::AptitudeCheckup => "aptitude",
        DocumentCategory::Other => "other",
    };

    let s3_key = s3_service::generate_s3_key(category_str, upload.employee_id, &upload.filename);

    // S3にアップロード
    let s3_url = s3_service::upload_file(&s3_key, data.clone().into(), mime_type)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // DBに保存
    let create_doc = CreateEmployeeDocument {
        employee_id: upload.employee_id,
        category: Some(upload.category),
        filename: upload.filename,
        s3_key: Some(s3_key.clone()),
        s3_url: Some(s3_url.clone()),
        mime_type: Some(mime_type.to_string()),
        file_size: Some(data.len() as i64),
        description: upload.description,
        related_id: upload.related_id,
    };

    let document = repository::EmployeeDocumentRepository::create(create_doc)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(document)
}

/// ドキュメントを取得
#[server(GetDocument)]
pub async fn get_document(id: i32) -> Result<Option<EmployeeDocument>, ServerFnError> {
    let document = repository::EmployeeDocumentRepository::get_by_id(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(document)
}

/// 従業員のドキュメント一覧を取得
#[server(GetEmployeeDocuments)]
pub async fn get_employee_documents(
    employee_id: i32,
) -> Result<Vec<EmployeeDocument>, ServerFnError> {
    let documents = repository::EmployeeDocumentRepository::get_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(documents)
}

/// カテゴリでフィルタしたドキュメント一覧を取得
#[server(GetDocumentsByCategory)]
pub async fn get_documents_by_category(
    employee_id: i32,
    category: DocumentCategory,
) -> Result<Vec<EmployeeDocument>, ServerFnError> {
    let documents =
        repository::EmployeeDocumentRepository::get_by_category(employee_id, category.as_str())
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(documents)
}

/// フィルタ条件でドキュメント一覧を取得
#[server(GetDocumentsByFilter)]
pub async fn get_documents_by_filter(
    filter: DocumentFilter,
) -> Result<Vec<EmployeeDocument>, ServerFnError> {
    // Filter is not implemented in new repository structure, using basic query
    if let Some(employee_id) = filter.employee_id {
        if let Some(category) = filter.category {
            let documents = repository::EmployeeDocumentRepository::get_by_category(
                employee_id,
                category.as_str(),
            )
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
            return Ok(documents);
        }
        let documents = repository::EmployeeDocumentRepository::get_by_employee_id(employee_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        return Ok(documents);
    }
    // Return empty if no employee_id
    Ok(Vec::new())
}

/// ドキュメントを更新
#[server(UpdateDocument)]
pub async fn update_document(
    id: i32,
    update: UpdateEmployeeDocument,
) -> Result<EmployeeDocument, ServerFnError> {
    let document = repository::EmployeeDocumentRepository::update(id, update)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(document)
}

/// ドキュメントを削除（S3からも削除）
#[server(DeleteDocument)]
pub async fn delete_document(id: i32) -> Result<(), ServerFnError> {
    // ドキュメント情報を取得
    let document = repository::EmployeeDocumentRepository::get_by_id(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Document not found".to_string()))?;

    // S3から削除（s3_keyがある場合のみ）
    if let Some(s3_key) = &document.s3_key {
        s3_service::delete_file(s3_key)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    // DBから削除
    repository::EmployeeDocumentRepository::delete(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

/// ドキュメントのダウンロードURL取得（署名付きURL）
#[server(GetDocumentDownloadUrl)]
pub async fn get_document_download_url(id: i32) -> Result<String, ServerFnError> {
    let document = repository::EmployeeDocumentRepository::get_by_id(id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Document not found".to_string()))?;

    // 現状は直接S3 URLを返す
    // 本番環境では署名付きURLを生成することを推奨
    Ok(document.s3_url.unwrap_or_default())
}

/// 従業員のドキュメント数を取得
#[server(CountEmployeeDocuments)]
pub async fn count_employee_documents(employee_id: i32) -> Result<i64, ServerFnError> {
    let count = repository::EmployeeDocumentRepository::count_by_employee_id(employee_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(count)
}

/// カテゴリ別のドキュメント数を取得
#[server(CountDocumentsByCategory)]
pub async fn count_documents_by_category(
    employee_id: i32,
    category: DocumentCategory,
) -> Result<i64, ServerFnError> {
    let count =
        repository::EmployeeDocumentRepository::count_by_category(employee_id, category.as_str())
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(count)
}

/// Base64文字列をデコード
#[cfg(feature = "server")]
fn base64_decode(encoded: &str) -> Result<Vec<u8>, ServerFnError> {
    // "data:image/jpeg;base64," のようなプレフィックスを削除
    let data = if let Some(comma_pos) = encoded.find(',') {
        &encoded[comma_pos + 1..]
    } else {
        encoded
    };

    // Base64デコード
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD
        .decode(data)
        .map_err(|e| ServerFnError::new(format!("Base64 decode error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_decode() {
        let encoded = "data:image/jpeg;base64,SGVsbG8gV29ybGQ=";
        let decoded = base64_decode(encoded).unwrap();
        assert_eq!(decoded, b"Hello World");
    }

    #[test]
    fn test_base64_decode_without_prefix() {
        let encoded = "SGVsbG8gV29ybGQ=";
        let decoded = base64_decode(encoded).unwrap();
        assert_eq!(decoded, b"Hello World");
    }
}
