# db

## memo

`pg_ctl -D /opt/homebrew/var/postgresql@14 start`

`CREATE USER new_user WITH PASSWORD 'password123';`

`CREATE DATABASE new_database;`

`GRANT ALL PRIVILEGES ON DATABASE new_database TO new_user;`

```sql
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO new_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO new_user;
```

## ChatGPT GPT-4

- 下記の SQL を参考にして、mermaid で ER 図を書いてください。

```sql
create_table.sql をコピーして貼る
```

- 下記の SQL を参考にして、Rust の sqlx で各テーブルのデータを取得するコードを書いてください。

```sql
create_table.sql をコピーして貼る
```
