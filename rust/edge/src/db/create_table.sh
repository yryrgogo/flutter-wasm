#!/bin/bash

# create_table.sql ファイルを読み込む
SQL=$(cat create_table.sql)

# psql コマンドを使ってテーブルを作成する
echo "${SQL}" | psql -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" -d "${PGDATABASE}"
