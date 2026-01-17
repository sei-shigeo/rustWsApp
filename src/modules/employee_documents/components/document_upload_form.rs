//! ドキュメントアップロードフォームコンポーネント

use crate::modules::employee_documents::models::{DocumentCategory, FileUpload};
use dioxus::prelude::*;

/// ファイル選択状態を表す構造体
#[derive(Debug, Clone, Default)]
struct SelectedFile {
    name: String,
    size: u64,
    data: Option<String>, // Base64エンコードされたデータ
}

/// ドキュメントアップロードフォームのプロパティ
#[derive(Props, Clone, PartialEq)]
pub struct DocumentUploadFormProps {
    /// 従業員ID
    pub employee_id: i32,
    /// アップロード成功時のコールバック
    pub on_upload_success: EventHandler<()>,
}

/// Base64エンコード関数（web feature用）
#[cfg(feature = "web")]
fn encode_base64(data: &[u8]) -> String {
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD.encode(data)
}

/// ドキュメントアップロードフォームコンポーネント
#[component]
pub fn DocumentUploadForm(props: DocumentUploadFormProps) -> Element {
    let mut selected_category = use_signal(|| DocumentCategory::FacePhoto);
    let mut description = use_signal(|| String::new());
    let mut selected_file = use_signal(SelectedFile::default);
    let mut is_uploading = use_signal(|| false);
    let mut upload_error = use_signal(|| None::<String>);
    let mut upload_success = use_signal(|| false);

    let on_upload_success = props.on_upload_success;

    // ファイル選択ハンドラ
    let handle_file_select = move |evt: Event<FormData>| {
        spawn(async move {
            upload_error.set(None);
            upload_success.set(false);

            let files = evt.files();
            if files.is_empty() {
                selected_file.set(SelectedFile::default());
                return;
            }

            let file = &files[0];
            let name = file.name();
            let size = file.size();

            // ファイルサイズチェック（10MB制限）
            if size > 10 * 1024 * 1024 {
                upload_error.set(Some("ファイルサイズは10MB以下にしてください".to_string()));
                selected_file.set(SelectedFile::default());
                return;
            }

            // ファイル拡張子チェック
            let lower_name = name.to_lowercase();
            let allowed_extensions = [".jpg", ".jpeg", ".png", ".gif", ".pdf"];
            let is_valid_extension = allowed_extensions
                .iter()
                .any(|ext| lower_name.ends_with(ext));
            if !is_valid_extension {
                upload_error.set(Some(
                    "対応ファイル形式: JPG, JPEG, PNG, GIF, PDF".to_string(),
                ));
                selected_file.set(SelectedFile::default());
                return;
            }

            // ファイル内容を読み込み
            #[cfg(feature = "web")]
            {
                match file.read_bytes().await {
                    Ok(bytes) => {
                        let base64_data = encode_base64(&bytes);
                        selected_file.set(SelectedFile {
                            name,
                            size,
                            data: Some(base64_data),
                        });
                    }
                    Err(e) => {
                        upload_error.set(Some(format!("ファイル読み込みエラー: {}", e)));
                        selected_file.set(SelectedFile::default());
                    }
                }
            }

            #[cfg(not(feature = "web"))]
            {
                selected_file.set(SelectedFile {
                    name,
                    size,
                    data: None,
                });
            }
        });
    };

    // アップロードハンドラ
    let handle_upload = move |_| {
        let file = selected_file.read().clone();
        let category = *selected_category.read();
        let desc = description.read().clone();
        let employee_id = props.employee_id;

        spawn(async move {
            if file.data.is_none() {
                upload_error.set(Some("ファイルが選択されていません".to_string()));
                return;
            }

            is_uploading.set(true);
            upload_error.set(None);

            let upload_data = FileUpload {
                employee_id,
                category,
                filename: file.name.clone(),
                data: file.data.clone().unwrap(),
                description: if desc.is_empty() { None } else { Some(desc) },
                related_id: None,
            };

            // サーバー関数を直接呼び出し
            match crate::modules::employee_documents::upload_document(upload_data).await {
                Ok(_) => {
                    // 成功
                    upload_success.set(true);
                    selected_file.set(SelectedFile::default());
                    description.set(String::new());
                    // コールバックを呼び出し
                    on_upload_success.call(());
                }
                Err(e) => {
                    upload_error.set(Some(format!("アップロードエラー: {}", e)));
                }
            }

            is_uploading.set(false);
        });
    };

    // ファイルサイズをフォーマット
    let format_size = |size: u64| -> String {
        if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.1} KB", size as f64 / 1024.0)
        } else {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        }
    };

    let file = selected_file.read();
    let has_file = file.data.is_some();
    let is_loading = *is_uploading.read();

    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md",
            h3 { class: "text-lg font-semibold mb-4", "ドキュメントアップロード" }

            // 成功メッセージ
            if *upload_success.read() {
                div { class: "mb-4 p-4 bg-green-50 border border-green-200 rounded-lg",
                    p { class: "text-sm text-green-800",
                        "✅ ファイルが正常にアップロードされました"
                    }
                }
            }

            // エラーメッセージ
            if let Some(error) = upload_error.read().as_ref() {
                div { class: "mb-4 p-4 bg-red-50 border border-red-200 rounded-lg",
                    p { class: "text-sm text-red-800",
                        "❌ {error}"
                    }
                }
            }

            div { class: "space-y-4",
                // カテゴリ選択
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-2",
                        "ドキュメントの種類"
                    }
                    select {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                        disabled: is_loading,
                        onchange: move |evt| {
                            let value = evt.value();
                            let category = match value.as_str() {
                                "FACE_PHOTO" => DocumentCategory::FacePhoto,
                                "LICENSE_FRONT" => DocumentCategory::LicenseFront,
                                "LICENSE_BACK" => DocumentCategory::LicenseBack,
                                "QUALIFICATION_CERT" => DocumentCategory::QualificationCert,
                                "VEHICLE_INSPECTION" => DocumentCategory::VehicleInspection,
                                "HEALTH_CHECKUP" => DocumentCategory::HealthCheckup,
                                "APTITUDE_CHECKUP" => DocumentCategory::AptitudeCheckup,
                                _ => DocumentCategory::Other,
                            };
                            selected_category.set(category);
                        },
                        option { value: "FACE_PHOTO", "顔写真" }
                        option { value: "LICENSE_FRONT", "免許証（表）" }
                        option { value: "LICENSE_BACK", "免許証（裏）" }
                        option { value: "QUALIFICATION_CERT", "資格証明書" }
                        option { value: "VEHICLE_INSPECTION", "車検証" }
                        option { value: "HEALTH_CHECKUP", "健康診断書" }
                        option { value: "APTITUDE_CHECKUP", "適性診断書" }
                        option { value: "OTHER", "その他" }
                    }
                }

                // ファイル選択
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-2",
                        "ファイルを選択"
                    }
                    input {
                        r#type: "file",
                        accept: ".jpg,.jpeg,.png,.gif,.pdf",
                        disabled: is_loading,
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100",
                        onchange: handle_file_select,
                    }
                    p { class: "mt-1 text-xs text-gray-500",
                        "対応形式: 画像（JPG, PNG, GIF）、PDF（最大10MB）"
                    }
                }

                // 選択されたファイル情報
                if has_file {
                    div { class: "p-3 bg-gray-50 rounded-md",
                        p { class: "text-sm text-gray-700",
                            "📄 {file.name} ({format_size(file.size)})"
                        }
                    }
                }

                // 説明
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-2",
                        "説明・メモ（任意）"
                    }
                    textarea {
                        class: "w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                        rows: "3",
                        disabled: is_loading,
                        placeholder: "ドキュメントに関するメモを入力してください...",
                        value: "{description()}",
                        oninput: move |evt| {
                            description.set(evt.value());
                        },
                    }
                }

                // アップロードボタン
                div { class: "flex justify-end",
                    button {
                        r#type: "button",
                        class: if has_file && !is_loading {
                            "px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        } else {
                            "px-6 py-2 bg-gray-400 text-white rounded-md cursor-not-allowed"
                        },
                        disabled: !has_file || is_loading,
                        onclick: handle_upload,
                        if is_loading {
                            "アップロード中..."
                        } else {
                            "アップロード"
                        }
                    }
                }
            }
        }
    }
}
