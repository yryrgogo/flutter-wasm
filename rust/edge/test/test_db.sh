#!/bin/bash

# docker-compose down まで流す
# set -euo pipefail

cd ./test || exit 1

# Start the PostgreSQL container
docker-compose up -d postgres

echo "waiting for message queue..."

is_healthy() {
	echo "waiting for $1 to be healthy..."
	service="$1"
	container_id="$(docker-compose ps -q "$service")"
	state_status="$(docker inspect -f "{{.State.Status}}" "$container_id")"
	echo "state status: $state_status"

	if [ "$state_status" = "running" ]; then
		return 0
	else
		return 1
	fi
}

while ! is_healthy postgres; do sleep 1; done

echo "starting ingest manager"

export PGHOST="localhost"
export PGUSER="postgres"
export PGPASSWORD="postgres"
export PGDATABASE="test_db"
export PGPORT="5433"

docker-compose ps

# Create the test database and tables
docker-compose exec postgres psql -U $PGUSER -d $PGDATABASE -f docker-entrypoint-initdb.d/init/create_table.sql

# Create test_user
export TEST_USER="test_user"
export TEST_PASSWORD="postgres"
docker-compose exec postgres psql -U $PGUSER -d $PGDATABASE -c "CREATE USER $TEST_USER WITH LOGIN PASSWORD '$TEST_PASSWORD';
GRANT ALL PRIVILEGES ON DATABASE $PGDATABASE TO $TEST_USER;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO $TEST_USER;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO $TEST_USER;"
export PGUSER="test_user"
echo "Test database and tables and role created successfully"

cd ..

cargo test

cd ./test || return

docker-compose down
