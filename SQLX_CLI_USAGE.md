# SQLx CLI の主な使い方

## インストール

```bash
# 全データベースサポート版
cargo install sqlx-cli

# SQLite のみ（推奨）
cargo install sqlx-cli --no-default-features --features sqlite
```

## 初期設定

### 1. 環境変数の設定

プロジェクトルートに `.env` ファイルを作成:

```env
DATABASE_URL=sqlite:database.db
```

### 2. データベースの作成

```bash
sqlx database create
```

## マイグレーション

### マイグレーションファイルの作成

```bash
sqlx migrate add create_employees_table
```

`migrations` フォルダに新しいファイルが作成されます:

```sql
CREATE TABLE employees (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### マイグレーションの実行

```bash
sqlx migrate run
```

### マイグレーションの取り消し

```bash
sqlx migrate revert
```

## オフラインモード（重要）

コンパイル時にデータベース接続なしでクエリを検証するために使用:

```bash
sqlx prepare
```

生成された `sqlx-data.json` をコミットに含めます。

### `Cargo.toml` の設定

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "macros"] }
tokio = { version = "1", features = ["full"] }
```

### CI/CD での使用

```toml
[profile.release]
# オフラインモードを有効化
sqlx-macros = { offline = true }
```

## よく使うコマンド

```bash
# データベース削除
sqlx database drop

# マイグレーション状態確認
sqlx migrate info

# 全マイグレーションを取り消し
sqlx migrate revert --target-version 0

# オフラインデータ削除（再生成前）
rm sqlx-data.json
```

## トラブルシューティング

### エラー: "DATABASE_URL must be set"

`.env` ファイルを作成し、`DATABASE_URL` を設定してください。

### エラー: コンパイル時にクエリ検証が失敗

```bash
# オフラインデータを再生成
sqlx prepare
```
