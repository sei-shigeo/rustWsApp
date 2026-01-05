# マイグレーションファイル構造

このディレクトリには、データベーススキーマを管理するためのマイグレーションファイルが含まれています。

## マイグレーションファイル一覧

マイグレーションは機能ごとに分割されており、以下の順序で実行されます：

### 1. 会社・組織マスター (20260103052523)
**ファイル**: `20260103052523_company_master.up.sql`

- `companies` - 会社マスタ
- `company_bank_accounts` - 会社銀行口座
- `nationalities` - 国籍マスタ
- `offices` - 営業所マスタ
- `departments` - 部署マスタ
- `positions` - 役職マスタ

### 2. 従業員関連マスター (20260103052524)
**ファイル**: `20260103052524_master_tables.up.sql`

- `residence_card_types` - 在留カード種別マスタ
- `license_types` - 免許種別マスタ
- `qualification_types` - 資格種別マスタ
- `insurance_types` - 保険種別マスタ
- `guidance_education_types` - 指導教育種別マスタ
- `aptitude_checkup_types` - 適性診断種別マスタ
- `health_checkup_types` - 健康診断種別マスタ

### 3. 取引先管理 (20260103052525)
**ファイル**: `20260103052525_clients.up.sql`

- `client_types` - 取引先種別マスタ
- `clients` - 取引先マスタ
- `client_offices` - 取引先営業所
- `client_contacts` - 取引先担当者

### 4. 従業員基本情報 (20260103052526)
**ファイル**: `20260103052526_employees.up.sql`

- `employees` - 従業員基本情報
- `residence_cards` - 在留カード情報
- `emergency_contacts` - 緊急連絡先
- `addresses` - 住所履歴
- `employment_history` - 職歴
- `education_history` - 学歴
- `employee_bank_accounts` - 従業員銀行口座
- `department_position_history` - 部署・役職・営業所履歴

### 5. 免許・資格・健康診断 (20260103052527)
**ファイル**: `20260103052527_licenses_qualifications.up.sql`

- `licenses` - 運転免許証
- `qualifications` - 資格証
- `insurance_history` - 保険証履歴
- `health_checkup_history` - 健康診断履歴
- `aptitude_checkup_history` - 適性診断履歴
- `guidance_education_history` - 指導教育履歴
- `document_types` - 書類種別マスタ
- `employee_documents` - 従業員書類

### 6. 車両管理 (20260103052528)
**ファイル**: `20260103052528_vehicles.up.sql`

- `vehicle_manufacturers` - 車両メーカーマスタ
- `vehicle_types` - 車両種別マスタ
- `vehicle_ownership_types` - 車両所有者種別マスタ
- `vehicles` - 車両マスタ
- `vehicle_inspection_history` - 車両点検履歴
- `vehicle_repair_history` - 車両修理履歴

## マイグレーションの実行

### アップマイグレーション（適用）

```bash
# すべてのマイグレーションを実行
sqlx migrate run

# 特定のバージョンまで実行
sqlx migrate run --target-version 20260103052526
```

### ダウンマイグレーション（ロールバック）

```bash
# 最後のマイグレーションをロールバック
sqlx migrate revert

# すべてのマイグレーションをロールバック
sqlx migrate revert --all
```

## 設計原則

### 1. テーブル設計
- **タイムスタンプ**: すべてのテーブルに `created_at` と `updated_at` を含む
- **論理削除**: 重要なデータには `is_active` フラグを使用
- **外部キー制約**: データ整合性を保証するために適切に設定
- **CHECK制約**: 日付の整合性や列挙値を検証

### 2. 命名規則
- **テーブル名**: 複数形、スネークケース（例: `employees`, `vehicle_types`）
- **カラム名**: スネークケース（例: `first_name`, `created_at`）
- **インデックス名**: `idx_<テーブル名>_<カラム名>` の形式

### 3. インデックス戦略
- **主キー**: 自動的にインデックスが作成される
- **外部キー**: 検索性能向上のためインデックスを作成
- **有効期限**: `WHERE is_active = TRUE` の条件付き部分インデックス
- **複合検索**: よく使用される検索条件に対して複合インデックスを作成

### 4. データ型の使用
- **ID**: `SERIAL` (自動増分整数)
- **日付**: `DATE` (日付のみ)、`TIMESTAMPTZ` (タイムスタンプ、タイムゾーン付き)
- **金額**: `DECIMAL(精度, 桁数)` (浮動小数点の誤差を避ける)
- **真偽値**: `BOOLEAN`
- **文字列**: `VARCHAR(長さ)` (固定長が不要な場合)、`TEXT` (長い文章)

## 注意事項

### マイグレーションの追加
新しいマイグレーションを追加する場合：

1. **タイムスタンプの採番**: 既存のマイグレーションより大きい番号を使用
2. **依存関係の確認**: 外部キー参照先のテーブルが先に作成されることを確認
3. **ロールバック用ファイル**: 必ず `.down.sql` ファイルも作成

### マイグレーションの変更
**既にデプロイされたマイグレーションは変更しないでください。**

変更が必要な場合は、新しいマイグレーションファイルを作成してください。

```bash
# 例: カラム追加のマイグレーション
sqlx migrate add add_employee_nickname
```

### データベースの初期化

```bash
# データベースを作成
sqlx database create

# マイグレーションを実行
sqlx migrate run

# データベースを削除（開発環境のみ）
sqlx database drop
```

## トラブルシューティング

### マイグレーションエラーが発生した場合

1. エラーメッセージを確認
2. 依存関係の順序を確認
3. データ型の互換性を確認
4. 既存データとの整合性を確認

### マイグレーション履歴の確認

```sql
SELECT * FROM _sqlx_migrations ORDER BY version;
```

## レガシーファイル

以下のファイルは参考用として保持されていますが、使用しないでください：

- `20260103052522_employees_table.up.sql.backup` - 分割前の元ファイル

## 関連ドキュメント

- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)