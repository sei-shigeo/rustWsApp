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
    let mut is_dragging = use_signal(|| false);
    let mut upload_progress = use_signal(|| 0u8); // 0-100の進捗率

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
                let read_result = file.read_bytes().await;
                match read_result {
                    Ok(bytes) => {
                        let bytes_vec: Vec<u8> = bytes.to_vec();
                        let base64_data = encode_base64(&bytes_vec);
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

    // ドラッグ＆ドロップハンドラ（視覚的フィードバックのみ）
    let handle_drag_over = move |evt: Event<DragData>| {
        evt.prevent_default();
        is_dragging.set(true);
    };

    let handle_drag_leave = move |_evt: Event<DragData>| {
        is_dragging.set(false);
    };

    let handle_drop = move |evt: Event<DragData>| {
        evt.prevent_default();
        is_dragging.set(false);
        // DragDataからのファイル取得はDioxus 0.7では現在サポートされていないため、
        // クリックでファイル選択を使用してください
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
            upload_progress.set(0);

            // 進捗シミュレーション: ファイル準備中
            upload_progress.set(10);

            let upload_data = FileUpload {
                employee_id,
                category,
                filename: file.name.clone(),
                data: file.data.clone().unwrap(),
                description: if desc.is_empty() { None } else { Some(desc) },
                related_id: None,
            };

            // 進捗シミュレーション: アップロード開始
            upload_progress.set(30);

            // サーバー関数を直接呼び出し
            match crate::modules::employee_documents::upload_document(upload_data).await {
                Ok(_) => {
                    // 進捗: 完了
                    upload_progress.set(100);
                    // 成功
                    upload_success.set(true);
                    selected_file.set(SelectedFile::default());
                    description.set(String::new());
                    // コールバックを呼び出し
                    on_upload_success.call(());
                }
                Err(e) => {
                    upload_progress.set(0);
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

                // ファイル選択（ドラッグ＆ドロップ対応）
                div {
                    label { class: "block text-sm font-medium text-gray-700 mb-2",
                        "ファイルを選択"
                    }

                    // 隠しファイル入力
                    input {
                        r#type: "file",
                        id: "file-input",
                        accept: ".jpg,.jpeg,.png,.gif,.pdf",
                        disabled: is_loading,
                        class: "hidden",
                        onchange: handle_file_select,
                    }

                    // ドラッグ＆ドロップエリア（クリックでもファイル選択可能）
                    label {
                        r#for: "file-input",
                        class: if *is_dragging.read() {
                            "border-2 border-dashed border-blue-500 bg-blue-50 rounded-lg p-8 text-center cursor-pointer transition-colors block"
                        } else if has_file {
                            "border-2 border-dashed border-green-400 bg-green-50 rounded-lg p-8 text-center cursor-pointer transition-colors block"
                        } else {
                            "border-2 border-dashed border-gray-300 rounded-lg p-8 text-center cursor-pointer transition-colors hover:border-blue-400 hover:bg-blue-50 block"
                        },
                        ondragover: handle_drag_over,
                        ondragleave: handle_drag_leave,
                        ondrop: handle_drop,

                        if has_file {
                            // 選択されたファイル情報
                            div { class: "text-green-700",
                                p { class: "text-xl mb-2", "✅ ファイルが選択されました" }
                                p { class: "text-base font-medium",
                                    "📄 {file.name}"
                                }
                                p { class: "text-sm text-green-600 mt-1",
                                    "({format_size(file.size)})"
                                }
                                p { class: "text-sm text-gray-500 mt-3",
                                    "別のファイルを選択するにはここをクリックするか、ドラッグ＆ドロップしてください"
                                }
                            }
                        } else if *is_dragging.read() {
                            // ドラッグ中の表示
                            div { class: "text-blue-600",
                                p { class: "text-2xl mb-2", "📥" }
                                p { class: "text-lg font-medium mb-1", "ここにドロップ" }
                                p { class: "text-sm", "ファイルをドロップしてアップロード" }
                            }
                        } else {
                            // 通常の表示
                            div { class: "text-gray-500",
                                p { class: "text-3xl mb-3", "📁" }
                                p { class: "text-lg font-medium mb-1", "ファイルをドラッグ＆ドロップ" }
                                p { class: "text-sm mb-2", "または" }
                                p { class: "text-base text-blue-600 font-medium", "クリックしてファイルを選択" }
                            }
                        }
                    }

                    p { class: "mt-2 text-xs text-gray-500 text-center",
                        "対応形式: 画像（JPG, PNG, GIF）、PDF（最大10MB）"
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

                // アップロード進捗表示
                if is_loading {
                    div { class: "space-y-2",
                        // 進捗バー
                        div { class: "w-full bg-gray-200 rounded-full h-3 overflow-hidden",
                            div {
                                class: "bg-blue-600 h-3 rounded-full transition-all duration-300 ease-out",
                                style: "width: {upload_progress()}%",
                            }
                        }
                        // 進捗テキスト
                        div { class: "flex justify-between text-sm text-gray-600",
                            span {
                                if *upload_progress.read() < 30 {
                                    "ファイル準備中..."
                                } else if *upload_progress.read() < 100 {
                                    "S3にアップロード中..."
                                } else {
                                    "完了!"
                                }
                            }
                            span { "{upload_progress()}%" }
                        }
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
