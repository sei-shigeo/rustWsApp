#!/bin/bash
# マイグレーション履歴クリーンアップスクリプト
# 古いマイグレーション履歴を削除して、新しいマイグレーションを適用できるようにします

set -e

# カラー定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

DB_NAME="rustWsApp"
DB_USER="postgres"
DB_PASSWORD="postgres"
DB_HOST="localhost"
DB_PORT="5432"
DB_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

OLD_MIGRATION="20260103052522"

echo -e "${YELLOW}========================================${NC}"
echo -e "${YELLOW}マイグレーション履歴クリーンアップ${NC}"
echo -e "${YELLOW}========================================${NC}"
echo ""
echo -e "${BLUE}このスクリプトは以下の操作を行います:${NC}"
echo -e "  1. 古いマイグレーション (${OLD_MIGRATION}) を履歴から削除"
echo -e "  2. 新しいマイグレーションを実行"
echo ""
echo -e "${RED}注意: データは削除されません（スキーマのみ更新）${NC}"
echo ""
read -p "続行しますか？ (yes/no): " -r
echo
if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
    echo -e "${YELLOW}キャンセルされました${NC}"
    exit 0
fi

echo ""
echo -e "${BLUE}[1/4] 現在のマイグレーション履歴を確認...${NC}"

# SQLiteまたはPostgreSQLのどちらかを使用
if command -v psql &> /dev/null; then
    export PGPASSWORD="${DB_PASSWORD}"
    echo ""
    echo "現在のマイグレーション履歴:"
    psql -U "${DB_USER}" -h "${DB_HOST}" -p "${DB_PORT}" -d "${DB_NAME}" -c "
    SELECT version, description, installed_on
    FROM _sqlx_migrations
    ORDER BY version;
    " 2>/dev/null || echo "  マイグレーションテーブルが見つかりません"
    unset PGPASSWORD
else
    echo -e "${YELLOW}  psql が見つかりません。スキップします${NC}"
fi

echo ""
echo -e "${BLUE}[2/4] 古いマイグレーション履歴を削除...${NC}"

if command -v psql &> /dev/null; then
    export PGPASSWORD="${DB_PASSWORD}"

    # 古いマイグレーションを削除
    psql -U "${DB_USER}" -h "${DB_HOST}" -p "${DB_PORT}" -d "${DB_NAME}" -c "
    DELETE FROM _sqlx_migrations
    WHERE version = ${OLD_MIGRATION};
    " 2>/dev/null || echo "  削除に失敗しました（レコードが存在しない可能性があります）"

    echo -e "${GREEN}✓ 古いマイグレーション履歴を削除しました${NC}"
    unset PGPASSWORD
else
    echo -e "${YELLOW}  手動で削除してください:${NC}"
    echo -e "  ${BLUE}DELETE FROM _sqlx_migrations WHERE version = ${OLD_MIGRATION};${NC}"
    echo ""
    read -p "手動で削除しましたか？ (yes/no): " -r
    echo
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        echo -e "${RED}処理を中断します${NC}"
        exit 1
    fi
fi

echo ""
echo -e "${BLUE}[3/4] マイグレーション状態を確認...${NC}"

sqlx migrate info --database-url "${DB_URL}" || true

echo ""
echo -e "${BLUE}[4/4] 新しいマイグレーションを実行...${NC}"

sqlx migrate run --database-url "${DB_URL}"

echo -e "${GREEN}✓ マイグレーション完了${NC}"
echo ""

# 最終確認
if command -v psql &> /dev/null; then
    echo -e "${BLUE}新しいマイグレーション履歴:${NC}"
    export PGPASSWORD="${DB_PASSWORD}"
    psql -U "${DB_USER}" -h "${DB_HOST}" -p "${DB_PORT}" -d "${DB_NAME}" -c "
    SELECT version, description, installed_on
    FROM _sqlx_migrations
    ORDER BY version;
    " 2>/dev/null || echo "  マイグレーション履歴の取得に失敗しました"
    unset PGPASSWORD
fi

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}クリーンアップ完了！${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "確認コマンド:"
echo "  sqlx migrate info"
echo ""
