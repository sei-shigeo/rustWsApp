//! ドキュメント一覧表示コンポーネント

use crate::modules::employee_documents::models::{DocumentCategory, EmployeeDocument};
use crate::modules::employee_documents::{delete_document, get_employee_documents};
use dioxus::prelude::*;

/// ドキュメント一覧のプロパティ
#[derive(Props, Clone, PartialEq)]
pub struct DocumentListProps {
    /// 従業員ID
    pub employee_id: i32,
    /// 再読み込みトリガー（親コンポーネントからの更新通知用）
    #[props(default = 0)]
    pub refresh_trigger: i32,
}

/// カテゴリフィルターの選択肢
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CategoryFilter {
    All,
    Category(DocumentCategory),
}

impl CategoryFilter {
    fn display_name(&self) -> &'static str {
        match self {
            CategoryFilter::All => "すべて",
            CategoryFilter::Category(cat) => cat.display_name(),
        }
    }
}

/// ドキュメント一覧表示コンポーネント
#[component]
pub fn DocumentList(props: DocumentListProps) -> Element {
    let mut documents = use_signal(Vec::<EmployeeDocument>::new);
    let mut is_loading = use_signal(|| true);
    let mut error = use_signal(|| None::<String>);
    let mut category_filter = use_signal(|| CategoryFilter::All);
    let mut delete_confirm_id = use_signal(|| None::<i32>);
    let mut is_deleting = use_signal(|| false);

    // 初回読み込みと更新トリガー時にドキュメントを取得
    use_effect(move || {
        let employee_id = props.employee_id;
        let _refresh = props.refresh_trigger; // 依存関係として使用
        spawn(async move {
            is_loading.set(true);
            error.set(None);

            match get_employee_documents(employee_id).await {
                Ok(docs) => {
                    documents.set(docs);
                }
                Err(e) => {
                    error.set(Some(format!("ドキュメントの取得に失敗しました: {}", e)));
                }
            }

            is_loading.set(false);
        });
    });

    // 削除ハンドラ
    let handle_delete = move |id: i32| {
        spawn(async move {
            is_deleting.set(true);

            match delete_document(id).await {
                Ok(_) => {
                    // 一覧から削除
                    documents.write().retain(|doc| doc.id != id);
                    delete_confirm_id.set(None);
                }
                Err(e) => {
                    error.set(Some(format!("削除に失敗しました: {}", e)));
                }
            }

            is_deleting.set(false);
        });
    };

    // フィルタリングされたドキュメント
    let filtered_docs: Vec<EmployeeDocument> = {
        let docs = documents.read();
        let filter = *category_filter.read();
        match filter {
            CategoryFilter::All => docs.clone(),
            CategoryFilter::Category(cat) => docs
                .iter()
                .filter(|doc| doc.category.as_ref() == Some(&cat))
                .cloned()
                .collect(),
        }
    };

    // ファイルサイズをフォーマット
    let format_size = |size: Option<i64>| -> String {
        match size {
            Some(s) if s < 1024 => format!("{} B", s),
            Some(s) if s < 1024 * 1024 => format!("{:.1} KB", s as f64 / 1024.0),
            Some(s) => format!("{:.1} MB", s as f64 / (1024.0 * 1024.0)),
            None => "不明".to_string(),
        }
    };

    // 日付をフォーマット
    let format_date =
        |dt: chrono::DateTime<chrono::Utc>| -> String { dt.format("%Y/%m/%d %H:%M").to_string() };

    rsx! {
        div { class: "bg-white p-6 rounded-lg shadow-md",
            // ヘッダー
            div { class: "flex justify-between items-center mb-4",
                h3 { class: "text-lg font-semibold", "ドキュメント一覧" }

                // カテゴリフィルター
                select {
                    class: "px-3 py-1 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500",
                    onchange: move |evt| {
                        let value = evt.value();
                        let filter = match value.as_str() {
                            "ALL" => CategoryFilter::All,
                            "FACE_PHOTO" => CategoryFilter::Category(DocumentCategory::FacePhoto),
                            "LICENSE_FRONT" => CategoryFilter::Category(DocumentCategory::LicenseFront),
                            "LICENSE_BACK" => CategoryFilter::Category(DocumentCategory::LicenseBack),
                            "QUALIFICATION_CERT" => CategoryFilter::Category(DocumentCategory::QualificationCert),
                            "VEHICLE_INSPECTION" => CategoryFilter::Category(DocumentCategory::VehicleInspection),
                            "HEALTH_CHECKUP" => CategoryFilter::Category(DocumentCategory::HealthCheckup),
                            "APTITUDE_CHECKUP" => CategoryFilter::Category(DocumentCategory::AptitudeCheckup),
                            "OTHER" => CategoryFilter::Category(DocumentCategory::Other),
                            _ => CategoryFilter::All,
                        };
                        category_filter.set(filter);
                    },
                    option { value: "ALL", "すべて" }
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

            // エラーメッセージ
            if let Some(err) = error.read().as_ref() {
                div { class: "mb-4 p-4 bg-red-50 border border-red-200 rounded-lg",
                    p { class: "text-sm text-red-800", "❌ {err}" }
                }
            }

            // ローディング状態
            if *is_loading.read() {
                div { class: "flex justify-center py-8",
                    p { class: "text-gray-500", "読み込み中..." }
                }
            } else if filtered_docs.is_empty() {
                // 空状態
                div { class: "text-center py-8",
                    p { class: "text-gray-500", "ドキュメントがありません" }
                }
            } else {
                // ドキュメントリスト
                div { class: "space-y-3",
                    for doc in filtered_docs.iter() {
                        div {
                            key: "{doc.id}",
                            class: "border border-gray-200 rounded-lg p-4 hover:bg-gray-50 transition-colors",

                            div { class: "flex items-start justify-between",
                                // ドキュメント情報
                                div { class: "flex-1",
                                    div { class: "flex items-center gap-2 mb-1",
                                        // ファイルアイコン
                                        span { class: "text-xl",
                                            if doc.mime_type.as_ref().map(|m| m.starts_with("image/")).unwrap_or(false) {
                                                "🖼️"
                                            } else if doc.mime_type.as_ref().map(|m| m == "application/pdf").unwrap_or(false) {
                                                "📄"
                                            } else {
                                                "📎"
                                            }
                                        }
                                        // ファイル名
                                        span { class: "font-medium text-gray-900",
                                            "{doc.filename}"
                                        }
                                        // サイズ
                                        span { class: "text-sm text-gray-500",
                                            "({format_size(doc.file_size)})"
                                        }
                                    }

                                    // カテゴリとアップロード日時
                                    div { class: "flex items-center gap-3 text-sm text-gray-600",
                                        if let Some(cat) = &doc.category {
                                            span { class: "bg-blue-100 text-blue-800 px-2 py-0.5 rounded-full text-xs",
                                                "{cat.display_name()}"
                                            }
                                        }
                                        span { "{format_date(doc.uploaded_at)}" }
                                    }

                                    // 説明
                                    if let Some(desc) = &doc.description {
                                        if !desc.is_empty() {
                                            p { class: "mt-2 text-sm text-gray-600",
                                                "{desc}"
                                            }
                                        }
                                    }
                                }

                                // アクションボタン
                                div { class: "flex items-center gap-2 ml-4",
                                    // ダウンロードボタン
                                    if let Some(url) = &doc.s3_url {
                                        a {
                                            href: "{url}",
                                            target: "_blank",
                                            class: "px-3 py-1 text-sm bg-gray-100 text-gray-700 rounded hover:bg-gray-200 transition-colors",
                                            "ダウンロード"
                                        }
                                    }

                                    // 削除ボタン
                                    button {
                                        class: "px-3 py-1 text-sm bg-red-100 text-red-700 rounded hover:bg-red-200 transition-colors",
                                        onclick: {
                                            let doc_id = doc.id;
                                            move |_| delete_confirm_id.set(Some(doc_id))
                                        },
                                        "削除"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 件数表示
            if !*is_loading.read() && !filtered_docs.is_empty() {
                div { class: "mt-4 text-sm text-gray-500 text-right",
                    "{filtered_docs.len()}件のドキュメント"
                }
            }
        }

        // 削除確認ダイアログ
        if let Some(id) = *delete_confirm_id.read() {
            div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                div { class: "bg-white rounded-lg p-6 max-w-sm w-full mx-4 shadow-xl",
                    h4 { class: "text-lg font-semibold mb-4", "削除の確認" }
                    p { class: "text-gray-600 mb-6",
                        "このドキュメントを削除しますか？この操作は取り消せません。"
                    }
                    div { class: "flex justify-end gap-3",
                        button {
                            class: "px-4 py-2 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors",
                            disabled: *is_deleting.read(),
                            onclick: move |_| delete_confirm_id.set(None),
                            "キャンセル"
                        }
                        button {
                            class: "px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 transition-colors disabled:opacity-50",
                            disabled: *is_deleting.read(),
                            onclick: move |_| handle_delete(id),
                            if *is_deleting.read() {
                                "削除中..."
                            } else {
                                "削除"
                            }
                        }
                    }
                }
            }
        }
    }
}
