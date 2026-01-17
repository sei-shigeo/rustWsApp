# 和清商事 従業員管理システム

Dioxus フルスタックフレームワークを使用した運送会社向け従業員台帳＋運転者台帳管理アプリケーション

## 🎯 プロジェクト状況: 70% 完成

**最終更新**: 2025年1月9日  
**次のタスク**: ファイルアップロード機能の完成 → [TODO.md](TODO.md) を参照

## 📚 ドキュメント

- **[TODO.md](TODO.md)** - 次に実装するタスクと詳細な技術情報
- **[PROJECT_STATUS.md](PROJECT_STATUS.md)** - プロジェクト全体の状況サマリー
- **[docs/S3_SETUP.md](docs/S3_SETUP.md)** - AWS S3セットアップガイド

## ✨ 主要機能

### 完成済み
- ✅ 従業員管理（CRUD）
- ✅ 住所・緊急連絡先・銀行口座管理
- ✅ 運転免許証・資格証管理
- ✅ 部署・役職・営業所履歴管理
- ✅ 健康診断・適性診断・保険証履歴
- ✅ 指導教育履歴
- ✅ AWS S3統合（バックエンド完成）

### 実装中
- 🚧 ドキュメント管理システム（ファイルアップロード機能）
  - ✅ サーバー側API
  - ✅ データベーススキーマ
  - ✅ S3バケット作成
  - ⏳ クライアント側ファイル読み込み

## プロジェクト構造

```
rustWsApp/
├── assets/                      # 静的アセット（画像、CSS等）
│   ├── favicon.ico
│   └── tailwind.css
├── migrations/                  # データベースマイグレーション
├── src/
│   ├── main.rs                 # エントリーポイント
│   ├── db.rs                   # データベース接続管理
│   ├── routes.rs               # ルーティング設定
│   ├── components/             # 共通UIコンポーネント
│   │   ├── mod.rs
│   │   ├── icon.rs            # アイコンコンポーネント
│   │   ├── nav.rs             # ナビゲーション
│   │   └── search_bar.rs      # 検索バー
│   └── modules/               # 機能モジュール
│       ├── mod.rs
│       ├── employees/         # 従業員管理モジュール
│       │   ├── mod.rs
│       │   ├── models.rs      # データモデル
│       │   ├── handlers.rs    # サーバー関数（API）
│       │   ├── repository.rs  # データベースアクセス層
│       │   ├── validation.rs  # バリデーションロジック
│       │   ├── page.rs        # 一覧ページ
│       │   └── components/    # 従業員関連コンポーネント
│       ├── employee_documents/# ドキュメント管理（NEW! 🎉）
│       │   ├── mod.rs
│       │   ├── models.rs      # ドキュメントモデル
│       │   ├── handlers.rs    # アップロードAPI
│       │   ├── repository.rs  # DB操作
│       │   └── components/    # アップロードフォーム
│       ├── s3_service/        # AWS S3統合（NEW! 🎉）
│       │   ├── mod.rs
│       │   └── service.rs     # S3操作
│       ├── addresses/         # 住所管理
│       ├── bank_accounts/     # 銀行口座管理
│       ├── licenses/          # 運転免許証管理
│       ├── qualifications/    # 資格管理
│       ├── department_position_history/ # 配属履歴
│       ├── health_checkup_history/      # 健康診断履歴
│       ├── aptitude_checkup_history/    # 適性診断履歴
│       ├── insurance_history/           # 保険証履歴
│       ├── guidance_education_history/  # 指導教育履歴
│       ├── education_history/ # 学歴管理
│       ├── emergency_contacts/# 緊急連絡先管理
│       ├── employment_history/# 職歴管理
│       └── residence_cards/   # 在留カード管理
├── docs/                      # ドキュメント
│   └── S3_SETUP.md           # AWS S3セットアップガイド
├── Cargo.toml
├── Dioxus.toml
├── TODO.md                    # タスクリスト
├── PROJECT_STATUS.md          # プロジェクト状況
└── docker-compose.yml
```

## アーキテクチャ

### レイヤー構造

このプロジェクトは3層アーキテクチャを採用しています：

1. **プレゼンテーション層** (`page.rs`, `components/`)
   - ユーザーインターフェース
   - Dioxusコンポーネント

2. **ビジネスロジック層** (`handlers.rs`, `validation.rs`)
   - サーバー関数（`#[server]`マクロ）
   - バリデーション
   - エラーハンドリング

3. **データアクセス層** (`repository.rs`, `models.rs`)
   - データベース操作
   - SQLクエリ
   - データモデル定義

### データベース接続管理

`db.rs`では遅延初期化パターンを使用しています：

```rust
static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();
```

- 初回アクセス時に自動的に接続プールを確立
- アプリケーション起動時のデータベース接続エラーを回避
- スレッドセーフな静的変数で全体から利用可能

## リファクタリング内容

### 2025年1月実施

#### 1. データベース接続の簡潔化
- `OnceLock`と`OnceCell`の併用を`OnceCell`のみに統一
- コードが約30%削減され、可読性が向上

#### 2. エラーハンドリングの統一
- `db_error_to_server_error()`ヘルパー関数を追加
- 重複エラー処理ロジックを一箇所に集約
- 重複キー制約違反を分かりやすいメッセージに変換

#### 3. バリデーションの強化
- `validate_postal_code()` - 郵便番号検証
- `validate_address_field()` - 住所フィールド検証
- 包括的なユニットテスト追加

