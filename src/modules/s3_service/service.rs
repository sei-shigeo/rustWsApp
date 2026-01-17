//! S3サービスモジュール
//! AWS S3へのファイルアップロード、ダウンロード、削除を行う

use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use bytes::Bytes;
use std::sync::OnceLock;

/// S3クライアントのグローバルインスタンス
static S3_CLIENT: OnceLock<Client> = OnceLock::new();

/// S3バケット名
static BUCKET_NAME: OnceLock<String> = OnceLock::new();

/// S3サービスのエラー型
#[derive(Debug, Clone)]
pub struct S3Error {
    pub message: String,
}

impl std::fmt::Display for S3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S3 Error: {}", self.message)
    }
}

impl std::error::Error for S3Error {}

impl From<String> for S3Error {
    fn from(message: String) -> Self {
        S3Error { message }
    }
}

impl From<&str> for S3Error {
    fn from(message: &str) -> Self {
        S3Error {
            message: message.to_string(),
        }
    }
}

/// S3クライアントを初期化
pub async fn init_s3_client(bucket_name: String) {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    S3_CLIENT.set(client).ok();
    BUCKET_NAME.set(bucket_name).ok();

    println!("S3 client initialized");
}

/// S3クライアントを取得
fn get_client() -> Result<&'static Client, S3Error> {
    S3_CLIENT
        .get()
        .ok_or_else(|| S3Error::from("S3 client not initialized"))
}

/// バケット名を取得
fn get_bucket_name() -> Result<&'static str, S3Error> {
    BUCKET_NAME
        .get()
        .map(|s| s.as_str())
        .ok_or_else(|| S3Error::from("Bucket name not set"))
}

/// S3にファイルをアップロード
///
/// # Arguments
/// * `key` - S3オブジェクトキー（パス）
/// * `data` - アップロードするファイルのバイトデータ
/// * `content_type` - ファイルのMIMEタイプ
///
/// # Returns
/// アップロード成功時はS3のオブジェクトURL
pub async fn upload_file(key: &str, data: Bytes, content_type: &str) -> Result<String, S3Error> {
    let client = get_client()?;
    let bucket_name = get_bucket_name()?;

    let body = ByteStream::from(data);

    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body)
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| S3Error::from(format!("Failed to upload file: {}", e)))?;

    // S3のURLを生成（リージョンによって異なる場合があるので注意）
    let url = format!("https://{}.s3.amazonaws.com/{}", bucket_name, key);

    Ok(url)
}

/// S3からファイルをダウンロード
///
/// # Arguments
/// * `key` - S3オブジェクトキー（パス）
///
/// # Returns
/// ファイルのバイトデータ
pub async fn download_file(key: &str) -> Result<Bytes, S3Error> {
    let client = get_client()?;
    let bucket_name = get_bucket_name()?;

    let response = client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await
        .map_err(|e| S3Error::from(format!("Failed to download file: {}", e)))?;

    let data = response
        .body
        .collect()
        .await
        .map_err(|e| S3Error::from(format!("Failed to read file data: {}", e)))?
        .into_bytes();

    Ok(data)
}

/// S3からファイルを削除
///
/// # Arguments
/// * `key` - S3オブジェクトキー（パス）
pub async fn delete_file(key: &str) -> Result<(), S3Error> {
    let client = get_client()?;
    let bucket_name = get_bucket_name()?;

    client
        .delete_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await
        .map_err(|e| S3Error::from(format!("Failed to delete file: {}", e)))?;

    Ok(())
}

/// S3に複数のファイルを一括削除
///
/// # Arguments
/// * `keys` - 削除するS3オブジェクトキーのリスト
pub async fn delete_files(keys: Vec<String>) -> Result<(), S3Error> {
    for key in keys {
        delete_file(&key).await?;
    }
    Ok(())
}

/// ファイルのS3キーを生成
///
/// # Arguments
/// * `category` - カテゴリ（例: "face", "licenses", "qualifications"）
/// * `employee_id` - 従業員ID
/// * `filename` - ファイル名
///
/// # Returns
/// S3オブジェクトキー（例: "employees/123/face/20240101_120000_photo.jpg"）
pub fn generate_s3_key(category: &str, employee_id: i32, filename: &str) -> String {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    format!(
        "employees/{}/{}/{}_{}",
        employee_id, category, timestamp, filename
    )
}

/// ファイル拡張子からMIMEタイプを推測
pub fn guess_mime_type(filename: &str) -> &'static str {
    let extension = filename.split('.').last().unwrap_or("");

    match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_s3_key() {
        let key = generate_s3_key("face", 123, "photo.jpg");
        assert!(key.starts_with("employees/123/face/"));
        assert!(key.ends_with("_photo.jpg"));
    }

    #[test]
    fn test_guess_mime_type() {
        assert_eq!(guess_mime_type("photo.jpg"), "image/jpeg");
        assert_eq!(guess_mime_type("image.png"), "image/png");
        assert_eq!(guess_mime_type("document.pdf"), "application/pdf");
        assert_eq!(guess_mime_type("unknown.xyz"), "application/octet-stream");
    }
}
