#!/bin/bash

echo "This script will drop all tables in the database."
exit 1

# create_table.sql ファイルを読み込む
SQL=$(cat drop_tables.sql)

# psql コマンドを使ってテーブルを作成する
echo "${SQL}" | psql -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" -d "${PGDATABASE}"
