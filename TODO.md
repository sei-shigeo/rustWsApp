# TODO - 従業員台帳＋運転者台帳アプリ

## 🎉 最近完成した機能

### ドキュメント管理システム（2025年1月17日完成）
- ✅ ファイルアップロード機能
  - Dioxus 0.7.3の`FileData::read_bytes()`を使用
  - Base64エンコードでサーバーに送信
  - ファイルサイズチェック（最大10MB）
  - ファイル形式チェック（JPG, PNG, GIF, PDF）
  - カテゴリ選択（顔写真、免許証、資格証明書など）
- ✅ ドキュメント一覧表示
  - カテゴリフィルター機能
  - ファイル情報表示（サイズ、アップロード日時）
  - アップロード成功時に自動更新
- ✅ ダウンロード機能（S3 URLへの直接リンク）
- ✅ 削除機能（確認ダイアログ付き、S3とDBから同時削除）

---

## 🟡 次のステップ（優先度：中）

### 1. 画像プレビュー機能
**場所**: `src/modules/employee_documents/components/`

**実装内容**:
- モーダルウィンドウで画像を大きく表示
- PDFのプレビュー（iframe使用）
- 画像の拡大・縮小

### 2. ドラッグ＆ドロップ対応
**実装内容**:
- ファイルをドロップエリアにドラッグ＆ドロップでアップロード
- 複数ファイルの同時選択

### 3. アップロード進捗表示
**実装内容**:
- 大きいファイルのアップロード時にプログレスバー表示

---

## 🟢 将来の実装（優先度：低）

### 4. 高度な機能

#### 4.1 画像の自動リサイズ
- アップロード前にクライアント側で画像を圧縮
- Canvas APIを使用して画像リサイズ
- 目標: 1920x1080以下、品質80%

#### 4.2 署名付きURL
- セキュアなファイルアクセス
- 期限付きダウンロードURL
- S3 Presigned URLの生成

#### 4.3 ドキュメント検索機能
- ファイル名での検索
- 説明文での検索
- 日付範囲での絞り込み

---

## 📋 既に完成している機能

### ✅ AWS S3統合
- [x] S3サービスモジュール（`src/modules/s3_service/`）
  - [x] `upload_file()` - S3へのファイルアップロード
  - [x] `download_file()` - S3からのファイルダウンロード
  - [x] `delete_file()` - S3からのファイル削除
  - [x] `generate_s3_key()` - S3キーの自動生成
  - [x] `guess_mime_type()` - MIMEタイプの推測

### ✅ データベース
- [x] マイグレーション実行完了（17ファイル）
  - [x] `employee_documents` テーブル
  - [x] S3関連カラム追加（category, s3_key, s3_url, related_id, uploaded_at）
  - [x] NOT NULL制約の修正（document_type_id, file_path）
- [x] リポジトリ層（`src/modules/employee_documents/repository.rs`）

### ✅ サーバー関数
- [x] `upload_document()` - ファイルアップロード
- [x] `get_document()` - ドキュメント取得
- [x] `get_employee_documents()` - 従業員のドキュメント一覧
- [x] `get_documents_by_category()` - カテゴリ別取得
- [x] `update_document()` - ドキュメント更新
- [x] `delete_document()` - ドキュメント削除
- [x] `get_document_download_url()` - ダウンロードURL取得
- [x] `count_employee_documents()` - ドキュメント数取得

### ✅ UIコンポーネント
- [x] `DocumentUploadForm` - アップロードフォーム
- [x] `DocumentList` - ドキュメント一覧
- [x] `DocumentManagementSection` - 統合セクション（従業員詳細ページ）

### ✅ サーバー設定
- [x] ボディサイズ制限を15MBに設定（`DefaultBodyLimit`）

---

## 🔧 技術スタック

### バックエンド
- Rust
- Axum 0.8 (Webフレームワーク)
- SQLx 0.8 (データベースアクセス)
- PostgreSQL
- AWS SDK for Rust

### フロントエンド
- Dioxus 0.7.3 (フルスタックフレームワーク)
- TailwindCSS

### インフラ
- AWS S3 (ファイルストレージ)
- PostgreSQL (データベース)

---

## 📝 開発メモ

### 依存関係
```toml
# 主要な依存関係
dioxus = { version = "0.7", features = ["fullstack", "router"] }
aws-sdk-s3 = "1"
aws-config = "1"
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio", "chrono"] }
base64 = "0.22"
tower = "0.5"

# Web専用
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["File", "FileReader", "HtmlInputElement"] }
```

### 環境変数（.env）
```env
DATABASE_URL=postgresql://username:password@localhost/rust_ws_app
AWS_REGION=ap-northeast-1
S3_BUCKET_NAME=rust-ws-app-documents
```

### サーバー起動コマンド
```bash
dx serve --web
```

### マイグレーション実行
```bash
sqlx migrate run
```

---

## 📚 参考資料

### Dioxus
- 公式ドキュメント: https://dioxuslabs.com/learn/0.7/
- ファイルAPI: `FileData::read_bytes()` でバイト列を取得

### AWS S3
- AWS SDK for Rust: https://github.com/awslabs/aws-sdk-rust
- S3セットアップガイド: `docs/S3_SETUP.md`

---

**最終更新**: 2025年1月17日
**作成者**: 従業員台帳開発チーム
