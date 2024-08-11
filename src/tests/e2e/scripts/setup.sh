original_dir=$(pwd)
function return_to_original_dir {
    cd "$original_dir"
}
trap return_to_original_dir EXIT
cd "$(git rev-parse --show-toplevel)"

docker compose -f docker-compose-test.yml down

docker compose --env-file .env.test -f docker-compose-test.yml up -d
sleep 2

diesel migration run --database-url "postgres://helios:123456789@localhost:5555/heliosdb"
psql postgresql://helios:123456789@localhost:5555/heliosdb -f src/tests/e2e/sql/seed.sql

cd "$original_dir"