#### 4. Repository層の改善
- `pool()`ヘルパーメソッドでコード重複を削減
- 一貫性のあるドキュメントコメント追加
- すべてのメソッドに`///`形式のdocコメント

#### 5. UI定数化
- 繰り返し使用されるCSSクラスを定数化
- `HEADER_CLASS`, `CONTENT_CLASS`等
- メンテナンス性と一貫性の向上

## 🚀 クイックスタート

### 必要なツール

- Rust 1.70+ (最新安定版推奨)
- PostgreSQL 14+
- Docker & Docker Compose (オプション)
- Node.js & npm (Tailwind CSS用)
- Dioxus CLI
- AWS CLI (ドキュメント管理機能を使用する場合)

### インストール

```bash
# Dioxus CLIのインストール
cargo install dioxus-cli

# プロジェクトのクローン（または既存のディレクトリに移動）
cd rustWsApp
```

### データベースのセットアップ

```bash
# PostgreSQLデータベースの作成
createdb rust_ws_app

# または、Dockerコンテナの起動
docker-compose up -d

# マイグレーションの実行（15ファイル）
sqlx migrate run
```

### 環境変数

`.env`ファイルを作成し、以下を設定：

```env
# データベース接続
DATABASE_URL=postgresql://username:password@localhost:5432/rust_ws_app

# AWS S3設定（ドキュメント管理機能を使用する場合）
AWS_REGION=ap-northeast-1
S3_BUCKET_NAME=rust-ws-app-documents
# AWS認証情報は ~/.aws/credentials に設定
```

### AWS S3のセットアップ（オプション）

ドキュメント管理機能を使用する場合は、[docs/S3_SETUP.md](docs/S3_SETUP.md) を参照してください。

### Tailwind CSS

自動Tailwind（Dioxus 0.7+）が有効です。`dx serve`を実行するだけで使用できます。

カスタマイズが必要な場合：

```bash
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

### アプリケーションの起動

```bash
# Webアプリとして起動（推奨）
dx serve --web

# ブラウザでアクセス
# http://127.0.0.1:8080/

# デスクトップアプリとして起動
dx serve --platform desktop

# リリースビルド
dx build --release
```

サーバーが起動すると以下のログが表示されます：
```
🚀 Starting server with lazy database initialization...
📦 S3 client initialized
🎉 Server listening on: http://127.0.0.1:XXXXX
```

## 🛠️ 技術スタック

### バックエンド
- **Rust** - 安全で高速なシステムプログラミング言語
- **Axum 0.8** - Webフレームワーク
- **SQLx 0.8** - 非同期SQLデータベースドライバ
- **PostgreSQL** - リレーショナルデータベース
- **AWS SDK for Rust** - S3統合

### フロントエンド
- **Dioxus 0.7** - Rustフルスタックフレームワーク
- **TailwindCSS** - ユーティリティファーストCSS

### インフラ
- **AWS S3** - ドキュメントストレージ
- **AWS MCP Server** - 開発支援ツール

## コーディング規約

### ファイル構成

各機能モジュールは以下の構造に従います：

```
module_name/
├── mod.rs           # モジュールの公開インターフェース
├── models.rs        # データ構造定義
├── handlers.rs      # サーバー関数（API）
├── repository.rs    # DB操作（#[cfg(feature = "server")]）
├── validation.rs    # バリデーション
├── page.rs          # メインページコンポーネント
└── components/      # 関連UIコンポーネント
```

### 命名規則

- **関数**: `snake_case`
- **構造体**: `PascalCase`
- **定数**: `SCREAMING_SNAKE_CASE`
- **CSS定数**: `*_CLASS`サフィックス

### バリデーション

- クライアント側とサーバー側で二重検証
- バリデーションロジックは`validation.rs`に集約
- すべてのバリデーション関数にユニットテストを記述

### エラーハンドリング

- `Result<T, ServerFnError>`を一貫して使用
- データベースエラーは`db_error_to_server_error()`で変換
- ユーザーフレンドリーなエラーメッセージを提供

## テスト

```bash
# すべてのテストを実行
cargo test

# 特定のモジュールのテストを実行
cargo test employees::validation

# カバレッジレポート生成
cargo tarpaulin --out Html
```

## 🎯 次のステップ

このプロジェクトを引き継ぐ、または開発を続ける場合：

1. **[TODO.md](TODO.md)** を読んで次のタスクを確認
2. **[PROJECT_STATUS.md](PROJECT_STATUS.md)** で全体像を把握
3. ファイルアップロード機能の実装から始める
4. `src/modules/employee_documents/components/document_upload_form.rs` を編集

### 現在の課題
- クライアント側のファイル読み込みAPI実装
- Dioxus 0.7の`evt.files()`からバイト列を取得する方法
- WASM環境での条件コンパイル

詳細は **[TODO.md](TODO.md)** を参照してください。

## デプロイ

```bash
# プロダクションビルド
dx build --release

# Dockerイメージのビルド（実装予定）
docker build -t rustws-app .

# コンテナの起動
docker run -p 8080:8080 rustws-app
```

## 📊 プロジェクト統計

- **完成度**: 70%
- **モジュール数**: 12+
- **サーバー関数**: 50+ API
- **データベーステーブル**: 20+
- **コード行数**: 約9,000行（Rust + SQL + ドキュメント）

## 🤝 貢献

このプロジェクトへの貢献を歓迎します！

1. フォークする
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add some amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

## ライセンス

© 2025 和清商事
