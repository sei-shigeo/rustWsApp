-- データベースリセットスクリプト
-- 注意: このスクリプトはすべてのデータを削除します！

-- アクティブな接続を切断
SELECT pg_terminate_backend(pg_stat_activity.pid)
FROM pg_stat_activity
WHERE pg_stat_activity.datname = 'rustWsApp'
  AND pid <> pg_backend_pid();

-- データベースを削除
DROP DATABASE IF EXISTS rustWsApp;

-- データベースを作成
CREATE DATABASE rustWsApp;
