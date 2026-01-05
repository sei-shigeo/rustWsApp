#!/bin/bash
# データベースリセットスクリプト
# 警告: このスクリプトはすべてのデータを削除します！

set -e

# カラー定義
COLOR_RED='\033[0;31m'
COLOR_GREEN='\033[0;32m'
COLOR_YELLOW='\033[1;33m'
COLOR_BLUE='\033[0;34m'
COLOR_RESET='\033[0m'

DB_NAME="rustWsApp"
DB_USER="postgres"
DB_PASSWORD="postgres"
DB_HOST="localhost"
DB_PORT="5432"

echo -e "${COLOR_YELLOW}========================================${COLOR_RESET}"
echo -e "${COLOR_YELLOW}データベースリセット${COLOR_RESET}"
echo -e "${COLOR_YELLOW}========================================${COLOR_RESET}"
echo ""
echo -e "${COLOR_RED}警告: このスクリプトは以下の操作を行います:${COLOR_RESET}"
echo -e "${COLOR_RED}  1. データベース '${DB_NAME}' への全接続を切断${COLOR_RESET}"
echo -e "${COLOR_RED}  2. データベース '${DB_NAME}' を削除${COLOR_RESET}"
echo -e "${COLOR_RED}  3. 新しいデータベース '${DB_NAME}' を作成${COLOR_RESET}"
echo -e "${COLOR_RED}  4. マイグレーションを実行${COLOR_RESET}"
echo ""
read -p "続行しますか？ (yes/no): " -r
echo
if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
    echo -e "${COLOR_YELLOW}キャンセルされました${COLOR_RESET}"
    exit 0
fi

echo ""
echo -e "${COLOR_BLUE}[1/4] アクティブな接続を確認...${COLOR_RESET}"

# PostgreSQLに接続してアクティブな接続を切断
export PGPASSWORD="${DB_PASSWORD}"
psql -U "${DB_USER}" -h "${DB_HOST}" -p "${DB_PORT}" -d postgres -c "
SELECT pg_terminate_backend(pg_stat_activity.pid)
FROM pg_stat_activity
WHERE pg_stat_activity.datname = '${DB_NAME}'
  AND pid <> pg_backend_pid();
" 2>/dev/null || echo "  接続の切断をスキップ（既に切断されている可能性があります）"

echo -e "${COLOR_GREEN}✓ 接続を切断しました${COLOR_RESET}"
echo ""

echo -e "${COLOR_BLUE}[2/4] 既存のデータベースを削除...${COLOR_RESET}"

# sqlxコマンドを使用してデータベースを削除
sqlx database drop --database-url "postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}" -y 2>/dev/null || echo "  データベースが存在しない可能性があります"

echo -e "${COLOR_GREEN}✓ データベースを削除しました${COLOR_RESET}"
echo ""

echo -e "${COLOR_BLUE}[3/4] 新しいデータベースを作成...${COLOR_RESET}"

# sqlxコマンドを使用してデータベースを作成
sqlx database create --database-url "postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

echo -e "${COLOR_GREEN}✓ データベースを作成しました${COLOR_RESET}"
echo ""

echo -e "${COLOR_BLUE}[4/4] マイグレーションを実行...${COLOR_RESET}"

# マイグレーションを実行
sqlx migrate run --database-url "postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

echo -e "${COLOR_GREEN}✓ マイグレーションを実行しました${COLOR_RESET}"
echo ""

# マイグレーション履歴を表示
echo -e "${COLOR_BLUE}マイグレーション履歴:${COLOR_RESET}"
export PGPASSWORD="${DB_PASSWORD}"
psql -U "${DB_USER}" -h "${DB_HOST}" -p "${DB_PORT}" -d "${DB_NAME}" -c "
SELECT
    version,
    description,
    installed_on,
    execution_time
FROM _sqlx_migrations
ORDER BY version;
" 2>/dev/null || echo "マイグレーション履歴の取得に失敗しました"

echo ""
echo -e "${COLOR_GREEN}========================================${COLOR_RESET}"
echo -e "${COLOR_GREEN}データベースリセット完了！${COLOR_RESET}"
echo -e "${COLOR_GREEN}========================================${COLOR_RESET}"
echo ""
echo "データベース情報:"
echo "  名前: ${DB_NAME}"
echo "  ホスト: ${DB_HOST}:${DB_PORT}"
echo "  接続URL: postgres://${DB_USER}:***@${DB_HOST}:${DB_PORT}/${DB_NAME}"
echo ""

# テーブル一覧を表示
echo -e "${COLOR_BLUE}作成されたテーブル:${COLOR_RESET}"
export PGPASSWORD="${DB_PASSWORD}"
psql -U "${DB_USER}" -h "${DB_HOST}" -p "${DB_PORT}" -d "${DB_NAME}" -c "
SELECT
    schemaname,
    tablename
FROM pg_tables
WHERE schemaname = 'public'
ORDER BY tablename;
" 2>/dev/null || echo "テーブル一覧の取得に失敗しました"

unset PGPASSWORD
