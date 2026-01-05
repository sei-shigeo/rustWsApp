#!/bin/bash
# マイグレーション構造検証スクリプト

set -e

MIGRATIONS_DIR="migrations"
COLOR_GREEN='\033[0;32m'
COLOR_RED='\033[0;31m'
COLOR_YELLOW='\033[1;33m'
COLOR_BLUE='\033[0;34m'
COLOR_RESET='\033[0m'

echo -e "${COLOR_BLUE}========================================${COLOR_RESET}"
echo -e "${COLOR_BLUE}マイグレーションファイル構造検証${COLOR_RESET}"
echo -e "${COLOR_BLUE}========================================${COLOR_RESET}"
echo ""

# マイグレーションファイルのリスト
MIGRATIONS=(
    "20260103052523_company_master"
    "20260103052524_master_tables"
    "20260103052525_clients"
    "20260103052526_employees"
    "20260103052527_licenses_qualifications"
    "20260103052528_vehicles"
)

# ファイルの存在確認
echo -e "${COLOR_YELLOW}[1/4] ファイル存在確認...${COLOR_RESET}"
MISSING_FILES=0

for migration in "${MIGRATIONS[@]}"; do
    UP_FILE="${MIGRATIONS_DIR}/${migration}.up.sql"
    DOWN_FILE="${MIGRATIONS_DIR}/${migration}.down.sql"

    if [ -f "$UP_FILE" ]; then
        echo -e "  ${COLOR_GREEN}✓${COLOR_RESET} ${UP_FILE}"
    else
        echo -e "  ${COLOR_RED}✗${COLOR_RESET} ${UP_FILE} が見つかりません"
        MISSING_FILES=$((MISSING_FILES + 1))
    fi

    if [ -f "$DOWN_FILE" ]; then
        echo -e "  ${COLOR_GREEN}✓${COLOR_RESET} ${DOWN_FILE}"
    else
        echo -e "  ${COLOR_RED}✗${COLOR_RESET} ${DOWN_FILE} が見つかりません"
        MISSING_FILES=$((MISSING_FILES + 1))
    fi
done

if [ $MISSING_FILES -eq 0 ]; then
    echo -e "${COLOR_GREEN}すべてのマイグレーションファイルが存在します${COLOR_RESET}"
else
    echo -e "${COLOR_RED}${MISSING_FILES} 個のファイルが見つかりません${COLOR_RESET}"
    exit 1
fi

echo ""

# ファイルサイズ確認
echo -e "${COLOR_YELLOW}[2/4] ファイルサイズ確認...${COLOR_RESET}"
for migration in "${MIGRATIONS[@]}"; do
    UP_FILE="${MIGRATIONS_DIR}/${migration}.up.sql"
    if [ -f "$UP_FILE" ]; then
        SIZE=$(wc -c < "$UP_FILE")
        LINES=$(wc -l < "$UP_FILE")
        echo -e "  ${migration}.up.sql: ${SIZE} bytes, ${LINES} lines"
    fi
done

echo ""

# SQL構文の基本チェック
echo -e "${COLOR_YELLOW}[3/4] SQL構文基本チェック...${COLOR_RESET}"
SYNTAX_ERRORS=0

for migration in "${MIGRATIONS[@]}"; do
    UP_FILE="${MIGRATIONS_DIR}/${migration}.up.sql"

    if [ -f "$UP_FILE" ]; then
        # CREATE TABLEの存在確認
        CREATE_COUNT=$(grep -c "CREATE TABLE" "$UP_FILE" || true)
        echo -e "  ${migration}: ${CREATE_COUNT} テーブル定義"

        # DROP TABLEの存在確認（up.sqlには含まれないはず）
        DROP_COUNT=$(grep -c "DROP TABLE" "$UP_FILE" || true)
        if [ $DROP_COUNT -gt 0 ]; then
            echo -e "  ${COLOR_RED}⚠ ${migration}.up.sql に DROP TABLE が含まれています${COLOR_RESET}"
            SYNTAX_ERRORS=$((SYNTAX_ERRORS + 1))
        fi
    fi
done

if [ $SYNTAX_ERRORS -eq 0 ]; then
    echo -e "${COLOR_GREEN}基本的なSQL構文チェックに合格しました${COLOR_RESET}"
else
    echo -e "${COLOR_YELLOW}${SYNTAX_ERRORS} 個の警告があります${COLOR_RESET}"
fi

echo ""

# マイグレーション順序の確認
echo -e "${COLOR_YELLOW}[4/4] マイグレーション順序確認...${COLOR_RESET}"
echo -e "  実行順序:"
for i in "${!MIGRATIONS[@]}"; do
    echo -e "    $((i + 1)). ${MIGRATIONS[$i]}"
done

echo ""

# テーブル依存関係の概要
echo -e "${COLOR_BLUE}========================================${COLOR_RESET}"
echo -e "${COLOR_BLUE}テーブル依存関係の概要${COLOR_RESET}"
echo -e "${COLOR_BLUE}========================================${COLOR_RESET}"
echo ""
echo "1. 会社・組織マスター (company_master)"
echo "   └─ companies, offices, departments, positions, nationalities"
echo ""
echo "2. 従業員関連マスター (master_tables)"
echo "   └─ 各種マスターテーブル (types)"
echo ""
echo "3. 取引先管理 (clients)"
echo "   └─ clients, client_offices, client_contacts"
echo "   └─ 依存: companies"
echo ""
echo "4. 従業員基本情報 (employees)"
echo "   └─ employees, addresses, bank_accounts, etc."
echo "   └─ 依存: companies, offices, departments, nationalities"
echo ""
echo "5. 免許・資格・健康診断 (licenses_qualifications)"
echo "   └─ licenses, qualifications, health_checkup, etc."
echo "   └─ 依存: employees, 各種マスターテーブル"
echo ""
echo "6. 車両管理 (vehicles)"
echo "   └─ vehicles, inspection_history, repair_history"
echo "   └─ 依存: companies, offices, clients"
echo ""

# 完了メッセージ
echo -e "${COLOR_BLUE}========================================${COLOR_RESET}"
echo -e "${COLOR_GREEN}検証完了！${COLOR_RESET}"
echo -e "${COLOR_BLUE}========================================${COLOR_RESET}"
echo ""
echo "マイグレーションを実行するには:"
echo "  sqlx migrate run"
echo ""
echo "ロールバックするには:"
echo "  sqlx migrate revert"
echo ""
