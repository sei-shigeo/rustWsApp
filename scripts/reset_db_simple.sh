#!/bin/bash
# シンプルなデータベースリセットスクリプト
# 警告: このスクリプトはすべてのデータを削除します！

set -e

# カラー定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

DB_URL="postgres://postgres:postgres@localhost:5432/rustWsApp"

echo -e "${YELLOW}========================================${NC}"
echo -e "${YELLOW}データベースリセット (簡易版)${NC}"
echo -e "${YELLOW}========================================${NC}"
echo ""
echo -e "${RED}警告: このスクリプトはデータベースを完全に削除して再作成します！${NC}"
echo ""
read -p "続行しますか？ (yes/no): " -r
echo
if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
    echo -e "${YELLOW}キャンセルされました${NC}"
    exit 0
fi

echo ""
echo -e "${BLUE}[1/3] 既存のデータベースを削除...${NC}"

# 強制的にデータベースを削除
sqlx database drop -y --database-url "${DB_URL}" 2>&1 | grep -v "error:" || true

echo -e "${GREEN}✓ 削除完了${NC}"
echo ""

echo -e "${BLUE}[2/3] 新しいデータベースを作成...${NC}"

sqlx database create --database-url "${DB_URL}"

echo -e "${GREEN}✓ 作成完了${NC}"
echo ""

echo -e "${BLUE}[3/3] マイグレーションを実行...${NC}"

sqlx migrate run --database-url "${DB_URL}"

echo -e "${GREEN}✓ マイグレーション完了${NC}"
echo ""

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}データベースリセット完了！${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "接続URL: ${DB_URL}"
echo ""
echo "確認コマンド:"
echo "  sqlx migrate info"
echo ""
