original_dir=$(pwd)
function return_to_original_dir {
    cd "$original_dir"
}
trap return_to_original_dir EXIT
cd "$(git rev-parse --show-toplevel)"

CONTAINER_NAME=test-database

if [ "$(docker ps -q -f name=$CONTAINER_NAME)" ]; then
    echo "Container $CONTAINER_NAME is already running."
else 
    docker compose --env-file .env.test -f docker-compose-test.yml up -d
    sleep 2
fi

psql postgresql://helios:123456789@localhost:5555/heliosdb -f migrations/00000000000000_diesel_initial_setup/up.sql > /dev/null
psql postgresql://helios:123456789@localhost:5555/heliosdb -f migrations/2024-08-02-114025_initial/up.sql > /dev/null
psql postgresql://helios:123456789@localhost:5555/heliosdb -f src/tests/e2e/sql/seed.sql > /dev/null

cargo test e2e

psql postgresql://helios:123456789@localhost:5555/heliosdb -f src/tests/e2e/sql/dump.sql > /dev/null

cd "$original_dir"
