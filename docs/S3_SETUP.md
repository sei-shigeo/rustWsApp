# AWS S3セットアップガイド

このドキュメントでは、従業員ドキュメント管理機能で使用するAWS S3の設定方法を説明します。

## 目次

1. [AWS S3バケットの作成](#aws-s3バケットの作成)
2. [IAMユーザーとアクセスキーの作成](#iamユーザーとアクセスキーの作成)
3. [環境変数の設定](#環境変数の設定)
4. [ローカル開発環境の設定](#ローカル開発環境の設定)
5. [料金について](#料金について)

## AWS S3バケットの作成

### 1. AWSマネジメントコンソールにログイン

[AWS Console](https://console.aws.amazon.com/) にアクセスしてログインします。

### 2. S3サービスを開く

- サービス検索から「S3」を検索
- S3ダッシュボードを開く

### 3. バケットを作成

1. 「バケットを作成」ボタンをクリック
2. 以下の設定を行う：

```
バケット名: rust-ws-app-documents（ユニークな名前を指定）
リージョン: ap-northeast-1（東京）
```

3. **パブリックアクセスのブロック設定**
   - ⚠️ セキュリティのため、すべてのパブリックアクセスをブロック
   - 署名付きURLでアクセスする場合はこの設定が推奨

4. **バージョニング**（オプション）
   - 有効にすると、誤って削除したファイルを復元可能
   - ストレージコストが増加するので注意

5. **暗号化**
   - デフォルトの暗号化を有効化（SSE-S3）を推奨

6. 「バケットを作成」ボタンをクリック

## IAMユーザーとアクセスキーの作成

### 1. IAMサービスを開く

- サービス検索から「IAM」を検索
- IAMダッシュボードを開く

### 2. 新しいIAMユーザーを作成

1. 左メニューから「ユーザー」を選択
2. 「ユーザーを追加」ボタンをクリック
3. ユーザー名: `rust-ws-app-s3-user`
4. 「プログラムによるアクセス」にチェック

### 3. アクセス許可の設定

「既存のポリシーを直接アタッチ」を選択し、以下のカスタムポリシーを作成：

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "s3:PutObject",
        "s3:GetObject",
        "s3:DeleteObject",
        "s3:ListBucket"
      ],
      "Resource": [
        "arn:aws:s3:::rust-ws-app-documents",
        "arn:aws:s3:::rust-ws-app-documents/*"
      ]
    }
  ]
}
```

### 4. アクセスキーの取得

1. ユーザー作成完了後、**アクセスキーID**と**シークレットアクセスキー**が表示される
2. ⚠️ この情報は一度しか表示されないので、必ず保存する
3. `.csv` ファイルとしてダウンロード可能

## 環境変数の設定

### `.env` ファイルの作成

プロジェクトルートに `.env` ファイルを作成し、以下の内容を追加：

```env
# データベース接続
DATABASE_URL=postgresql://username:password@localhost/rust_ws_app

# AWS S3設定
AWS_REGION=ap-northeast-1
AWS_ACCESS_KEY_ID=YOUR_ACCESS_KEY_ID
AWS_SECRET_ACCESS_KEY=YOUR_SECRET_ACCESS_KEY
S3_BUCKET_NAME=rust-ws-app-documents
```

### 環境変数の説明

| 変数名 | 説明 | 例 |
|--------|------|-----|
| `AWS_REGION` | S3バケットのリージョン | `ap-northeast-1` |
| `AWS_ACCESS_KEY_ID` | IAMユーザーのアクセスキーID | `AKIAIOSFODNN7EXAMPLE` |
| `AWS_SECRET_ACCESS_KEY` | IAMユーザーのシークレットアクセスキー | `wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY` |
| `S3_BUCKET_NAME` | 作成したS3バケット名 | `rust-ws-app-documents` |

### セキュリティ上の注意

⚠️ **重要**: `.env` ファイルは絶対にGitにコミットしないでください！

`.gitignore` に以下を追加済みであることを確認：

```
.env
.env.local
```

## ローカル開発環境の設定

### オプション1: 実際のAWS S3を使用

上記の手順で設定した環境変数を使用します。

**メリット**:
- 本番環境と同じ環境でテスト可能
- 追加のセットアップ不要

**デメリット**:
- 通信が発生するため若干遅い
- AWS料金が発生（無料枠内であれば問題なし）

### オプション2: MinIO（ローカルS3互換サーバー）

Dockerを使ってローカルでS3互換サーバーを起動：

```bash
# MinIOコンテナを起動
docker run -p 9000:9000 -p 9001:9001 \
  -e MINIO_ROOT_USER=minioadmin \
  -e MINIO_ROOT_PASSWORD=minioadmin \
  minio/minio server /data --console-address ":9001"
```

`.env` ファイルをMinIO用に変更：

```env
AWS_REGION=us-east-1
AWS_ACCESS_KEY_ID=minioadmin
AWS_SECRET_ACCESS_KEY=minioadmin
S3_BUCKET_NAME=rust-ws-app-documents
AWS_ENDPOINT_URL=http://localhost:9000
```

**メリット**:
- 完全無料
- 高速（ローカルなので）
- オフラインで開発可能

**デメリット**:
- 初期セットアップが必要
- 本番環境と若干の差異がある可能性

### バケットの作成（MinIOの場合）

MinIOコンソール（http://localhost:9001）にアクセス：
1. ユーザー名: `minioadmin`、パスワード: `minioadmin` でログイン
2. 「Buckets」→「Create Bucket」
3. バケット名: `rust-ws-app-documents`

## 料金について

### AWS S3無料枠（12ヶ月間）

- **ストレージ**: 5GB まで無料
- **PUTリクエスト**: 2,000回/月 まで無料
- **GETリクエスト**: 20,000回/月 まで無料
- **データ転送（アウト）**: 15GB/月 まで無料

### 無料枠超過後の料金（東京リージョン）

| 項目 | 料金 |
|------|------|
| ストレージ（Standard） | $0.025/GB/月（約3円/GB） |
| PUTリクエスト | $0.0047/1000回 |
| GETリクエスト | $0.00037/1000回 |
| データ転送（アウト） | 最初の1GB無料、その後$0.114/GB |

### 運送会社の使用例

従業員50人、各従業員に以下のドキュメントがある場合：

```
- 顔写真: 500KB × 50人 = 25MB
- 免許証（表裏）: 1MB × 2枚 × 50人 = 100MB
- 資格証: 500KB × 3枚 × 50人 = 75MB
- 車検証: 2MB × 20台 = 40MB

合計: 約240MB → 無料枠5GB内で十分
```

月間コスト: **0円**（無料枠内）

### コスト削減のヒント

1. **画像の最適化**
   - アップロード前に画像を圧縮（例: 1920x1080 → 1280x720）
   - JPEG品質を80%程度に設定

2. **古いファイルのアーカイブ**
   - S3 Glacierに移行（ストレージコストが1/10に）
   - ライフサイクルポリシーで自動化

3. **不要なファイルの削除**
   - 定期的に不要なドキュメントを削除
   - 重複ファイルのチェック

## トラブルシューティング

### アクセス拒否エラー

```
S3 Error: Access Denied
```

**原因**: IAMユーザーの権限が不足

**解決策**: IAMポリシーを確認し、必要な権限が付与されているか確認

### バケットが見つからない

```
S3 Error: NoSuchBucket
```

**原因**: バケット名またはリージョンが間違っている

**解決策**: `.env` ファイルの `S3_BUCKET_NAME` と `AWS_REGION` を確認

### 認証エラー

```
S3 Error: InvalidAccessKeyId
```

**原因**: アクセスキーが間違っているか無効

**解決策**: `.env` ファイルの `AWS_ACCESS_KEY_ID` と `AWS_SECRET_ACCESS_KEY` を確認

## 次のステップ

1. [ドキュメント管理機能の使い方](./DOCUMENT_MANAGEMENT.md)
2. [セキュリティベストプラクティス](./SECURITY.md)
3. [バックアップとリカバリ](./BACKUP.md)

## 参考リンク

- [AWS S3公式ドキュメント](https://docs.aws.amazon.com/s3/)
- [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust)
- [MinIO Documentation](https://min.io/docs/minio/linux/index.html)
